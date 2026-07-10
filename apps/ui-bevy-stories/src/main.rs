#[cfg(target_arch = "wasm32")]
use bevy::{
    asset::{AssetMetaCheck, AssetPlugin},
    color::Alpha,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    text::{FontWeight, LineBreak},
    window::{Window, WindowPlugin, WindowResolution},
};
#[cfg(target_arch = "wasm32")]
use rs_dean_blocks::{
    BevyBlockPrimitive, BevyBlockPrimitiveKind, BlockId, BlockInstance, bevy_block_primitives,
    block_by_slug,
};
#[cfg(target_arch = "wasm32")]
use rs_dean_ui::{
    ActiveTheme, AlertDensity, AlertModel, AlertTone, BevyUiPrimitive, BevyUiStoryVariant,
    BreadcrumbDensity, BreadcrumbModel, GridAlign, SHADCN_COMPONENTS, Theme, ThemeId, UiBlockRole,
    UiComponentId, UiStoryModel, UiWidgetIntent, UiWidgetSlotKind,
    bevy_story_variants_for_component, scale,
};

#[cfg(target_arch = "wasm32")]
const CANVAS_SELECTOR: &str = "#ui-bevy-stories-canvas";
#[cfg(target_arch = "wasm32")]
const CONTENT_MAX_WIDTH: f32 = 1024.0;
#[cfg(target_arch = "wasm32")]
const RESPONSIVE_BREAKPOINT: f32 = 720.0;
#[cfg(target_arch = "wasm32")]
const SCROLL_LINE_HEIGHT: f32 = 24.0;
#[cfg(target_arch = "wasm32")]
const BLOCK_CONTENT_MAX_WIDTH: f32 = 1_200.0;
#[cfg(target_arch = "wasm32")]
const BLOCK_CONTENT_HEIGHT: f32 = 664.0;

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StoryId {
    Component(UiComponentId),
    Block(BlockId),
}

#[cfg(target_arch = "wasm32")]
impl StoryId {
    fn name(self) -> &'static str {
        match self {
            Self::Component(id) => id.definition().name,
            Self::Block(id) => id.definition().name,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Resource, Debug, Clone, Copy)]
struct StorySelection {
    id: StoryId,
}

#[cfg(target_arch = "wasm32")]
#[derive(Resource, Clone)]
struct StoryFonts {
    inter: Handle<Font>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Component)]
struct StoryViewport;

#[cfg(target_arch = "wasm32")]
#[derive(Component)]
struct StoryGrid {
    single_column: bool,
}

#[cfg(target_arch = "wasm32")]
#[derive(Component)]
struct InteractivePrimitive {
    rest: Color,
    selected: Color,
    hover: Color,
    pressed: Color,
    latched: bool,
    stateful: bool,
}

#[cfg(target_arch = "wasm32")]
impl InteractivePrimitive {
    fn momentary(rest: Color, theme: &Theme) -> Self {
        Self {
            rest,
            selected: theme.selected_tint().to_bevy(),
            hover: theme.hover_tint().to_bevy(),
            pressed: theme.press_tint().to_bevy(),
            latched: false,
            stateful: false,
        }
    }

    fn stateful(rest: Color, selected: Color, latched: bool, theme: &Theme) -> Self {
        Self {
            rest,
            selected,
            hover: theme.hover_tint().to_bevy(),
            pressed: theme.press_tint().to_bevy(),
            latched,
            stateful: true,
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    let selected = selected_story_id();
    let theme_id = ThemeId::Dark;
    let theme = theme_id.palette();

    App::new()
        .insert_resource(ActiveTheme(theme_id))
        .insert_resource(StorySelection { id: selected })
        .insert_resource(ClearColor(theme.surface_1().to_bevy()))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("rs-dean Bevy UI story: {}", selected.name()),
                        resolution: WindowResolution::new(960, 704),
                        canvas: Some(CANVAS_SELECTOR.to_owned()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup_story_scene)
        .add_systems(
            Update,
            (
                scroll_story_viewport,
                update_story_grid_columns,
                update_interactive_primitives,
            ),
        )
        .run();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(target_arch = "wasm32")]
fn selected_story_id() -> StoryId {
    web_sys::window()
        .and_then(|window| window.location().search().ok())
        .as_deref()
        .and_then(story_id_from_search)
        .unwrap_or(StoryId::Component(UiComponentId::Button))
}

#[cfg(target_arch = "wasm32")]
fn story_id_from_search(search: &str) -> Option<StoryId> {
    let query = search.strip_prefix('?').unwrap_or(search);
    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        (key == "story")
            .then_some(value)
            .and_then(story_id_for_route)
    })
}

#[cfg(target_arch = "wasm32")]
fn story_id_for_route(value: &str) -> Option<StoryId> {
    if let Some(slug) = value.strip_prefix("ui-") {
        return SHADCN_COMPONENTS
            .iter()
            .find(|definition| definition.slug == slug)
            .map(|definition| StoryId::Component(definition.id));
    }

    block_by_slug(value.strip_prefix("block-")?).map(|definition| StoryId::Block(definition.id))
}

#[cfg(target_arch = "wasm32")]
fn setup_story_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    active_theme: Res<ActiveTheme>,
    selection: Res<StorySelection>,
    windows: Query<&Window>,
) {
    let theme = active_theme.palette();
    let fonts = StoryFonts {
        inter: asset_server.load("fonts/InterVariable.ttf"),
    };
    let window_width = windows
        .single()
        .map(Window::width)
        .unwrap_or(RESPONSIVE_BREAKPOINT);

    commands.insert_resource(fonts.clone());
    commands.spawn((Camera2d, IsDefaultUiCamera));
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                padding: UiRect::all(px(scale::space::S)),
                overflow: Overflow::scroll_y(),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(theme.surface_1().to_bevy()),
            ScrollPosition::default(),
            StoryViewport,
        ))
        .with_children(|viewport| match selection.id {
            StoryId::Component(id) => {
                spawn_component_story(viewport, id, active_theme.0, window_width, &fonts);
            }
            StoryId::Block(id) => {
                spawn_block_story(viewport, id, &theme, window_width, &fonts);
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_component_story(
    viewport: &mut ChildSpawnerCommands,
    id: UiComponentId,
    theme_id: ThemeId,
    window_width: f32,
    fonts: &StoryFonts,
) {
    let variants = bevy_story_variants_for_component(id, theme_id);
    let single_column = id == UiComponentId::Accordion;
    let columns = story_grid_column_count(window_width, single_column);

    viewport
        .spawn((
            Node {
                display: Display::Grid,
                width: percent(100),
                max_width: px(if single_column {
                    448.0
                } else {
                    CONTENT_MAX_WIDTH
                }),
                margin: UiRect::horizontal(auto()),
                grid_template_columns: RepeatedGridTrack::fr(columns, 1.0),
                grid_auto_rows: GridTrack::auto(),
                row_gap: px(scale::space::S),
                column_gap: px(scale::space::S),
                align_items: AlignItems::Start,
                padding: UiRect::bottom(px(scale::space::S)),
                ..default()
            },
            StoryGrid { single_column },
        ))
        .with_children(|grid| {
            for variant in &variants {
                spawn_story_variant(grid, id, variant, fonts);
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_block_story(
    viewport: &mut ChildSpawnerCommands,
    id: BlockId,
    theme: &Theme,
    window_width: f32,
    fonts: &StoryFonts,
) {
    let content_width =
        (window_width - scale::space::S * 2.0).clamp(240.0, BLOCK_CONTENT_MAX_WIDTH);
    let content_size = Vec2::new(content_width, BLOCK_CONTENT_HEIGHT);
    let instance = BlockInstance::fixture(id.definition());

    match bevy_block_primitives(&instance, theme, content_size) {
        Ok(primitives) => spawn_block_primitives(viewport, &primitives, content_size, theme, fonts),
        Err(report) => spawn_text(
            viewport,
            format!("Block validation failed: {report}"),
            fonts.inter.clone(),
            scale::font_size::F0,
            FontWeight::SEMIBOLD,
            theme.danger().to_bevy(),
        ),
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_block_primitives(
    viewport: &mut ChildSpawnerCommands,
    primitives: &[BevyBlockPrimitive],
    content_size: Vec2,
    theme: &Theme,
    fonts: &StoryFonts,
) {
    let section_fill = primitives
        .iter()
        .find(|primitive| primitive.kind == BevyBlockPrimitiveKind::Section)
        .map_or_else(|| theme.surface_1().to_bevy(), |primitive| primitive.fill);

    viewport
        .spawn((
            Node {
                position_type: PositionType::Relative,
                width: px(content_size.x),
                height: px(content_size.y),
                min_width: px(content_size.x),
                min_height: px(content_size.y),
                flex_shrink: 0.0,
                margin: UiRect::horizontal(auto()),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(section_fill),
        ))
        .with_children(|root| {
            for primitive in primitives
                .iter()
                .filter(|primitive| primitive.kind != BevyBlockPrimitiveKind::Section)
            {
                spawn_block_primitive(root, primitive, content_size, theme, fonts);
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_block_primitive(
    parent: &mut ChildSpawnerCommands,
    primitive: &BevyBlockPrimitive,
    content_size: Vec2,
    theme: &Theme,
    fonts: &StoryFonts,
) {
    let origin = block_primitive_origin(primitive.center, primitive.size, content_size);
    let justify_content = block_justify_content(primitive.text_align);
    let text_justify = block_text_justify(primitive.text_align);
    let disabled = primitive.ui.as_ref().is_some_and(|ui| ui.disabled);
    let fill = if disabled {
        primitive.fill.with_alpha(0.42)
    } else {
        primitive.fill
    };
    let text_color = if disabled {
        primitive.text.with_alpha(0.38)
    } else {
        primitive.text
    };
    let mut entity = parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(origin.x),
            top: px(origin.y),
            width: px(primitive.size.x.max(0.0)),
            height: px(primitive.size.y.max(0.0)),
            min_width: px(0.0),
            padding: block_primitive_padding(primitive.kind),
            border: UiRect::all(px(primitive.border_width.max(0.0))),
            border_radius: block_primitive_radius(primitive.kind, theme),
            justify_content,
            align_items: AlignItems::Center,
            overflow: Overflow::clip(),
            ..default()
        },
        BackgroundColor(fill),
        BorderColor::all(primitive.border),
    ));

    if primitive.font_size > 0.0 && !primitive.label.trim().is_empty() {
        entity.insert((
            Text::new(primitive.label.clone()),
            TextFont {
                font: fonts.inter.clone().into(),
                font_size: FontSize::Px(primitive.font_size),
                weight: block_primitive_font_weight(primitive.kind),
                ..default()
            },
            TextColor(text_color),
            TextLayout::new(text_justify, LineBreak::WordOrCharacter),
        ));
    }

    let ui_interactive = primitive
        .ui
        .as_ref()
        .is_some_and(|ui| ui.intent != UiWidgetIntent::None && !ui.disabled);
    if primitive.kind == BevyBlockPrimitiveKind::Action || ui_interactive {
        let interaction = if let Some(ui) = &primitive.ui {
            InteractivePrimitive::stateful(
                fill,
                theme.selected_tint().to_bevy(),
                ui.selected,
                theme,
            )
        } else {
            InteractivePrimitive::momentary(fill, theme)
        };
        entity.insert((Button, interaction));
    }
}

#[cfg(target_arch = "wasm32")]
fn block_primitive_origin(center: Vec2, size: Vec2, content_size: Vec2) -> Vec2 {
    Vec2::new(
        content_size.x / 2.0 + center.x - size.x / 2.0,
        content_size.y / 2.0 - center.y - size.y / 2.0,
    )
}

#[cfg(target_arch = "wasm32")]
const fn block_justify_content(align: GridAlign) -> JustifyContent {
    match align {
        GridAlign::Center => JustifyContent::Center,
        GridAlign::End => JustifyContent::FlexEnd,
        GridAlign::Start | GridAlign::Stretch => JustifyContent::FlexStart,
    }
}

#[cfg(target_arch = "wasm32")]
const fn block_text_justify(align: GridAlign) -> Justify {
    match align {
        GridAlign::Center => Justify::Center,
        GridAlign::End => Justify::Right,
        GridAlign::Start | GridAlign::Stretch => Justify::Left,
    }
}

#[cfg(target_arch = "wasm32")]
const fn block_primitive_font_weight(kind: BevyBlockPrimitiveKind) -> FontWeight {
    match kind {
        BevyBlockPrimitiveKind::Heading => FontWeight::BOLD,
        BevyBlockPrimitiveKind::Action | BevyBlockPrimitiveKind::UiComponent => {
            FontWeight::SEMIBOLD
        }
        BevyBlockPrimitiveKind::Section
        | BevyBlockPrimitiveKind::Text
        | BevyBlockPrimitiveKind::Item
        | BevyBlockPrimitiveKind::Media => FontWeight::NORMAL,
    }
}

#[cfg(target_arch = "wasm32")]
fn block_primitive_padding(kind: BevyBlockPrimitiveKind) -> UiRect {
    match kind {
        BevyBlockPrimitiveKind::Heading | BevyBlockPrimitiveKind::Text => {
            UiRect::horizontal(px(scale::space::XS2))
        }
        BevyBlockPrimitiveKind::Action
        | BevyBlockPrimitiveKind::Item
        | BevyBlockPrimitiveKind::Media
        | BevyBlockPrimitiveKind::UiComponent => UiRect::horizontal(px(scale::space::XS)),
        BevyBlockPrimitiveKind::Section => UiRect::ZERO,
    }
}

#[cfg(target_arch = "wasm32")]
fn block_primitive_radius(kind: BevyBlockPrimitiveKind, theme: &Theme) -> BorderRadius {
    let radius = match kind {
        BevyBlockPrimitiveKind::Action => theme.radius_field,
        BevyBlockPrimitiveKind::Item
        | BevyBlockPrimitiveKind::Media
        | BevyBlockPrimitiveKind::UiComponent => theme.radius_box,
        BevyBlockPrimitiveKind::Section
        | BevyBlockPrimitiveKind::Heading
        | BevyBlockPrimitiveKind::Text => 0.0,
    };
    BorderRadius::all(px(radius))
}

#[cfg(target_arch = "wasm32")]
fn spawn_story_variant(
    parent: &mut ChildSpawnerCommands,
    id: UiComponentId,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    match &variant.model {
        UiStoryModel::Alert(model) => {
            spawn_alert(parent, model, variant.theme_id.palette(), fonts);
        }
        UiStoryModel::Breadcrumb(model) => {
            spawn_breadcrumb(parent, model, variant.theme_id.palette(), fonts);
        }
        _ => spawn_primitive_component(parent, id, variant, fonts),
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_alert(
    parent: &mut ChildSpawnerCommands,
    model: &AlertModel,
    theme: Theme,
    fonts: &StoryFonts,
) {
    let dense = model.density == AlertDensity::Dense;
    let padding = if dense {
        scale::space::XS
    } else {
        scale::space::S
    };
    let gap = if dense {
        scale::space::XS2
    } else {
        scale::space::XS
    };
    let radius = if dense {
        theme.radius_field
    } else {
        theme.radius_box
    };
    let (background, border, marker, marker_text) = alert_colors(model.tone, &theme);

    parent
        .spawn((
            Node {
                width: auto(),
                justify_self: JustifySelf::Stretch,
                padding: UiRect::all(px(padding)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(radius)),
                ..default()
            },
            BackgroundColor(background),
            BorderColor::all(border),
        ))
        .with_children(|alert| {
            alert
                .spawn(Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    align_items: AlignItems::Start,
                    row_gap: px(gap),
                    column_gap: px(gap),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Node {
                            width: px(scale::space::S),
                            height: px(scale::space::S),
                            min_width: px(scale::space::S),
                            min_height: px(scale::space::S),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::MAX,
                            ..default()
                        },
                        BackgroundColor(marker),
                        Text::new(alert_marker_label(model.tone)),
                        TextFont {
                            font: fonts.inter.clone().into(),
                            font_size: FontSize::Px(scale::font_size::F00),
                            weight: FontWeight::BOLD,
                            ..default()
                        },
                        TextColor(marker_text),
                        TextLayout::new(Justify::Center, LineBreak::NoWrap),
                    ));

                    row.spawn(Node {
                        min_width: px(180.0),
                        flex_basis: px(220.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        row_gap: px(scale::space::XS2),
                        ..default()
                    })
                    .with_children(|body| {
                        spawn_text(
                            body,
                            model.title.clone(),
                            fonts.inter.clone(),
                            if dense {
                                scale::font_size::F0
                            } else {
                                scale::font_size::F1
                            },
                            FontWeight::BOLD,
                            theme.text_1().to_bevy(),
                        );
                        spawn_text(
                            body,
                            model.description.clone(),
                            fonts.inter.clone(),
                            if dense {
                                scale::font_size::F00
                            } else {
                                scale::font_size::F0
                            },
                            FontWeight::NORMAL,
                            theme.text_2().to_bevy(),
                        );
                    });

                    if let Some(action) = &model.action {
                        let disabled = model.disabled || model.loading || action.disabled;
                        let rest = theme.surface_2().to_bevy();
                        row.spawn((
                            Button,
                            Node {
                                min_height: px(scale::space::L),
                                padding: UiRect::axes(px(scale::space::XS), px(scale::space::XS2)),
                                border: UiRect::all(px(theme.border.max(1.0))),
                                border_radius: BorderRadius::all(px(theme.radius_field)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(if disabled {
                                rest.with_alpha(0.42)
                            } else {
                                rest
                            }),
                            BorderColor::all(theme.border_strong().to_bevy()),
                            Text::new(if model.loading {
                                "Loading".to_owned()
                            } else {
                                action.label.clone()
                            }),
                            TextFont {
                                font: fonts.inter.clone().into(),
                                font_size: FontSize::Px(scale::font_size::F0),
                                weight: FontWeight::SEMIBOLD,
                                ..default()
                            },
                            TextColor(if disabled {
                                theme.text_disabled().to_bevy()
                            } else {
                                theme.text_1().to_bevy()
                            }),
                            TextLayout::new(Justify::Center, LineBreak::NoWrap),
                            InteractivePrimitive::momentary(rest, &theme),
                        ));
                    }
                });
        });
}

#[cfg(target_arch = "wasm32")]
fn alert_colors(tone: AlertTone, theme: &Theme) -> (Color, Color, Color, Color) {
    match tone {
        AlertTone::Default => (
            theme.surface_1().to_bevy(),
            theme.border_subtle().to_bevy(),
            theme.surface_2().to_bevy(),
            theme.text_muted().to_bevy(),
        ),
        AlertTone::Info => (
            theme.info_soft().to_bevy(),
            theme.info.to_bevy(),
            theme.info.to_bevy(),
            theme.text_on_brand().to_bevy(),
        ),
        AlertTone::Success => (
            theme.success_soft().to_bevy(),
            theme.success.to_bevy(),
            theme.success.to_bevy(),
            theme.text_on_brand().to_bevy(),
        ),
        AlertTone::Warning => (
            theme.warning_soft().to_bevy(),
            theme.warning.to_bevy(),
            theme.warning.to_bevy(),
            theme.text_on_brand().to_bevy(),
        ),
        AlertTone::Destructive => (
            theme.error_soft().to_bevy(),
            theme.danger().to_bevy(),
            theme.danger().to_bevy(),
            theme.text_on_brand().to_bevy(),
        ),
    }
}

#[cfg(target_arch = "wasm32")]
const fn alert_marker_label(tone: AlertTone) -> &'static str {
    match tone {
        AlertTone::Default | AlertTone::Info => "i",
        AlertTone::Success => "+",
        AlertTone::Warning | AlertTone::Destructive => "!",
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_breadcrumb(
    parent: &mut ChildSpawnerCommands,
    model: &BreadcrumbModel,
    theme: Theme,
    fonts: &StoryFonts,
) {
    let dense = model.density == BreadcrumbDensity::Dense;
    let gap = if dense {
        scale::space::XS3
    } else {
        scale::space::XS2
    };
    let last_index = model.entries.len().saturating_sub(1);

    parent
        .spawn(Node {
            width: auto(),
            min_height: px(scale::space::L),
            justify_self: JustifySelf::Stretch,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            align_items: AlignItems::Center,
            row_gap: px(gap),
            column_gap: px(gap),
            ..default()
        })
        .with_children(|breadcrumb| {
            for (index, entry) in model.entries.iter().enumerate() {
                let current = index == last_index;
                let disabled = model.disabled || model.loading || entry.disabled;
                let label = if model.loading {
                    "Loading".to_owned()
                } else {
                    entry.label.clone()
                };
                let rest = if model.loading {
                    theme.surface_sunken().to_bevy()
                } else {
                    Color::NONE
                };
                let mut item = breadcrumb.spawn((
                    Node {
                        min_height: px(scale::space::S),
                        padding: if current {
                            UiRect::ZERO
                        } else {
                            UiRect::axes(px(scale::space::XS2), px(scale::space::XS3))
                        },
                        border_radius: BorderRadius::all(px(theme.radius_field)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Text::new(label),
                    TextFont {
                        font: fonts.inter.clone().into(),
                        font_size: FontSize::Px(if dense {
                            scale::font_size::F00
                        } else {
                            scale::font_size::F0
                        }),
                        weight: if current {
                            FontWeight::BOLD
                        } else {
                            FontWeight::NORMAL
                        },
                        ..default()
                    },
                    TextColor(if disabled {
                        theme.text_disabled().to_bevy()
                    } else if current {
                        theme.text_1().to_bevy()
                    } else {
                        theme.text_2().to_bevy()
                    }),
                    TextLayout::new(Justify::Left, LineBreak::NoWrap),
                    BackgroundColor(rest),
                ));
                if !current && !disabled {
                    item.insert((Button, InteractivePrimitive::momentary(rest, &theme)));
                }

                if !current {
                    breadcrumb.spawn((
                        Text::new(model.separator.clone()),
                        TextFont {
                            font: fonts.inter.clone().into(),
                            font_size: FontSize::Px(if dense {
                                scale::font_size::F00
                            } else {
                                scale::font_size::F0
                            }),
                            ..default()
                        },
                        TextColor(theme.text_muted().to_bevy()),
                        TextLayout::new(Justify::Center, LineBreak::NoWrap),
                    ));
                }
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_primitive_component(
    parent: &mut ChildSpawnerCommands,
    id: UiComponentId,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    match id {
        UiComponentId::Accordion => spawn_accordion_primitives(parent, variant, fonts),
        UiComponentId::AspectRatio => spawn_aspect_ratio_primitives(parent, variant, fonts),
        UiComponentId::Avatar => spawn_avatar_primitives(parent, variant, fonts),
        UiComponentId::Calendar | UiComponentId::DatePicker => {
            spawn_calendar_primitives(parent, id, variant, fonts);
        }
        UiComponentId::Checkbox | UiComponentId::RadioGroup | UiComponentId::Switch => {
            spawn_selection_primitives(parent, id, variant, fonts);
        }
        UiComponentId::DataTable | UiComponentId::Table => {
            spawn_table_primitives(parent, variant, fonts);
        }
        UiComponentId::Progress | UiComponentId::Slider | UiComponentId::Spinner => {
            spawn_meter_primitives(parent, id, variant, fonts);
        }
        UiComponentId::Resizable => spawn_resizable_primitives(parent, variant, fonts),
        UiComponentId::ScrollArea => spawn_scroll_area_primitives(parent, variant, fonts),
        UiComponentId::Separator => spawn_separator_primitives(parent, variant, fonts),
        UiComponentId::Skeleton => spawn_skeleton_primitives(parent, variant),
        UiComponentId::Tabs => spawn_tabs_primitives(parent, variant, fonts),
        UiComponentId::Typography => spawn_typography_primitives(parent, variant, fonts),
        _ if component_is_control_row(id) => {
            spawn_control_row_primitives(parent, id, variant, fonts);
        }
        _ => spawn_stack_primitives(parent, id, variant, fonts),
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_accordion_primitives(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };

    parent
        .spawn((
            Node {
                width: percent(100),
                justify_self: JustifySelf::Stretch,
                min_width: px(0.0),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: FlexDirection::Column,
                row_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|accordion| {
            let mut index = 1;
            while index < variant.primitives.len() {
                let item = &variant.primitives[index];
                if !item.part.starts_with("AccordionItem") {
                    index += 1;
                    continue;
                }
                let trigger = variant.primitives.get(index + 1);
                let content = variant.primitives.get(index + 2);
                accordion
                    .spawn((
                        Node {
                            width: percent(100),
                            min_width: px(0.0),
                            border: UiRect::all(px(theme.border.max(1.0))),
                            border_radius: BorderRadius::all(px(theme.radius_field)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        BackgroundColor(if item.selected {
                            theme.primary_soft().to_bevy()
                        } else {
                            theme.surface_1().to_bevy()
                        }),
                        BorderColor::all(if item.selected {
                            theme.brand().to_bevy()
                        } else {
                            theme.border_subtle().to_bevy()
                        }),
                    ))
                    .with_children(|item_node| {
                        if let Some(trigger) = trigger {
                            let rest = if trigger.selected {
                                theme.selected_tint().to_bevy()
                            } else {
                                Color::NONE
                            };
                            item_node.spawn((
                                Button,
                                Node {
                                    width: percent(100),
                                    min_height: px(scale::space::L),
                                    padding: UiRect::axes(
                                        px(scale::space::XS),
                                        px(scale::space::XS2),
                                    ),
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    border_radius: BorderRadius::all(px(theme.radius_field)),
                                    ..default()
                                },
                                BackgroundColor(rest),
                                Text::new(format!(
                                    "{}  {}",
                                    trigger.label,
                                    if trigger.selected { "-" } else { "+" }
                                )),
                                TextFont {
                                    font: fonts.inter.clone().into(),
                                    font_size: FontSize::Px(scale::font_size::F0),
                                    weight: FontWeight::BOLD,
                                    ..default()
                                },
                                TextColor(if trigger.disabled {
                                    theme.text_disabled().to_bevy()
                                } else {
                                    theme.text_1().to_bevy()
                                }),
                                TextLayout::new(Justify::Left, LineBreak::WordOrCharacter),
                                InteractivePrimitive::stateful(
                                    rest,
                                    theme.selected_tint().to_bevy(),
                                    trigger.selected,
                                    &theme,
                                ),
                            ));
                        }
                        if let Some(content) = content.filter(|content| content.selected) {
                            item_node
                                .spawn(Node {
                                    width: percent(100),
                                    padding: UiRect::new(
                                        px(scale::space::XS),
                                        px(scale::space::XS),
                                        px(0.0),
                                        px(scale::space::XS),
                                    ),
                                    ..default()
                                })
                                .with_children(|copy| {
                                    spawn_text(
                                        copy,
                                        content.value.clone(),
                                        fonts.inter.clone(),
                                        scale::font_size::F0,
                                        FontWeight::NORMAL,
                                        theme.text_2().to_bevy(),
                                    );
                                });
                        }
                    });
                index += 3;
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_aspect_ratio_primitives(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    let media = variant
        .primitives
        .iter()
        .find(|primitive| primitive.kind == UiWidgetSlotKind::Media)
        .unwrap_or(root);
    let ratio = if root.size.y > 0.0 {
        root.size.x / root.size.y
    } else {
        1.0
    };

    parent
        .spawn(Node {
            width: percent(100),
            min_width: px(0.0),
            flex_direction: FlexDirection::Column,
            row_gap: px(scale::space::XS2),
            ..default()
        })
        .with_children(|figure| {
            figure
                .spawn((
                    Node {
                        width: percent(100),
                        aspect_ratio: Some(ratio),
                        max_height: px(260.0),
                        padding: UiRect::all(px(scale::space::XS2)),
                        border: UiRect::all(px(theme.border.max(1.0))),
                        border_radius: BorderRadius::all(px(theme.radius_box)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(media.fill),
                    BorderColor::all(theme.border_subtle().to_bevy()),
                ))
                .with_children(|frame| {
                    frame
                        .spawn(Node {
                            width: percent(82),
                            max_width: percent(82),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Stretch,
                            row_gap: px(scale::space::XS2),
                            ..default()
                        })
                        .with_children(|copy| {
                            spawn_text(
                                copy,
                                media.label.clone(),
                                fonts.inter.clone(),
                                scale::font_size::F1,
                                FontWeight::BOLD,
                                media.text,
                            );
                            spawn_text(
                                copy,
                                media.value.clone(),
                                fonts.inter.clone(),
                                scale::font_size::F00,
                                FontWeight::NORMAL,
                                theme.text_2().to_bevy(),
                            );
                        });
                });
            spawn_text(
                figure,
                format!("{}  {:.2}:1", media.label, ratio),
                fonts.inter.clone(),
                scale::font_size::F00,
                FontWeight::SEMIBOLD,
                theme.text_muted().to_bevy(),
            );
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_avatar_primitives(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    let fallback = variant
        .primitives
        .iter()
        .find(|primitive| primitive.part == "AvatarFallback")
        .unwrap_or(root);
    let diameter = root.size.x.clamp(scale::space::L, scale::space::XL2);
    parent.spawn((
        Node {
            width: px(diameter),
            height: px(diameter),
            min_width: px(diameter),
            min_height: px(diameter),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(px(theme.border.max(1.0))),
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BackgroundColor(root.fill),
        BorderColor::all(theme.border_subtle().to_bevy()),
        Text::new(avatar_initials(&fallback.label)),
        TextFont {
            font: fonts.inter.clone().into(),
            font_size: FontSize::Px(scale::font_size::F0),
            weight: FontWeight::BOLD,
            ..default()
        },
        TextColor(if root.disabled {
            theme.text_disabled().to_bevy()
        } else {
            root.text
        }),
        TextLayout::new(Justify::Center, LineBreak::NoWrap),
    ));
}

#[cfg(target_arch = "wasm32")]
fn spawn_calendar_primitives(
    parent: &mut ChildSpawnerCommands,
    id: UiComponentId,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    let date_picker = id == UiComponentId::DatePicker;
    let popover = variant
        .primitives
        .iter()
        .find(|primitive| primitive.part == "DatePickerPopover");
    let show_calendar = !date_picker || popover.is_some_and(|primitive| primitive.selected);
    let header = variant.primitives.iter().find(|primitive| {
        primitive.kind == UiWidgetSlotKind::Header || primitive.part == "DatePickerPopover"
    });
    let day_prefix = if date_picker {
        "DatePickerCalendar:"
    } else {
        "CalendarDay:"
    };

    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: FlexDirection::Column,
                row_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|calendar| {
            if date_picker
                && let Some(trigger) = variant
                    .primitives
                    .iter()
                    .find(|primitive| primitive.part == "DatePickerTrigger")
            {
                spawn_primitive(calendar, id, trigger, &theme, fonts);
            }
            if !show_calendar {
                return;
            }
            calendar
                .spawn(Node {
                    width: percent(100),
                    min_height: px(scale::space::L),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|heading| {
                    spawn_text(
                        heading,
                        "<",
                        fonts.inter.clone(),
                        scale::font_size::F0,
                        FontWeight::BOLD,
                        theme.text_2().to_bevy(),
                    );
                    spawn_text(
                        heading,
                        header.map_or_else(|| root.label.clone(), |value| value.label.clone()),
                        fonts.inter.clone(),
                        scale::font_size::F1,
                        FontWeight::BOLD,
                        theme.text_1().to_bevy(),
                    );
                    spawn_text(
                        heading,
                        ">",
                        fonts.inter.clone(),
                        scale::font_size::F0,
                        FontWeight::BOLD,
                        theme.text_2().to_bevy(),
                    );
                });
            calendar
                .spawn(Node {
                    display: Display::Grid,
                    width: percent(100),
                    grid_template_columns: RepeatedGridTrack::fr(7, 1.0),
                    grid_auto_rows: GridTrack::auto(),
                    row_gap: px(scale::space::XS3),
                    column_gap: px(scale::space::XS3),
                    ..default()
                })
                .with_children(|grid| {
                    for weekday in ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
                        spawn_centered_label(
                            grid,
                            weekday,
                            scale::space::M,
                            scale::font_size::F000,
                            theme.text_muted().to_bevy(),
                            fonts,
                        );
                    }
                    for day in variant
                        .primitives
                        .iter()
                        .filter(|primitive| primitive.part.starts_with(day_prefix))
                    {
                        let rest = if day.selected {
                            day.fill
                        } else {
                            theme.surface_1().to_bevy()
                        };
                        grid.spawn((
                            Button,
                            Node {
                                width: auto(),
                                min_width: px(0.0),
                                min_height: px(scale::space::M),
                                justify_self: JustifySelf::Stretch,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(px(theme.border.max(1.0))),
                                border_radius: BorderRadius::all(px(theme.radius_field)),
                                ..default()
                            },
                            BackgroundColor(rest),
                            BorderColor::all(if day.selected {
                                theme.brand().to_bevy()
                            } else {
                                theme.border_faint().to_bevy()
                            }),
                            Text::new(day.label.clone()),
                            TextFont {
                                font: fonts.inter.clone().into(),
                                font_size: FontSize::Px(scale::font_size::F00),
                                weight: if day.selected {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                },
                                ..default()
                            },
                            TextColor(if day.disabled {
                                theme.text_disabled().to_bevy()
                            } else {
                                day.text
                            }),
                            TextLayout::new(Justify::Center, LineBreak::NoWrap),
                            InteractivePrimitive::stateful(rest, day.fill, day.selected, &theme),
                        ));
                    }
                });
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_selection_primitives(
    parent: &mut ChildSpawnerCommands,
    id: UiComponentId,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    match id {
        UiComponentId::RadioGroup => spawn_radio_group(parent, variant, fonts),
        UiComponentId::Checkbox => spawn_checkbox(parent, variant, fonts),
        UiComponentId::Switch => spawn_switch(parent, variant, fonts),
        _ => {}
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_checkbox(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    let indicator = variant
        .primitives
        .iter()
        .find(|primitive| primitive.kind == UiWidgetSlotKind::Checkbox);
    let description = variant
        .primitives
        .iter()
        .find(|primitive| primitive.kind == UiWidgetSlotKind::Description);

    parent
        .spawn((
            Button,
            Node {
                width: percent(100),
                min_width: px(0.0),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Start,
                column_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
            InteractivePrimitive::stateful(
                root.fill,
                theme.selected_tint().to_bevy(),
                root.selected,
                &theme,
            ),
        ))
        .with_children(|checkbox| {
            if let Some(indicator) = indicator {
                spawn_selection_indicator(checkbox, indicator, &theme, fonts, "x");
            }
            checkbox
                .spawn(Node {
                    min_width: px(0.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(scale::space::XS3),
                    ..default()
                })
                .with_children(|copy| {
                    spawn_text(
                        copy,
                        root.label.clone(),
                        fonts.inter.clone(),
                        scale::font_size::F0,
                        FontWeight::BOLD,
                        if root.disabled {
                            theme.text_disabled().to_bevy()
                        } else {
                            theme.text_1().to_bevy()
                        },
                    );
                    if let Some(description) = description {
                        spawn_text(
                            copy,
                            description.value.clone(),
                            fonts.inter.clone(),
                            scale::font_size::F00,
                            FontWeight::NORMAL,
                            theme.text_2().to_bevy(),
                        );
                    }
                });
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_radio_group(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                row_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|group| {
            for item in variant
                .primitives
                .iter()
                .filter(|primitive| primitive.part.starts_with("RadioGroupItem:"))
            {
                let key = item.part.split_once(':').map_or("", |(_, key)| key);
                let indicator = variant
                    .primitives
                    .iter()
                    .find(|primitive| primitive.part == format!("RadioGroupIndicator:{key}"));
                let label = variant
                    .primitives
                    .iter()
                    .find(|primitive| primitive.part == format!("RadioGroupLabel:{key}"));
                let rest = if item.selected {
                    theme.primary_soft().to_bevy()
                } else {
                    theme.surface_1().to_bevy()
                };
                group
                    .spawn((
                        Button,
                        Node {
                            width: percent(100),
                            padding: UiRect::all(px(scale::space::XS2)),
                            border: UiRect::all(px(theme.border.max(1.0))),
                            border_radius: BorderRadius::all(px(theme.radius_field)),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: px(scale::space::XS2),
                            ..default()
                        },
                        BackgroundColor(rest),
                        BorderColor::all(if item.selected {
                            theme.brand().to_bevy()
                        } else {
                            theme.border_subtle().to_bevy()
                        }),
                        InteractivePrimitive::stateful(
                            rest,
                            theme.primary_soft().to_bevy(),
                            item.selected,
                            &theme,
                        ),
                    ))
                    .with_children(|row| {
                        if let Some(indicator) = indicator {
                            spawn_selection_indicator(row, indicator, &theme, fonts, "o");
                        }
                        if let Some(label) = label {
                            row.spawn(Node {
                                min_width: px(0.0),
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            })
                            .with_children(|copy| {
                                spawn_text(
                                    copy,
                                    label.label.clone(),
                                    fonts.inter.clone(),
                                    scale::font_size::F0,
                                    FontWeight::BOLD,
                                    label.text,
                                );
                                if !label.value.is_empty() && label.value != label.label {
                                    spawn_text(
                                        copy,
                                        label.value.clone(),
                                        fonts.inter.clone(),
                                        scale::font_size::F00,
                                        FontWeight::NORMAL,
                                        theme.text_2().to_bevy(),
                                    );
                                }
                            });
                        }
                    });
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_switch(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    let track = variant
        .primitives
        .iter()
        .find(|primitive| primitive.part == "SwitchTrack")
        .unwrap_or(root);
    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                column_gap: px(scale::space::XS),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|switch| {
            switch
                .spawn(Node {
                    min_width: px(0.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(scale::space::XS3),
                    ..default()
                })
                .with_children(|copy| {
                    spawn_text(
                        copy,
                        root.label.clone(),
                        fonts.inter.clone(),
                        scale::font_size::F0,
                        FontWeight::BOLD,
                        root.text,
                    );
                    spawn_text(
                        copy,
                        root.value.clone(),
                        fonts.inter.clone(),
                        scale::font_size::F00,
                        FontWeight::NORMAL,
                        theme.text_2().to_bevy(),
                    );
                });
            let rest = if track.selected {
                theme.brand().to_bevy()
            } else {
                theme.surface_3().to_bevy()
            };
            switch
                .spawn((
                    Button,
                    Node {
                        width: px(scale::space::XL),
                        min_width: px(scale::space::XL),
                        height: px(scale::space::M),
                        padding: UiRect::all(px(scale::space::XS3)),
                        border: UiRect::all(px(theme.border.max(1.0))),
                        border_radius: BorderRadius::MAX,
                        justify_content: if track.selected {
                            JustifyContent::FlexEnd
                        } else {
                            JustifyContent::FlexStart
                        },
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(rest),
                    BorderColor::all(if track.selected {
                        theme.brand().to_bevy()
                    } else {
                        theme.border_strong().to_bevy()
                    }),
                    InteractivePrimitive::stateful(
                        theme.surface_3().to_bevy(),
                        theme.brand().to_bevy(),
                        track.selected,
                        &theme,
                    ),
                ))
                .with_children(|control| {
                    control.spawn((
                        Node {
                            width: px(scale::space::S),
                            height: px(scale::space::S),
                            border_radius: BorderRadius::MAX,
                            ..default()
                        },
                        BackgroundColor(theme.surface_1().to_bevy()),
                    ));
                });
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_selection_indicator(
    parent: &mut ChildSpawnerCommands,
    primitive: &BevyUiPrimitive,
    theme: &Theme,
    fonts: &StoryFonts,
    selected_label: &str,
) {
    parent.spawn((
        Node {
            width: px(scale::space::S),
            height: px(scale::space::S),
            min_width: px(scale::space::S),
            min_height: px(scale::space::S),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(px(theme.border.max(1.0))),
            border_radius: if primitive.kind == UiWidgetSlotKind::Radio {
                BorderRadius::MAX
            } else {
                BorderRadius::all(px(theme.radius_selector))
            },
            ..default()
        },
        BackgroundColor(if primitive.selected {
            theme.brand().to_bevy()
        } else {
            theme.surface_1().to_bevy()
        }),
        BorderColor::all(if primitive.selected {
            theme.brand().to_bevy()
        } else {
            theme.border_strong().to_bevy()
        }),
        Text::new(if primitive.selected {
            selected_label
        } else {
            ""
        }),
        TextFont {
            font: fonts.inter.clone().into(),
            font_size: FontSize::Px(scale::font_size::F000),
            weight: FontWeight::BOLD,
            ..default()
        },
        TextColor(theme.text_on_brand().to_bevy()),
        TextLayout::new(Justify::Center, LineBreak::NoWrap),
    ));
}

#[cfg(target_arch = "wasm32")]
fn spawn_table_primitives(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    let headers = variant
        .primitives
        .iter()
        .filter(|primitive| {
            primitive.part.starts_with("DataTableHeader:")
                || primitive.part.starts_with("TableHead:")
        })
        .collect::<Vec<_>>();
    let cells = variant
        .primitives
        .iter()
        .filter(|primitive| {
            primitive.part.starts_with("DataTableCell:") || primitive.part.starts_with("TableCell:")
        })
        .collect::<Vec<_>>();
    let columns = u16::try_from(headers.len().max(1)).unwrap_or(1);

    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: FlexDirection::Column,
                row_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|table| {
            if let Some(toolbar) = variant
                .primitives
                .iter()
                .find(|primitive| primitive.part == "DataTableToolbar")
            {
                spawn_primitive(table, UiComponentId::DataTable, toolbar, &theme, fonts);
            }
            table
                .spawn((
                    Node {
                        display: Display::Grid,
                        width: percent(100),
                        min_width: px(0.0),
                        grid_template_columns: RepeatedGridTrack::fr(columns, 1.0),
                        grid_auto_rows: GridTrack::auto(),
                        border: UiRect::all(px(theme.border.max(1.0))),
                        border_radius: BorderRadius::all(px(theme.radius_field)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BorderColor::all(theme.border_subtle().to_bevy()),
                ))
                .with_children(|grid| {
                    for header in &headers {
                        spawn_table_cell(grid, header, true, &theme, fonts);
                    }
                    for cell in &cells {
                        spawn_table_cell(grid, cell, false, &theme, fonts);
                    }
                });
            if let Some(caption) = variant
                .primitives
                .iter()
                .find(|primitive| primitive.part == "TableCaption")
            {
                spawn_text(
                    table,
                    caption.value.clone(),
                    fonts.inter.clone(),
                    scale::font_size::F00,
                    FontWeight::NORMAL,
                    theme.text_2().to_bevy(),
                );
            }
            if let Some(pagination) = variant
                .primitives
                .iter()
                .find(|primitive| primitive.part == "DataTablePagination")
            {
                spawn_primitive(table, UiComponentId::DataTable, pagination, &theme, fonts);
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_table_cell(
    parent: &mut ChildSpawnerCommands,
    primitive: &BevyUiPrimitive,
    header: bool,
    theme: &Theme,
    fonts: &StoryFonts,
) {
    let background = if header {
        theme.surface_2().to_bevy()
    } else if primitive.selected {
        theme.primary_soft().to_bevy()
    } else {
        Color::NONE
    };
    parent.spawn((
        Node {
            min_width: px(0.0),
            min_height: px(if header {
                scale::space::M
            } else {
                scale::space::L
            }),
            padding: UiRect::axes(px(scale::space::XS2), px(scale::space::XS3)),
            border: UiRect::bottom(px(theme.border.max(1.0))),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(background),
        BorderColor::all(theme.border_subtle().to_bevy()),
        Text::new(primitive.label.clone()),
        TextFont {
            font: fonts.inter.clone().into(),
            font_size: FontSize::Px(if header {
                scale::font_size::F00
            } else {
                scale::font_size::F0
            }),
            weight: if header {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            },
            ..default()
        },
        TextColor(if primitive.disabled {
            theme.text_disabled().to_bevy()
        } else if header {
            theme.text_muted().to_bevy()
        } else {
            primitive.text
        }),
        TextLayout::new(Justify::Left, LineBreak::WordOrCharacter),
    ));
}

#[cfg(target_arch = "wasm32")]
fn spawn_meter_primitives(
    parent: &mut ChildSpawnerCommands,
    id: UiComponentId,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    if id == UiComponentId::Spinner {
        parent
            .spawn((
                Node {
                    width: percent(100),
                    min_width: px(0.0),
                    min_height: px(scale::space::L),
                    padding: UiRect::axes(px(scale::space::XS), px(scale::space::XS2)),
                    border: UiRect::all(px(theme.border.max(1.0))),
                    border_radius: BorderRadius::all(px(theme.radius_box)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: px(scale::space::XS2),
                    ..default()
                },
                BackgroundColor(root.fill),
                BorderColor::all(theme.border_subtle().to_bevy()),
            ))
            .with_children(|spinner| {
                spinner.spawn((
                    Node {
                        width: px(scale::space::S),
                        height: px(scale::space::S),
                        min_width: px(scale::space::S),
                        border: UiRect::all(px((theme.border * 2.0).max(2.0))),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BorderColor::all(theme.brand().to_bevy()),
                ));
                spawn_text(
                    spinner,
                    root.label.clone(),
                    fonts.inter.clone(),
                    scale::font_size::F0,
                    FontWeight::SEMIBOLD,
                    root.text,
                );
            });
        return;
    }

    let track = variant.primitives.iter().find(|primitive| {
        primitive.part.ends_with("Track")
            || matches!(
                primitive.kind,
                UiWidgetSlotKind::Progress | UiWidgetSlotKind::Slider
            )
    });
    let indicator = variant.primitives.iter().find(|primitive| {
        primitive.part.ends_with("Indicator") || primitive.part.ends_with("Range")
    });
    let ratio = match (track, indicator) {
        (Some(track), Some(indicator)) if track.size.x > 0.0 => {
            (indicator.size.x / track.size.x * 100.0).clamp(8.0, 100.0)
        }
        _ => 50.0,
    };

    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: FlexDirection::Column,
                row_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|meter| {
            meter
                .spawn(Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    column_gap: px(scale::space::XS2),
                    ..default()
                })
                .with_children(|header| {
                    spawn_text(
                        header,
                        root.label.clone(),
                        fonts.inter.clone(),
                        scale::font_size::F0,
                        FontWeight::BOLD,
                        root.text,
                    );
                    spawn_text(
                        header,
                        indicator.map_or_else(
                            || root.value.clone(),
                            |indicator| indicator.label.clone(),
                        ),
                        fonts.inter.clone(),
                        scale::font_size::F00,
                        FontWeight::SEMIBOLD,
                        theme.text_muted().to_bevy(),
                    );
                });
            meter
                .spawn((
                    Node {
                        width: percent(100),
                        min_height: px(scale::space::XS),
                        border_radius: BorderRadius::MAX,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(
                        track.map_or_else(|| theme.surface_3().to_bevy(), |track| track.fill),
                    ),
                ))
                .with_children(|bar| {
                    bar.spawn((
                        Node {
                            width: percent(ratio),
                            height: percent(100),
                            min_height: px(scale::space::XS),
                            border_radius: BorderRadius::MAX,
                            ..default()
                        },
                        BackgroundColor(
                            indicator.map_or_else(
                                || theme.brand().to_bevy(),
                                |indicator| indicator.fill,
                            ),
                        ),
                    ));
                });
            if !root.value.is_empty() {
                spawn_text(
                    meter,
                    root.value.clone(),
                    fonts.inter.clone(),
                    scale::font_size::F00,
                    FontWeight::NORMAL,
                    theme.text_2().to_bevy(),
                );
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_resizable_primitives(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    let horizontal = root.size.x >= root.size.y;
    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                min_height: px(if horizontal { 180.0 } else { 260.0 }),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: if horizontal {
                    FlexDirection::Row
                } else {
                    FlexDirection::Column
                },
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(theme.surface_1().to_bevy()),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|group| {
            for primitive in variant.primitives.iter().skip(1) {
                if primitive.kind == UiWidgetSlotKind::Panel {
                    let basis = if horizontal {
                        primitive.size.x / root.size.x * 100.0
                    } else {
                        primitive.size.y / root.size.y * 100.0
                    };
                    group
                        .spawn((
                            Node {
                                min_width: px(0.0),
                                flex_basis: percent(basis.clamp(15.0, 80.0)),
                                flex_grow: 1.0,
                                padding: UiRect::all(px(scale::space::XS)),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(scale::space::XS2),
                                ..default()
                            },
                            BackgroundColor(if primitive.selected {
                                theme.primary_soft().to_bevy()
                            } else {
                                primitive.fill
                            }),
                        ))
                        .with_children(|panel| {
                            spawn_text(
                                panel,
                                primitive.label.clone(),
                                fonts.inter.clone(),
                                scale::font_size::F0,
                                FontWeight::BOLD,
                                primitive.text,
                            );
                            spawn_text(
                                panel,
                                primitive.value.clone(),
                                fonts.inter.clone(),
                                scale::font_size::F00,
                                FontWeight::NORMAL,
                                theme.text_2().to_bevy(),
                            );
                        });
                } else if primitive.kind == UiWidgetSlotKind::Separator {
                    group.spawn((
                        Node {
                            width: if horizontal {
                                px(scale::space::XS2)
                            } else {
                                percent(100)
                            },
                            height: if horizontal {
                                percent(100)
                            } else {
                                px(scale::space::XS2)
                            },
                            ..default()
                        },
                        BackgroundColor(if primitive.selected {
                            theme.brand().to_bevy()
                        } else {
                            theme.border_strong().to_bevy()
                        }),
                    ));
                }
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_scroll_area_primitives(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: FlexDirection::Column,
                row_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|area| {
            area.spawn((
                Node {
                    width: percent(100),
                    max_height: px(210.0),
                    padding: UiRect::all(px(scale::space::XS2)),
                    border: UiRect::all(px(theme.border.max(1.0))),
                    border_radius: BorderRadius::all(px(theme.radius_field)),
                    overflow: Overflow::clip_y(),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(scale::space::XS2),
                    ..default()
                },
                BackgroundColor(theme.surface_2().to_bevy()),
                BorderColor::all(theme.border_subtle().to_bevy()),
            ))
            .with_children(|viewport| {
                for item in variant
                    .primitives
                    .iter()
                    .filter(|primitive| primitive.part.starts_with("ScrollContent:"))
                {
                    viewport
                        .spawn((
                            Node {
                                width: percent(100),
                                padding: UiRect::all(px(scale::space::XS2)),
                                border: UiRect::all(px(theme.border.max(1.0))),
                                border_radius: BorderRadius::all(px(theme.radius_field)),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(scale::space::XS3),
                                ..default()
                            },
                            BackgroundColor(if item.selected {
                                theme.primary_soft().to_bevy()
                            } else {
                                theme.surface_1().to_bevy()
                            }),
                            BorderColor::all(theme.border_subtle().to_bevy()),
                        ))
                        .with_children(|copy| {
                            spawn_text(
                                copy,
                                item.label.clone(),
                                fonts.inter.clone(),
                                scale::font_size::F00,
                                FontWeight::BOLD,
                                item.text,
                            );
                            spawn_text(
                                copy,
                                item.value.clone(),
                                fonts.inter.clone(),
                                scale::font_size::F000,
                                FontWeight::NORMAL,
                                theme.text_2().to_bevy(),
                            );
                        });
                }
            });
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_separator_primitives(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    let line = variant
        .primitives
        .iter()
        .find(|primitive| primitive.kind == UiWidgetSlotKind::Separator);
    let label = variant
        .primitives
        .iter()
        .find(|primitive| primitive.part == "SeparatorLabel");
    let vertical = line.is_some_and(|line| line.size.y > line.size.x);
    parent
        .spawn((
            Node {
                width: if vertical { auto() } else { percent(100) },
                min_width: if vertical {
                    px(scale::space::XL)
                } else {
                    px(0.0)
                },
                min_height: px(if vertical {
                    scale::space::XL
                } else {
                    scale::space::L
                }),
                padding: UiRect::all(px(scale::space::XS2)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_field)),
                flex_direction: if vertical {
                    FlexDirection::Row
                } else {
                    FlexDirection::Column
                },
                align_items: AlignItems::Center,
                row_gap: px(scale::space::XS2),
                column_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|separator| {
            if let Some(line) = line {
                separator.spawn((
                    Node {
                        width: if vertical {
                            px(scale::space::XS2)
                        } else {
                            percent(100)
                        },
                        height: if vertical {
                            px(scale::space::XL)
                        } else {
                            px(scale::space::XS3)
                        },
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(line.fill),
                ));
            }
            if let Some(label) = label.filter(|label| !label.disabled) {
                spawn_text(
                    separator,
                    label.label.clone(),
                    fonts.inter.clone(),
                    scale::font_size::F00,
                    FontWeight::BOLD,
                    theme.text_muted().to_bevy(),
                );
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_skeleton_primitives(parent: &mut ChildSpawnerCommands, variant: &BevyUiStoryVariant) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                min_height: px(180.0),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: FlexDirection::Column,
                row_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|skeleton| {
            for primitive in variant.primitives.iter().skip(1) {
                let (width, height) = match primitive.kind {
                    UiWidgetSlotKind::Media => (percent(100), px(scale::space::XL)),
                    UiWidgetSlotKind::Skeleton if primitive.part.ends_with("Text") => {
                        (percent(78), px(scale::space::XS))
                    }
                    _ => (percent(100), px(scale::space::L)),
                };
                skeleton.spawn((
                    Node {
                        width,
                        height,
                        min_height: height,
                        border_radius: BorderRadius::all(px(theme.radius_field)),
                        ..default()
                    },
                    BackgroundColor(primitive.fill),
                ));
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_tabs_primitives(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    let list = variant
        .primitives
        .iter()
        .find(|primitive| primitive.part == "TabsList");
    let vertical = list.is_some_and(|list| list.size.y > list.size.x);
    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: if vertical {
                    FlexDirection::Row
                } else {
                    FlexDirection::Column
                },
                row_gap: px(scale::space::XS2),
                column_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|tabs| {
            tabs.spawn((
                Node {
                    min_width: if vertical { px(120.0) } else { px(0.0) },
                    width: if vertical { auto() } else { percent(100) },
                    padding: UiRect::all(px(scale::space::XS2)),
                    border_radius: BorderRadius::all(px(theme.radius_field)),
                    flex_direction: if vertical {
                        FlexDirection::Column
                    } else {
                        FlexDirection::Row
                    },
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: px(scale::space::XS3),
                    column_gap: px(scale::space::XS3),
                    ..default()
                },
                BackgroundColor(theme.surface_2().to_bevy()),
            ))
            .with_children(|triggers| {
                for trigger in variant
                    .primitives
                    .iter()
                    .filter(|primitive| primitive.part.starts_with("TabsTrigger:"))
                {
                    let rest = if trigger.selected {
                        theme.primary_soft().to_bevy()
                    } else {
                        Color::NONE
                    };
                    triggers.spawn((
                        Button,
                        Node {
                            min_height: px(scale::space::M),
                            padding: UiRect::axes(px(scale::space::XS2), px(scale::space::XS3)),
                            border: UiRect::all(px(theme.border.max(1.0))),
                            border_radius: BorderRadius::all(px(theme.radius_field)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(rest),
                        BorderColor::all(if trigger.selected {
                            theme.brand().to_bevy()
                        } else {
                            Color::NONE
                        }),
                        Text::new(trigger.label.clone()),
                        TextFont {
                            font: fonts.inter.clone().into(),
                            font_size: FontSize::Px(scale::font_size::F00),
                            weight: if trigger.selected {
                                FontWeight::BOLD
                            } else {
                                FontWeight::NORMAL
                            },
                            ..default()
                        },
                        TextColor(if trigger.disabled {
                            theme.text_disabled().to_bevy()
                        } else {
                            trigger.text
                        }),
                        TextLayout::new(Justify::Center, LineBreak::NoWrap),
                        InteractivePrimitive::stateful(
                            rest,
                            theme.primary_soft().to_bevy(),
                            trigger.selected,
                            &theme,
                        ),
                    ));
                }
            });
            if let Some(content) = variant
                .primitives
                .iter()
                .find(|primitive| primitive.part.starts_with("TabsContent:") && primitive.selected)
            {
                tabs.spawn((
                    Node {
                        min_width: px(0.0),
                        flex_grow: 1.0,
                        padding: UiRect::all(px(scale::space::XS)),
                        border: UiRect::all(px(theme.border.max(1.0))),
                        border_radius: BorderRadius::all(px(theme.radius_field)),
                        ..default()
                    },
                    BackgroundColor(theme.surface_elevated().to_bevy()),
                    BorderColor::all(theme.border_subtle().to_bevy()),
                    Text::new(content.value.clone()),
                    TextFont {
                        font: fonts.inter.clone().into(),
                        font_size: FontSize::Px(scale::font_size::F0),
                        ..default()
                    },
                    TextColor(theme.text_2().to_bevy()),
                    TextLayout::new(Justify::Left, LineBreak::WordOrCharacter),
                ));
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_typography_primitives(
    parent: &mut ChildSpawnerCommands,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    parent
        .spawn(Node {
            width: percent(100),
            min_width: px(0.0),
            padding: if root.selected {
                UiRect::all(px(scale::space::XS))
            } else {
                UiRect::ZERO
            },
            flex_direction: FlexDirection::Column,
            row_gap: px(scale::space::XS),
            ..default()
        })
        .with_children(|typography| {
            for primitive in variant.primitives.iter().skip(1) {
                if primitive.part == "TypographyList" {
                    continue;
                }
                let (copy, size, weight, color) = if primitive.part == "TypographyH1" {
                    (
                        primitive.label.clone(),
                        scale::font_size::F3,
                        FontWeight::BOLD,
                        primitive.text,
                    )
                } else if primitive.part == "TypographyH2" {
                    (
                        primitive.label.clone(),
                        scale::font_size::F2,
                        FontWeight::BOLD,
                        primitive.text,
                    )
                } else if primitive.part.starts_with("TypographyList:") {
                    (
                        format!("• {}", primitive.label),
                        scale::font_size::F0,
                        FontWeight::NORMAL,
                        theme.text_2().to_bevy(),
                    )
                } else {
                    (
                        primitive.value.clone(),
                        scale::font_size::F0,
                        FontWeight::NORMAL,
                        theme.text_2().to_bevy(),
                    )
                };
                if primitive.part == "TypographyBlockquote" {
                    typography
                        .spawn((
                            Node {
                                width: percent(100),
                                padding: UiRect::left(px(scale::space::XS)),
                                border: UiRect::left(px((theme.border * 3.0).max(3.0))),
                                ..default()
                            },
                            BorderColor::all(theme.brand().to_bevy()),
                        ))
                        .with_children(|quote| {
                            spawn_text(quote, copy, fonts.inter.clone(), size, weight, color);
                        });
                } else {
                    spawn_text(
                        typography,
                        copy,
                        fonts.inter.clone(),
                        size,
                        weight,
                        if primitive.disabled {
                            theme.text_disabled().to_bevy()
                        } else {
                            color
                        },
                    );
                }
            }
        });
}

#[cfg(target_arch = "wasm32")]
const fn component_is_control_row(id: UiComponentId) -> bool {
    matches!(
        id,
        UiComponentId::Badge
            | UiComponentId::Button
            | UiComponentId::ButtonGroup
            | UiComponentId::Input
            | UiComponentId::InputGroup
            | UiComponentId::InputOtp
            | UiComponentId::Kbd
            | UiComponentId::Label
            | UiComponentId::Marker
            | UiComponentId::Menubar
            | UiComponentId::NativeSelect
            | UiComponentId::Pagination
            | UiComponentId::Toggle
            | UiComponentId::ToggleGroup
    )
}

#[cfg(target_arch = "wasm32")]
fn spawn_control_row_primitives(
    parent: &mut ChildSpawnerCommands,
    id: UiComponentId,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    if matches!(
        id,
        UiComponentId::Badge
            | UiComponentId::Button
            | UiComponentId::Label
            | UiComponentId::Marker
            | UiComponentId::Toggle
    ) {
        let primary = variant
            .primitives
            .iter()
            .rev()
            .find(|primitive| {
                !matches!(
                    primitive.kind,
                    UiWidgetSlotKind::Section | UiWidgetSlotKind::Marker
                )
            })
            .unwrap_or(root);
        let interactive = primary.intent != UiWidgetIntent::None && !primary.disabled;
        let background = root.fill;
        let mut entity = parent.spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                min_height: px(root.size.y.max(scale::space::M)),
                padding: UiRect::axes(px(scale::space::XS), px(scale::space::XS2)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: if id == UiComponentId::Badge {
                    BorderRadius::MAX
                } else {
                    BorderRadius::all(px(theme.radius_field))
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(background),
            BorderColor::all(if primary.selected {
                theme.brand().to_bevy()
            } else {
                theme.border_subtle().to_bevy()
            }),
            Text::new(primary.label.clone()),
            TextFont {
                font: fonts.inter.clone().into(),
                font_size: FontSize::Px(if id == UiComponentId::Badge {
                    scale::font_size::F00
                } else {
                    scale::font_size::F0
                }),
                weight: FontWeight::BOLD,
                ..default()
            },
            TextColor(if primary.disabled {
                theme.text_disabled().to_bevy()
            } else {
                primary.text
            }),
            TextLayout::new(Justify::Center, LineBreak::WordOrCharacter),
        ));
        if interactive {
            entity.insert((
                Button,
                InteractivePrimitive::stateful(
                    background,
                    theme.selected_tint().to_bevy(),
                    primary.selected,
                    &theme,
                ),
            ));
        }
        return;
    }

    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                min_height: px(scale::space::L),
                padding: UiRect::all(px(scale::space::XS2)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_field)),
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                align_items: AlignItems::Center,
                row_gap: px(scale::space::XS3),
                column_gap: px(scale::space::XS3),
                ..default()
            },
            BackgroundColor(root.fill),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|row| {
            for primitive in variant.primitives.iter().skip(1).filter(|primitive| {
                !matches!(
                    primitive.kind,
                    UiWidgetSlotKind::Section | UiWidgetSlotKind::List | UiWidgetSlotKind::ListItem
                ) && primitive.role != UiBlockRole::Layout
            }) {
                spawn_row_primitive(row, primitive, &theme, fonts);
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn spawn_row_primitive(
    parent: &mut ChildSpawnerCommands,
    primitive: &BevyUiPrimitive,
    theme: &Theme,
    fonts: &StoryFonts,
) {
    let grow = matches!(
        primitive.kind,
        UiWidgetSlotKind::Input | UiWidgetSlotKind::Select | UiWidgetSlotKind::Text
    );
    let background = primitive_background(primitive);
    let interactive = primitive.intent != UiWidgetIntent::None && !primitive.disabled;
    let mut entity = parent.spawn((
        Node {
            min_width: px(if grow { 120.0 } else { scale::space::S }),
            flex_grow: if grow { 1.0 } else { 0.0 },
            min_height: px(primitive_min_height(primitive)),
            padding: primitive_padding(primitive.kind),
            border: if primitive_has_border(primitive.kind) {
                UiRect::all(px(theme.border.max(1.0)))
            } else {
                UiRect::ZERO
            },
            border_radius: primitive_radius(primitive.kind, theme),
            justify_content: primitive_justify(primitive.kind),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(background),
        BorderColor::all(if primitive.selected {
            theme.brand().to_bevy()
        } else {
            theme.border_subtle().to_bevy()
        }),
        Text::new(primitive_copy(primitive)),
        TextFont {
            font: fonts.inter.clone().into(),
            font_size: FontSize::Px(primitive_font_size(UiComponentId::Button, primitive)),
            weight: primitive_font_weight(primitive),
            ..default()
        },
        TextColor(if primitive.disabled {
            theme.text_disabled().to_bevy()
        } else {
            primitive.text
        }),
        TextLayout::new(Justify::Left, LineBreak::WordOrCharacter),
    ));
    if interactive {
        entity.insert((
            Button,
            InteractivePrimitive::stateful(
                background,
                theme.selected_tint().to_bevy(),
                primitive.selected,
                theme,
            ),
        ));
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_stack_primitives(
    parent: &mut ChildSpawnerCommands,
    id: UiComponentId,
    variant: &BevyUiStoryVariant,
    fonts: &StoryFonts,
) {
    let theme = variant.theme_id.palette();
    let Some(root) = story_root(variant) else {
        return;
    };
    parent
        .spawn((
            Node {
                width: percent(100),
                min_width: px(0.0),
                min_height: px(scale::space::L),
                padding: UiRect::all(px(scale::space::XS)),
                border: UiRect::all(px(theme.border.max(1.0))),
                border_radius: BorderRadius::all(px(theme.radius_box)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                row_gap: px(scale::space::XS2),
                ..default()
            },
            BackgroundColor(if component_is_overlay(id) {
                theme.surface_1().to_bevy()
            } else {
                root.fill
            }),
            BorderColor::all(theme.border_subtle().to_bevy()),
        ))
        .with_children(|stack| {
            for primitive in variant.primitives.iter().skip(1) {
                if matches!(
                    primitive.kind,
                    UiWidgetSlotKind::List
                        | UiWidgetSlotKind::ListItem
                        | UiWidgetSlotKind::Overlay
                        | UiWidgetSlotKind::Row
                        | UiWidgetSlotKind::Section
                        | UiWidgetSlotKind::Table
                ) || primitive.role == UiBlockRole::Layout
                {
                    continue;
                }
                if matches!(
                    primitive.kind,
                    UiWidgetSlotKind::Header | UiWidgetSlotKind::Panel
                ) {
                    let copy = if !primitive.value.is_empty() && primitive.value != primitive.label
                    {
                        primitive.value.clone()
                    } else {
                        primitive.label.clone()
                    };
                    stack.spawn((
                        Node {
                            width: percent(100),
                            min_width: px(0.0),
                            padding: if primitive.kind == UiWidgetSlotKind::Panel {
                                UiRect::all(px(scale::space::XS2))
                            } else {
                                UiRect::ZERO
                            },
                            border: if primitive.kind == UiWidgetSlotKind::Panel {
                                UiRect::all(px(theme.border.max(1.0)))
                            } else {
                                UiRect::ZERO
                            },
                            border_radius: BorderRadius::all(px(theme.radius_field)),
                            ..default()
                        },
                        BackgroundColor(if primitive.kind == UiWidgetSlotKind::Panel {
                            primitive.fill
                        } else {
                            Color::NONE
                        }),
                        BorderColor::all(theme.border_subtle().to_bevy()),
                        Text::new(copy),
                        TextFont {
                            font: fonts.inter.clone().into(),
                            font_size: FontSize::Px(
                                if primitive.kind == UiWidgetSlotKind::Header {
                                    scale::font_size::F1
                                } else {
                                    scale::font_size::F0
                                },
                            ),
                            weight: if primitive.kind == UiWidgetSlotKind::Header {
                                FontWeight::BOLD
                            } else {
                                FontWeight::NORMAL
                            },
                            ..default()
                        },
                        TextColor(if primitive.disabled {
                            theme.text_disabled().to_bevy()
                        } else {
                            primitive.text
                        }),
                        TextLayout::new(Justify::Left, LineBreak::WordOrCharacter),
                    ));
                    continue;
                }
                spawn_primitive(stack, id, primitive, &theme, fonts);
            }
        });
}

#[cfg(target_arch = "wasm32")]
fn story_root(variant: &BevyUiStoryVariant) -> Option<&BevyUiPrimitive> {
    variant
        .primitives
        .iter()
        .find(|primitive| primitive.role == UiBlockRole::Root)
        .or_else(|| variant.primitives.first())
}

#[cfg(target_arch = "wasm32")]
fn avatar_initials(label: &str) -> String {
    let mut words = label
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2);
    let first = words.next().unwrap_or('?');
    let second = words.next();
    second.map_or_else(
        || first.to_uppercase().collect(),
        |second| format!("{}{}", first.to_uppercase(), second.to_uppercase()),
    )
}

#[cfg(target_arch = "wasm32")]
const fn component_is_overlay(id: UiComponentId) -> bool {
    matches!(
        id,
        UiComponentId::AlertDialog
            | UiComponentId::ContextMenu
            | UiComponentId::Dialog
            | UiComponentId::Drawer
            | UiComponentId::DropdownMenu
            | UiComponentId::HoverCard
            | UiComponentId::Popover
            | UiComponentId::Select
            | UiComponentId::Sheet
            | UiComponentId::Tooltip
    )
}

#[cfg(target_arch = "wasm32")]
fn spawn_centered_label(
    parent: &mut ChildSpawnerCommands,
    value: impl Into<String>,
    min_height: f32,
    font_size: f32,
    color: Color,
    fonts: &StoryFonts,
) {
    parent.spawn((
        Node {
            min_width: px(0.0),
            min_height: px(min_height),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Text::new(value),
        TextFont {
            font: fonts.inter.clone().into(),
            font_size: FontSize::Px(font_size),
            weight: FontWeight::SEMIBOLD,
            ..default()
        },
        TextColor(color),
        TextLayout::new(Justify::Center, LineBreak::NoWrap),
    ));
}

#[cfg(target_arch = "wasm32")]
fn spawn_primitive(
    parent: &mut ChildSpawnerCommands,
    id: UiComponentId,
    primitive: &BevyUiPrimitive,
    theme: &Theme,
    fonts: &StoryFonts,
) {
    if primitive.kind == UiWidgetSlotKind::Separator {
        if id == UiComponentId::Breadcrumb {
            spawn_text(
                parent,
                primitive.label.clone(),
                fonts.inter.clone(),
                scale::font_size::F0,
                FontWeight::NORMAL,
                theme.text_muted().to_bevy(),
            );
            return;
        }
        parent.spawn((
            Node {
                width: if primitive.size.x >= primitive.size.y {
                    percent(100)
                } else {
                    px(theme.border.max(1.0))
                },
                height: if primitive.size.x >= primitive.size.y {
                    px(theme.border.max(1.0))
                } else {
                    px(scale::space::L)
                },
                ..default()
            },
            BackgroundColor(primitive.fill),
        ));
        return;
    }

    let full_width = primitive_uses_full_width(id, primitive.kind);
    let compact = primitive_is_compact(primitive.kind);
    let interactive = primitive.intent != UiWidgetIntent::None && !primitive.disabled;
    let copy = primitive_copy(primitive);
    let background = primitive_background(primitive);
    let text_color = if primitive.disabled {
        primitive.text.with_alpha(0.38)
    } else {
        primitive.text
    };
    let font_size = primitive_font_size(id, primitive);
    let weight = primitive_font_weight(primitive);
    let font = fonts.inter.clone();
    let min_height = primitive_min_height(primitive);
    let mut entity = parent.spawn((
        Node {
            width: if full_width { percent(100) } else { auto() },
            max_width: percent(100),
            min_width: if full_width {
                px(0.0)
            } else if compact {
                px(primitive.size.x.max(scale::space::S))
            } else {
                px(primitive.size.x.max(40.0))
            },
            min_height: px(min_height),
            padding: primitive_padding(primitive.kind),
            border: if primitive_has_border(primitive.kind) {
                UiRect::all(px(theme.border.max(1.0)))
            } else {
                UiRect::ZERO
            },
            border_radius: primitive_radius(primitive.kind, theme),
            justify_content: primitive_justify(primitive.kind),
            align_items: AlignItems::Center,
            ..default()
        },
        Text::new(copy),
        TextFont {
            font: font.into(),
            font_size: FontSize::Px(font_size),
            weight,
            ..default()
        },
        TextColor(text_color),
        TextLayout::new(Justify::Left, LineBreak::WordOrCharacter),
        BackgroundColor(background),
        BorderColor::all(if primitive.selected {
            theme.brand().to_bevy()
        } else {
            theme.border_subtle().to_bevy()
        }),
    ));

    if interactive {
        let rest = background;
        entity.insert((
            Button,
            InteractivePrimitive::stateful(
                rest,
                theme.selected_tint().to_bevy(),
                primitive.selected,
                theme,
            ),
        ));
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_text(
    parent: &mut ChildSpawnerCommands,
    value: impl Into<String>,
    font: Handle<Font>,
    font_size: f32,
    weight: FontWeight,
    color: Color,
) {
    parent.spawn((
        Node {
            min_width: px(0.0),
            ..default()
        },
        Text::new(value),
        TextFont {
            font: font.into(),
            font_size: FontSize::Px(font_size),
            weight,
            ..default()
        },
        TextColor(color),
        TextLayout::new(Justify::Left, LineBreak::WordOrCharacter),
    ));
}

#[cfg(target_arch = "wasm32")]
fn scroll_story_viewport(
    mut wheel: MessageReader<MouseWheel>,
    mut viewport: Query<(&mut ScrollPosition, &ComputedNode), With<StoryViewport>>,
) {
    let Ok((mut scroll_position, computed)) = viewport.single_mut() else {
        return;
    };
    let max_y =
        (computed.content_size().y - computed.size().y).max(0.0) * computed.inverse_scale_factor();

    for event in wheel.read() {
        let delta = match event.unit {
            MouseScrollUnit::Line => event.y * SCROLL_LINE_HEIGHT,
            MouseScrollUnit::Pixel => event.y,
        };
        scroll_position.y = (scroll_position.y - delta).clamp(0.0, max_y);
    }
}

#[cfg(target_arch = "wasm32")]
fn update_story_grid_columns(windows: Query<&Window>, mut grids: Query<(&StoryGrid, &mut Node)>) {
    let Ok(window) = windows.single() else {
        return;
    };
    for (grid, mut node) in &mut grids {
        let columns = story_grid_column_count(window.width(), grid.single_column);
        let expected = RepeatedGridTrack::fr(columns, 1.0);
        if node.grid_template_columns != expected {
            node.grid_template_columns = expected;
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn update_interactive_primitives(
    mut interactions: Query<
        (
            &Interaction,
            &mut InteractivePrimitive,
            &mut BackgroundColor,
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut colors, mut background) in &mut interactions {
        if *interaction == Interaction::Pressed && colors.stateful {
            colors.latched = !colors.latched;
        }
        background.0 = match interaction {
            Interaction::Pressed => colors.pressed,
            Interaction::Hovered => colors.hover,
            Interaction::None if colors.latched => colors.selected,
            Interaction::None => colors.rest,
        };
    }
}

#[cfg(target_arch = "wasm32")]
const fn story_grid_column_count(width: f32, single_column: bool) -> u16 {
    if single_column || width < RESPONSIVE_BREAKPOINT {
        1
    } else {
        2
    }
}

#[cfg(target_arch = "wasm32")]
const fn primitive_uses_full_width(id: UiComponentId, kind: UiWidgetSlotKind) -> bool {
    if matches!(id, UiComponentId::Breadcrumb) {
        return false;
    }
    if matches!(
        id,
        UiComponentId::Combobox
            | UiComponentId::Command
            | UiComponentId::ContextMenu
            | UiComponentId::DropdownMenu
            | UiComponentId::NavigationMenu
            | UiComponentId::Select
            | UiComponentId::Sidebar
    ) && matches!(
        kind,
        UiWidgetSlotKind::Button | UiWidgetSlotKind::Link | UiWidgetSlotKind::Option
    ) {
        return true;
    }
    matches!(
        kind,
        UiWidgetSlotKind::Chart
            | UiWidgetSlotKind::Description
            | UiWidgetSlotKind::Header
            | UiWidgetSlotKind::Input
            | UiWidgetSlotKind::Media
            | UiWidgetSlotKind::Progress
            | UiWidgetSlotKind::Select
            | UiWidgetSlotKind::Skeleton
            | UiWidgetSlotKind::Slider
            | UiWidgetSlotKind::Text
            | UiWidgetSlotKind::Textarea
            | UiWidgetSlotKind::Title
    )
}

#[cfg(target_arch = "wasm32")]
const fn primitive_is_compact(kind: UiWidgetSlotKind) -> bool {
    matches!(
        kind,
        UiWidgetSlotKind::Avatar
            | UiWidgetSlotKind::Badge
            | UiWidgetSlotKind::Checkbox
            | UiWidgetSlotKind::IconButton
            | UiWidgetSlotKind::Key
            | UiWidgetSlotKind::Marker
            | UiWidgetSlotKind::Radio
            | UiWidgetSlotKind::Spinner
            | UiWidgetSlotKind::Switch
    )
}

#[cfg(target_arch = "wasm32")]
const fn primitive_has_border(kind: UiWidgetSlotKind) -> bool {
    matches!(
        kind,
        UiWidgetSlotKind::Badge
            | UiWidgetSlotKind::Button
            | UiWidgetSlotKind::Cell
            | UiWidgetSlotKind::Checkbox
            | UiWidgetSlotKind::IconButton
            | UiWidgetSlotKind::Input
            | UiWidgetSlotKind::Key
            | UiWidgetSlotKind::Media
            | UiWidgetSlotKind::Option
            | UiWidgetSlotKind::Radio
            | UiWidgetSlotKind::Select
            | UiWidgetSlotKind::Slider
            | UiWidgetSlotKind::Switch
            | UiWidgetSlotKind::Textarea
    )
}

#[cfg(target_arch = "wasm32")]
fn primitive_background(primitive: &BevyUiPrimitive) -> Color {
    match primitive.kind {
        UiWidgetSlotKind::Description
        | UiWidgetSlotKind::Header
        | UiWidgetSlotKind::Link
        | UiWidgetSlotKind::Text
        | UiWidgetSlotKind::Title => Color::NONE,
        _ if primitive.disabled => primitive.fill.with_alpha(0.42),
        _ => primitive.fill,
    }
}

#[cfg(target_arch = "wasm32")]
fn primitive_copy(primitive: &BevyUiPrimitive) -> String {
    match primitive.kind {
        UiWidgetSlotKind::Description | UiWidgetSlotKind::Textarea => {
            if primitive.value.trim().is_empty() {
                primitive.label.clone()
            } else {
                primitive.value.clone()
            }
        }
        UiWidgetSlotKind::Input | UiWidgetSlotKind::Select => {
            if primitive.value.trim().is_empty() || primitive.value == primitive.label {
                primitive.label.clone()
            } else {
                format!("{}\n{}", primitive.label, primitive.value)
            }
        }
        _ => primitive.label.clone(),
    }
}

#[cfg(target_arch = "wasm32")]
fn primitive_font_size(id: UiComponentId, primitive: &BevyUiPrimitive) -> f32 {
    if id == UiComponentId::Typography && primitive.kind == UiWidgetSlotKind::Title {
        return scale::font_size::F2;
    }
    match primitive.kind {
        UiWidgetSlotKind::Title => scale::font_size::F1,
        UiWidgetSlotKind::Badge
        | UiWidgetSlotKind::Key
        | UiWidgetSlotKind::Marker
        | UiWidgetSlotKind::Spinner => scale::font_size::F00,
        _ => scale::font_size::F0,
    }
}

#[cfg(target_arch = "wasm32")]
const fn primitive_font_weight(primitive: &BevyUiPrimitive) -> FontWeight {
    match primitive.kind {
        UiWidgetSlotKind::Title => FontWeight::BOLD,
        UiWidgetSlotKind::Badge
        | UiWidgetSlotKind::Button
        | UiWidgetSlotKind::IconButton
        | UiWidgetSlotKind::Key
        | UiWidgetSlotKind::Link => FontWeight::SEMIBOLD,
        _ => FontWeight::NORMAL,
    }
}

#[cfg(target_arch = "wasm32")]
fn primitive_min_height(primitive: &BevyUiPrimitive) -> f32 {
    match primitive.kind {
        UiWidgetSlotKind::Avatar => primitive.size.y.max(scale::space::L),
        UiWidgetSlotKind::Chart | UiWidgetSlotKind::Media => primitive.size.y.max(scale::space::XL),
        UiWidgetSlotKind::Description
        | UiWidgetSlotKind::Header
        | UiWidgetSlotKind::Text
        | UiWidgetSlotKind::Title => 0.0,
        _ => scale::space::L,
    }
}

#[cfg(target_arch = "wasm32")]
fn primitive_padding(kind: UiWidgetSlotKind) -> UiRect {
    match kind {
        UiWidgetSlotKind::Description
        | UiWidgetSlotKind::Header
        | UiWidgetSlotKind::Link
        | UiWidgetSlotKind::Text
        | UiWidgetSlotKind::Title => UiRect::ZERO,
        UiWidgetSlotKind::Badge | UiWidgetSlotKind::Key | UiWidgetSlotKind::Marker => {
            UiRect::axes(px(scale::space::XS2), px(scale::space::XS3))
        }
        _ => UiRect::axes(px(scale::space::XS), px(scale::space::XS2)),
    }
}

#[cfg(target_arch = "wasm32")]
fn primitive_radius(kind: UiWidgetSlotKind, theme: &Theme) -> BorderRadius {
    if matches!(
        kind,
        UiWidgetSlotKind::Avatar
            | UiWidgetSlotKind::Badge
            | UiWidgetSlotKind::Checkbox
            | UiWidgetSlotKind::Marker
            | UiWidgetSlotKind::Radio
            | UiWidgetSlotKind::Spinner
            | UiWidgetSlotKind::Switch
    ) {
        BorderRadius::MAX
    } else {
        BorderRadius::all(px(theme.radius_field))
    }
}

#[cfg(target_arch = "wasm32")]
const fn primitive_justify(kind: UiWidgetSlotKind) -> JustifyContent {
    if matches!(
        kind,
        UiWidgetSlotKind::Button
            | UiWidgetSlotKind::IconButton
            | UiWidgetSlotKind::Key
            | UiWidgetSlotKind::Marker
    ) {
        JustifyContent::Center
    } else {
        JustifyContent::FlexStart
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn parser_accepts_catalog_story_routes() {
        assert_eq!(
            story_id_from_search("?story=ui-alert"),
            Some(StoryId::Component(UiComponentId::Alert))
        );
        assert_eq!(
            story_id_from_search("foo=bar&story=ui-typography"),
            Some(StoryId::Component(UiComponentId::Typography))
        );
        let block = block_by_slug("marketing-sections-heroes-01-simple-centered")
            .expect("the first block route remains registered");
        assert_eq!(
            story_id_from_search("?story=block-marketing-sections-heroes-01-simple-centered"),
            Some(StoryId::Block(block.id))
        );
    }

    #[test]
    fn parser_rejects_unknown_or_unprefixed_story_routes() {
        assert_eq!(story_id_from_search("?story=button"), None);
        assert_eq!(story_id_from_search("?story=ui-not-registered"), None);
        assert_eq!(story_id_from_search("?story=block-not-registered"), None);
    }

    #[test]
    fn responsive_grid_never_squeezes_narrow_stories() {
        assert_eq!(story_grid_column_count(960.0, false), 2);
        assert_eq!(story_grid_column_count(719.0, false), 1);
        assert_eq!(story_grid_column_count(960.0, true), 1);
    }

    #[test]
    fn block_coordinates_map_to_the_ui_node_origin() {
        let content_size = Vec2::new(1_200.0, 664.0);
        assert_eq!(
            block_primitive_origin(Vec2::ZERO, Vec2::new(200.0, 100.0), content_size),
            Vec2::new(500.0, 282.0)
        );
        assert_eq!(
            block_primitive_origin(
                Vec2::new(-500.0, 282.0),
                Vec2::new(200.0, 100.0),
                content_size,
            ),
            Vec2::ZERO
        );
    }
}
