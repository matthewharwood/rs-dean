use garde::Validate;
use rs_dean_idb::{Database, DatabaseConfig, IdbError, StoreSpec};
use rs_dean_schema::AppSnapshot;
use thiserror::Error;

pub const DB_NAME: &str = "rs-dean";
pub const DB_VERSION: u32 = 1;
pub const SNAPSHOTS_STORE: &str = "snapshots";
pub const APP_SNAPSHOT_KEY: &str = "app";

#[derive(Debug, Error)]
pub enum StateError {
    #[error("snapshot validation failed: {0}")]
    Validation(#[from] garde::Report),
    #[error("persistent state failed: {0}")]
    Idb(#[from] IdbError),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HydratedState {
    pub snapshot: AppSnapshot,
}

impl HydratedState {
    pub fn new(snapshot: AppSnapshot) -> Result<Self, StateError> {
        snapshot.validate()?;
        Ok(Self { snapshot })
    }
}

pub async fn hydrate() -> Result<HydratedState, StateError> {
    let database = open_state_database().await?;
    hydrate_from_database(&database).await
}

pub async fn ensure_durable_snapshot() -> Result<HydratedState, StateError> {
    let database = open_state_database().await?;
    ensure_durable_snapshot_in_database(&database).await
}

pub async fn update_snapshot(
    mut update: impl FnMut(&mut AppSnapshot),
) -> Result<HydratedState, StateError> {
    let database = open_state_database().await?;
    update_snapshot_in_database(&database, &mut update).await
}

pub async fn open_state_database() -> Result<Database, StateError> {
    Database::open(database_config()).await.map_err(Into::into)
}

pub fn database_config() -> DatabaseConfig {
    DatabaseConfig::new(DB_NAME, DB_VERSION).with_store(StoreSpec::new(SNAPSHOTS_STORE))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn database_config_with_native_path(path: impl Into<std::path::PathBuf>) -> DatabaseConfig {
    database_config().with_native_path(path)
}

pub async fn hydrate_from_database(database: &Database) -> Result<HydratedState, StateError> {
    let snapshot = database
        .get(SNAPSHOTS_STORE, APP_SNAPSHOT_KEY)
        .await?
        .unwrap_or_default();
    HydratedState::new(snapshot)
}

pub async fn persist_snapshot(
    database: &Database,
    snapshot: &AppSnapshot,
) -> Result<(), StateError> {
    snapshot.validate()?;
    database
        .put(SNAPSHOTS_STORE, APP_SNAPSHOT_KEY, snapshot)
        .await?;
    Ok(())
}

pub async fn ensure_durable_snapshot_in_database(
    database: &Database,
) -> Result<HydratedState, StateError> {
    let hydrated = hydrate_from_database(database).await?;
    persist_snapshot(database, &hydrated.snapshot).await?;
    Ok(hydrated)
}

pub async fn update_snapshot_in_database(
    database: &Database,
    update: &mut impl FnMut(&mut AppSnapshot),
) -> Result<HydratedState, StateError> {
    let mut hydrated = hydrate_from_database(database).await?;
    update(&mut hydrated.snapshot);
    persist_snapshot(database, &hydrated.snapshot).await?;
    Ok(hydrated)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::{
        HydratedState, database_config_with_native_path, ensure_durable_snapshot_in_database,
        hydrate_from_database, persist_snapshot, update_snapshot_in_database,
    };
    use rs_dean_idb::Database;
    use rs_dean_schema::AppSnapshot;

    #[test]
    fn default_state_is_valid() {
        assert!(HydratedState::new(AppSnapshot::default()).is_ok());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_hydration_round_trips_snapshot() {
        let path = std::env::temp_dir().join(format!(
            "rs-dean-state-{}-round-trip.redb",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);

        poll(async {
            let database = Database::open(database_config_with_native_path(&path))
                .await
                .unwrap();
            let mut snapshot = AppSnapshot::default();
            snapshot.progress[0].completed = true;
            persist_snapshot(&database, &snapshot).await.unwrap();

            let hydrated = hydrate_from_database(&database).await.unwrap();
            assert_eq!(hydrated.snapshot, snapshot);
        });

        let _ = std::fs::remove_file(path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_update_mutates_and_persists_snapshot() {
        let path =
            std::env::temp_dir().join(format!("rs-dean-state-{}-update.redb", std::process::id()));
        let _ = std::fs::remove_file(&path);

        poll(async {
            let database = Database::open(database_config_with_native_path(&path))
                .await
                .unwrap();
            let mut mark_complete = |snapshot: &mut AppSnapshot| {
                snapshot.progress[0].level = 3;
                snapshot.progress[0].completed = true;
            };
            let updated = update_snapshot_in_database(&database, &mut mark_complete)
                .await
                .unwrap();
            assert_eq!(updated.snapshot.progress[0].level, 3);
            assert!(updated.snapshot.progress[0].completed);
            database.close();

            let reopened = Database::open(database_config_with_native_path(&path))
                .await
                .unwrap();
            let hydrated = hydrate_from_database(&reopened).await.unwrap();
            assert_eq!(hydrated.snapshot.progress[0].level, 3);
            assert!(hydrated.snapshot.progress[0].completed);
        });

        let _ = std::fs::remove_file(path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_ensure_writes_default_snapshot() {
        let path =
            std::env::temp_dir().join(format!("rs-dean-state-{}-ensure.redb", std::process::id()));
        let _ = std::fs::remove_file(&path);

        poll(async {
            let database = Database::open(database_config_with_native_path(&path))
                .await
                .unwrap();
            let hydrated = ensure_durable_snapshot_in_database(&database)
                .await
                .unwrap();
            assert_eq!(hydrated, HydratedState::default());
            database.close();

            let reopened = Database::open(database_config_with_native_path(&path))
                .await
                .unwrap();
            assert_eq!(
                hydrate_from_database(&reopened).await.unwrap(),
                HydratedState::default()
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
