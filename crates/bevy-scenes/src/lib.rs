use bevy::prelude::*;

pub struct DeanScenePlugin;

impl Plugin for DeanScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

#[derive(Component)]
struct SceneMarker;

fn setup_scene(mut commands: Commands) {
    commands.spawn((Camera2d, SceneMarker));
}
