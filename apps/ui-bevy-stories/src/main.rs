#[cfg(target_arch = "wasm32")]
use bevy::{
    color::Alpha,
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};
#[cfg(target_arch = "wasm32")]
use rs_dean_ui::{
    ActiveTheme, BevyUiPrimitive, SHADCN_COMPONENTS, Theme, ThemeId, UiComponentId,
    bevy_primitives_for_component, component_implementation, scale,
};

#[cfg(target_arch = "wasm32")]
const CANVAS_SELECTOR: &str = "#ui-bevy-stories-canvas";

#[cfg(target_arch = "wasm32")]
#[derive(Resource, Debug, Clone, Copy)]
struct StorySelection {
    id: UiComponentId,
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    let selected = selected_component_id();
    let theme_id = ThemeId::Dark;
    let theme = theme_id.palette();

    App::new()
        .insert_resource(ActiveTheme(theme_id))
        .insert_resource(StorySelection { id: selected })
        .insert_resource(ClearColor(theme.surface_1().to_bevy()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("rs-dean Bevy UI story: {}", selected.definition().name),
                resolution: WindowResolution::new(960, 640),
                canvas: Some(CANVAS_SELECTOR.to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_story_scene)
        .run();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(target_arch = "wasm32")]
fn selected_component_id() -> UiComponentId {
    web_sys::window()
        .and_then(|window| window.location().search().ok())
        .as_deref()
        .and_then(component_id_from_search)
        .unwrap_or(UiComponentId::Button)
}

#[cfg(target_arch = "wasm32")]
fn component_id_from_search(search: &str) -> Option<UiComponentId> {
    let query = search.strip_prefix('?').unwrap_or(search);
    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        (key == "story")
            .then_some(value)
            .and_then(component_id_for_story_id)
    })
}

#[cfg(target_arch = "wasm32")]
fn component_id_for_story_id(value: &str) -> Option<UiComponentId> {
    let slug = value.strip_prefix("ui-")?;
    SHADCN_COMPONENTS
        .iter()
        .find(|definition| definition.slug == slug)
        .map(|definition| definition.id)
}

#[cfg(target_arch = "wasm32")]
fn setup_story_scene(
    mut commands: Commands,
    active_theme: Res<ActiveTheme>,
    selection: Res<StorySelection>,
) {
    let theme = active_theme.palette();
    let definition = selection.id.definition();
    let implementation = component_implementation(selection.id);
    let primitives = bevy_primitives_for_component(selection.id, &theme);

    commands.spawn(Camera2d);
    spawn_story_header(
        &mut commands,
        &theme,
        definition.name,
        definition.summary,
        primitives.len(),
    );
    spawn_contract_footer(&mut commands, &theme, implementation.state.label());
    spawn_primitives(&mut commands, &theme, &primitives);
}

#[cfg(target_arch = "wasm32")]
fn spawn_story_header(
    commands: &mut Commands,
    theme: &Theme,
    name: &str,
    summary: &str,
    primitive_count: usize,
) {
    spawn_text(
        commands,
        format!("{name} · Bevy primitive story"),
        Vec3::new(-420.0, 280.0, 3.0),
        scale::font_size::F2,
        theme.text_1().to_bevy(),
    );
    spawn_text(
        commands,
        format!("{} primitives · {}", primitive_count, truncate(summary, 82)),
        Vec3::new(-420.0, 248.0, 3.0),
        scale::font_size::F0,
        theme.text_2().to_bevy(),
    );
}

#[cfg(target_arch = "wasm32")]
fn spawn_contract_footer(commands: &mut Commands, theme: &Theme, state_label: &str) {
    spawn_text(
        commands,
        format!("Shared rs-dean-ui Bevy adapter · state: {state_label}"),
        Vec3::new(-420.0, -292.0, 3.0),
        scale::font_size::F00,
        theme.text_muted().to_bevy(),
    );
}

#[cfg(target_arch = "wasm32")]
fn spawn_primitives(commands: &mut Commands, theme: &Theme, primitives: &[BevyUiPrimitive]) {
    let columns = columns_for_count(primitives.len());
    let gap = Vec2::new(24.0, 26.0);
    let cell = Vec2::new(260.0, 84.0);
    let total_width = (columns as f32 * cell.x) + ((columns.saturating_sub(1)) as f32 * gap.x);
    let origin_x = -total_width / 2.0 + cell.x / 2.0;
    let origin_y = 168.0;

    for (index, primitive) in primitives.iter().enumerate() {
        let column = index % columns;
        let row = index / columns;
        let x = origin_x + column as f32 * (cell.x + gap.x);
        let y = origin_y - row as f32 * (cell.y + gap.y);
        spawn_primitive(commands, theme, primitive, Vec2::new(x, y), index);
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_primitive(
    commands: &mut Commands,
    theme: &Theme,
    primitive: &BevyUiPrimitive,
    center: Vec2,
    index: usize,
) {
    let size = primitive_size(primitive);
    let z = index as f32 * 0.01;
    let border_color = if primitive.selected {
        theme.brand().to_bevy()
    } else {
        theme.border_subtle().to_bevy()
    };
    let fill = if primitive.disabled {
        primitive.fill.with_alpha(0.42)
    } else {
        primitive.fill
    };

    commands.spawn((
        Sprite {
            color: border_color,
            custom_size: Some(size + Vec2::splat(4.0)),
            ..default()
        },
        Transform::from_xyz(center.x, center.y, z),
    ));
    commands.spawn((
        Sprite {
            color: fill,
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(center.x, center.y, z + 0.1),
    ));

    spawn_text(
        commands,
        primitive_label(primitive),
        Vec3::new(center.x - size.x / 2.0 + 12.0, center.y + 8.0, z + 0.2),
        scale::font_size::F00,
        primitive.text,
    );
    spawn_text(
        commands,
        primitive_meta(primitive),
        Vec3::new(center.x - size.x / 2.0 + 12.0, center.y - 16.0, z + 0.2),
        scale::font_size::F000,
        theme.text_2().to_bevy(),
    );
}

#[cfg(target_arch = "wasm32")]
fn spawn_text(commands: &mut Commands, text: String, position: Vec3, font_size: f32, color: Color) {
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size: FontSize::Px(font_size),
            ..default()
        },
        TextColor(color),
        Transform::from_translation(position),
    ));
}

#[cfg(target_arch = "wasm32")]
fn primitive_size(primitive: &BevyUiPrimitive) -> Vec2 {
    Vec2::new(
        primitive.size.x.clamp(96.0, 244.0),
        primitive.size.y.clamp(42.0, 76.0),
    )
}

#[cfg(target_arch = "wasm32")]
fn columns_for_count(count: usize) -> usize {
    match count {
        0..=4 => 2,
        5..=12 => 3,
        _ => 4,
    }
}

#[cfg(target_arch = "wasm32")]
fn primitive_label(primitive: &BevyUiPrimitive) -> String {
    let label = if primitive.label.trim().is_empty() {
        primitive.part.as_str()
    } else {
        primitive.label.as_str()
    };
    truncate(label, 24)
}

#[cfg(target_arch = "wasm32")]
fn primitive_meta(primitive: &BevyUiPrimitive) -> String {
    let state = if primitive.selected {
        "selected"
    } else if primitive.disabled {
        "disabled"
    } else {
        "ready"
    };
    truncate(
        &format!("{} · {:?} · {:?}", primitive.part, primitive.kind, state),
        34,
    )
}

#[cfg(target_arch = "wasm32")]
fn truncate(text: &str, max_chars: usize) -> String {
    let mut result = String::new();
    for (index, character) in text.chars().enumerate() {
        if index >= max_chars {
            result.push_str("...");
            return result;
        }
        result.push(character);
    }
    result
}
