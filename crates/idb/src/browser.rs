use std::{cell::RefCell, rc::Rc};

use futures_channel::oneshot;
use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{
    DomException, Event, IdbDatabase, IdbObjectStore, IdbOpenDbRequest, IdbRequest, IdbTransaction,
    IdbTransactionMode,
};

use crate::{DatabaseConfig, IdbError, ensure_store};

#[derive(Clone)]
pub struct Database {
    config: Rc<DatabaseConfig>,
    db: IdbDatabase,
}

impl Database {
    pub async fn open(config: DatabaseConfig) -> Result<Self, IdbError> {
        let factory = web_sys::window()
            .ok_or_else(|| IdbError::Backend("browser window is unavailable".to_owned()))?
            .indexed_db()
            .map_err(js_error)?
            .ok_or_else(|| IdbError::Backend("IndexedDB is unavailable".to_owned()))?;

        let request = factory
            .open_with_u32(&config.name, config.version)
            .map_err(js_error)?;
        let upgrade = upgrade_handler(&request, config.stores.clone());
        request.set_onupgradeneeded(Some(upgrade.as_ref().unchecked_ref()));

        let request_as_request: IdbRequest = request.clone().unchecked_into();
        let result = request_value(request_as_request).await?;
        request.set_onupgradeneeded(None);
        drop(upgrade);

        let db = result.dyn_into::<IdbDatabase>().map_err(js_error)?;
        Ok(Self {
            config: Rc::new(config),
            db,
        })
    }

    pub async fn get<T>(&self, store: &str, key: &str) -> Result<Option<T>, IdbError>
    where
        T: DeserializeOwned,
    {
        let (_, object_store) = self.object_store(store, IdbTransactionMode::Readonly)?;
        let request = object_store
            .get(&JsValue::from_str(key))
            .map_err(js_error)?;
        let value = request_value(request).await?;
        if value.is_undefined() {
            Ok(None)
        } else {
            serde_wasm_bindgen::from_value(value)
                .map(Some)
                .map_err(decode_error)
        }
    }

    pub async fn get_all<T>(&self, store: &str) -> Result<Vec<T>, IdbError>
    where
        T: DeserializeOwned,
    {
        let (_, object_store) = self.object_store(store, IdbTransactionMode::Readonly)?;
        let request = object_store.get_all().map_err(js_error)?;
        let value = request_value(request).await?;
        serde_wasm_bindgen::from_value(value).map_err(decode_error)
    }

    pub async fn put<T>(&self, store: &str, key: &str, value: &T) -> Result<(), IdbError>
    where
        T: Serialize,
    {
        let (transaction, object_store) =
            self.object_store(store, IdbTransactionMode::Readwrite)?;
        let encoded = serde_wasm_bindgen::to_value(value).map_err(encode_error)?;
        let key = JsValue::from_str(key);
        let request = object_store
            .put_with_key(&encoded, &key)
            .map_err(js_error)?;
        let done = transaction_done(transaction);
        let request_result = request_value(request).await;
        let transaction_result = done.await;
        request_result?;
        transaction_result
    }

    pub async fn delete(&self, store: &str, key: &str) -> Result<(), IdbError> {
        let (transaction, object_store) =
            self.object_store(store, IdbTransactionMode::Readwrite)?;
        let request = object_store
            .delete(&JsValue::from_str(key))
            .map_err(js_error)?;
        let done = transaction_done(transaction);
        let request_result = request_value(request).await;
        let transaction_result = done.await;
        request_result?;
        transaction_result
    }

    pub async fn clear(&self, store: &str) -> Result<(), IdbError> {
        let (transaction, object_store) =
            self.object_store(store, IdbTransactionMode::Readwrite)?;
        let request = object_store.clear().map_err(js_error)?;
        let done = transaction_done(transaction);
        let request_result = request_value(request).await;
        let transaction_result = done.await;
        request_result?;
        transaction_result
    }

    pub fn close(self) {
        self.db.close();
    }

    fn object_store(
        &self,
        store: &str,
        mode: IdbTransactionMode,
    ) -> Result<(IdbTransaction, IdbObjectStore), IdbError> {
        ensure_store(&self.config, store)?;
        let transaction = self
            .db
            .transaction_with_str_and_mode(store, mode)
            .map_err(js_error)?;
        let object_store = transaction.object_store(store).map_err(js_error)?;
        Ok((transaction, object_store))
    }
}

fn upgrade_handler(
    request: &IdbOpenDbRequest,
    stores: Vec<crate::StoreSpec>,
) -> Closure<dyn FnMut(Event)> {
    let request = request.clone();
    Closure::new(move |_| {
        let request_as_request: IdbRequest = request.clone().unchecked_into();
        let Ok(result) = request_as_request.result() else {
            return;
        };
        let Ok(database) = result.dyn_into::<IdbDatabase>() else {
            return;
        };
        let existing = database.object_store_names();
        for store in &stores {
            if !existing.contains(store.name) {
                let _ = database.create_object_store(store.name);
            }
        }
    })
}

async fn request_value(request: IdbRequest) -> Result<JsValue, IdbError> {
    let (sender, receiver) = oneshot::channel();
    let sender = Rc::new(RefCell::new(Some(sender)));

    let success_sender = Rc::clone(&sender);
    let success_request = request.clone();
    let success = Closure::<dyn FnMut(Event)>::new(move |_| {
        if let Some(sender) = success_sender.borrow_mut().take() {
            let _ = sender.send(success_request.result().map_err(js_error));
        }
    });

    let error_sender = Rc::clone(&sender);
    let error_request = request.clone();
    let error = Closure::<dyn FnMut(Event)>::new(move |_| {
        if let Some(sender) = error_sender.borrow_mut().take() {
            let _ = sender.send(Err(request_error(&error_request)));
        }
    });

    request.set_onsuccess(Some(success.as_ref().unchecked_ref()));
    request.set_onerror(Some(error.as_ref().unchecked_ref()));

    let result = receiver
        .await
        .map_err(|_| IdbError::Backend("IndexedDB request callback dropped".to_owned()))?;

    request.set_onsuccess(None);
    request.set_onerror(None);
    drop(success);
    drop(error);

    result
}

fn transaction_done(
    transaction: IdbTransaction,
) -> impl std::future::Future<Output = Result<(), IdbError>> {
    let (sender, receiver) = oneshot::channel();
    let sender = Rc::new(RefCell::new(Some(sender)));

    let complete_sender = Rc::clone(&sender);
    let complete = Closure::<dyn FnMut(Event)>::new(move |_| {
        if let Some(sender) = complete_sender.borrow_mut().take() {
            let _ = sender.send(Ok(()));
        }
    });

    let error_sender = Rc::clone(&sender);
    let error_transaction = transaction.clone();
    let error = Closure::<dyn FnMut(Event)>::new(move |_| {
        if let Some(sender) = error_sender.borrow_mut().take() {
            let _ = sender.send(Err(transaction_error(&error_transaction)));
        }
    });

    let abort_sender = Rc::clone(&sender);
    let abort_transaction = transaction.clone();
    let abort = Closure::<dyn FnMut(Event)>::new(move |_| {
        if let Some(sender) = abort_sender.borrow_mut().take() {
            let _ = sender.send(Err(transaction_error(&abort_transaction)));
        }
    });

    transaction.set_oncomplete(Some(complete.as_ref().unchecked_ref()));
    transaction.set_onerror(Some(error.as_ref().unchecked_ref()));
    transaction.set_onabort(Some(abort.as_ref().unchecked_ref()));

    async move {
        let result = receiver
            .await
            .map_err(|_| IdbError::Backend("IndexedDB transaction callback dropped".to_owned()))?;

        transaction.set_oncomplete(None);
        transaction.set_onerror(None);
        transaction.set_onabort(None);
        drop(complete);
        drop(error);
        drop(abort);

        result
    }
}

fn request_error(request: &IdbRequest) -> IdbError {
    match request.error() {
        Ok(Some(error)) => dom_exception_error(error),
        Ok(None) => IdbError::Backend("IndexedDB request failed".to_owned()),
        Err(error) => js_error(error),
    }
}

fn transaction_error(transaction: &IdbTransaction) -> IdbError {
    transaction.error().map_or_else(
        || IdbError::Backend("IndexedDB transaction failed".to_owned()),
        dom_exception_error,
    )
}

fn dom_exception_error(error: DomException) -> IdbError {
    let name = error.name();
    let message = error.message();
    if message.is_empty() {
        IdbError::Backend(name)
    } else {
        IdbError::Backend(format!("{name}: {message}"))
    }
}

fn encode_error(error: serde_wasm_bindgen::Error) -> IdbError {
    IdbError::Encode(error.to_string())
}

fn decode_error(error: serde_wasm_bindgen::Error) -> IdbError {
    IdbError::Decode(error.to_string())
}

fn js_error(error: JsValue) -> IdbError {
    IdbError::Backend(format!("{error:?}"))
}
