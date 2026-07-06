use bevy::prelude::*;
use rs_dean_ui::{ActiveTheme, UiComponentId, bevy_primitives_for_component};

pub struct DeanScenePlugin;

impl Plugin for DeanScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveTheme>()
            .add_systems(Startup, setup_scene);
    }
}

#[derive(Component)]
struct SceneMarker;

fn setup_scene(mut commands: Commands, active_theme: Res<ActiveTheme>) {
    let theme = active_theme.palette();
    let card_primitives = bevy_primitives_for_component(UiComponentId::Card, &theme);
    commands.spawn(Camera2d);
    commands.spawn((
        Text2d::new(format!(
            "Hello world - Card spec: {} primitives",
            card_primitives.len()
        )),
        TextFont {
            font_size: FontSize::Px(72.0),
            ..default()
        },
        TextColor(theme.text_1().to_bevy()),
        Transform::default(),
        SceneMarker,
    ));
}
