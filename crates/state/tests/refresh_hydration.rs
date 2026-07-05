use rs_dean_idb::{Database, DatabaseConfig};
use rs_dean_schema::{AppSnapshot, Theme};
use rs_dean_state::{HydratedState, SNAPSHOTS_STORE, hydrate_from_database, persist_snapshot};

#[cfg(not(target_arch = "wasm32"))]
use rs_dean_state::database_config_with_native_path;

#[cfg(target_arch = "wasm32")]
use rs_dean_idb::StoreSpec;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone, Debug, Eq, PartialEq)]
struct LeptosSignalCache {
    snapshot: AppSnapshot,
}

impl LeptosSignalCache {
    fn hydrate(state: &HydratedState) -> Self {
        Self {
            snapshot: state.snapshot.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BevyResourceCache {
    snapshot: AppSnapshot,
}

impl BevyResourceCache {
    fn hydrate(state: &HydratedState) -> Self {
        Self {
            snapshot: state.snapshot.clone(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_refresh_rehydrates_leptos_and_bevy_caches_from_durable_state() {
    let path = std::env::temp_dir().join(format!(
        "rs-dean-refresh-hydration-{}.redb",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);

    poll(async {
        assert_refresh_hydration(database_config_with_native_path(&path)).await;
    });

    let _ = std::fs::remove_file(path);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn browser_refresh_rehydrates_leptos_and_bevy_caches_from_indexed_db() {
    assert_refresh_hydration(
        DatabaseConfig::new("rs-dean-browser-refresh-hydration-test", 1)
            .with_store(StoreSpec::new(SNAPSHOTS_STORE)),
    )
    .await;
}

async fn assert_refresh_hydration(config: DatabaseConfig) {
    let expected = resumed_snapshot();
    let database = Database::open(config.clone())
        .await
        .expect("open durable state database");
    database
        .clear(SNAPSHOTS_STORE)
        .await
        .expect("clear stale test records");
    persist_snapshot(&database, &expected)
        .await
        .expect("persist snapshot before refresh");

    let first_boot = hydrate_from_database(&database)
        .await
        .expect("hydrate before refresh");
    assert_framework_caches_resume(&first_boot, &expected);
    database.close();

    let reopened = Database::open(config)
        .await
        .expect("reopen durable state database after refresh");
    let after_refresh = hydrate_from_database(&reopened)
        .await
        .expect("hydrate after refresh");
    assert_framework_caches_resume(&after_refresh, &expected);
    reopened
        .clear(SNAPSHOTS_STORE)
        .await
        .expect("clear test records after refresh proof");
}

fn assert_framework_caches_resume(hydrated: &HydratedState, expected: &AppSnapshot) {
    let leptos_cache = LeptosSignalCache::hydrate(hydrated);
    let bevy_cache = BevyResourceCache::hydrate(hydrated);

    assert_eq!(leptos_cache.snapshot, *expected);
    assert_eq!(bevy_cache.snapshot, *expected);
    assert_eq!(leptos_cache.snapshot, bevy_cache.snapshot);
}

fn resumed_snapshot() -> AppSnapshot {
    let mut snapshot = AppSnapshot::default();
    snapshot.settings.theme = Theme::Dark;
    snapshot.settings.reduced_motion = true;
    snapshot.progress[0].level = 4;
    snapshot.progress[0].completed = true;
    snapshot
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
