use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpaceToken {
    None,
    ThreeExtraSmall,
    TwoExtraSmall,
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    TwoExtraLarge,
    ThreeExtraLarge,
    FourExtraLarge,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContainerWidth {
    Full,
    Narrow,
    Prose,
    #[default]
    Content,
    Wide,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SectionSurface {
    Transparent,
    #[default]
    Surface,
    Subtle,
    Elevated,
    Sunken,
    Brand,
    Accent,
    Contrast,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GridPreset {
    #[default]
    Stack,
    Split,
    SidebarStart,
    SidebarEnd,
    ThreeUp,
    FourUp,
    Twelve,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GridAlign {
    Start,
    Center,
    End,
    #[default]
    Stretch,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GridJustify {
    #[default]
    Start,
    Center,
    End,
    Between,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GridSpan {
    One,
    Two,
    Three,
    Four,
    Five,
    #[default]
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextStyle {
    Eyebrow,
    Caption,
    Small,
    #[default]
    Body,
    Lead,
    Heading,
    Title,
    Display,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextTone {
    #[default]
    Primary,
    Secondary,
    Muted,
    Brand,
    OnBrand,
    NeutralContent,
    Danger,
    Success,
    Warning,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextAlign {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum HeadingLevel {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TextElement {
    #[default]
    Paragraph,
    Span,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MediaRatio {
    Square,
    Portrait,
    #[default]
    Landscape,
    Wide,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct SectionSpec {
    #[garde(skip)]
    pub surface: SectionSurface,
    #[garde(skip)]
    pub padding_block: SpaceToken,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ContainerSpec {
    #[garde(skip)]
    pub width: ContainerWidth,
    #[garde(skip)]
    pub gutter: SpaceToken,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct GridSpec {
    #[garde(skip)]
    pub preset: GridPreset,
    #[garde(skip)]
    pub gap: SpaceToken,
    #[garde(skip)]
    pub align: GridAlign,
    #[garde(skip)]
    pub justify: GridJustify,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct GridItemSpec {
    #[garde(skip)]
    pub span: GridSpan,
    #[garde(skip)]
    pub align: GridAlign,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct StackSpec {
    #[garde(skip)]
    pub gap: SpaceToken,
    #[garde(skip)]
    pub align: GridAlign,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct ClusterSpec {
    #[garde(skip)]
    pub gap: SpaceToken,
    #[garde(skip)]
    pub align: GridAlign,
    #[garde(skip)]
    pub justify: GridJustify,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutRect {
    pub center_x: f32,
    pub center_y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for SectionSpec {
    fn default() -> Self {
        Self {
            surface: SectionSurface::Surface,
            padding_block: SpaceToken::ExtraLarge,
        }
    }
}

impl Default for ContainerSpec {
    fn default() -> Self {
        Self {
            width: ContainerWidth::Content,
            gutter: SpaceToken::Small,
        }
    }
}

impl Default for GridSpec {
    fn default() -> Self {
        Self {
            preset: GridPreset::Stack,
            gap: SpaceToken::Medium,
            align: GridAlign::Stretch,
            justify: GridJustify::Start,
        }
    }
}

impl Default for GridItemSpec {
    fn default() -> Self {
        Self {
            span: GridSpan::Six,
            align: GridAlign::Stretch,
        }
    }
}

impl Default for StackSpec {
    fn default() -> Self {
        Self {
            gap: SpaceToken::Small,
            align: GridAlign::Stretch,
        }
    }
}

impl Default for ClusterSpec {
    fn default() -> Self {
        Self {
            gap: SpaceToken::ExtraSmall,
            align: GridAlign::Center,
            justify: GridJustify::Start,
        }
    }
}

impl SpaceToken {
    pub const fn points(self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::ThreeExtraSmall => crate::scale::space::XS3,
            Self::TwoExtraSmall => crate::scale::space::XS2,
            Self::ExtraSmall => crate::scale::space::XS,
            Self::Small => crate::scale::space::S,
            Self::Medium => crate::scale::space::M,
            Self::Large => crate::scale::space::L,
            Self::ExtraLarge => crate::scale::space::XL,
            Self::TwoExtraLarge => crate::scale::space::XL2,
            Self::ThreeExtraLarge => crate::scale::space::XL3,
            Self::FourExtraLarge => crate::scale::space::XL4,
        }
    }

    pub const fn gap_class(self) -> &'static str {
        match self {
            Self::None => "gap-0",
            Self::ThreeExtraSmall => "gap-3xs",
            Self::TwoExtraSmall => "gap-2xs",
            Self::ExtraSmall => "gap-xs",
            Self::Small => "gap-s",
            Self::Medium => "gap-m",
            Self::Large => "gap-l",
            Self::ExtraLarge => "gap-xl",
            Self::TwoExtraLarge => "gap-2xl",
            Self::ThreeExtraLarge => "gap-3xl",
            Self::FourExtraLarge => "gap-4xl",
        }
    }

    pub const fn padding_inline_class(self) -> &'static str {
        match self {
            Self::None => "px-0",
            Self::ThreeExtraSmall => "px-3xs",
            Self::TwoExtraSmall => "px-2xs",
            Self::ExtraSmall => "px-xs",
            Self::Small => "px-s",
            Self::Medium => "px-m",
            Self::Large => "px-l",
            Self::ExtraLarge => "px-xl",
            Self::TwoExtraLarge => "px-2xl",
            Self::ThreeExtraLarge => "px-3xl",
            Self::FourExtraLarge => "px-4xl",
        }
    }

    pub const fn padding_block_class(self) -> &'static str {
        match self {
            Self::None => "py-0",
            Self::ThreeExtraSmall => "py-3xs",
            Self::TwoExtraSmall => "py-2xs",
            Self::ExtraSmall => "py-xs",
            Self::Small => "py-s",
            Self::Medium => "py-m",
            Self::Large => "py-l",
            Self::ExtraLarge => "py-xl",
            Self::TwoExtraLarge => "py-2xl",
            Self::ThreeExtraLarge => "py-3xl",
            Self::FourExtraLarge => "py-4xl",
        }
    }
}

impl ContainerWidth {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Full => "w-full",
            Self::Narrow => "w-full max-w-narrow",
            Self::Prose => "w-full max-w-prose",
            Self::Content => "w-full max-w-content",
            Self::Wide => "w-full max-w-wide",
        }
    }

    pub const fn points(self) -> f32 {
        match self {
            Self::Full => f32::MAX,
            Self::Narrow => crate::scale::container::NARROW,
            Self::Prose => crate::scale::container::PROSE,
            Self::Content => crate::scale::container::CONTENT,
            Self::Wide => crate::scale::container::WIDE,
        }
    }
}

impl SectionSurface {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Transparent => "bg-transparent text-text-1",
            Self::Surface => "bg-surface-1 text-text-1",
            Self::Subtle => "bg-surface-2 text-text-1",
            Self::Elevated => "bg-surface-elevated text-text-1",
            Self::Sunken => "bg-surface-sunken text-text-1",
            Self::Brand => "bg-brand text-text-on-brand",
            Self::Accent => "bg-accent-soft text-text-1",
            Self::Contrast => "bg-neutral text-neutral-content",
        }
    }
}

impl GridPreset {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Stack => "grid-cols-1",
            Self::Split => "grid-cols-1 lg:grid-cols-2",
            Self::SidebarStart | Self::SidebarEnd | Self::Twelve => "grid-cols-1 lg:grid-cols-12",
            Self::ThreeUp => "grid-cols-1 md:grid-cols-3",
            Self::FourUp => "grid-cols-1 sm:grid-cols-2 lg:grid-cols-4",
        }
    }

    pub const fn columns(self, viewport_width: f32) -> u8 {
        match self {
            Self::Stack => 1,
            Self::Split if viewport_width >= 1024.0 => 2,
            Self::SidebarStart | Self::SidebarEnd | Self::Twelve if viewport_width >= 1024.0 => 12,
            Self::ThreeUp if viewport_width >= 768.0 => 3,
            Self::FourUp if viewport_width >= 1024.0 => 4,
            Self::FourUp if viewport_width >= 640.0 => 2,
            Self::Split
            | Self::SidebarStart
            | Self::SidebarEnd
            | Self::ThreeUp
            | Self::FourUp
            | Self::Twelve => 1,
        }
    }
}

impl GridAlign {
    pub const fn items_class(self) -> &'static str {
        match self {
            Self::Start => "items-start",
            Self::Center => "items-center",
            Self::End => "items-end",
            Self::Stretch => "items-stretch",
        }
    }

    pub const fn self_class(self) -> &'static str {
        match self {
            Self::Start => "self-start",
            Self::Center => "self-center",
            Self::End => "self-end",
            Self::Stretch => "self-stretch",
        }
    }
}

impl GridJustify {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Start => "justify-start",
            Self::Center => "justify-center",
            Self::End => "justify-end",
            Self::Between => "justify-between",
        }
    }
}

impl GridSpan {
    pub const fn columns(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Eleven => 11,
            Self::Twelve => 12,
        }
    }

    pub const fn class(self) -> &'static str {
        match self {
            Self::One => "col-span-1 lg:col-span-1",
            Self::Two => "col-span-1 lg:col-span-2",
            Self::Three => "col-span-1 lg:col-span-3",
            Self::Four => "col-span-1 lg:col-span-4",
            Self::Five => "col-span-1 lg:col-span-5",
            Self::Six => "col-span-1 lg:col-span-6",
            Self::Seven => "col-span-1 lg:col-span-7",
            Self::Eight => "col-span-1 lg:col-span-8",
            Self::Nine => "col-span-1 lg:col-span-9",
            Self::Ten => "col-span-1 lg:col-span-10",
            Self::Eleven => "col-span-1 lg:col-span-11",
            Self::Twelve => "col-span-1 lg:col-span-12",
        }
    }
}

impl SectionSpec {
    pub fn class(self) -> String {
        format!(
            "w-full {} {}",
            self.surface.class(),
            self.padding_block.padding_block_class()
        )
    }
}

impl ContainerSpec {
    pub fn class(self) -> String {
        format!(
            "mx-auto {} {}",
            self.width.class(),
            self.gutter.padding_inline_class()
        )
    }
}

impl GridSpec {
    pub fn class(self) -> String {
        format!(
            "grid {} {} {} {}",
            self.preset.class(),
            self.gap.gap_class(),
            self.align.items_class(),
            self.justify.class()
        )
    }

    pub fn equal_track_cells(
        self,
        bounds: LayoutRect,
        viewport_width: f32,
        item_count: usize,
    ) -> Vec<LayoutRect> {
        if item_count == 0 {
            return Vec::new();
        }

        let columns = usize::from(self.preset.columns(viewport_width))
            .min(item_count)
            .max(1);
        let rows = item_count.div_ceil(columns);
        let gap = self.gap.points();
        let cell_width =
            ((bounds.width - gap * (columns.saturating_sub(1) as f32)) / columns as f32).max(1.0);
        let cell_height =
            ((bounds.height - gap * (rows.saturating_sub(1) as f32)) / rows as f32).max(1.0);
        let left = bounds.center_x - bounds.width / 2.0;
        let top = bounds.center_y + bounds.height / 2.0;

        (0..item_count)
            .map(|index| {
                let column = index % columns;
                let row = index / columns;
                LayoutRect {
                    center_x: left + cell_width / 2.0 + column as f32 * (cell_width + gap),
                    center_y: top - cell_height / 2.0 - row as f32 * (cell_height + gap),
                    width: cell_width,
                    height: cell_height,
                }
            })
            .collect()
    }
}

impl GridItemSpec {
    pub fn class(self) -> String {
        format!("min-w-0 {} {}", self.span.class(), self.align.self_class())
    }
}

impl StackSpec {
    pub fn class(self) -> String {
        format!("grid {} {}", self.gap.gap_class(), self.align.items_class())
    }
}

impl ClusterSpec {
    pub fn class(self) -> String {
        format!(
            "flex flex-wrap {} {} {}",
            self.gap.gap_class(),
            self.align.items_class(),
            self.justify.class()
        )
    }
}

impl TextStyle {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Eyebrow => "text-00 font-7 leading-00 uppercase tracking-label",
            Self::Caption => "text-00 font-5 leading-00",
            Self::Small => "text-0 font-4 leading-0",
            Self::Body => "text-0 font-4 leading-0",
            Self::Lead => "text-1 font-4 leading-1",
            Self::Heading => "text-2 font-7 leading-2",
            Self::Title => "text-4 font-7 leading-4",
            Self::Display => "text-6 font-8 leading-6",
        }
    }
}

impl TextTone {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Primary => "text-text-1",
            Self::Secondary => "text-text-2",
            Self::Muted => "text-text-muted",
            Self::Brand => "text-brand",
            Self::OnBrand => "text-text-on-brand",
            Self::NeutralContent => "text-neutral-content",
            Self::Danger => "text-danger",
            Self::Success => "text-success",
            Self::Warning => "text-warning",
        }
    }
}

impl TextAlign {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Start => "text-left",
            Self::Center => "text-center",
            Self::End => "text-right",
        }
    }
}

impl MediaRatio {
    pub const fn class(self) -> &'static str {
        match self {
            Self::Square => "aspect-square",
            Self::Portrait => "aspect-3/4",
            Self::Landscape => "aspect-4/3",
            Self::Wide => "aspect-video",
        }
    }
}

#[cfg(feature = "leptos")]
mod leptos_components {
    use leptos::prelude::*;

    use super::{
        ClusterSpec, ContainerSpec, GridItemSpec, GridSpec, HeadingLevel, MediaRatio, SectionSpec,
        StackSpec, TextAlign, TextElement, TextStyle, TextTone,
    };

    #[component]
    pub fn Section(
        #[prop(optional, default = SectionSpec::default())] spec: SectionSpec,
        children: Children,
    ) -> impl IntoView {
        view! { <section class=spec.class() data-ui-layout="section">{children()}</section> }
    }

    #[component]
    pub fn Container(
        #[prop(optional, default = ContainerSpec::default())] spec: ContainerSpec,
        children: Children,
    ) -> impl IntoView {
        view! { <div class=spec.class() data-ui-layout="container">{children()}</div> }
    }

    #[component]
    pub fn Grid(
        #[prop(optional, default = GridSpec::default())] spec: GridSpec,
        children: Children,
    ) -> impl IntoView {
        view! { <div class=spec.class() data-ui-layout="grid">{children()}</div> }
    }

    #[component]
    pub fn GridItem(
        #[prop(optional, default = GridItemSpec::default())] spec: GridItemSpec,
        children: Children,
    ) -> impl IntoView {
        view! { <div class=spec.class() data-ui-layout="grid-item">{children()}</div> }
    }

    #[component]
    pub fn Stack(
        #[prop(optional, default = StackSpec::default())] spec: StackSpec,
        children: Children,
    ) -> impl IntoView {
        view! { <div class=spec.class() data-ui-layout="stack">{children()}</div> }
    }

    #[component]
    pub fn Cluster(
        #[prop(optional, default = ClusterSpec::default())] spec: ClusterSpec,
        children: Children,
    ) -> impl IntoView {
        view! { <div class=spec.class() data-ui-layout="cluster">{children()}</div> }
    }

    #[component]
    pub fn UiHeading(
        text: String,
        #[prop(optional, default = HeadingLevel::One)] level: HeadingLevel,
        #[prop(optional, default = TextStyle::Title)] style: TextStyle,
        #[prop(optional, default = TextTone::Primary)] tone: TextTone,
        #[prop(optional, default = TextAlign::Start)] align: TextAlign,
    ) -> AnyView {
        let class = format!("m-0 {} {} {}", style.class(), tone.class(), align.class());
        match level {
            HeadingLevel::One => view! { <h1 class=class>{text}</h1> }.into_any(),
            HeadingLevel::Two => view! { <h2 class=class>{text}</h2> }.into_any(),
            HeadingLevel::Three => view! { <h3 class=class>{text}</h3> }.into_any(),
            HeadingLevel::Four => view! { <h4 class=class>{text}</h4> }.into_any(),
            HeadingLevel::Five => view! { <h5 class=class>{text}</h5> }.into_any(),
            HeadingLevel::Six => view! { <h6 class=class>{text}</h6> }.into_any(),
        }
    }

    #[component]
    pub fn UiText(
        text: String,
        #[prop(optional, default = TextElement::Paragraph)] element: TextElement,
        #[prop(optional, default = TextStyle::Body)] style: TextStyle,
        #[prop(optional, default = TextTone::Secondary)] tone: TextTone,
        #[prop(optional, default = TextAlign::Start)] align: TextAlign,
    ) -> AnyView {
        let class = format!("m-0 {} {} {}", style.class(), tone.class(), align.class());
        match element {
            TextElement::Paragraph => view! { <p class=class>{text}</p> }.into_any(),
            TextElement::Span => view! { <span class=class>{text}</span> }.into_any(),
        }
    }

    #[component]
    pub fn UiMediaFrame(
        label: String,
        #[prop(optional, default = MediaRatio::Landscape)] ratio: MediaRatio,
    ) -> impl IntoView {
        let class = format!(
            "grid w-full place-items-center overflow-hidden rounded-box border border-border-subtle bg-surface-2 p-s text-center shadow-1 {}",
            ratio.class()
        );
        view! {
            <figure class=class data-ui-layout="media">
                <figcaption class="m-0 text-0 font-6 leading-0 text-text-muted">{label}</figcaption>
            </figure>
        }
    }
}

#[cfg(feature = "leptos")]
pub use leptos_components::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_presets_collapse_before_their_breakpoint() {
        assert_eq!(GridPreset::Split.columns(640.0), 1);
        assert_eq!(GridPreset::Split.columns(1_024.0), 2);
        assert_eq!(GridPreset::FourUp.columns(768.0), 2);
        assert_eq!(GridPreset::FourUp.columns(1_024.0), 4);
    }

    #[test]
    fn layout_classes_use_the_shared_token_vocabulary() {
        let section = SectionSpec::default().class();
        let container = ContainerSpec::default().class();
        let grid = GridSpec::default().class();

        assert!(section.contains("py-xl"));
        assert!(container.contains("max-w-content"));
        assert!(container.contains("px-s"));
        assert!(grid.contains("gap-m"));
    }

    #[test]
    fn all_grid_spans_are_bounded_by_the_twelve_column_contract() {
        for span in [
            GridSpan::One,
            GridSpan::Two,
            GridSpan::Three,
            GridSpan::Four,
            GridSpan::Five,
            GridSpan::Six,
            GridSpan::Seven,
            GridSpan::Eight,
            GridSpan::Nine,
            GridSpan::Ten,
            GridSpan::Eleven,
            GridSpan::Twelve,
        ] {
            assert!((1..=12).contains(&span.columns()));
        }
    }

    #[test]
    fn equal_track_geometry_matches_responsive_css_track_counts() {
        let bounds = LayoutRect {
            center_x: 0.0,
            center_y: 0.0,
            width: 1_200.0,
            height: 320.0,
        };
        let grid = GridSpec {
            preset: GridPreset::ThreeUp,
            gap: SpaceToken::Small,
            ..GridSpec::default()
        };

        let desktop = grid.equal_track_cells(bounds, 1_200.0, 3);
        let mobile = grid.equal_track_cells(bounds, 640.0, 3);

        assert_eq!(desktop.len(), 3);
        assert_eq!(desktop[0].center_y, desktop[2].center_y);
        assert!(desktop[0].center_x < desktop[1].center_x);
        assert!(mobile[0].center_y > mobile[1].center_y);
        assert_eq!(mobile[0].center_x, mobile[2].center_x);
    }
}
