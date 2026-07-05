use std::sync::Arc;

use redb::{Database as RedbDatabase, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Serialize, de::DeserializeOwned};

use crate::{DatabaseConfig, IdbError, ensure_store};

const VERSION_TABLE: TableDefinition<&str, u64> = TableDefinition::new("__rs_dean_meta");
const VERSION_KEY: &str = "version";

#[derive(Clone)]
pub struct Database {
    config: Arc<DatabaseConfig>,
    db: Arc<RedbDatabase>,
}

impl Database {
    pub async fn open(config: DatabaseConfig) -> Result<Self, IdbError> {
        if let Some(parent) = config.native_path.parent() {
            std::fs::create_dir_all(parent).map_err(backend_error)?;
        }

        let db = RedbDatabase::create(&config.native_path).map_err(backend_error)?;
        let database = Self {
            config: Arc::new(config),
            db: Arc::new(db),
        };
        database.migrate_native()?;
        Ok(database)
    }

    pub async fn get<T>(&self, store: &str, key: &str) -> Result<Option<T>, IdbError>
    where
        T: DeserializeOwned,
    {
        let table_definition = table(declared_store(&self.config, store)?);
        let transaction = self.db.begin_read().map_err(backend_error)?;
        let table = match transaction.open_table(table_definition) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
            Err(error) => return Err(backend_error(error)),
        };

        let Some(value) = table.get(key).map_err(backend_error)? else {
            return Ok(None);
        };
        let decoded = serde_json::from_slice(value.value()).map_err(decode_error)?;
        Ok(Some(decoded))
    }

    pub async fn get_all<T>(&self, store: &str) -> Result<Vec<T>, IdbError>
    where
        T: DeserializeOwned,
    {
        let table_definition = table(declared_store(&self.config, store)?);
        let transaction = self.db.begin_read().map_err(backend_error)?;
        let table = match transaction.open_table(table_definition) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(Vec::new()),
            Err(error) => return Err(backend_error(error)),
        };

        let mut records = Vec::new();
        for entry in table.iter().map_err(backend_error)? {
            let (_, value) = entry.map_err(backend_error)?;
            records.push(serde_json::from_slice(value.value()).map_err(decode_error)?);
        }
        Ok(records)
    }

    pub async fn put<T>(&self, store: &str, key: &str, value: &T) -> Result<(), IdbError>
    where
        T: Serialize,
    {
        let table_definition = table(declared_store(&self.config, store)?);
        let encoded = serde_json::to_vec(value).map_err(encode_error)?;
        let transaction = self.db.begin_write().map_err(backend_error)?;
        {
            let mut table = transaction
                .open_table(table_definition)
                .map_err(backend_error)?;
            table
                .insert(key, encoded.as_slice())
                .map_err(backend_error)?;
        }
        transaction.commit().map_err(backend_error)
    }

    pub async fn delete(&self, store: &str, key: &str) -> Result<(), IdbError> {
        let table_definition = table(declared_store(&self.config, store)?);
        let transaction = self.db.begin_write().map_err(backend_error)?;
        {
            let mut table = transaction
                .open_table(table_definition)
                .map_err(backend_error)?;
            table.remove(key).map_err(backend_error)?;
        }
        transaction.commit().map_err(backend_error)
    }

    pub async fn clear(&self, store: &str) -> Result<(), IdbError> {
        let table_definition = table(declared_store(&self.config, store)?);
        let transaction = self.db.begin_write().map_err(backend_error)?;
        {
            let mut table = transaction
                .open_table(table_definition)
                .map_err(backend_error)?;
            table.retain(|_, _| false).map_err(backend_error)?;
        }
        transaction.commit().map_err(backend_error)
    }

    pub fn close(self) {}

    fn migrate_native(&self) -> Result<(), IdbError> {
        let transaction = self.db.begin_write().map_err(backend_error)?;
        {
            let mut versions = transaction
                .open_table(VERSION_TABLE)
                .map_err(backend_error)?;
            versions
                .insert(VERSION_KEY, u64::from(self.config.version))
                .map_err(backend_error)?;
        }
        for store in &self.config.stores {
            let _ = transaction
                .open_table(table(store.name))
                .map_err(backend_error)?;
        }
        transaction.commit().map_err(backend_error)
    }
}

fn declared_store(config: &DatabaseConfig, store: &str) -> Result<&'static str, IdbError> {
    ensure_store(config, store)?;
    config
        .stores
        .iter()
        .find(|candidate| candidate.name == store)
        .map(|candidate| candidate.name)
        .ok_or_else(|| IdbError::UnknownStore(store.to_owned()))
}

fn table(store: &'static str) -> TableDefinition<'static, &'static str, &'static [u8]> {
    TableDefinition::new(store)
}

fn encode_error(error: serde_json::Error) -> IdbError {
    IdbError::Encode(error.to_string())
}

fn decode_error(error: serde_json::Error) -> IdbError {
    IdbError::Decode(error.to_string())
}

fn backend_error(error: impl std::error::Error) -> IdbError {
    IdbError::Backend(error.to_string())
}
