#[cfg(target_arch = "wasm32")]
use bevy::{
    camera::ScalingMode,
    color::Alpha,
    prelude::*,
    sprite::Anchor,
    window::{Window, WindowPlugin, WindowResolution},
};
#[cfg(target_arch = "wasm32")]
use rs_dean_ui::{
    ActiveTheme, BevyUiPrimitive, BevyUiStoryVariant, SHADCN_COMPONENTS, Theme, ThemeId,
    UiComponentId, bevy_story_variants_for_component, component_implementation, scale,
};

#[cfg(target_arch = "wasm32")]
const CANVAS_SELECTOR: &str = "#ui-bevy-stories-canvas";
#[cfg(target_arch = "wasm32")]
const STORY_CARD_WIDTH: f32 = 410.0;
#[cfg(target_arch = "wasm32")]
const STORY_CARD_HEIGHT: f32 = 178.0;

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
                resolution: WindowResolution::new(960, 704),
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
    let variants = bevy_story_variants_for_component(selection.id, active_theme.0);

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 960.0,
                height: 704.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    spawn_story_header(
        &mut commands,
        &theme,
        definition.name,
        definition.summary,
        variants.len(),
        implementation.state.label(),
    );
    spawn_story_variants(&mut commands, &variants);
}

#[cfg(target_arch = "wasm32")]
fn spawn_story_header(
    commands: &mut Commands,
    theme: &Theme,
    name: &str,
    summary: &str,
    variant_count: usize,
    state_label: &str,
) {
    spawn_text(
        commands,
        name.to_owned(),
        Vec3::new(-430.0, 314.0, 3.0),
        scale::font_size::F2,
        theme.text_1().to_bevy(),
        Anchor::CENTER_LEFT,
    );
    spawn_text(
        commands,
        truncate(summary, 96),
        Vec3::new(-430.0, 282.0, 3.0),
        scale::font_size::F0,
        theme.text_2().to_bevy(),
        Anchor::CENTER_LEFT,
    );
    spawn_text(
        commands,
        format!("{variant_count} variants | shared rs-dean-ui Bevy adapter | state: {state_label}"),
        Vec3::new(-430.0, 254.0, 3.0),
        scale::font_size::F00,
        theme.text_muted().to_bevy(),
        Anchor::CENTER_LEFT,
    );
}

#[cfg(target_arch = "wasm32")]
fn spawn_story_variants(commands: &mut Commands, variants: &[BevyUiStoryVariant]) {
    let columns = 2;
    let gap = Vec2::new(26.0, 24.0);
    let origin_x = -((STORY_CARD_WIDTH + gap.x) / 2.0);
    let origin_y = 154.0;

    for (index, variant) in variants.iter().enumerate() {
        let column = index % columns;
        let row = index / columns;
        let x = origin_x + column as f32 * (STORY_CARD_WIDTH + gap.x);
        let y = origin_y - row as f32 * (STORY_CARD_HEIGHT + gap.y);
        spawn_variant_card(commands, variant, Vec2::new(x, y), index);
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_variant_card(
    commands: &mut Commands,
    variant: &BevyUiStoryVariant,
    center: Vec2,
    index: usize,
) {
    let theme = variant.theme_id.palette();
    let z = index as f32 * 0.1;
    let card_size = Vec2::new(STORY_CARD_WIDTH, STORY_CARD_HEIGHT);

    commands.spawn((
        Sprite {
            color: theme.border_subtle().to_bevy(),
            custom_size: Some(card_size + Vec2::splat(2.0)),
            ..default()
        },
        Transform::from_xyz(center.x, center.y, z),
    ));
    commands.spawn((
        Sprite {
            color: theme.surface_1().to_bevy(),
            custom_size: Some(card_size),
            ..default()
        },
        Transform::from_xyz(center.x, center.y, z + 0.01),
    ));
    commands.spawn((
        Sprite {
            color: theme.selected_tint().to_bevy(),
            custom_size: Some(Vec2::new(card_size.x, 4.0)),
            ..default()
        },
        Transform::from_xyz(center.x, center.y + card_size.y / 2.0 - 2.0, z + 0.02),
    ));

    let left = center.x - card_size.x / 2.0 + 18.0;
    let top = center.y + card_size.y / 2.0 - 24.0;
    spawn_text(
        commands,
        format!("{} - {}", variant.label(), variant.theme_id.label()),
        Vec3::new(left, top, z + 0.1),
        scale::font_size::F00,
        theme.text_1().to_bevy(),
        Anchor::CENTER_LEFT,
    );
    spawn_text(
        commands,
        variant.detail().to_owned(),
        Vec3::new(left, top - 24.0, z + 0.1),
        scale::font_size::F000,
        theme.text_2().to_bevy(),
        Anchor::CENTER_LEFT,
    );

    let visible_count = variant.primitives.len().min(6);
    let primitive_columns = if visible_count <= 3 { 1 } else { 2 };
    let primitive_gap = Vec2::new(10.0, 10.0);
    let primitive_cell = Vec2::new(174.0, 32.0);
    let primitive_origin_x = left + primitive_cell.x / 2.0;
    let primitive_origin_y = top - 58.0;

    for (primitive_index, primitive) in variant.primitives.iter().take(6).enumerate() {
        let column = primitive_index % primitive_columns;
        let row = primitive_index / primitive_columns;
        let primitive_center = Vec2::new(
            primitive_origin_x + column as f32 * (primitive_cell.x + primitive_gap.x),
            primitive_origin_y - row as f32 * (primitive_cell.y + primitive_gap.y),
        );
        spawn_primitive(commands, &theme, primitive, primitive_center, z + 0.2);
    }

    if variant.primitives.len() > visible_count {
        spawn_text(
            commands,
            format!(
                "+ {} shared render nodes",
                variant.primitives.len() - visible_count
            ),
            Vec3::new(left, center.y - card_size.y / 2.0 + 18.0, z + 0.2),
            scale::font_size::F000,
            theme.text_muted().to_bevy(),
            Anchor::CENTER_LEFT,
        );
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_primitive(
    commands: &mut Commands,
    theme: &Theme,
    primitive: &BevyUiPrimitive,
    center: Vec2,
    z: f32,
) {
    let size = primitive_size(primitive);
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

    if primitive.selected {
        commands.spawn((
            Sprite {
                color: theme.brand().to_bevy(),
                custom_size: Some(Vec2::splat(7.0)),
                ..default()
            },
            Transform::from_xyz(center.x - size.x / 2.0 + 11.0, center.y, z + 0.2),
        ));
    }

    spawn_text(
        commands,
        primitive_label(primitive),
        Vec3::new(center.x - size.x / 2.0 + 22.0, center.y + 1.0, z + 0.3),
        scale::font_size::F000,
        primitive.text,
        Anchor::CENTER_LEFT,
    );
}

#[cfg(target_arch = "wasm32")]
fn spawn_text(
    commands: &mut Commands,
    text: String,
    position: Vec3,
    font_size: f32,
    color: Color,
    anchor: Anchor,
) {
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size: FontSize::Px(font_size),
            ..default()
        },
        TextColor(color),
        anchor,
        Transform::from_translation(position),
    ));
}

#[cfg(target_arch = "wasm32")]
fn primitive_size(primitive: &BevyUiPrimitive) -> Vec2 {
    Vec2::new(
        primitive.size.x.clamp(96.0, 170.0),
        primitive.size.y.clamp(26.0, 30.0),
    )
}

#[cfg(target_arch = "wasm32")]
fn primitive_label(primitive: &BevyUiPrimitive) -> String {
    let label = if primitive.label.trim().is_empty() {
        primitive.part.as_str()
    } else {
        primitive.label.as_str()
    };
    truncate(label, 18)
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
