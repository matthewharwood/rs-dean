#[cfg(target_arch = "wasm32")]
use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.03, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "rs-dean cube smoke".to_owned(),
                resolution: WindowResolution::new(512, 512),
                canvas: Some("#cube-smoke-canvas".to_owned()),
                fit_canvas_to_parent: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_cube_scene)
        .run();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(target_arch = "wasm32")]
#[derive(Component)]
struct GreenCube;

#[cfg(target_arch = "wasm32")]
fn setup_cube_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.9, 0.18),
            perceptual_roughness: 0.55,
            ..default()
        })),
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.35, 0.58, 0.0)),
        GreenCube,
    ));
    info!("GREEN_CUBE_READY base_color=srgb(0.0,0.9,0.18)");

    commands.spawn((
        PointLight {
            intensity: 5_000.0,
            range: 16.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(3.0, 4.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
