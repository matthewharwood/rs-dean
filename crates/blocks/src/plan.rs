use garde::Validate;
use rs_dean_ui::{
    ClusterSpec, ContainerSpec, GridAlign, GridItemSpec, GridJustify, GridSpec, SectionSpec,
    SectionSurface, SpaceToken, StackSpec, UiComponentId,
};
use serde::{Deserialize, Serialize};

use crate::{
    BlockContent, BlockId, BlockInstance, BlockInteraction, BlockLayoutPreset, BlockMediaKind,
    BlockPattern, BlockSurfaceChrome,
};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct BlockPlan {
    #[garde(dive)]
    pub block: BlockId,
    #[garde(skip)]
    pub pattern: BlockPattern,
    #[garde(skip)]
    pub layout: BlockLayoutPreset,
    #[garde(skip)]
    pub media: BlockMediaKind,
    #[garde(skip)]
    pub chrome: BlockSurfaceChrome,
    #[garde(skip)]
    pub interaction: BlockInteraction,
    #[garde(dive)]
    pub section: SectionSpec,
    #[garde(dive)]
    pub container: ContainerSpec,
    #[garde(dive)]
    pub outer_grid: GridSpec,
    #[garde(dive)]
    pub content_item: GridItemSpec,
    #[garde(dive)]
    pub support_item: GridItemSpec,
    #[garde(dive)]
    pub collection_grid: GridSpec,
    #[garde(dive)]
    pub content_stack: StackSpec,
    #[garde(dive)]
    pub action_cluster: ClusterSpec,
    #[garde(skip)]
    pub primary_component: UiComponentId,
    #[garde(skip)]
    pub reverse: bool,
    #[garde(dive)]
    pub content: BlockContent,
}

impl BlockPlan {
    pub const fn shows_primary_component(&self) -> bool {
        match self.pattern {
            BlockPattern::Hero
            | BlockPattern::Feature
            | BlockPattern::CallToAction
            | BlockPattern::Bento
            | BlockPattern::Pricing
            | BlockPattern::Metrics
            | BlockPattern::Testimonial
            | BlockPattern::Collection
            | BlockPattern::Page => false,
            BlockPattern::Data => !matches!(
                self.primary_component,
                UiComponentId::Card | UiComponentId::Item
            ),
            BlockPattern::Navigation
            | BlockPattern::Form
            | BlockPattern::Editorial
            | BlockPattern::Faq
            | BlockPattern::Footer
            | BlockPattern::Menu
            | BlockPattern::Feedback
            | BlockPattern::Empty
            | BlockPattern::Shell
            | BlockPattern::Heading
            | BlockPattern::Calendar
            | BlockPattern::Overlay
            | BlockPattern::Component
            | BlockPattern::Layout
            | BlockPattern::Commerce
            | BlockPattern::CommerceSummary => true,
        }
    }

    pub const fn shows_collection_items(&self) -> bool {
        matches!(
            self.pattern,
            BlockPattern::Feature
                | BlockPattern::Bento
                | BlockPattern::Pricing
                | BlockPattern::Metrics
                | BlockPattern::Testimonial
                | BlockPattern::Collection
                | BlockPattern::Page
        ) || (matches!(self.pattern, BlockPattern::Data)
            && matches!(
                self.primary_component,
                UiComponentId::Card | UiComponentId::Item
            ))
    }

    pub const fn has_support(&self) -> bool {
        self.media.is_visible() || self.shows_collection_items() || self.shows_primary_component()
    }
}

pub fn plan_block(instance: &BlockInstance) -> Result<BlockPlan, garde::Report> {
    crate::validate_block_instance(instance)?;
    let definition = instance.definition();
    let family = definition.family_definition();
    let surface = section_surface(family.pattern, definition.chrome, definition.ordinal);
    let padding_block = section_padding(family.pattern);
    let layout = definition.resolved_layout();
    let plan = BlockPlan {
        block: definition.id,
        pattern: family.pattern,
        layout,
        media: definition.media,
        chrome: definition.chrome,
        interaction: definition.interaction,
        section: SectionSpec {
            surface,
            padding_block,
        },
        container: ContainerSpec {
            width: layout.container_width(),
            gutter: SpaceToken::Small,
        },
        outer_grid: GridSpec {
            preset: layout.outer_grid(),
            gap: SpaceToken::Large,
            align: GridAlign::Center,
            justify: GridJustify::Start,
        },
        content_item: GridItemSpec {
            span: layout.content_span(),
            align: GridAlign::Center,
        },
        support_item: GridItemSpec {
            span: layout.support_span(),
            align: GridAlign::Stretch,
        },
        collection_grid: GridSpec {
            preset: layout.collection_grid(),
            gap: SpaceToken::Small,
            align: GridAlign::Stretch,
            justify: GridJustify::Start,
        },
        content_stack: StackSpec {
            gap: SpaceToken::Small,
            align: if matches!(
                layout,
                BlockLayoutPreset::Centered | BlockLayoutPreset::CenteredNarrow
            ) {
                GridAlign::Center
            } else {
                GridAlign::Start
            },
        },
        action_cluster: ClusterSpec {
            gap: SpaceToken::ExtraSmall,
            align: GridAlign::Center,
            justify: if matches!(
                layout,
                BlockLayoutPreset::Centered | BlockLayoutPreset::CenteredNarrow
            ) {
                GridJustify::Center
            } else {
                GridJustify::Start
            },
        },
        primary_component: family.primary_component,
        reverse: layout.reverses_content(),
        content: instance.content.clone(),
    };
    validate_block_plan(&plan)?;
    Ok(plan)
}

pub fn validate_block_plan(plan: &BlockPlan) -> Result<(), garde::Report> {
    plan.validate()
}

const fn section_surface(
    pattern: BlockPattern,
    chrome: BlockSurfaceChrome,
    ordinal: u8,
) -> SectionSurface {
    match chrome {
        BlockSurfaceChrome::Dark => return SectionSurface::Contrast,
        BlockSurfaceChrome::Brand => return SectionSurface::Brand,
        BlockSurfaceChrome::Subtle => return SectionSurface::Subtle,
        BlockSurfaceChrome::Panel | BlockSurfaceChrome::Bordered => {
            return SectionSurface::Elevated;
        }
        BlockSurfaceChrome::Plain => {}
    }
    match pattern {
        BlockPattern::CallToAction if ordinal.is_multiple_of(3) => SectionSurface::Brand,
        BlockPattern::Hero if ordinal.is_multiple_of(4) => SectionSurface::Contrast,
        BlockPattern::Overlay => SectionSurface::Sunken,
        BlockPattern::Shell | BlockPattern::Data => SectionSurface::Subtle,
        BlockPattern::Pricing | BlockPattern::CommerceSummary => SectionSurface::Elevated,
        _ if ordinal.is_multiple_of(2) => SectionSurface::Subtle,
        _ => SectionSurface::Surface,
    }
}

const fn section_padding(pattern: BlockPattern) -> SpaceToken {
    match pattern {
        BlockPattern::Hero | BlockPattern::Page => SpaceToken::TwoExtraLarge,
        BlockPattern::CallToAction
        | BlockPattern::Feature
        | BlockPattern::Pricing
        | BlockPattern::Editorial
        | BlockPattern::Commerce => SpaceToken::ExtraLarge,
        BlockPattern::Navigation
        | BlockPattern::Footer
        | BlockPattern::Heading
        | BlockPattern::Component => SpaceToken::Medium,
        BlockPattern::Bento
        | BlockPattern::Form
        | BlockPattern::Metrics
        | BlockPattern::Testimonial
        | BlockPattern::Collection
        | BlockPattern::Faq
        | BlockPattern::Menu
        | BlockPattern::Feedback
        | BlockPattern::Empty
        | BlockPattern::Shell
        | BlockPattern::Data
        | BlockPattern::Calendar
        | BlockPattern::Overlay
        | BlockPattern::Layout
        | BlockPattern::CommerceSummary => SpaceToken::Large,
    }
}

#[cfg(test)]
mod tests {
    use crate::{BLOCKS, BlockInstance};

    use super::*;

    #[test]
    fn every_fixture_builds_a_valid_cross_renderer_plan() {
        for definition in BLOCKS {
            plan_block(&BlockInstance::fixture(&definition))
                .expect("catalog fixture should produce a valid plan");
        }
    }
}
