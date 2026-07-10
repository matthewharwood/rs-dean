use garde::Validate;
use rs_dean_ui::{ContainerWidth, GridPreset, GridSpan, MediaRatio, UiComponentId};
use serde::{Deserialize, Serialize};

pub const BLOCK_FAMILY_COUNT: usize = 93;
pub const BLOCK_COUNT: usize = 657;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Serialize, Validate)]
#[serde(transparent)]
pub struct BlockId(#[garde(custom(validate_block_id_value))] pub(crate) u16);

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockSurface {
    Marketing,
    ApplicationUi,
    Ecommerce,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockSourceKind {
    Component,
    PageExample,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockPattern {
    Hero,
    Feature,
    CallToAction,
    Bento,
    Pricing,
    Navigation,
    Form,
    Metrics,
    Testimonial,
    Collection,
    Editorial,
    Faq,
    Footer,
    Menu,
    Feedback,
    Empty,
    Page,
    Shell,
    Heading,
    Data,
    Calendar,
    Overlay,
    Component,
    Layout,
    Commerce,
    CommerceSummary,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockLayoutPreset {
    Stack,
    Full,
    Centered,
    CenteredNarrow,
    Split,
    SplitReverse,
    SidebarStart,
    SidebarEnd,
    ThreeColumn,
    FourColumn,
    TwelveColumn,
    Bento,
    Overlay,
    Shell,
    Page,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockMediaKind {
    None,
    Image,
    BackgroundImage,
    Screenshot,
    Mockup,
    Product,
    Avatar,
    Logo,
    Icon,
    Video,
    Code,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockSurfaceChrome {
    Plain,
    Subtle,
    Panel,
    Bordered,
    Brand,
    Dark,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockInteraction {
    Static,
    Actions,
    Form,
    Toggle,
    Filter,
    Search,
    Menu,
    Overlay,
    Pagination,
    Tabs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockFamilyDefinition {
    pub id: BlockFamily,
    pub surface: BlockSurface,
    pub area: &'static str,
    pub slug: &'static str,
    pub name: &'static str,
    pub source_url: &'static str,
    pub block_count: usize,
    pub source_kind: BlockSourceKind,
    pub pattern: BlockPattern,
    pub primary_component: UiComponentId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockDefinition {
    pub id: BlockId,
    pub family: BlockFamily,
    pub ordinal: u8,
    pub slug: &'static str,
    pub name: &'static str,
    pub source_id: &'static str,
    pub layout: BlockLayoutPreset,
    pub media: BlockMediaKind,
    pub chrome: BlockSurfaceChrome,
    pub interaction: BlockInteraction,
}

impl BlockId {
    const fn new(index: u16) -> Self {
        Self(index)
    }

    pub const fn from_index(index: usize) -> Option<Self> {
        if index < BLOCK_COUNT {
            Some(Self(index as u16))
        } else {
            None
        }
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }

    pub const fn definition(self) -> &'static BlockDefinition {
        &BLOCKS[self.index()]
    }
}

impl BlockSurface {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Marketing => "Marketing",
            Self::ApplicationUi => "Application UI",
            Self::Ecommerce => "Ecommerce",
        }
    }

    pub const fn slug(self) -> &'static str {
        match self {
            Self::Marketing => "marketing",
            Self::ApplicationUi => "application-ui",
            Self::Ecommerce => "ecommerce",
        }
    }
}

impl BlockSourceKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Component => "Component",
            Self::PageExample => "Page example",
        }
    }
}

impl BlockPattern {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hero => "Hero",
            Self::Feature => "Feature",
            Self::CallToAction => "Call to action",
            Self::Bento => "Bento",
            Self::Pricing => "Pricing",
            Self::Navigation => "Navigation",
            Self::Form => "Form",
            Self::Metrics => "Metrics",
            Self::Testimonial => "Testimonial",
            Self::Collection => "Collection",
            Self::Editorial => "Editorial",
            Self::Faq => "FAQ",
            Self::Footer => "Footer",
            Self::Menu => "Menu",
            Self::Feedback => "Feedback",
            Self::Empty => "Empty state",
            Self::Page => "Page composition",
            Self::Shell => "Application shell",
            Self::Heading => "Heading",
            Self::Data => "Data display",
            Self::Calendar => "Calendar",
            Self::Overlay => "Overlay",
            Self::Component => "Component composition",
            Self::Layout => "Layout",
            Self::Commerce => "Commerce feature",
            Self::CommerceSummary => "Commerce summary",
        }
    }

    pub const fn summary(self) -> &'static str {
        match self {
            Self::Hero => "Lead with a focused value proposition, actions, and optional media.",
            Self::Feature => {
                "Explain product capabilities through repeatable content and media slots."
            }
            Self::CallToAction => {
                "Resolve a section around one clear decision and supporting action."
            }
            Self::Bento => "Arrange heterogeneous feature cards on a constrained modular grid.",
            Self::Pricing => "Compare offers, included value, and purchase actions.",
            Self::Navigation => {
                "Provide predictable wayfinding and responsive navigation behavior."
            }
            Self::Form => {
                "Collect validated input with clear labels, help, errors, and submission intent."
            }
            Self::Metrics => "Make key values and their context easy to scan and compare.",
            Self::Testimonial => {
                "Present attributed customer evidence with readable quote hierarchy."
            }
            Self::Collection => "Render repeatable records as a list or responsive card grid.",
            Self::Editorial => "Compose long-form copy and media with controlled reading width.",
            Self::Faq => "Disclose answers progressively while preserving keyboard access.",
            Self::Footer => "Close a page with grouped navigation, identity, and policy links.",
            Self::Menu => "Expose contextual choices without losing focus or selection state.",
            Self::Feedback => "Communicate status, urgency, and recovery actions.",
            Self::Empty => "Explain an unavailable state and offer the most useful next action.",
            Self::Page => "Compose registered blocks into a validated vertical page sequence.",
            Self::Shell => "Define persistent navigation and work regions for an application.",
            Self::Heading => "Orient the user with title, metadata, breadcrumbs, and actions.",
            Self::Data => "Display structured records with comparison and interaction affordances.",
            Self::Calendar => "Place date-oriented data and actions in a stable time grid.",
            Self::Overlay => {
                "Layer a focused task over the current context with explicit dismissal."
            }
            Self::Component => "Arrange a focused set of existing UI component variants.",
            Self::Layout => "Demonstrate container, surface, divider, and media layout contracts.",
            Self::Commerce => {
                "Combine product media, information, selection, and purchase actions."
            }
            Self::CommerceSummary => {
                "Summarize transactional line items, totals, and next actions."
            }
        }
    }
}

impl BlockLayoutPreset {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Stack => "Stack",
            Self::Full => "Full width",
            Self::Centered => "Centered",
            Self::CenteredNarrow => "Centered narrow",
            Self::Split => "Split",
            Self::SplitReverse => "Split reverse",
            Self::SidebarStart => "Sidebar start",
            Self::SidebarEnd => "Sidebar end",
            Self::ThreeColumn => "Three column",
            Self::FourColumn => "Four column",
            Self::TwelveColumn => "Twelve column",
            Self::Bento => "Bento",
            Self::Overlay => "Overlay",
            Self::Shell => "Shell",
            Self::Page => "Page",
        }
    }

    pub const fn container_width(self) -> ContainerWidth {
        match self {
            Self::CenteredNarrow => ContainerWidth::Narrow,
            Self::Centered | Self::Stack => ContainerWidth::Prose,
            Self::Full => ContainerWidth::Full,
            Self::Split
            | Self::SplitReverse
            | Self::SidebarStart
            | Self::SidebarEnd
            | Self::ThreeColumn
            | Self::FourColumn
            | Self::TwelveColumn
            | Self::Bento
            | Self::Overlay
            | Self::Shell
            | Self::Page => ContainerWidth::Wide,
        }
    }

    pub const fn outer_grid(self) -> GridPreset {
        match self {
            Self::Split | Self::SplitReverse => GridPreset::Split,
            Self::SidebarStart | Self::SidebarEnd | Self::TwelveColumn | Self::Shell => {
                GridPreset::Twelve
            }
            Self::Stack
            | Self::Full
            | Self::Centered
            | Self::CenteredNarrow
            | Self::ThreeColumn
            | Self::FourColumn
            | Self::Bento
            | Self::Overlay
            | Self::Page => GridPreset::Stack,
        }
    }

    pub const fn collection_grid(self) -> GridPreset {
        match self {
            Self::ThreeColumn | Self::Bento => GridPreset::ThreeUp,
            Self::FourColumn => GridPreset::FourUp,
            Self::TwelveColumn => GridPreset::ThreeUp,
            Self::Split | Self::SplitReverse => GridPreset::Split,
            Self::SidebarStart | Self::SidebarEnd => GridPreset::ThreeUp,
            Self::Stack
            | Self::Full
            | Self::Centered
            | Self::CenteredNarrow
            | Self::Overlay
            | Self::Shell
            | Self::Page => GridPreset::Stack,
        }
    }

    pub const fn content_span(self) -> GridSpan {
        match self {
            Self::SidebarStart => GridSpan::Four,
            Self::SidebarEnd => GridSpan::Eight,
            Self::TwelveColumn | Self::Shell => GridSpan::Four,
            Self::Split | Self::SplitReverse => GridSpan::One,
            Self::Stack
            | Self::Full
            | Self::Centered
            | Self::CenteredNarrow
            | Self::ThreeColumn
            | Self::FourColumn
            | Self::Bento
            | Self::Overlay
            | Self::Page => GridSpan::Twelve,
        }
    }

    pub const fn support_span(self) -> GridSpan {
        match self {
            Self::SidebarStart => GridSpan::Eight,
            Self::SidebarEnd => GridSpan::Four,
            Self::TwelveColumn | Self::Shell => GridSpan::Eight,
            Self::Split | Self::SplitReverse => GridSpan::One,
            Self::Stack
            | Self::Full
            | Self::Centered
            | Self::CenteredNarrow
            | Self::ThreeColumn
            | Self::FourColumn
            | Self::Bento
            | Self::Overlay
            | Self::Page => GridSpan::Twelve,
        }
    }

    pub const fn reverses_content(self) -> bool {
        matches!(self, Self::SplitReverse | Self::SidebarEnd)
    }
}

impl BlockMediaKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "No media",
            Self::Image => "Image",
            Self::BackgroundImage => "Background image",
            Self::Screenshot => "Screenshot",
            Self::Mockup => "Mockup",
            Self::Product => "Product media",
            Self::Avatar => "Avatar",
            Self::Logo => "Logo",
            Self::Icon => "Icon",
            Self::Video => "Video",
            Self::Code => "Code",
        }
    }

    pub const fn is_visible(self) -> bool {
        !matches!(self, Self::None)
    }

    pub const fn ratio(self) -> MediaRatio {
        match self {
            Self::None | Self::Image | Self::Product => MediaRatio::Landscape,
            Self::BackgroundImage | Self::Screenshot | Self::Video | Self::Code => MediaRatio::Wide,
            Self::Mockup => MediaRatio::Portrait,
            Self::Avatar | Self::Logo | Self::Icon => MediaRatio::Square,
        }
    }
}

impl BlockSurfaceChrome {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Plain => "Plain",
            Self::Subtle => "Subtle",
            Self::Panel => "Panel",
            Self::Bordered => "Bordered",
            Self::Brand => "Brand",
            Self::Dark => "Dark",
        }
    }
}

impl BlockInteraction {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Static => "Static",
            Self::Actions => "Actions",
            Self::Form => "Form",
            Self::Toggle => "Toggle",
            Self::Filter => "Filter",
            Self::Search => "Search",
            Self::Menu => "Menu",
            Self::Overlay => "Overlay",
            Self::Pagination => "Pagination",
            Self::Tabs => "Tabs",
        }
    }
}

impl BlockFamilyDefinition {
    pub fn blocks(self) -> &'static [BlockDefinition] {
        let mut start = 0;
        while start < BLOCKS.len() {
            if BLOCKS[start].family as u8 == self.id as u8 {
                break;
            }
            start += 1;
        }
        &BLOCKS[start..start + self.block_count]
    }
}

impl BlockDefinition {
    pub const fn family_definition(self) -> &'static BlockFamilyDefinition {
        self.family.definition()
    }

    pub fn source_url(self) -> String {
        format!(
            "{}#component-{}",
            self.family.definition().source_url,
            self.source_id
        )
    }

    pub fn resolved_layout(self) -> BlockLayoutPreset {
        if self.layout != BlockLayoutPreset::Page {
            return self.layout;
        }

        let name = self.name.to_ascii_lowercase();
        if name.contains("filters sidebar") {
            BlockLayoutPreset::SidebarStart
        } else if name.contains("sidebar") {
            BlockLayoutPreset::SidebarEnd
        } else if name.contains("split")
            || name.contains("two-column")
            || name.contains("two column")
            || name.contains("order summary")
        {
            BlockLayoutPreset::Split
        } else if name.contains("four tiers") {
            BlockLayoutPreset::FourColumn
        } else if name.contains("three tiers") {
            BlockLayoutPreset::ThreeColumn
        } else if name.contains("stacked") {
            BlockLayoutPreset::Stack
        } else {
            self.layout
        }
    }
}

pub fn block_by_slug(slug: &str) -> Option<&'static BlockDefinition> {
    BLOCKS.iter().find(|definition| definition.slug == slug)
}

fn validate_block_id_value(value: &u16, _context: &()) -> garde::Result {
    if usize::from(*value) < BLOCK_COUNT {
        Ok(())
    } else {
        Err(garde::Error::new(format!(
            "block id must be lower than {BLOCK_COUNT}"
        )))
    }
}

macro_rules! define_block_families {
    ($(
        $variant:ident => {
            surface: $surface:ident,
            area: $area:literal,
            slug: $slug:literal,
            name: $name:literal,
            source: $source:literal,
            count: $count:literal,
            kind: $kind:ident,
            pattern: $pattern:ident,
            primary: $primary:ident
        };
    )+) => {
        #[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, Serialize)]
        #[repr(u8)]
        #[serde(rename_all = "kebab-case")]
        pub enum BlockFamily {
            $($variant,)+
        }

        impl BlockFamily {
            pub const ALL: [Self; BLOCK_FAMILY_COUNT] = [$(Self::$variant,)+];

            pub const fn index(self) -> usize {
                self as usize
            }

            pub const fn definition(self) -> &'static BlockFamilyDefinition {
                &BLOCK_FAMILIES[self.index()]
            }
        }

        pub const BLOCK_FAMILIES: [BlockFamilyDefinition; BLOCK_FAMILY_COUNT] = [$(
            BlockFamilyDefinition {
                id: BlockFamily::$variant,
                surface: BlockSurface::$surface,
                area: $area,
                slug: $slug,
                name: $name,
                source_url: $source,
                block_count: $count,
                source_kind: BlockSourceKind::$kind,
                pattern: BlockPattern::$pattern,
                primary_component: UiComponentId::$primary,
            },
        )+];
    };
}

macro_rules! define_blocks {
    ($(
        $index:literal => {
            family: $family:ident,
            ordinal: $ordinal:literal,
            slug: $slug:literal,
            name: $name:literal,
            source_id: $source_id:literal,
            layout: $layout:ident,
            media: $media:ident,
            chrome: $chrome:ident,
            interaction: $interaction:ident
        };
    )+) => {
        pub const BLOCKS: [BlockDefinition; BLOCK_COUNT] = [$(
            BlockDefinition {
                id: BlockId::new($index),
                family: BlockFamily::$family,
                ordinal: $ordinal,
                slug: $slug,
                name: $name,
                source_id: $source_id,
                layout: BlockLayoutPreset::$layout,
                media: BlockMediaKind::$media,
                chrome: BlockSurfaceChrome::$chrome,
                interaction: BlockInteraction::$interaction,
            },
        )+];
    };
}

include!("catalog_data.rs");

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn catalog_is_complete_and_index_aligned() {
        assert_eq!(BLOCK_FAMILIES.len(), BLOCK_FAMILY_COUNT);
        assert_eq!(BLOCKS.len(), BLOCK_COUNT);
        for (index, definition) in BLOCKS.iter().enumerate() {
            assert_eq!(definition.id.index(), index);
            assert_eq!(
                definition.family_definition().blocks()[definition.ordinal as usize - 1],
                *definition
            );
        }
    }

    #[test]
    fn catalog_slugs_and_source_ids_are_unique() {
        let slugs = BLOCKS
            .iter()
            .map(|definition| definition.slug)
            .collect::<HashSet<_>>();
        let source_ids = BLOCKS
            .iter()
            .map(|definition| definition.source_id)
            .collect::<HashSet<_>>();
        assert_eq!(slugs.len(), BLOCK_COUNT);
        assert_eq!(source_ids.len(), BLOCK_COUNT);
    }

    #[test]
    fn family_counts_cover_every_block_once() {
        assert_eq!(
            BLOCK_FAMILIES
                .iter()
                .map(|definition| definition.block_count)
                .sum::<usize>(),
            BLOCK_COUNT
        );
        assert!(BLOCK_FAMILIES.iter().all(|family| {
            family.blocks().len() == family.block_count
                && family
                    .blocks()
                    .iter()
                    .all(|block| block.family == family.id)
        }));
    }

    #[test]
    fn page_examples_preserve_named_grid_structure() {
        let split =
            block_by_slug("ecommerce-page-examples-checkout-pages-05-split-with-order-summary")
                .expect("catalog should contain the split checkout page");
        let sidebar = block_by_slug("application-ui-page-examples-settings-screens-01-sidebar")
            .expect("catalog should contain the sidebar settings page");

        assert_eq!(split.resolved_layout(), BlockLayoutPreset::Split);
        assert_eq!(sidebar.resolved_layout(), BlockLayoutPreset::SidebarEnd);
    }
}
