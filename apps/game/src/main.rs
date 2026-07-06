#[cfg(target_arch = "wasm32")]
use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};
#[cfg(target_arch = "wasm32")]
use rs_dean_bevy_scenes::DeanScenePlugin;
#[cfg(target_arch = "wasm32")]
use rs_dean_state::ensure_durable_snapshot;

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        if let Err(error) = ensure_durable_snapshot().await {
            bevy::log::error!("failed to hydrate persistent game state: {error}");
        }
    });
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.03, 0.04, 0.05)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "rs-dean game".to_owned(),
                resolution: WindowResolution::new(960, 540),
                canvas: Some("#game-canvas".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DeanScenePlugin)
        .run();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
