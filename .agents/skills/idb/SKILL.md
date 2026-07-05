---
name: idb
description: >-
  Isomorphic durable storage for rs-dean. Use when changing crates/idb,
  crates/state, persistence schemas, hydration, native storage behavior, or
  wasm IndexedDB access for Leptos and Bevy flows.
---

# IDB

`rs-dean-idb` is the only durable storage boundary in this workspace. It gives
Leptos, Bevy, and domain crates one async object-store API while selecting the
backend by compilation target:

- wasm browser bundles use IndexedDB.
- native bundles use the native embedded store.

Do not import backend-specific storage crates from app, UI, Bevy, or domain
code. Keep those details inside `crates/idb`.

## API Shape

Use the public wrapper:

```rust
let database = Database::open(
    DatabaseConfig::new("rs-dean", 1).with_store(StoreSpec::new("snapshots")),
)
.await?;

database.put("snapshots", "app", &snapshot).await?;
let snapshot: Option<AppSnapshot> = database.get("snapshots", "app").await?;
let snapshots: Vec<AppSnapshot> = database.get_all("snapshots").await?;
database.delete("snapshots", "app").await?;
database.clear("snapshots").await?;
```

The caller should see `Option<T>`, `Vec<T>`, and `()`. Backend request builders,
browser iterator wrappers, native table handles, and transaction types stay
private.

## Schema Rules

- Declare every object store with `StoreSpec::new`.
- Reject empty schemas, duplicate stores, and version `0`.
- Validate persisted records in `crates/state` or the owning schema crate before
  they enter hydrated application state.
- Treat all persisted records as boundary input. Deserialize with `serde`, then
  validate before use.
- Keep store names stable. Renames need explicit migration logic and tests.

## Target Rules

- Put browser-only code behind `#[cfg(target_arch = "wasm32")]`.
- Put native-only code behind `#[cfg(not(target_arch = "wasm32"))]`.
- Keep the public API target-neutral unless a method is explicitly native-only,
  such as test path injection.
- Compile both paths whenever persistence changes:

```bash
cargo check -p rs-dean-idb -p rs-dean-state
cargo check --target wasm32-unknown-unknown -p rs-dean-idb -p rs-dean-state
```

## State Integration

`crates/state` owns application hydration. Leptos signals and Bevy resources
should be hydrated caches over `rs-dean-idb`, not independent storage systems.

For app snapshots:

- Open the database through `open_state_database`.
- Load with `hydrate_from_database`.
- Persist with `persist_snapshot`.
- Keep native tests for round-trip persistence.
- Keep `crates/state/tests/refresh_hydration.rs` green. It proves that a
  durable snapshot can hydrate separate Leptos-style and Bevy-style caches,
  survive dropped in-memory state, reopen the same database, and resume both
  caches from the persisted snapshot.
- Add browser-level tests when native tests cannot exercise the behavior. The
  refresh hydration regression has a wasm browser test; the gate compiles it,
  and it can be run on machines with working browser automation via:

```bash
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner cargo test --target wasm32-unknown-unknown -p rs-dean-state --test refresh_hydration
```

## Gate Expectations

Persistence changes must pass:

```bash
cargo xtask gate
```

The one-pass gate compiles `rs-dean-idb` and `rs-dean-state` for native and wasm
targets, runs native tests, compiles the browser refresh hydration regression,
checks dependency policy, and sweeps docs/skills for retired stack references.
