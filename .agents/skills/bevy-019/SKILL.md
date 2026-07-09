---
name: bevy-019
description: "Use before writing or reviewing Bevy 0.19 code in rs-dean. Covers WebGPU-only browser scenes, component spawning, schedules, messages, events, assets, and testing boundaries."
---

# Bevy 0.19 For `rs-dean`

`rs-dean` uses Bevy `0.19.0` only for browser scene rendering. The app target is
`wasm32-unknown-unknown`; the Bevy dependency is WebGPU-only and must not enable
WebGL features.

## Hard Rules

- Use Bevy `0.19.0` APIs only.
- Keep Bevy scene state in ECS resources/components; durable game progress lives
  in the repo state layer, not in transient render objects.
- Spawn components directly. Do not use legacy bundle patterns.
- Use `Camera2d` / `Camera3d` components directly.
- Use `Mesh2d`, `MeshMaterial2d`, `Sprite`, `Text`, and other current
  component APIs directly.
- Keep browser-only scene crates compiling on `wasm32-unknown-unknown`.
- Do not enable `webgl2`; the gate enforces `webgpu` and rejects `webgl`.
- Themeable scene colors that correspond to app UI themes must come from
  `rs-dean-ui` with `default-features = false` and `features = ["bevy"]`; do
  not pull Leptos into the game dependency tree.
- Prefer deterministic math and explicit checked/strict arithmetic for gameplay
  rules. Rendering-only interpolation can use normal floating-point math.

## App Shape

For reusable scene crates, expose a plugin. `apps/game` is the required
Bevy-only game binary and must not depend on Leptos. `apps/ui-bevy-stories` is
the Bevy-only UI primitive story harness for the generated UI mdBook and must
also stay Leptos-free. `apps/marketing` may use Bevy for canvas moments, but it
owns the Leptos marketing surface.

```rust
use bevy::prelude::*;

pub struct DeanScenePlugin;

impl Plugin for DeanScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

When building an executable scene surface, keep the app setup minimal and
feature-driven:

```rust
use bevy::prelude::*;
use rs_dean_bevy_scenes::DeanScenePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DeanScenePlugin)
        .run();
}
```

In `rs-dean`, Bevy is the game and scene layer, not the marketing shell. Do not
move marketing routing or persisted state into Bevy.

## Components Instead Of Bundles

Bevy 0.19 removed the old bundle-first style from most examples. Prefer direct
component tuples:

```rust
commands.spawn((
    Sprite {
        color: Color::srgb(0.2, 0.7, 1.0),
        custom_size: Some(Vec2::new(64.0, 64.0)),
        ..default()
    },
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```

For meshes:

```rust
commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(80.0, 40.0))),
    MeshMaterial2d(materials.add(Color::srgb(0.9, 0.4, 0.1))),
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```

## Scheduling

Use typed schedules and system sets:

```rust
app.add_systems(Startup, setup_scene)
    .add_systems(Update, (advance_animation, sync_scene_state));
```

Use `FixedUpdate` for deterministic gameplay simulation and `Update` for
render-facing interpolation or input handling.

## Messages And Events

Bevy 0.19 distinguishes buffered messages from observed events.

Use messages for frame-to-frame buffered communication:

```rust
#[derive(Message)]
pub struct CardReviewed {
    pub card_id: u64,
}

fn send_review(mut writer: MessageWriter<CardReviewed>) {
    writer.write(CardReviewed { card_id: 7 });
}

fn read_reviews(mut reader: MessageReader<CardReviewed>) {
    for review in reader.read() {
        let _ = review.card_id;
    }
}
```

Register messages on the app:

```rust
app.add_message::<CardReviewed>();
```

Use observed events when an event targets an entity or should trigger observers:

```rust
#[derive(Event)]
pub struct Selected;

fn on_selected(trigger: On<Selected>) {
    let _entity = trigger.target();
}
```

## Assets

Use typed handles and asset collections. Keep paths stable and make missing
assets fail early in tests or story harness routes.

```rust
#[derive(Resource)]
pub struct SceneAssets {
    pub card_texture: Handle<Image>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SceneAssets {
        card_texture: asset_server.load("cards/card.png"),
    });
}
```

## Browser Targeting

The workspace dependency enables:

- `2d_bevy_render`
- `3d_bevy_render`
- `bevy_winit`
- `default_app`
- `png`
- `web`
- `webgpu`

Do not add broad default features to Bevy. If a feature is needed, add only that
feature and run:

```bash
cargo xtask gate
```

The gate checks the wasm feature tree and fails if WebGL is present.

## Testing Boundaries

- Pure gameplay logic belongs in native unit tests under crates such as
  `crates/srs`, `crates/cards`, or future domain crates.
- Bevy scene wiring should have compile tests via `cargo check` and
  story-harness proofs when visible output matters. UI primitive stories belong
  in `apps/ui-bevy-stories` and should be linked from `docs/crates/ui` beside
  the matching Leptos story route.
- `templates/app/cube-smoke` is copied into generated `apps/test-project` as
  the required visible 3D smoke surface. It renders one green
  `StandardMaterial` cube in a centered square canvas with a light and camera,
  then `cargo xtask cube-smoke` verifies the WebGPU renderer, centered canvas
  layout, and green material scene marker. It attempts green-pixel readback
  first where headless browser capture supports it.
- Browser-only compile guarantees are provided by the wasm clippy/check steps in
  `cargo xtask gate`.
- Keep renderer tests deterministic. Avoid real time, ambient randomness, and
  platform-dependent asset discovery in tests.

## Anti-Patterns

- Enabling Bevy default features without a concrete reason.
- Enabling WebGL fallback.
- Reintroducing bundle names from older Bevy examples.
- Putting durable progress or settings only in Bevy resources.
- Hiding scene setup behind generic abstractions before there are repeated
  scene patterns in this repo.
- Adding a browser scene feature without a route in `apps/stories` or
  `apps/ui-bevy-stories`, depending on whether the proof is DOM UI or Bevy UI
  primitives.
