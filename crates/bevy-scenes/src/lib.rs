use bevy::{color::palettes::css::WHITE, prelude::*};

pub struct DeanScenePlugin;

impl Plugin for DeanScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

#[derive(Component)]
struct SceneMarker;

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text2d::new("Hello world"),
        TextFont {
            font_size: FontSize::Px(72.0),
            ..default()
        },
        TextColor(WHITE.into()),
        Transform::default(),
        SceneMarker,
    ));
}
