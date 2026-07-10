use bevy::prelude::{Color, Vec2};
use rs_dean_ui::{
    BevyUiPrimitive, ButtonSize, ButtonVariant, GridAlign, GridJustify, LayoutRect, SectionSurface,
    Theme, UiBlockRole, bevy_primitives_for_component, scale,
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
    pub ui: Option<BevyUiPrimitive>,
}

pub fn bevy_block_primitives(
    instance: &BlockInstance,
    theme: &Theme,
    viewport: Vec2,
) -> Result<Vec<BevyBlockPrimitive>, garde::Report> {
    let plan = plan_block(instance)?;
    let gutter = plan.container.gutter.points();
    let max_width = plan.container.width.points();
    let width = if max_width.is_finite() {
        (viewport.x - gutter * 2.0).min(max_width)
    } else {
        viewport.x - gutter * 2.0
    }
    .max(240.0);
    let height = viewport.y.max(480.0);
    let gap = plan.outer_grid.gap.points();
    let stacked = plan.layout.outer_grid().columns(viewport.x) == 1;
    let (content_center, content_size, support_center, support_size) = if !plan.has_support() {
        (
            Vec2::ZERO,
            Vec2::new(width, height),
            Vec2::ZERO,
            Vec2::new(width, height),
        )
    } else if stacked {
        let region_height = (height - gap) / 2.0;
        (
            Vec2::new(0.0, region_height / 2.0 + gap / 2.0),
            Vec2::new(width, region_height),
            Vec2::new(0.0, -(region_height / 2.0 + gap / 2.0)),
            Vec2::new(width, region_height),
        )
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
                Vec2::new((width - content_width) / 2.0, 0.0),
                Vec2::new(-(width - support_width) / 2.0, 0.0),
            )
        } else {
            (
                Vec2::new(-(width - content_width) / 2.0, 0.0),
                Vec2::new((width - support_width) / 2.0, 0.0),
            )
        };
        (
            content,
            Vec2::new(content_width, height),
            support,
            Vec2::new(support_width, height),
        )
    };

    let (section_fill, section_text) = section_colors(plan.section.surface, theme);
    let mut primitives = vec![BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::Section,
        part: "section".to_owned(),
        label: plan.block.definition().name.to_owned(),
        value: plan.block.definition().slug.to_owned(),
        center: Vec2::ZERO,
        size: Vec2::new(viewport.x.max(width), height),
        fill: section_fill,
        border: Color::NONE,
        border_width: 0.0,
        text: section_text,
        text_align: GridAlign::Stretch,
        font_size: 0.0,
        ui: None,
    }];

    push_copy_primitives(&mut primitives, &plan, theme, content_center, content_size);
    push_support_primitives(
        &mut primitives,
        &plan,
        theme,
        support_center,
        support_size,
        viewport.x,
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
) {
    let left = center.x - size.x / 2.0;
    let (eyebrow_text, heading_text, body_text) = copy_colors(plan.section.surface, theme);
    for (index, (kind, part, label, value, font_size)) in [
        (
            BevyBlockPrimitiveKind::Text,
            "eyebrow",
            plan.content.eyebrow.as_str(),
            plan.pattern.label(),
            scale::font_size::F00,
        ),
        (
            BevyBlockPrimitiveKind::Heading,
            "title",
            plan.content.title.as_str(),
            plan.block.definition().slug,
            if plan.pattern == crate::BlockPattern::Hero {
                scale::font_size::F4
            } else {
                scale::font_size::F2
            },
        ),
        (
            BevyBlockPrimitiveKind::Text,
            "body",
            plan.content.body.as_str(),
            plan.pattern.summary(),
            scale::font_size::F0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        output.push(BevyBlockPrimitive {
            kind,
            part: part.to_owned(),
            label: label.to_owned(),
            value: value.to_owned(),
            center: Vec2::new(
                left + size.x / 2.0,
                center.y
                    + match index {
                        0 => 120.0,
                        1 => 50.0,
                        _ => -40.0,
                    },
            ),
            size: Vec2::new(
                size.x,
                match kind {
                    BevyBlockPrimitiveKind::Heading => 72.0,
                    BevyBlockPrimitiveKind::Text if part == "body" => 96.0,
                    BevyBlockPrimitiveKind::Section
                    | BevyBlockPrimitiveKind::Text
                    | BevyBlockPrimitiveKind::Action
                    | BevyBlockPrimitiveKind::Item
                    | BevyBlockPrimitiveKind::Media
                    | BevyBlockPrimitiveKind::UiComponent => 40.0,
                },
            ),
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
            ui: None,
        });
    }

    let action_gap = plan.action_cluster.gap.points();
    let mut rows = Vec::<Vec<(usize, Vec2)>>::new();
    for (index, action) in plan.content.actions.iter().enumerate() {
        let action_size = bevy_action_size(action.size);
        let row = rows.last_mut();
        let row_width = row
            .as_deref()
            .map(|items| {
                items.iter().map(|(_, item_size)| item_size.x).sum::<f32>()
                    + items.len().saturating_sub(1) as f32 * action_gap
            })
            .unwrap_or(0.0);
        if row.is_none() || (row_width + action_gap + action_size.x > size.x && row_width > 0.0) {
            rows.push(vec![(index, action_size)]);
        } else if let Some(row) = rows.last_mut() {
            row.push((index, action_size));
        }
    }

    let mut action_y = center.y - 130.0;
    for row in rows {
        let row_height = row
            .iter()
            .map(|(_, action_size)| action_size.y)
            .fold(0.0_f32, f32::max);
        let row_width = row
            .iter()
            .map(|(_, action_size)| action_size.x)
            .sum::<f32>()
            + row.len().saturating_sub(1) as f32 * action_gap;
        let mut action_left = if plan.action_cluster.justify == GridJustify::Center {
            center.x - row_width / 2.0
        } else {
            left
        };
        for (index, action_size) in row {
            let action = &plan.content.actions[index];
            let (fill, border, text) = bevy_action_colors(action.variant, theme);
            let action_x = action_left + action_size.x / 2.0;
            action_left += action_size.x + action_gap;
            output.push(BevyBlockPrimitive {
                kind: BevyBlockPrimitiveKind::Action,
                part: "action".to_owned(),
                label: action.label.clone(),
                value: action.value.clone(),
                center: Vec2::new(action_x, action_y),
                size: action_size,
                fill,
                border,
                border_width: 1.0,
                text,
                text_align: GridAlign::Center,
                font_size: scale::font_size::F00,
                ui: None,
            });
        }
        action_y -= row_height + action_gap;
    }
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

fn bevy_action_size(size: ButtonSize) -> Vec2 {
    match size {
        ButtonSize::Small => Vec2::new(150.0, 36.0),
        ButtonSize::Medium => Vec2::new(170.0, 44.0),
        ButtonSize::Large => Vec2::new(190.0, 52.0),
        ButtonSize::Icon => Vec2::splat(44.0),
    }
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
) {
    let shows_collection = plan.shows_collection_items();
    let shows_component = plan.shows_primary_component();
    let has_secondary_support = shows_collection || shows_component;
    let support_gap = plan.content_stack.gap.points();
    let (media_bounds, secondary_bounds) = if plan.media.is_visible() && has_secondary_support {
        let media_height = (size.y * 0.46).max(1.0);
        let secondary_height = (size.y - media_height - support_gap).max(1.0);
        (
            LayoutRect {
                center_x: center.x,
                center_y: center.y + (size.y - media_height) / 2.0,
                width: size.x,
                height: media_height,
            },
            LayoutRect {
                center_x: center.x,
                center_y: center.y - (size.y - secondary_height) / 2.0,
                width: size.x,
                height: secondary_height,
            },
        )
    } else {
        let media_height = size.y * 0.78;
        (
            LayoutRect {
                center_x: center.x,
                center_y: center.y,
                width: size.x,
                height: media_height,
            },
            LayoutRect {
                center_x: center.x,
                center_y: center.y,
                width: size.x,
                height: size.y,
            },
        )
    };

    if plan.media.is_visible() {
        output.push(BevyBlockPrimitive {
            kind: BevyBlockPrimitiveKind::Media,
            part: "media".to_owned(),
            label: plan.content.media_label.clone(),
            value: plan.block.definition().slug.to_owned(),
            center: Vec2::new(media_bounds.center_x, media_bounds.center_y),
            size: Vec2::new(media_bounds.width, media_bounds.height),
            fill: theme.surface_2().to_bevy(),
            border: theme.border_subtle().to_bevy(),
            border_width: 1.0,
            text: theme.text_muted().to_bevy(),
            text_align: GridAlign::Center,
            font_size: scale::font_size::F00,
            ui: None,
        });
    }

    if shows_collection {
        push_collection_primitives(
            output,
            plan,
            theme,
            Vec2::new(secondary_bounds.center_x, secondary_bounds.center_y),
            Vec2::new(secondary_bounds.width, secondary_bounds.height),
            viewport_width,
        );
    }

    if shows_component {
        push_component_primitives(output, plan, theme, secondary_bounds);
    }
}

fn push_component_primitives(
    output: &mut Vec<BevyBlockPrimitive>,
    plan: &crate::BlockPlan,
    theme: &Theme,
    bounds: LayoutRect,
) {
    let mut primitives = bevy_primitives_for_component(plan.primary_component, theme)
        .into_iter()
        .filter(|primitive| !(primitive.role == UiBlockRole::Feedback && primitive.disabled))
        .collect::<Vec<_>>();
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
    let gap = 8.0;
    let padding = 16.0;
    let component_width = bounds.width.clamp(240.0, 520.0);
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
        kind: BevyBlockPrimitiveKind::UiComponent,
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
            ui: Some(ui),
        });
        cursor_y -= row_height / 2.0 + gap * row_scale;
    }
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
    let cells = plan.collection_grid.equal_track_cells(
        LayoutRect {
            center_x: center.x,
            center_y: center.y,
            width: size.x,
            height: size.y,
        },
        viewport_width,
        plan.content.items.len(),
    );

    for (item, cell) in plan.content.items.iter().zip(cells) {
        output.push(BevyBlockPrimitive {
            kind: BevyBlockPrimitiveKind::Item,
            part: "item".to_owned(),
            label: String::new(),
            value: item.value.clone(),
            center: Vec2::new(cell.center_x, cell.center_y),
            size: Vec2::new(cell.width, cell.height),
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
            border_width: 1.0,
            text: theme.text_1().to_bevy(),
            text_align: GridAlign::Start,
            font_size: 0.0,
            ui: None,
        });

        let inset = 16.0_f32.min(cell.width / 6.0).min(cell.height / 6.0);
        let text_width = (cell.width - inset * 2.0).max(24.0);
        let top = cell.center_y + cell.height / 2.0;
        let bottom = cell.center_y - cell.height / 2.0;
        let title_height = 30.0_f32.min((cell.height - inset * 2.0).max(1.0));
        let title_y = if cell.height >= 72.0 {
            top - inset - title_height / 2.0
        } else {
            cell.center_y
        };
        push_collection_text(
            output,
            "item-title",
            item.title.clone(),
            Vec2::new(cell.center_x, title_y),
            Vec2::new(text_width, title_height),
            theme.text_1().to_bevy(),
            scale::font_size::F0,
        );
        let mut content_top = title_y - title_height / 2.0;
        if cell.height >= 72.0 {
            let meta_height = 24.0;
            let meta_y = content_top - 4.0 - meta_height / 2.0;
            push_collection_text(
                output,
                "item-meta",
                item.meta.clone(),
                Vec2::new(cell.center_x, meta_y),
                Vec2::new(text_width, meta_height),
                theme.text_muted().to_bevy(),
                scale::font_size::F000,
            );
            content_top = meta_y - meta_height / 2.0;
        }

        let action_geometry = if cell.height >= 104.0 {
            let default_size = bevy_action_size(ButtonSize::Small);
            let button_size = Vec2::new(
                default_size.x.min((cell.width - inset * 2.0).max(1.0)),
                default_size.y,
            );
            let button_center = Vec2::new(
                cell.center_x + cell.width / 2.0 - inset - button_size.x / 2.0,
                bottom + inset + button_size.y / 2.0,
            );
            let (fill, border, text) = bevy_action_colors(ButtonVariant::Outline, theme);
            output.push(BevyBlockPrimitive {
                kind: BevyBlockPrimitiveKind::Action,
                part: "item-action".to_owned(),
                label: "Open".to_owned(),
                value: item.value.clone(),
                center: button_center,
                size: button_size,
                fill,
                border,
                border_width: 1.0,
                text,
                text_align: GridAlign::Center,
                font_size: scale::font_size::F00,
                ui: None,
            });
            Some((button_center, button_size))
        } else {
            None
        };

        if cell.height >= 152.0 {
            let body_bottom = action_geometry
                .map_or(bottom + inset, |(button_center, button_size)| {
                    button_center.y + button_size.y / 2.0 + 8.0
                });
            let body_height = content_top - 8.0 - body_bottom;
            if body_height >= 24.0 {
                push_collection_text(
                    output,
                    "item-body",
                    item.body.clone(),
                    Vec2::new(cell.center_x, body_bottom + body_height / 2.0),
                    Vec2::new(text_width, body_height),
                    theme.text_2().to_bevy(),
                    scale::font_size::F00,
                );
            }
        }
    }
}

fn push_collection_text(
    output: &mut Vec<BevyBlockPrimitive>,
    part: &str,
    label: String,
    center: Vec2,
    size: Vec2,
    text: Color,
    font_size: f32,
) {
    output.push(BevyBlockPrimitive {
        kind: BevyBlockPrimitiveKind::Text,
        part: part.to_owned(),
        label,
        value: String::new(),
        center,
        size,
        fill: Color::NONE,
        border: Color::NONE,
        border_width: 0.0,
        text,
        text_align: GridAlign::Start,
        font_size,
        ui: None,
    });
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;
    use rs_dean_ui::ThemeId;

    use crate::{BLOCKS, BlockInstance};

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
                            primitive.kind == BevyBlockPrimitiveKind::UiComponent
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
    fn every_theme_changes_palette_without_changing_block_geometry() {
        let viewport = Vec2::new(1_280.0, 720.0);
        for definition in BLOCKS {
            let instance = BlockInstance::fixture(&definition);
            let baseline = bevy_block_primitives(&instance, &ThemeId::Light.palette(), viewport)
                .expect("light block plan should render");
            for theme_id in ThemeId::ALL {
                let themed = bevy_block_primitives(&instance, &theme_id.palette(), viewport)
                    .expect("themed block plan should render");
                assert_eq!(themed.len(), baseline.len(), "{}", definition.slug);
                for (actual, expected) in themed.iter().zip(&baseline) {
                    assert_eq!(actual.kind, expected.kind, "{}", definition.slug);
                    assert_eq!(actual.part, expected.part, "{}", definition.slug);
                    assert_eq!(actual.center, expected.center, "{}", definition.slug);
                    assert_eq!(actual.size, expected.size, "{}", definition.slug);
                }
            }
        }
    }
}
