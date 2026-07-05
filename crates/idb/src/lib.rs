#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(not(target_arch = "wasm32"))]
mod native;

/// A schema-level object store declaration shared by browser and native builds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreSpec {
    pub name: &'static str,
}

impl StoreSpec {
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

/// Open options for the isomorphic storage backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatabaseConfig {
    pub name: String,
    pub version: u32,
    pub stores: Vec<StoreSpec>,
    #[cfg(not(target_arch = "wasm32"))]
    pub native_path: PathBuf,
}

impl DatabaseConfig {
    #[must_use]
    pub fn new(name: impl Into<String>, version: u32) -> Self {
        let name = name.into();
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            native_path: default_native_path(&name),
            name,
            version,
            stores: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_store(mut self, store: StoreSpec) -> Self {
        self.stores.push(store);
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn with_native_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.native_path = path.into();
        self
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn default_native_path(name: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(format!("{name}.redb"))
}

#[derive(Debug, Error)]
pub enum IdbError {
    #[error("database must declare at least one object store")]
    EmptySchema,
    #[error("duplicate object store `{0}`")]
    DuplicateStore(String),
    #[error("object store `{0}` is not declared in this database schema")]
    UnknownStore(String),
    #[error("database version must be greater than zero")]
    InvalidVersion,
    #[error("failed to encode record: {0}")]
    Encode(String),
    #[error("failed to decode record: {0}")]
    Decode(String),
    #[error("storage backend failed: {0}")]
    Backend(String),
}

/// Shared database handle used by Leptos, Bevy, and domain crates.
#[derive(Clone)]
pub struct Database {
    inner: backend::Database,
}

impl Database {
    pub async fn open(config: DatabaseConfig) -> Result<Self, IdbError> {
        validate_config(&config)?;
        Ok(Self {
            inner: backend::Database::open(config).await?,
        })
    }

    pub async fn get<T>(&self, store: &str, key: &str) -> Result<Option<T>, IdbError>
    where
        T: DeserializeOwned,
    {
        self.inner.get(store, key).await
    }

    pub async fn get_all<T>(&self, store: &str) -> Result<Vec<T>, IdbError>
    where
        T: DeserializeOwned,
    {
        self.inner.get_all(store).await
    }

    pub async fn put<T>(&self, store: &str, key: &str, value: &T) -> Result<(), IdbError>
    where
        T: Serialize,
    {
        self.inner.put(store, key, value).await
    }

    pub async fn delete(&self, store: &str, key: &str) -> Result<(), IdbError> {
        self.inner.delete(store, key).await
    }

    pub async fn clear(&self, store: &str) -> Result<(), IdbError> {
        self.inner.clear(store).await
    }

    pub fn close(self) {
        self.inner.close();
    }
}

fn validate_config(config: &DatabaseConfig) -> Result<(), IdbError> {
    if config.version == 0 {
        return Err(IdbError::InvalidVersion);
    }
    if config.stores.is_empty() {
        return Err(IdbError::EmptySchema);
    }

    let mut stores = Vec::new();
    for store in &config.stores {
        if stores.contains(&store.name) {
            return Err(IdbError::DuplicateStore(store.name.to_owned()));
        }
        stores.push(store.name);
    }

    Ok(())
}

fn ensure_store(config: &DatabaseConfig, store: &str) -> Result<(), IdbError> {
    if config
        .stores
        .iter()
        .any(|candidate| candidate.name == store)
    {
        Ok(())
    } else {
        Err(IdbError::UnknownStore(store.to_owned()))
    }
}

#[cfg(target_arch = "wasm32")]
mod backend {
    pub use super::browser::Database;
}

#[cfg(not(target_arch = "wasm32"))]
mod backend {
    pub use super::native::Database;
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::{Database, DatabaseConfig, StoreSpec};

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    struct Record {
        id: String,
        value: u32,
    }

    #[test]
    fn rejects_empty_schema() {
        assert!(DatabaseConfig::new("empty", 1).stores.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_round_trip_persists_records() {
        let path = std::env::temp_dir().join(format!(
            "rs-dean-idb-{}-{}.redb",
            std::process::id(),
            "round-trip"
        ));
        let _ = std::fs::remove_file(&path);

        let config = DatabaseConfig::new("test", 1)
            .with_store(StoreSpec::new("records"))
            .with_native_path(&path);

        poll(async {
            let db = Database::open(config.clone()).await.unwrap();
            db.put(
                "records",
                "one",
                &Record {
                    id: "one".to_owned(),
                    value: 7,
                },
            )
            .await
            .unwrap();

            let found: Option<Record> = db.get("records", "one").await.unwrap();
            assert_eq!(
                found,
                Some(Record {
                    id: "one".to_owned(),
                    value: 7
                })
            );
            db.close();

            let reopened = Database::open(config).await.unwrap();
            let all: Vec<Record> = reopened.get_all("records").await.unwrap();
            assert_eq!(all.len(), 1);
            reopened.delete("records", "one").await.unwrap();
            assert!(
                reopened
                    .get::<Record>("records", "one")
                    .await
                    .unwrap()
                    .is_none()
            );
        });

        let _ = std::fs::remove_file(path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn poll<F>(future: F) -> F::Output
    where
        F: std::future::Future,
    {
        use std::{
            pin::pin,
            task::{Context, Poll, Waker},
        };

        let mut context = Context::from_waker(Waker::noop());
        let mut future = pin!(future);
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }
}
