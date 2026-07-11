use bevy::prelude::{Color, Vec2};
use rs_dean_ui::{
    BevyUiFlow, BevyUiPrimitive, ButtonSize, ButtonVariant, CardDensity, CardLayoutMetrics,
    GridAlign, GridJustify, LayoutRect, MediaRatio, SectionSurface, Theme, UiBlockRole,
    UiComponentId, UiStoryModel, UiWidgetSlotKind, aspect_ratio_layout_metrics,
    bevy_primitives_for_component, bevy_ui_flow, canonical_ui_story_fixture, card_layout_metrics,
    checkbox_layout_metrics, command_layout_metrics, drawer_layout_metrics, field_layout_metrics,
    scale,
};

use crate::{BlockInstance, plan_block};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BevyBlockPrimitiveKind {
    Section,
    Heading,
    Text,
    Action,
    Item,
    Media,
    UiComponentHost(UiComponentId),
    UiComponent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BevyBlockPrimitive {
    pub kind: BevyBlockPrimitiveKind,
    pub part: String,
    pub label: String,
    pub value: String,
    pub center: Vec2,
    pub size: Vec2,
    pub fill: Color,
    pub border: Color,
    pub border_width: f32,
    pub text: Color,
    pub text_align: GridAlign,
    pub font_size: f32,
    pub font_weight: u16,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub ui: Option<BevyUiPrimitive>,
}

#[derive(Debug)]
struct CopyMetrics {
    inline_size: f32,
    eyebrow_font_size: f32,
    heading_font_size: f32,
    body_font_size: f32,
    eyebrow_height: f32,
    heading_height: f32,
    body_height: f32,
    stack_gap: f32,
    action_gap: f32,
    action_rows: Vec<ActionRow>,
    height: f32,
}

#[derive(Debug)]
struct ActionRow {
    items: Vec<(usize, Vec2)>,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Copy)]
struct SupportMetrics {
    media_height: f32,
    collection_height: f32,
    component_height: f32,
    height: f32,
}

#[derive(Debug)]
struct ItemRowMetrics {
    width: f32,
    height: f32,
    padding: f32,
    gap: f32,
    media_size: f32,
    content_width: f32,
    title_height: f32,
    description_height: f32,
    content_gap: f32,
    action_gap: f32,
    action_sizes: Vec<Vec2>,
    action_width: f32,
}

#[derive(Debug)]
struct CollectionMetrics {
    columns: usize,
    track_width: f32,
    gap: f32,
    cards: Vec<CardLayoutMetrics>,
    row_heights: Vec<f32>,
    height: f32,
}

pub fn bevy_block_primitives(
    instance: &BlockInstance,
    theme: &Theme,
    viewport: Vec2,
) -> Result<Vec<BevyBlockPrimitive>, garde::Report> {
    let plan = plan_block(instance)?;
    let gutter = plan.container.gutter.points_at(viewport.x);
    let max_width = plan.container.width.points();
    let outer_width = if max_width.is_finite() {
        viewport.x.min(max_width)
    } else {
        viewport.x
    };
    let width = (outer_width - gutter * 2.0).max(240.0);
    let gap = plan.outer_grid.gap.points_at(viewport.x);
    let stacked = plan.layout.outer_grid().columns(viewport.x) == 1;
    let (content_x, content_width, support_x, support_width) = if !plan.has_support() || stacked {
        (0.0, width, 0.0, width)
    } else {
        let columns = plan.layout.outer_grid().columns(viewport.x);
        let (content_ratio, support_ratio) = if columns == 2 {
            (0.5, 0.5)
        } else {
            let total_columns = 12.0;
            (
                f32::from(plan.content_item.span.columns()) / total_columns,
                f32::from(plan.support_item.span.columns()) / total_columns,
            )
        };
        let content_width = (width - gap) * content_ratio;
        let support_width = (width - gap) * support_ratio;
        let (content, support) = if plan.reverse {
            (
                (width - content_width) / 2.0,
                -(width - support_width) / 2.0,
            )
        } else {
            (
                -(width - content_width) / 2.0,
                (width - support_width) / 2.0,
            )
        };
        (content, content_width, support, support_width)
    };

    let copy = copy_metrics(&plan, content_width, viewport.x);
    let support = support_metrics(&plan, theme, support_width, viewport.x);
    let inner_height = if !plan.has_support() {
        copy.height
    } else if stacked {
        copy.height + gap + support.height
    } else {
        copy.height.max(support.height)
    };
    let section_padding = plan.section.padding_block.points_at(viewport.x);
    let section_height = (inner_height + section_padding * 2.0).max(1.0);
    let content_y = if plan.has_support() && stacked {
        section_height / 2.0 - section_padding - copy.height / 2.0
    } else {
        0.0
    };
    let support_y = if plan.has_support() && stacked {
        section_height / 2.0 - section_padding - copy.height - gap - support.height / 2.0
    } else {
        0.0
    };
    let content_center = Vec2::new(content_x, content_y);
    let content_size = Vec2::new(content_width, copy.height);
    let support_center = Vec2::new(support_x, support_y);
    let support_size = Vec2::new(support_width, support.height);

    let (section_fill, section_text) = section_colors(plan.section.surface, theme);
    let mut primitives = vec![BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::Section,
        part: "section".to_owned(),
        label: plan.block.definition().name.to_owned(),
        value: plan.block.definition().slug.to_owned(),
        center: Vec2::ZERO,
        size: Vec2::new(viewport.x.max(width), section_height),
        fill: section_fill,
        border: Color::NONE,
        border_width: 0.0,
        text: section_text,
        text_align: GridAlign::Stretch,
        font_size: 0.0,
        font_weight: scale::weight::W4,
        line_height: scale::line_height::LH0,
        letter_spacing: 0.0,
        ui: None,
    }];

    push_copy_primitives(
        &mut primitives,
        &plan,
        theme,
        content_center,
        content_size,
        &copy,
    );
    push_support_primitives(
        &mut primitives,
        &plan,
        theme,
        support_center,
        support_size,
        viewport.x,
        support,
    );
    Ok(primitives)
}

fn section_colors(surface: SectionSurface, theme: &Theme) -> (Color, Color) {
    match surface {
        SectionSurface::Transparent => (Color::NONE, theme.text_1().to_bevy()),
        SectionSurface::Surface => (theme.surface_1().to_bevy(), theme.text_1().to_bevy()),
        SectionSurface::Subtle => (theme.surface_2().to_bevy(), theme.text_1().to_bevy()),
        SectionSurface::Elevated => (theme.surface_elevated().to_bevy(), theme.text_1().to_bevy()),
        SectionSurface::Sunken => (theme.surface_sunken().to_bevy(), theme.text_1().to_bevy()),
        SectionSurface::Brand => (theme.brand().to_bevy(), theme.text_on_brand().to_bevy()),
        SectionSurface::Accent => (theme.accent_soft().to_bevy(), theme.text_1().to_bevy()),
        SectionSurface::Contrast => (theme.neutral.to_bevy(), theme.neutral_content.to_bevy()),
    }
}

fn push_copy_primitives(
    output: &mut Vec<BevyBlockPrimitive>,
    plan: &crate::BlockPlan,
    theme: &Theme,
    center: Vec2,
    size: Vec2,
    metrics: &CopyMetrics,
) {
    let left = center.x - size.x / 2.0;
    let (eyebrow_text, heading_text, body_text) = copy_colors(plan.section.surface, theme);
    let mut cursor = center.y + size.y / 2.0;
    for (kind, part, label, value, font_size, font_weight, line_height, letter_spacing, height) in [
        (
            BevyBlockPrimitiveKind::Text,
            "eyebrow",
            plan.content.eyebrow.to_uppercase(),
            plan.pattern.label(),
            metrics.eyebrow_font_size,
            scale::weight::W7,
            scale::line_height::LH00,
            0.08,
            metrics.eyebrow_height,
        ),
        (
            BevyBlockPrimitiveKind::Heading,
            "title",
            plan.content.title.clone(),
            plan.block.definition().slug,
            metrics.heading_font_size,
            scale::weight::W8,
            if plan.pattern == crate::BlockPattern::Hero {
                scale::line_height::LH6
            } else {
                scale::line_height::LH4
            },
            0.0,
            metrics.heading_height,
        ),
        (
            BevyBlockPrimitiveKind::Text,
            "body",
            plan.content.body.clone(),
            plan.pattern.summary(),
            metrics.body_font_size,
            scale::weight::W4,
            scale::line_height::LH1,
            0.0,
            metrics.body_height,
        ),
    ]
    .into_iter()
    {
        let primitive_center = cursor - height / 2.0;
        output.push(BevyBlockPrimitive {
            kind,
            part: part.to_owned(),
            label,
            value: value.to_owned(),
            center: Vec2::new(left + size.x / 2.0, primitive_center),
            size: Vec2::new(size.x, height),
            fill: Color::NONE,
            border: Color::NONE,
            border_width: 0.0,
            text: match part {
                "eyebrow" => eyebrow_text,
                "body" => body_text,
                _ => heading_text,
            },
            text_align: plan.content_stack.align,
            font_size,
            font_weight,
            line_height,
            letter_spacing,
            ui: None,
        });
        cursor -= height + metrics.stack_gap;
    }

    for row in &metrics.action_rows {
        let action_y = cursor - row.height / 2.0;
        let mut action_left = if plan.action_cluster.justify == GridJustify::Center {
            center.x - row.width / 2.0
        } else {
            left
        };
        for (index, action_size) in &row.items {
            let action = &plan.content.actions[*index];
            let (fill, border, text) = bevy_action_colors(action.variant, theme);
            let action_x = action_left + action_size.x / 2.0;
            action_left += action_size.x + metrics.action_gap;
            output.push(BevyBlockPrimitive {
                kind: BevyBlockPrimitiveKind::Action,
                part: "action".to_owned(),
                label: action.label.clone(),
                value: action.value.clone(),
                center: Vec2::new(action_x, action_y),
                size: *action_size,
                fill,
                border,
                border_width: 1.0,
                text,
                text_align: GridAlign::Center,
                font_size: match action.size {
                    ButtonSize::Small => scale::font_size::f00(metrics.inline_size),
                    ButtonSize::Medium | ButtonSize::Icon => {
                        scale::font_size::f0(metrics.inline_size)
                    }
                    ButtonSize::Large => scale::font_size::f1(metrics.inline_size),
                },
                font_weight: scale::weight::W7,
                line_height: match action.size {
                    ButtonSize::Large => scale::line_height::LH2,
                    ButtonSize::Small | ButtonSize::Medium | ButtonSize::Icon => {
                        scale::line_height::LH0
                    }
                },
                letter_spacing: 0.0,
                ui: None,
            });
        }
        cursor -= row.height + metrics.action_gap;
    }
}

fn copy_metrics(plan: &crate::BlockPlan, width: f32, inline_size: f32) -> CopyMetrics {
    let eyebrow_font_size = scale::font_size::f00(inline_size);
    let heading_font_size = if plan.pattern == crate::BlockPattern::Hero {
        scale::font_size::f6(inline_size)
    } else {
        scale::font_size::f4(inline_size)
    };
    let body_font_size = scale::font_size::f1(inline_size);
    let eyebrow_height = text_block_height(
        &plan.content.eyebrow,
        width,
        eyebrow_font_size,
        scale::line_height::LH00,
    );
    let heading_height = text_block_height(
        &plan.content.title,
        width,
        heading_font_size,
        if plan.pattern == crate::BlockPattern::Hero {
            scale::line_height::LH6
        } else {
            scale::line_height::LH4
        },
    );
    let body_height = text_block_height(
        &plan.content.body,
        width,
        body_font_size,
        scale::line_height::LH1,
    );
    let stack_gap = plan.content_stack.gap.points_at(inline_size);
    let action_gap = plan.action_cluster.gap.points_at(inline_size);
    let action_rows = action_rows(plan, width, action_gap, inline_size);
    let action_height = action_rows.iter().map(|row| row.height).sum::<f32>()
        + action_rows.len().saturating_sub(1) as f32 * action_gap;
    // The DOM renderer always mounts the action cluster. CSS Grid therefore
    // contributes the stack gap before that slot even when it has no actions.
    let child_count = 4_usize;
    let height = eyebrow_height
        + heading_height
        + body_height
        + action_height
        + child_count.saturating_sub(1) as f32 * stack_gap;
    CopyMetrics {
        inline_size,
        eyebrow_font_size,
        heading_font_size,
        body_font_size,
        eyebrow_height,
        heading_height,
        body_height,
        stack_gap,
        action_gap,
        action_rows,
        height,
    }
}

fn action_rows(plan: &crate::BlockPlan, width: f32, gap: f32, inline_size: f32) -> Vec<ActionRow> {
    let mut rows = Vec::<ActionRow>::new();
    for (index, action) in plan.content.actions.iter().enumerate() {
        let size = bevy_action_size(action.size, &action.label, inline_size);
        let row = rows.last_mut();
        if row.is_none()
            || row.is_some_and(|row| row.width + gap + size.x > width && row.width > 0.0)
        {
            rows.push(ActionRow {
                items: vec![(index, size)],
                width: size.x,
                height: size.y,
            });
        } else if let Some(row) = rows.last_mut() {
            row.items.push((index, size));
            row.width += gap + size.x;
            row.height = row.height.max(size.y);
        }
    }
    rows
}

fn text_block_height(text: &str, width: f32, font_size: f32, line_height: f32) -> f32 {
    text_block_height_with_glyph_ratio(text, width, font_size, line_height, 0.52)
}

fn text_block_height_with_glyph_ratio(
    text: &str,
    width: f32,
    font_size: f32,
    line_height: f32,
    glyph_ratio: f32,
) -> f32 {
    scale::estimate_text_block_height(text, width, font_size, line_height, glyph_ratio)
}

fn copy_colors(surface: SectionSurface, theme: &Theme) -> (Color, Color, Color) {
    match surface {
        SectionSurface::Brand => {
            let color = theme.text_on_brand().to_bevy();
            (color, color, color)
        }
        SectionSurface::Contrast => {
            let color = theme.neutral_content.to_bevy();
            (color, color, color)
        }
        SectionSurface::Transparent
        | SectionSurface::Surface
        | SectionSurface::Subtle
        | SectionSurface::Elevated
        | SectionSurface::Sunken
        | SectionSurface::Accent => (
            theme.brand().to_bevy(),
            theme.text_1().to_bevy(),
            theme.text_2().to_bevy(),
        ),
    }
}

fn bevy_action_size(size: ButtonSize, label: &str, inline_size: f32) -> Vec2 {
    let (font_size, padding_inline, padding_block, min_height, line_height) = match size {
        ButtonSize::Small => (
            scale::font_size::f00(inline_size),
            scale::space::xs2(inline_size),
            scale::space::xs3(inline_size),
            scale::space::s(inline_size),
            scale::line_height::LH0,
        ),
        ButtonSize::Medium => (
            scale::font_size::f0(inline_size),
            scale::space::xs(inline_size),
            scale::space::xs2(inline_size),
            40.0,
            scale::line_height::LH0,
        ),
        ButtonSize::Large => (
            scale::font_size::f1(inline_size),
            scale::space::s(inline_size),
            scale::space::xs(inline_size),
            40.0,
            scale::line_height::LH2,
        ),
        ButtonSize::Icon => {
            let side = scale::space::l(inline_size);
            return Vec2::splat(side);
        }
    };
    let text_width = label.chars().count() as f32 * font_size * 0.52;
    Vec2::new(
        text_width + padding_inline * 2.0,
        (font_size * line_height + padding_block * 2.0).max(min_height),
    )
}

fn bevy_action_colors(variant: ButtonVariant, theme: &Theme) -> (Color, Color, Color) {
    match variant {
        ButtonVariant::Primary => (
            theme.brand().to_bevy(),
            theme.brand().to_bevy(),
            theme.text_on_brand().to_bevy(),
        ),
        ButtonVariant::Secondary => (
            theme.surface_2().to_bevy(),
            theme.border_strong().to_bevy(),
            theme.text_1().to_bevy(),
        ),
        ButtonVariant::Destructive => (
            theme.danger().to_bevy(),
            theme.danger().to_bevy(),
            theme.text_on_brand().to_bevy(),
        ),
        ButtonVariant::Outline => (
            theme.surface_1().to_bevy(),
            theme.border_strong().to_bevy(),
            theme.text_1().to_bevy(),
        ),
        ButtonVariant::Ghost => (Color::NONE, Color::NONE, theme.text_1().to_bevy()),
        ButtonVariant::Link => (Color::NONE, Color::NONE, theme.brand().to_bevy()),
    }
}

fn push_support_primitives(
    output: &mut Vec<BevyBlockPrimitive>,
    plan: &crate::BlockPlan,
    theme: &Theme,
    center: Vec2,
    size: Vec2,
    viewport_width: f32,
    metrics: SupportMetrics,
) {
    let support_gap = plan.content_stack.gap.points_at(viewport_width);
    let mut cursor = center.y + size.y / 2.0;

    if plan.media.is_visible() {
        let media_bounds = LayoutRect {
            center_x: center.x,
            center_y: cursor - metrics.media_height / 2.0,
            width: size.x,
            height: metrics.media_height,
        };
        output.push(BevyBlockPrimitive {
            kind: BevyBlockPrimitiveKind::Media,
            part: "media".to_owned(),
            label: format!("{} - {}", plan.media.label(), plan.content.media_label),
            value: plan.block.definition().slug.to_owned(),
            center: Vec2::new(media_bounds.center_x, media_bounds.center_y),
            size: Vec2::new(media_bounds.width, media_bounds.height),
            fill: theme.surface_2().to_bevy(),
            border: theme.border_subtle().to_bevy(),
            border_width: 1.0,
            text: theme.text_muted().to_bevy(),
            text_align: GridAlign::Center,
            font_size: scale::font_size::f0(viewport_width),
            font_weight: scale::weight::W6,
            line_height: scale::line_height::LH0,
            letter_spacing: 0.0,
            ui: None,
        });
        cursor -= metrics.media_height + support_gap;
    }

    if plan.shows_collection_items() {
        let collection_bounds = LayoutRect {
            center_x: center.x,
            center_y: cursor - metrics.collection_height / 2.0,
            width: size.x,
            height: metrics.collection_height,
        };
        push_collection_primitives(
            output,
            plan,
            theme,
            Vec2::new(collection_bounds.center_x, collection_bounds.center_y),
            Vec2::new(collection_bounds.width, collection_bounds.height),
            viewport_width,
        );
        cursor -= metrics.collection_height + support_gap;
    }

    if plan.shows_primary_component() {
        let component_bounds = LayoutRect {
            center_x: center.x,
            center_y: cursor - metrics.component_height / 2.0,
            width: size.x,
            height: metrics.component_height,
        };
        push_component_primitives(output, plan, theme, component_bounds, viewport_width);
    }
}

fn support_metrics(
    plan: &crate::BlockPlan,
    theme: &Theme,
    width: f32,
    viewport_width: f32,
) -> SupportMetrics {
    let media_height = if plan.media.is_visible() {
        media_height(plan.media.ratio(), width)
    } else {
        0.0
    };
    let collection_height = if plan.shows_collection_items() {
        collection_metrics(plan, width, viewport_width, theme.border.max(1.0)).height
    } else {
        0.0
    };
    let component_height = if plan.shows_primary_component() {
        component_support_height(plan, theme, width, viewport_width)
    } else {
        0.0
    };
    let segment_count = usize::from(media_height > 0.0)
        + usize::from(collection_height > 0.0)
        + usize::from(component_height > 0.0);
    let gap = plan.content_stack.gap.points_at(viewport_width);
    let height = media_height
        + collection_height
        + component_height
        + segment_count.saturating_sub(1) as f32 * gap;
    SupportMetrics {
        media_height,
        collection_height,
        component_height,
        height,
    }
}

fn media_height(ratio: MediaRatio, width: f32) -> f32 {
    match ratio {
        MediaRatio::Square => width,
        MediaRatio::Portrait => width * 4.0 / 3.0,
        MediaRatio::Landscape => width * 3.0 / 4.0,
        MediaRatio::Wide => width * 9.0 / 16.0,
    }
}

fn component_support_height(
    plan: &crate::BlockPlan,
    theme: &Theme,
    width: f32,
    viewport_width: f32,
) -> f32 {
    match bevy_ui_flow(plan.primary_component) {
        BevyUiFlow::AspectRatioFrame => {
            return aspect_ratio_component_support_height(theme, width, viewport_width);
        }
        BevyUiFlow::CheckboxRow => {
            return checkbox_component_support_height(theme, width, viewport_width);
        }
        BevyUiFlow::CommandPalette => {
            return command_component_support_height(theme, width, viewport_width);
        }
        BevyUiFlow::ControlRow => return scale::space::L,
        BevyUiFlow::DrawerOverlay => {
            return drawer_component_support_height(theme, viewport_width);
        }
        BevyUiFlow::FormField => {
            return field_component_support_height(theme, width, viewport_width);
        }
        BevyUiFlow::ItemRow => {
            let primitives = bevy_primitives_for_component(plan.primary_component, theme);
            return item_row_metrics(&primitives, width, viewport_width, theme.border.max(1.0))
                .height;
        }
        BevyUiFlow::DirectionScope
        | BevyUiFlow::SeparatorLine
        | BevyUiFlow::SpinnerInline
        | BevyUiFlow::Stack
        | BevyUiFlow::ToggleButton
        | BevyUiFlow::TooltipOverlay => {}
    }
    let primitives = visible_component_primitives(plan.primary_component, theme);
    let children = primitives
        .iter()
        .filter(|primitive| primitive.role != UiBlockRole::Root)
        .take(6);
    let heights = children
        .map(|primitive| component_row_height(primitive.role))
        .collect::<Vec<_>>();
    let gap = scale::space::xs2(viewport_width);
    let padding = scale::space::s(viewport_width);
    (heights.iter().sum::<f32>() + heights.len().saturating_sub(1) as f32 * gap + padding * 2.0)
        .max(scale::space::xl2(viewport_width))
}

fn push_component_primitives(
    output: &mut Vec<BevyBlockPrimitive>,
    plan: &crate::BlockPlan,
    theme: &Theme,
    bounds: LayoutRect,
    viewport_width: f32,
) {
    match bevy_ui_flow(plan.primary_component) {
        BevyUiFlow::AspectRatioFrame => {
            push_aspect_ratio_component(output, plan, theme, bounds, viewport_width);
            return;
        }
        BevyUiFlow::ControlRow => {
            push_control_row_component(output, plan, theme, bounds, viewport_width);
            return;
        }
        BevyUiFlow::ItemRow => {
            push_item_row_component(output, plan, theme, bounds, viewport_width);
            return;
        }
        BevyUiFlow::CommandPalette => {
            push_command_palette_component(output, plan, theme, bounds, viewport_width);
            return;
        }
        BevyUiFlow::DrawerOverlay => {
            push_drawer_component(output, plan, theme, bounds, viewport_width);
            return;
        }
        BevyUiFlow::CheckboxRow
        | BevyUiFlow::DirectionScope
        | BevyUiFlow::FormField
        | BevyUiFlow::SeparatorLine
        | BevyUiFlow::SpinnerInline
        | BevyUiFlow::Stack
        | BevyUiFlow::ToggleButton
        | BevyUiFlow::TooltipOverlay => {}
    }
    let mut primitives = visible_component_primitives(plan.primary_component, theme);
    if primitives.is_empty() {
        return;
    }
    let root_index = primitives
        .iter()
        .position(|primitive| primitive.role == UiBlockRole::Root);
    let root = if let Some(root_index) = root_index {
        primitives.remove(root_index)
    } else {
        let mut root = primitives[0].clone();
        root.part = "component-root".to_owned();
        root.role = UiBlockRole::Root;
        root.label.clear();
        root
    };
    let children = primitives.into_iter().take(6).collect::<Vec<_>>();
    let gap = scale::space::xs2(viewport_width);
    let padding = scale::space::s(viewport_width);
    let component_width = bounds.width.max(240.0);
    let unscaled_height = children
        .iter()
        .map(|primitive| component_row_height(primitive.role))
        .sum::<f32>()
        + gap * children.len().saturating_sub(1) as f32
        + padding * 2.0;
    let component_height = unscaled_height.clamp(72.0, bounds.height);
    let row_scale = (component_height / unscaled_height).min(1.0);
    let component_center = Vec2::new(bounds.center_x, bounds.center_y);

    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::UiComponentHost(plan.primary_component),
        part: root.part.clone(),
        label: String::new(),
        value: root.value.clone(),
        center: component_center,
        size: Vec2::new(component_width, component_height),
        fill: root.fill,
        border: theme.border_subtle().to_bevy(),
        border_width: 1.0,
        text: root.text,
        text_align: GridAlign::Start,
        font_size: 0.0,
        font_weight: scale::weight::W4,
        line_height: scale::line_height::LH0,
        letter_spacing: 0.0,
        ui: Some(root),
    });

    let inner_width = (component_width - padding * 2.0).max(48.0);
    let mut cursor_y = component_center.y + component_height / 2.0 - padding * row_scale;
    for ui in children {
        let label = component_primitive_label(&ui);
        let row_height = component_row_height(ui.role) * row_scale;
        let row_width = if ui.role == UiBlockRole::Action {
            ui.size.x.max(120.0).min(inner_width)
        } else {
            inner_width
        };
        cursor_y -= row_height / 2.0;
        let boxed = component_role_is_boxed(ui.role);
        output.push(BevyBlockPrimitive {
            kind: if boxed {
                BevyBlockPrimitiveKind::UiComponent
            } else {
                BevyBlockPrimitiveKind::Text
            },
            part: ui.part.clone(),
            label,
            value: ui.value.clone(),
            center: Vec2::new(component_center.x, cursor_y),
            size: Vec2::new(row_width, row_height),
            fill: if boxed { ui.fill } else { Color::NONE },
            border: if boxed {
                theme.border_subtle().to_bevy()
            } else {
                Color::NONE
            },
            border_width: if boxed { 1.0 } else { 0.0 },
            text: ui.text,
            text_align: if ui.role == UiBlockRole::Action {
                GridAlign::Center
            } else {
                GridAlign::Start
            },
            font_size: scale::font_size::F00,
            font_weight: if matches!(ui.role, UiBlockRole::Action | UiBlockRole::Header) {
                scale::weight::W7
            } else {
                scale::weight::W4
            },
            line_height: scale::line_height::LH0,
            letter_spacing: 0.0,
            ui: Some(ui),
        });
        cursor_y -= row_height / 2.0 + gap * row_scale;
    }
}

fn visible_component_primitives(id: UiComponentId, theme: &Theme) -> Vec<BevyUiPrimitive> {
    bevy_primitives_for_component(id, theme)
        .into_iter()
        .filter(|primitive| !(primitive.role == UiBlockRole::Feedback && primitive.disabled))
        .collect()
}

fn aspect_ratio_component_support_height(theme: &Theme, width: f32, inline_size: f32) -> f32 {
    let fixture = canonical_ui_story_fixture(UiComponentId::AspectRatio);
    let UiStoryModel::AspectRatio(model) = fixture.model else {
        unreachable!("invariant: the canonical AspectRatio fixture contains an AspectRatio model");
    };
    aspect_ratio_layout_metrics(&model, width, inline_size, theme.border.max(1.0)).height
}

fn push_aspect_ratio_component(
    output: &mut Vec<BevyBlockPrimitive>,
    plan: &crate::BlockPlan,
    theme: &Theme,
    bounds: LayoutRect,
    inline_size: f32,
) {
    let fixture = canonical_ui_story_fixture(UiComponentId::AspectRatio);
    let UiStoryModel::AspectRatio(model) = fixture.model else {
        unreachable!("invariant: the canonical AspectRatio fixture contains an AspectRatio model");
    };
    let metrics =
        aspect_ratio_layout_metrics(&model, bounds.width, inline_size, theme.border.max(1.0));
    let root = bevy_primitives_for_component(plan.primary_component, theme)
        .into_iter()
        .find(|primitive| primitive.role == UiBlockRole::Root)
        .expect("invariant: AspectRatio Bevy primitives include a root");
    let top = bounds.center_y + bounds.height / 2.0;

    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::UiComponentHost(plan.primary_component),
        part: root.part.clone(),
        label: String::new(),
        value: root.value.clone(),
        center: Vec2::new(bounds.center_x, top - metrics.height / 2.0),
        size: Vec2::new(metrics.width, metrics.height),
        fill: root.fill,
        border: theme.border_subtle().to_bevy(),
        border_width: theme.border.max(1.0),
        text: root.text,
        text_align: GridAlign::Start,
        font_size: 0.0,
        font_weight: scale::weight::W4,
        line_height: scale::line_height::LH0,
        letter_spacing: 0.0,
        ui: Some(root),
    });
}

fn field_component_support_height(theme: &Theme, width: f32, inline_size: f32) -> f32 {
    let fixture = canonical_ui_story_fixture(UiComponentId::Field);
    let UiStoryModel::Field(model) = fixture.model else {
        unreachable!("invariant: the canonical Field fixture contains a Field model");
    };
    field_layout_metrics(&model, width, inline_size, theme.border.max(1.0)).height
}

fn checkbox_component_support_height(theme: &Theme, width: f32, inline_size: f32) -> f32 {
    let fixture = canonical_ui_story_fixture(UiComponentId::Checkbox);
    let UiStoryModel::Checkbox(model) = fixture.model else {
        unreachable!("invariant: the canonical Checkbox fixture contains a Checkbox model");
    };
    checkbox_layout_metrics(&model, width, inline_size, theme.border.max(1.0)).height
}

fn command_component_support_height(theme: &Theme, width: f32, inline_size: f32) -> f32 {
    let fixture = canonical_ui_story_fixture(UiComponentId::Command);
    let UiStoryModel::Command(model) = fixture.model else {
        unreachable!("invariant: the canonical Command fixture contains a Command model");
    };
    let state = model.state();
    command_layout_metrics(&model, &state, width, inline_size, theme.border.max(1.0)).height
}

fn drawer_component_support_height(theme: &Theme, inline_size: f32) -> f32 {
    let fixture = canonical_ui_story_fixture(UiComponentId::Drawer);
    let UiStoryModel::Drawer(model) = fixture.model else {
        unreachable!("invariant: the canonical Drawer fixture contains a Drawer model");
    };
    drawer_layout_metrics(
        &model,
        inline_size,
        inline_size,
        inline_size,
        theme.border.max(1.0),
    )
    .trigger_height
}

fn push_drawer_component(
    output: &mut Vec<BevyBlockPrimitive>,
    plan: &crate::BlockPlan,
    theme: &Theme,
    bounds: LayoutRect,
    inline_size: f32,
) {
    let fixture = canonical_ui_story_fixture(UiComponentId::Drawer);
    let UiStoryModel::Drawer(model) = fixture.model else {
        unreachable!("invariant: the canonical Drawer fixture contains a Drawer model");
    };
    let metrics = drawer_layout_metrics(
        &model,
        inline_size,
        inline_size,
        inline_size,
        theme.border.max(1.0),
    );
    let root = bevy_primitives_for_component(plan.primary_component, theme)
        .into_iter()
        .find(|primitive| primitive.role == UiBlockRole::Root)
        .expect("invariant: Drawer Bevy primitives include a root");
    let top = bounds.center_y + bounds.height / 2.0;

    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::UiComponentHost(plan.primary_component),
        part: root.part.clone(),
        label: String::new(),
        value: root.value.clone(),
        center: Vec2::new(bounds.center_x, top - metrics.trigger_height / 2.0),
        size: Vec2::new(bounds.width, metrics.trigger_height),
        fill: root.fill,
        border: theme.border_subtle().to_bevy(),
        border_width: theme.border.max(1.0),
        text: root.text,
        text_align: GridAlign::Start,
        font_size: 0.0,
        font_weight: scale::weight::W4,
        line_height: scale::line_height::LH0,
        letter_spacing: 0.0,
        ui: Some(root),
    });
}

fn push_command_palette_component(
    output: &mut Vec<BevyBlockPrimitive>,
    plan: &crate::BlockPlan,
    theme: &Theme,
    bounds: LayoutRect,
    inline_size: f32,
) {
    let fixture = canonical_ui_story_fixture(UiComponentId::Command);
    let UiStoryModel::Command(model) = fixture.model else {
        unreachable!("invariant: the canonical Command fixture contains a Command model");
    };
    let metrics = command_layout_metrics(
        &model,
        &model.state(),
        bounds.width,
        inline_size,
        theme.border.max(1.0),
    );
    let root = bevy_primitives_for_component(plan.primary_component, theme)
        .into_iter()
        .find(|primitive| primitive.role == UiBlockRole::Root)
        .expect("invariant: Command Bevy primitives include a root");
    let left = bounds.center_x - bounds.width / 2.0;
    let top = bounds.center_y + bounds.height / 2.0;

    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::UiComponentHost(plan.primary_component),
        part: root.part.clone(),
        label: String::new(),
        value: root.value.clone(),
        center: Vec2::new(left + metrics.width / 2.0, top - metrics.height / 2.0),
        size: Vec2::new(metrics.width, metrics.height),
        fill: root.fill,
        border: theme.border_subtle().to_bevy(),
        border_width: theme.border.max(1.0),
        text: root.text,
        text_align: GridAlign::Start,
        font_size: 0.0,
        font_weight: scale::weight::W4,
        line_height: scale::line_height::LH0,
        letter_spacing: 0.0,
        ui: Some(root),
    });
}

fn item_row_metrics(
    primitives: &[BevyUiPrimitive],
    available_width: f32,
    inline_size: f32,
    border_width: f32,
) -> ItemRowMetrics {
    let width = available_width.clamp(240.0, scale::container::CONTROL);
    let padding = scale::space::xs(inline_size);
    let gap = scale::space::xs(inline_size);
    let content_gap = scale::space::xs3(inline_size);
    let action_gap = scale::space::xs2(inline_size);
    let media_size = scale::space::xl(inline_size);
    let actions = primitives
        .iter()
        .filter(|primitive| primitive.part.starts_with("ItemActions"))
        .collect::<Vec<_>>();
    let action_sizes = actions
        .iter()
        .map(|primitive| bevy_action_size(ButtonSize::Small, &primitive.label, inline_size))
        .collect::<Vec<_>>();
    let action_width = action_sizes.iter().map(|size| size.x).sum::<f32>()
        + action_sizes.len().saturating_sub(1) as f32 * action_gap;
    let occupied_gaps = usize::from(primitives.iter().any(|item| item.part == "ItemMedia"))
        + usize::from(!action_sizes.is_empty());
    let content_width =
        (width - padding * 2.0 - media_size - action_width - gap * occupied_gaps as f32)
            .max(scale::space::XL2);
    let title_height = scale::font_size::f0(inline_size) * scale::line_height::LH0;
    let description = primitives
        .iter()
        .find(|primitive| primitive.part == "ItemDescription")
        .map(component_primitive_label)
        .unwrap_or_default();
    let description_height = text_block_height(
        &description,
        content_width,
        scale::font_size::f00(inline_size),
        scale::line_height::LH0,
    );
    let content_height = title_height + content_gap + description_height;
    let action_height = action_sizes.iter().map(|size| size.y).fold(0.0, f32::max);
    let height =
        media_size.max(content_height).max(action_height) + padding * 2.0 + border_width * 2.0;
    ItemRowMetrics {
        width,
        height,
        padding,
        gap,
        media_size,
        content_width,
        title_height,
        description_height,
        content_gap,
        action_gap,
        action_sizes,
        action_width,
    }
}

fn push_item_row_component(
    output: &mut Vec<BevyBlockPrimitive>,
    plan: &crate::BlockPlan,
    theme: &Theme,
    bounds: LayoutRect,
    viewport_width: f32,
) {
    let primitives = bevy_primitives_for_component(plan.primary_component, theme);
    let metrics = item_row_metrics(
        &primitives,
        bounds.width,
        viewport_width,
        theme.border.max(1.0),
    );
    let left = bounds.center_x - bounds.width / 2.0;
    let top = bounds.center_y + bounds.height / 2.0;
    let center = Vec2::new(left + metrics.width / 2.0, top - metrics.height / 2.0);
    let root = primitives
        .iter()
        .find(|primitive| primitive.role == UiBlockRole::Root)
        .expect("invariant: Item Bevy primitives include a root");
    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::UiComponentHost(plan.primary_component),
        part: root.part.clone(),
        label: String::new(),
        value: root.value.clone(),
        center,
        size: Vec2::new(metrics.width, metrics.height),
        fill: root.fill,
        border: theme.border_subtle().to_bevy(),
        border_width: theme.border.max(1.0),
        text: root.text,
        text_align: GridAlign::Start,
        font_size: 0.0,
        font_weight: scale::weight::W4,
        line_height: scale::line_height::LH0,
        letter_spacing: 0.0,
        ui: Some(root.clone()),
    });

    let mut content_left = left + metrics.padding;
    if let Some(media) = primitives
        .iter()
        .find(|primitive| primitive.part == "ItemMedia")
    {
        output.push(BevyBlockPrimitive {
            kind: BevyBlockPrimitiveKind::UiComponent,
            part: media.part.clone(),
            label: component_primitive_label(media),
            value: media.value.clone(),
            center: Vec2::new(
                content_left + metrics.media_size / 2.0,
                top - metrics.padding - metrics.media_size / 2.0,
            ),
            size: Vec2::splat(metrics.media_size),
            fill: media.fill,
            border: theme.border_subtle().to_bevy(),
            border_width: theme.border.max(1.0),
            text: theme.brand().to_bevy(),
            text_align: GridAlign::Center,
            font_size: scale::font_size::f00(viewport_width),
            font_weight: scale::weight::W7,
            line_height: scale::line_height::LH0,
            letter_spacing: 0.0,
            ui: Some(media.clone()),
        });
        content_left += metrics.media_size + metrics.gap;
    }

    if let Some(title) = primitives
        .iter()
        .find(|primitive| primitive.part == "ItemTitle")
    {
        output.push(BevyBlockPrimitive {
            kind: BevyBlockPrimitiveKind::Text,
            part: title.part.clone(),
            label: component_primitive_label(title),
            value: title.value.clone(),
            center: Vec2::new(
                content_left + metrics.content_width / 2.0,
                top - metrics.padding - metrics.title_height / 2.0,
            ),
            size: Vec2::new(metrics.content_width, metrics.title_height),
            fill: Color::NONE,
            border: Color::NONE,
            border_width: 0.0,
            text: title.text,
            text_align: GridAlign::Start,
            font_size: scale::font_size::f0(viewport_width),
            font_weight: scale::weight::W7,
            line_height: scale::line_height::LH0,
            letter_spacing: 0.0,
            ui: Some(title.clone()),
        });
    }
    if let Some(description) = primitives
        .iter()
        .find(|primitive| primitive.part == "ItemDescription")
    {
        output.push(BevyBlockPrimitive {
            kind: BevyBlockPrimitiveKind::Text,
            part: description.part.clone(),
            label: component_primitive_label(description),
            value: description.value.clone(),
            center: Vec2::new(
                content_left + metrics.content_width / 2.0,
                top - metrics.padding
                    - metrics.title_height
                    - metrics.content_gap
                    - metrics.description_height / 2.0,
            ),
            size: Vec2::new(metrics.content_width, metrics.description_height),
            fill: Color::NONE,
            border: Color::NONE,
            border_width: 0.0,
            text: theme.text_2().to_bevy(),
            text_align: GridAlign::Start,
            font_size: scale::font_size::f00(viewport_width),
            font_weight: scale::weight::W4,
            line_height: scale::line_height::LH0,
            letter_spacing: 0.0,
            ui: Some(description.clone()),
        });
    }

    let mut action_left = left + metrics.width - metrics.padding - metrics.action_width;
    for (action, size) in primitives
        .iter()
        .filter(|primitive| primitive.part.starts_with("ItemActions"))
        .zip(&metrics.action_sizes)
    {
        output.push(BevyBlockPrimitive {
            kind: BevyBlockPrimitiveKind::UiComponent,
            part: action.part.clone(),
            label: component_primitive_label(action),
            value: action.value.clone(),
            center: Vec2::new(
                action_left + size.x / 2.0,
                top - metrics.padding - size.y / 2.0,
            ),
            size: *size,
            fill: action.fill,
            border: theme.border_strong().to_bevy(),
            border_width: theme.border.max(1.0),
            text: action.text,
            text_align: GridAlign::Center,
            font_size: scale::font_size::f00(viewport_width),
            font_weight: scale::weight::W7,
            line_height: scale::line_height::LH0,
            letter_spacing: 0.0,
            ui: Some(action.clone()),
        });
        action_left += size.x + metrics.action_gap;
    }
}

fn push_control_row_component(
    output: &mut Vec<BevyBlockPrimitive>,
    plan: &crate::BlockPlan,
    theme: &Theme,
    bounds: LayoutRect,
    viewport_width: f32,
) {
    let mut primitives = bevy_primitives_for_component(plan.primary_component, theme);
    let root_index = primitives
        .iter()
        .position(|primitive| primitive.role == UiBlockRole::Root)
        .unwrap_or(0);
    let root = primitives.remove(root_index);
    let children = primitives
        .into_iter()
        .filter(|primitive| {
            !matches!(
                primitive.kind,
                UiWidgetSlotKind::Section | UiWidgetSlotKind::List | UiWidgetSlotKind::ListItem
            ) && primitive.role != UiBlockRole::Layout
        })
        .collect::<Vec<_>>();
    let fills_available_width = matches!(
        plan.primary_component,
        UiComponentId::Input | UiComponentId::InputGroup | UiComponentId::NativeSelect
    );
    let intrinsic_width = children
        .iter()
        .map(|primitive| control_row_slot_width(primitive, viewport_width))
        .sum::<f32>();
    let component_width = if fills_available_width {
        bounds.width.min(scale::container::CONTROL)
    } else {
        intrinsic_width.min(bounds.width)
    }
    .max(scale::space::L);
    let component_height = scale::space::L;
    let component_left = bounds.center_x - bounds.width / 2.0;
    let component_center = Vec2::new(
        component_left + component_width / 2.0,
        bounds.center_y + bounds.height / 2.0 - component_height / 2.0,
    );

    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::UiComponentHost(plan.primary_component),
        part: root.part.clone(),
        label: String::new(),
        value: root.value.clone(),
        center: component_center,
        size: Vec2::new(component_width, component_height),
        fill: root.fill,
        border: theme.border_strong().to_bevy(),
        border_width: theme.border.max(1.0),
        text: root.text,
        text_align: GridAlign::Start,
        font_size: 0.0,
        font_weight: scale::weight::W4,
        line_height: scale::line_height::LH0,
        letter_spacing: 0.0,
        ui: Some(root),
    });

    let fixed_width = children
        .iter()
        .filter(|primitive| primitive.role != UiBlockRole::Control)
        .map(|primitive| control_row_slot_width(primitive, viewport_width))
        .sum::<f32>();
    let control_count = children
        .iter()
        .filter(|primitive| primitive.role == UiBlockRole::Control)
        .count();
    let control_width = if control_count == 0 {
        0.0
    } else {
        ((component_width - fixed_width) / control_count as f32).max(scale::space::L)
    };
    let mut cursor = component_left;
    for ui in children {
        let width = if ui.role == UiBlockRole::Control {
            control_width
        } else {
            control_row_slot_width(&ui, viewport_width)
        };
        let boxed = !matches!(ui.kind, UiWidgetSlotKind::Text);
        output.push(BevyBlockPrimitive {
            kind: if boxed {
                BevyBlockPrimitiveKind::UiComponent
            } else {
                BevyBlockPrimitiveKind::Text
            },
            part: ui.part.clone(),
            label: component_primitive_label(&ui),
            value: ui.value.clone(),
            center: Vec2::new(cursor + width / 2.0, component_center.y),
            size: Vec2::new(width, component_height),
            fill: ui.fill,
            border: theme.border_subtle().to_bevy(),
            border_width: theme.border.max(1.0),
            text: ui.text,
            text_align: if ui.role == UiBlockRole::Action {
                GridAlign::Center
            } else {
                GridAlign::Start
            },
            font_size: scale::font_size::f0(viewport_width),
            font_weight: if ui.role == UiBlockRole::Action {
                scale::weight::W6
            } else {
                scale::weight::W4
            },
            line_height: scale::line_height::LH0,
            letter_spacing: 0.0,
            ui: Some(ui),
        });
        cursor += width;
    }
}

fn control_row_slot_width(primitive: &BevyUiPrimitive, inline_size: f32) -> f32 {
    if primitive.role == UiBlockRole::Control {
        return primitive.size.x.max(scale::space::L);
    }
    let font_size = scale::font_size::f0(inline_size);
    let padding = scale::space::xs(inline_size) * 2.0;
    let glyph_factor = if primitive.kind == UiWidgetSlotKind::Button {
        0.62
    } else {
        0.45
    };
    (primitive.label.chars().count() as f32 * font_size * glyph_factor + padding)
        .max(primitive.size.x)
}

fn component_primitive_label(primitive: &BevyUiPrimitive) -> String {
    if matches!(
        primitive.role,
        UiBlockRole::Content | UiBlockRole::Feedback | UiBlockRole::Text
    ) && !primitive.value.trim().is_empty()
    {
        primitive.value.clone()
    } else {
        primitive.label.clone()
    }
}

const fn component_row_height(role: UiBlockRole) -> f32 {
    match role {
        UiBlockRole::Control | UiBlockRole::Action => 44.0,
        UiBlockRole::Data
        | UiBlockRole::Item
        | UiBlockRole::Media
        | UiBlockRole::Navigation
        | UiBlockRole::Overlay => 52.0,
        UiBlockRole::Content | UiBlockRole::Feedback => 40.0,
        UiBlockRole::Header
        | UiBlockRole::Indicator
        | UiBlockRole::Layout
        | UiBlockRole::Text
        | UiBlockRole::Root => 30.0,
    }
}

const fn component_role_is_boxed(role: UiBlockRole) -> bool {
    matches!(
        role,
        UiBlockRole::Action
            | UiBlockRole::Control
            | UiBlockRole::Data
            | UiBlockRole::Indicator
            | UiBlockRole::Item
            | UiBlockRole::Layout
            | UiBlockRole::Media
            | UiBlockRole::Navigation
            | UiBlockRole::Overlay
    )
}

fn push_collection_primitives(
    output: &mut Vec<BevyBlockPrimitive>,
    plan: &crate::BlockPlan,
    theme: &Theme,
    center: Vec2,
    size: Vec2,
    viewport_width: f32,
) {
    let metrics = collection_metrics(plan, size.x, viewport_width, theme.border.max(1.0));
    let left = center.x - size.x / 2.0;
    let mut row_top = center.y + metrics.height / 2.0;

    for (row_index, row_height) in metrics.row_heights.iter().copied().enumerate() {
        let first_card = row_index * metrics.columns;
        let last_card = (first_card + metrics.columns).min(plan.content.items.len());
        for card_index in first_card..last_card {
            let column = card_index - first_card;
            let card_metrics = metrics.cards[card_index];
            let track_left = left + column as f32 * (metrics.track_width + metrics.gap);
            let card_center = Vec2::new(
                track_left + card_metrics.width / 2.0,
                row_top - card_metrics.height / 2.0,
            );
            push_collection_card_primitives(
                output,
                &plan.content.items[card_index],
                theme,
                card_center,
                card_metrics,
                viewport_width,
            );
        }
        row_top -= row_height + metrics.gap;
    }
}

fn collection_metrics(
    plan: &crate::BlockPlan,
    width: f32,
    viewport_width: f32,
    border_width: f32,
) -> CollectionMetrics {
    let item_count = plan.content.items.len();
    if item_count == 0 {
        return CollectionMetrics {
            columns: 1,
            track_width: width,
            gap: 0.0,
            cards: Vec::new(),
            row_heights: Vec::new(),
            height: 0.0,
        };
    }
    let columns = usize::from(plan.collection_grid.preset.columns(viewport_width))
        .min(item_count)
        .max(1);
    let gap = plan.collection_grid.gap.points_at(viewport_width);
    let track_width = ((width - gap * columns.saturating_sub(1) as f32) / columns as f32).max(1.0);
    let cards = plan
        .content
        .items
        .iter()
        .map(|item| {
            card_layout_metrics(
                &item.card_model(),
                track_width,
                viewport_width,
                border_width,
            )
        })
        .collect::<Vec<_>>();
    let row_heights = cards
        .chunks(columns)
        .map(|row| row.iter().map(|card| card.height).fold(0.0, f32::max))
        .collect::<Vec<_>>();
    let height = row_heights.iter().sum::<f32>() + gap * row_heights.len().saturating_sub(1) as f32;
    CollectionMetrics {
        columns,
        track_width,
        gap,
        cards,
        row_heights,
        height,
    }
}

fn push_collection_card_primitives(
    output: &mut Vec<BevyBlockPrimitive>,
    item: &crate::BlockItem,
    theme: &Theme,
    center: Vec2,
    metrics: CardLayoutMetrics,
    viewport_width: f32,
) {
    let model = item.card_model();
    let dense = model.density == CardDensity::Dense;
    let title_font_size = if dense {
        scale::font_size::f0(viewport_width)
    } else {
        scale::font_size::f1(viewport_width)
    };
    let title_line_height = if dense {
        scale::line_height::LH0
    } else {
        scale::line_height::LH2
    };
    let copy_font_size = if dense {
        scale::font_size::f00(viewport_width)
    } else {
        scale::font_size::f0(viewport_width)
    };
    let left = center.x - metrics.width / 2.0;
    let top = center.y + metrics.height / 2.0;
    let inner_left = left + metrics.border_width + metrics.padding;
    let inner_width = (metrics.width - metrics.border_width * 2.0 - metrics.padding * 2.0).max(1.0);

    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::Item,
        part: "item".to_owned(),
        label: String::new(),
        value: item.value.clone(),
        center,
        size: Vec2::new(metrics.width, metrics.height),
        fill: if item.selected {
            theme.surface_elevated().to_bevy()
        } else {
            theme.surface_1().to_bevy()
        },
        border: if item.selected {
            theme.border_subtle().to_bevy()
        } else {
            theme.border_strong().to_bevy()
        },
        border_width: metrics.border_width,
        text: theme.text_1().to_bevy(),
        text_align: GridAlign::Start,
        font_size: 0.0,
        font_weight: scale::weight::W4,
        line_height: scale::line_height::LH0,
        letter_spacing: 0.0,
        ui: None,
    });

    let title_top = top - metrics.border_width - metrics.padding;
    push_collection_text(
        output,
        CollectionTextSpec {
            part: "item-title",
            label: model.title.clone(),
            center: Vec2::new(
                inner_left + inner_width / 2.0,
                title_top - metrics.title_height / 2.0,
            ),
            size: Vec2::new(inner_width, metrics.title_height),
            text: theme.text_1().to_bevy(),
            font_size: title_font_size,
            font_weight: scale::weight::W7,
            line_height: title_line_height,
            letter_spacing: 0.0,
        },
    );
    let description_top = title_top - metrics.title_height - metrics.header_gap;
    push_collection_text(
        output,
        CollectionTextSpec {
            part: "item-meta",
            label: model.description.clone(),
            center: Vec2::new(
                inner_left + inner_width / 2.0,
                description_top - metrics.description_height / 2.0,
            ),
            size: Vec2::new(inner_width, metrics.description_height),
            text: theme.text_2().to_bevy(),
            font_size: copy_font_size,
            font_weight: scale::weight::W4,
            line_height: scale::line_height::LH0,
            letter_spacing: 0.0,
        },
    );

    let content_top = title_top - metrics.header_height - metrics.gap;
    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::UiComponent,
        part: "item-content".to_owned(),
        label: String::new(),
        value: item.value.clone(),
        center: Vec2::new(
            inner_left + inner_width / 2.0,
            content_top - metrics.content_height / 2.0,
        ),
        size: Vec2::new(inner_width, metrics.content_height),
        fill: theme.surface_2().to_bevy(),
        border: theme.border_subtle().to_bevy(),
        border_width: metrics.border_width,
        text: theme.text_1().to_bevy(),
        text_align: GridAlign::Start,
        font_size: 0.0,
        font_weight: scale::weight::W4,
        line_height: scale::line_height::LH0,
        letter_spacing: 0.0,
        ui: None,
    });
    let content_text_left = inner_left + metrics.border_width + metrics.content_padding;
    let content_text_width =
        (inner_width - metrics.border_width * 2.0 - metrics.content_padding * 2.0).max(1.0);
    push_collection_text(
        output,
        CollectionTextSpec {
            part: "item-body",
            label: model.content.clone(),
            center: Vec2::new(
                content_text_left + content_text_width / 2.0,
                content_top
                    - metrics.border_width
                    - metrics.content_padding
                    - metrics.content_text_height / 2.0,
            ),
            size: Vec2::new(content_text_width, metrics.content_text_height),
            text: theme.text_1().to_bevy(),
            font_size: copy_font_size,
            font_weight: scale::weight::W4,
            line_height: scale::line_height::LH0,
            letter_spacing: 0.0,
        },
    );

    let footer_top = content_top - metrics.content_height - metrics.gap;
    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::Text,
        part: "item-footer".to_owned(),
        label: String::new(),
        value: item.value.clone(),
        center: Vec2::new(
            inner_left + inner_width / 2.0,
            footer_top - metrics.footer_height / 2.0,
        ),
        size: Vec2::new(inner_width, metrics.footer_height),
        fill: Color::NONE,
        border: theme.border_subtle().to_bevy(),
        border_width: metrics.border_width,
        text: theme.text_muted().to_bevy(),
        text_align: GridAlign::Start,
        font_size: 0.0,
        font_weight: scale::weight::W4,
        line_height: scale::line_height::LH0,
        letter_spacing: 0.0,
        ui: None,
    });

    let footer_content_top = footer_top - metrics.border_width - metrics.footer_padding_top;
    let footer_text_center_y = if metrics.footer_wraps {
        footer_content_top - metrics.footer_text_height / 2.0
    } else {
        footer_content_top - metrics.footer_text_height.max(metrics.action_height) / 2.0
    };
    let footer_label_width = if metrics.footer_wraps || metrics.action_width <= 0.0 {
        inner_width
    } else {
        (inner_width - metrics.action_width - metrics.footer_gap).max(1.0)
    };
    push_collection_text(
        output,
        CollectionTextSpec {
            part: "item-footer-label",
            label: model.footer.to_uppercase(),
            center: Vec2::new(inner_left + footer_label_width / 2.0, footer_text_center_y),
            size: Vec2::new(footer_label_width, metrics.footer_text_height),
            text: theme.text_muted().to_bevy(),
            font_size: scale::font_size::f00(viewport_width),
            font_weight: scale::weight::W6,
            line_height: scale::line_height::LH00,
            letter_spacing: 0.08,
        },
    );

    if let Some(action) = model.action {
        let action_center_y = if metrics.footer_wraps {
            footer_content_top
                - metrics.footer_text_height
                - metrics.footer_gap
                - metrics.action_height / 2.0
        } else {
            footer_content_top - metrics.footer_text_height.max(metrics.action_height) / 2.0
        };
        let action_left = if metrics.footer_wraps {
            inner_left
        } else {
            inner_left + inner_width - metrics.action_width
        };
        output.push(BevyBlockPrimitive {
            kind: BevyBlockPrimitiveKind::Action,
            part: "item-action".to_owned(),
            label: action.label,
            value: action.value,
            center: Vec2::new(action_left + metrics.action_width / 2.0, action_center_y),
            size: Vec2::new(metrics.action_width, metrics.action_height),
            fill: theme.surface_2().to_bevy(),
            border: theme.border_strong().to_bevy(),
            border_width: metrics.border_width,
            text: theme.text_1().to_bevy(),
            text_align: GridAlign::Center,
            font_size: scale::font_size::f0(viewport_width),
            font_weight: scale::weight::W7,
            line_height: scale::line_height::LH0,
            letter_spacing: 0.0,
            ui: None,
        });
    }
}

struct CollectionTextSpec {
    part: &'static str,
    label: String,
    center: Vec2,
    size: Vec2,
    text: Color,
    font_size: f32,
    font_weight: u16,
    line_height: f32,
    letter_spacing: f32,
}

fn push_collection_text(output: &mut Vec<BevyBlockPrimitive>, spec: CollectionTextSpec) {
    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::Text,
        part: spec.part.to_owned(),
        label: spec.label,
        value: String::new(),
        center: spec.center,
        size: spec.size,
        fill: Color::NONE,
        border: Color::NONE,
        border_width: 0.0,
        text: spec.text,
        text_align: GridAlign::Start,
        font_size: spec.font_size,
        font_weight: spec.font_weight,
        line_height: spec.line_height,
        letter_spacing: spec.letter_spacing,
        ui: None,
    });
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;
    use rs_dean_ui::ThemeId;

    use crate::{BLOCKS, BlockInstance, block_by_slug};

    use super::*;

    #[test]
    fn every_block_produces_bevy_primitives_from_the_shared_plan() {
        let theme = ThemeId::Light.palette();
        for definition in BLOCKS {
            for viewport in [Vec2::new(1_280.0, 720.0), Vec2::new(375.0, 812.0)] {
                let primitives =
                    bevy_block_primitives(&BlockInstance::fixture(&definition), &theme, viewport)
                        .expect("block should produce Bevy primitives");
                assert!(!primitives.is_empty());
                assert!(
                    primitives
                        .iter()
                        .any(|primitive| primitive.kind != BevyBlockPrimitiveKind::Section)
                );
                let plan = plan_block(&BlockInstance::fixture(&definition))
                    .expect("fixture should produce a plan");
                if plan.shows_primary_component() {
                    assert!(
                        primitives.iter().any(|primitive| {
                            matches!(
                                primitive.kind,
                                BevyBlockPrimitiveKind::UiComponentHost(id)
                                    if id == plan.primary_component
                            )
                        }),
                        "{} must emit its primary UI component",
                        definition.slug,
                    );
                }
                let section = &primitives[0];
                assert_eq!(
                    section.size.x,
                    viewport.x.max(240.0),
                    "{} must keep the section surface full bleed",
                    definition.slug,
                );
                for primitive in &primitives[1..] {
                    assert!(
                        primitive.center.x - primitive.size.x / 2.0 >= -section.size.x / 2.0 - 1.0,
                        "{} `{}` escapes the left edge at viewport {viewport}",
                        definition.slug,
                        primitive.part,
                    );
                    assert!(
                        primitive.center.x + primitive.size.x / 2.0 <= section.size.x / 2.0 + 1.0,
                        "{} `{}` escapes the right edge at viewport {viewport}",
                        definition.slug,
                        primitive.part,
                    );
                    assert!(
                        primitive.center.y - primitive.size.y / 2.0 >= -section.size.y / 2.0 - 1.0,
                        "{} `{}` escapes the bottom edge at viewport {viewport}",
                        definition.slug,
                        primitive.part,
                    );
                    assert!(
                        primitive.center.y + primitive.size.y / 2.0 <= section.size.y / 2.0 + 1.0,
                        "{} `{}` escapes the top edge at viewport {viewport}",
                        definition.slug,
                        primitive.part,
                    );
                }
            }
        }
    }

    #[test]
    fn constrained_container_geometry_matches_css_border_box_gutters() {
        let definition =
            block_by_slug("marketing-sections-cta-sections-01-dark-panel-with-app-screenshot")
                .expect("CTA fixture should exist");
        let viewport = Vec2::new(1_000.0, 700.0);
        let primitives = bevy_block_primitives(
            &BlockInstance::fixture(definition),
            &ThemeId::Dark.palette(),
            viewport,
        )
        .expect("CTA fixture should produce primitives");
        let plan = plan_block(&BlockInstance::fixture(definition)).expect("fixture should plan");
        let expected_width =
            plan.container.width.points() - plan.container.gutter.points_at(viewport.x) * 2.0;
        let title = primitives
            .iter()
            .find(|primitive| primitive.part == "title")
            .expect("title primitive should exist");

        assert!((title.size.x - expected_width).abs() < 0.01);
        assert!((title.center.x - 0.0).abs() < 0.01);
    }

    #[test]
    fn empty_action_cluster_keeps_the_dom_stack_gap() {
        let definition =
            block_by_slug("marketing-sections-faq-sections-01-offset-with-supporting-text")
                .expect("FAQ fixture should exist");
        let plan = plan_block(&BlockInstance::fixture(definition)).expect("fixture should plan");
        assert!(plan.content.actions.is_empty());

        let metrics = copy_metrics(&plan, 964.0, 1_000.0);
        let expected = metrics.eyebrow_height
            + metrics.heading_height
            + metrics.body_height
            + metrics.stack_gap * 3.0;

        assert!((metrics.height - expected).abs() < 0.01);
    }

    #[test]
    fn stacked_component_support_starts_after_the_outer_grid_gap() {
        let definition = block_by_slug("marketing-sections-contact-sections-01-centered")
            .expect("contact fixture should exist");
        let viewport = Vec2::new(1_000.0, 700.0);
        let instance = BlockInstance::fixture(definition);
        let plan = plan_block(&instance).expect("fixture should plan");
        let primitives = bevy_block_primitives(&instance, &ThemeId::Dark.palette(), viewport)
            .expect("contact fixture should produce primitives");
        let action_bottom = primitives
            .iter()
            .filter(|primitive| primitive.kind == BevyBlockPrimitiveKind::Action)
            .map(|primitive| primitive.center.y - primitive.size.y / 2.0)
            .reduce(f32::min)
            .expect("contact fixture should include actions");
        let component_top = primitives
            .iter()
            .find(|primitive| {
                matches!(
                    primitive.kind,
                    BevyBlockPrimitiveKind::UiComponentHost(UiComponentId::Field)
                )
            })
            .map(|primitive| primitive.center.y + primitive.size.y / 2.0)
            .expect("contact fixture should include the Field host");
        let actual_gap = action_bottom - component_top;
        let expected_gap = plan.outer_grid.gap.points_at(viewport.x);
        assert!(
            (actual_gap - expected_gap).abs() < 0.01,
            "stacked support gap was {actual_gap}, expected {expected_gap}",
        );
    }

    #[test]
    fn input_component_uses_one_contiguous_control_row() {
        let definition =
            block_by_slug("marketing-sections-newsletter-sections-03-simple-side-by-side-on-brand")
                .expect("newsletter fixture should exist");
        let primitives = bevy_block_primitives(
            &BlockInstance::fixture(definition),
            &ThemeId::Dark.palette(),
            Vec2::new(1_000.0, 700.0),
        )
        .expect("newsletter fixture should produce primitives");
        let part = |name: &str| {
            primitives
                .iter()
                .find(|primitive| primitive.part == name)
                .expect("input primitive should exist")
        };
        let root = part("Input");
        let prefix = part("InputPrefix");
        let control = part("InputControl");
        let suffix = part("InputSuffix");

        assert!((root.size.x - scale::container::CONTROL).abs() < 0.01);
        assert!((root.size.y - scale::space::L).abs() < 0.01);
        assert!(
            (prefix.center.x + prefix.size.x / 2.0 - (control.center.x - control.size.x / 2.0))
                .abs()
                < 0.01
        );
        assert!(
            (control.center.x + control.size.x / 2.0 - (suffix.center.x - suffix.size.x / 2.0))
                .abs()
                < 0.01
        );
    }

    #[test]
    fn command_component_uses_the_shared_capped_palette_metrics() {
        let definition = block_by_slug("application-ui-navigation-command-palettes-01-simple")
            .expect("command palette fixture should exist");
        let viewport = Vec2::new(1_000.0, 700.0);
        let theme = ThemeId::Dark.palette();
        let primitives =
            bevy_block_primitives(&BlockInstance::fixture(definition), &theme, viewport)
                .expect("command palette fixture should produce primitives");
        let host = primitives
            .iter()
            .find(|primitive| {
                matches!(
                    primitive.kind,
                    BevyBlockPrimitiveKind::UiComponentHost(UiComponentId::Command)
                )
            })
            .expect("command palette fixture should include its shared host");
        let fixture = canonical_ui_story_fixture(UiComponentId::Command);
        let UiStoryModel::Command(model) = fixture.model else {
            unreachable!("canonical Command fixture must contain a Command model");
        };
        let expected = command_layout_metrics(
            &model,
            &model.state(),
            scale::container::CONTENT,
            viewport.x,
            theme.border.max(1.0),
        );

        assert_eq!(host.size.x, scale::container::CONTROL);
        assert!((host.size.y - expected.height).abs() < 0.01);
        assert!(
            host.center.x < 0.0,
            "the capped palette should remain left aligned"
        );
    }

    #[test]
    fn aspect_ratio_component_uses_the_full_shared_ratio_geometry() {
        let definition =
            block_by_slug("application-ui-layout-containers-02-constrained-with-padded-content")
                .expect("container fixture should exist");
        let viewport = Vec2::new(1_000.0, 700.0);
        let theme = ThemeId::Dark.palette();
        let instance = BlockInstance::fixture(definition);
        let plan = plan_block(&instance).expect("container fixture should plan");
        let primitives = bevy_block_primitives(&instance, &theme, viewport)
            .expect("container fixture should produce primitives");
        let host = primitives
            .iter()
            .find(|primitive| {
                matches!(
                    primitive.kind,
                    BevyBlockPrimitiveKind::UiComponentHost(UiComponentId::AspectRatio)
                )
            })
            .expect("container fixture should include its AspectRatio host");
        let expected_width =
            plan.container.width.points() - plan.container.gutter.points_at(viewport.x) * 2.0;
        let fixture = canonical_ui_story_fixture(UiComponentId::AspectRatio);
        let UiStoryModel::AspectRatio(model) = fixture.model else {
            unreachable!("canonical AspectRatio fixture must contain an AspectRatio model");
        };
        let expected =
            aspect_ratio_layout_metrics(&model, expected_width, viewport.x, theme.border.max(1.0));

        assert!((host.size.x - expected_width).abs() < 0.01);
        assert!((host.size.y - expected.height).abs() < 0.01);
        assert!(expected.frame_height > 260.0);
    }

    #[test]
    fn drawer_component_keeps_its_viewport_overlay_out_of_block_flow() {
        let definition = block_by_slug("application-ui-overlays-drawers-01-empty")
            .expect("drawer fixture should exist");
        let viewport = Vec2::new(1_000.0, 700.0);
        let theme = ThemeId::Dark.palette();
        let primitives =
            bevy_block_primitives(&BlockInstance::fixture(definition), &theme, viewport)
                .expect("drawer fixture should produce primitives");
        let host = primitives
            .iter()
            .find(|primitive| {
                matches!(
                    primitive.kind,
                    BevyBlockPrimitiveKind::UiComponentHost(UiComponentId::Drawer)
                )
            })
            .expect("drawer fixture should include its shared host");
        let fixture = canonical_ui_story_fixture(UiComponentId::Drawer);
        let UiStoryModel::Drawer(model) = fixture.model else {
            unreachable!("canonical Drawer fixture must contain a Drawer model");
        };
        let expected = drawer_layout_metrics(
            &model,
            viewport.x,
            viewport.y,
            viewport.x,
            theme.border.max(1.0),
        );

        assert!((host.size.y - expected.trigger_height).abs() < 0.01);
        assert!(expected.panel_height > host.size.y * 4.0);
        assert_eq!(
            primitives
                .iter()
                .filter(|primitive| matches!(
                    primitive.kind,
                    BevyBlockPrimitiveKind::UiComponentHost(UiComponentId::Drawer)
                ))
                .count(),
            1
        );
    }

    #[test]
    fn item_component_uses_the_canonical_horizontal_fixture() {
        let definition = block_by_slug("marketing-elements-headers-03-on-brand-background")
            .expect("header fixture should exist");
        let primitives = bevy_block_primitives(
            &BlockInstance::fixture(definition),
            &ThemeId::Dark.palette(),
            Vec2::new(1_000.0, 700.0),
        )
        .expect("header fixture should produce primitives");
        let root = primitives
            .iter()
            .find(|primitive| primitive.part == "Item")
            .expect("item root should exist");
        let title = primitives
            .iter()
            .find(|primitive| primitive.part == "ItemTitle")
            .expect("item title should exist");
        let actions = primitives
            .iter()
            .filter(|primitive| primitive.part.starts_with("ItemActions"))
            .collect::<Vec<_>>();

        assert!((root.size.x - scale::container::CONTROL).abs() < 0.01);
        assert_eq!(title.label, "Token migration");
        assert_eq!(actions.len(), 2);
        assert!(actions.iter().all(|action| action.center.y > root.center.y));
    }

    #[test]
    fn media_and_collection_regions_never_overlap() {
        let theme = ThemeId::Light.palette();
        let viewport = Vec2::new(1_280.0, 720.0);

        for definition in BLOCKS {
            let instance = BlockInstance::fixture(&definition);
            let plan = plan_block(&instance).expect("fixture should produce a plan");
            if !plan.media.is_visible() || !plan.shows_collection_items() {
                continue;
            }

            let primitives = bevy_block_primitives(&instance, &theme, viewport)
                .expect("block should produce Bevy primitives");
            let media = primitives
                .iter()
                .find(|primitive| primitive.kind == BevyBlockPrimitiveKind::Media)
                .expect("media plan should emit media");
            for item in primitives
                .iter()
                .filter(|primitive| primitive.kind == BevyBlockPrimitiveKind::Item)
            {
                assert!(
                    media.center.y - media.size.y / 2.0 >= item.center.y + item.size.y / 2.0,
                    "{} media and item regions overlap",
                    definition.slug,
                );
            }
        }
    }

    #[test]
    fn theme_geometry_changes_are_limited_to_the_border_token() {
        let viewport = Vec2::new(1_280.0, 720.0);
        let baseline_theme = ThemeId::Light.palette();
        for definition in BLOCKS {
            let instance = BlockInstance::fixture(&definition);
            for theme_id in ThemeId::ALL {
                let theme = theme_id.palette();
                let themed = bevy_block_primitives(&instance, &theme, viewport)
                    .expect("themed block plan should render");
                let border_only_theme = Theme {
                    border: theme.border,
                    ..baseline_theme
                };
                let border_only = bevy_block_primitives(&instance, &border_only_theme, viewport)
                    .expect("border-only block plan should render");
                assert_eq!(themed.len(), border_only.len(), "{}", definition.slug);
                for (actual, expected) in themed.iter().zip(&border_only) {
                    assert_eq!(actual.kind, expected.kind, "{}", definition.slug);
                    assert_eq!(actual.part, expected.part, "{}", definition.slug);
                    assert!(
                        (actual.center - expected.center).abs().max_element() <= f32::EPSILON,
                        "{} `{}` center changed independently of the border token for {}: actual={:?}, expected={:?}",
                        definition.slug,
                        actual.part,
                        theme_id.label(),
                        actual.center,
                        expected.center,
                    );
                    assert!(
                        (actual.size - expected.size).abs().max_element() <= f32::EPSILON,
                        "{} `{}` size changed independently of the border token for {}: actual={:?}, expected={:?}",
                        definition.slug,
                        actual.part,
                        theme_id.label(),
                        actual.size,
                        expected.size,
                    );
                }
            }
        }
    }
}
