use crate::{FrameworkMode, UiComponentCategory, UiComponentId, UiStateModel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImplementationMaturity {
    Implemented,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderContract {
    SharedLeptosBevy,
    LeptosDomOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateContract {
    ConsumerDurable,
    RendererEphemeral,
    Stateless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutContract {
    ActionRow,
    DataSurface,
    DisclosurePanel,
    DisplaySurface,
    FeedbackCallout,
    FormControl,
    LayoutFrame,
    MessagingThread,
    NavigationRegion,
    OverlaySurface,
    TypographyFlow,
    UtilityScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentImplementation {
    pub id: UiComponentId,
    pub maturity: ImplementationMaturity,
    pub render: RenderContract,
    pub state: StateContract,
    pub layout: LayoutContract,
    pub anatomy: &'static [&'static str],
    pub variants: &'static [&'static str],
    pub states: &'static [&'static str],
    pub accessibility: &'static [&'static str],
    pub consumer_contract: &'static str,
    pub end_user_outcome: &'static str,
}

pub fn component_implementation(id: UiComponentId) -> ComponentImplementation {
    let definition = id.definition();
    ComponentImplementation {
        id,
        maturity: ImplementationMaturity::Implemented,
        render: render_contract_for(definition.framework),
        state: state_contract_for(definition.state),
        layout: layout_contract_for(definition.category),
        anatomy: id.anatomy_parts(),
        variants: variants_for_category(definition.category),
        states: states_for_state(definition.state),
        accessibility: accessibility_for_category(definition.category),
        consumer_contract: consumer_contract_for(definition.state),
        end_user_outcome: end_user_outcome_for(id),
    }
}

pub fn implemented_components() -> impl Iterator<Item = ComponentImplementation> + use<> {
    UiComponentId::ALL.into_iter().map(component_implementation)
}

const fn render_contract_for(framework: FrameworkMode) -> RenderContract {
    match framework {
        FrameworkMode::LeptosOnly => RenderContract::LeptosDomOnly,
        FrameworkMode::SharedSpec => RenderContract::SharedLeptosBevy,
    }
}

const fn state_contract_for(state: UiStateModel) -> StateContract {
    match state {
        UiStateModel::ConsumerDurable => StateContract::ConsumerDurable,
        UiStateModel::Ephemeral => StateContract::RendererEphemeral,
        UiStateModel::Stateless => StateContract::Stateless,
    }
}

const fn layout_contract_for(category: UiComponentCategory) -> LayoutContract {
    match category {
        UiComponentCategory::Action => LayoutContract::ActionRow,
        UiComponentCategory::Data => LayoutContract::DataSurface,
        UiComponentCategory::Disclosure => LayoutContract::DisclosurePanel,
        UiComponentCategory::Display => LayoutContract::DisplaySurface,
        UiComponentCategory::Feedback => LayoutContract::FeedbackCallout,
        UiComponentCategory::Form => LayoutContract::FormControl,
        UiComponentCategory::Layout => LayoutContract::LayoutFrame,
        UiComponentCategory::Messaging => LayoutContract::MessagingThread,
        UiComponentCategory::Navigation => LayoutContract::NavigationRegion,
        UiComponentCategory::Overlay => LayoutContract::OverlaySurface,
        UiComponentCategory::Typography => LayoutContract::TypographyFlow,
        UiComponentCategory::Utility => LayoutContract::UtilityScope,
    }
}

const fn variants_for_category(category: UiComponentCategory) -> &'static [&'static str] {
    match category {
        UiComponentCategory::Action => &["primary", "secondary", "quiet", "destructive", "icon"],
        UiComponentCategory::Data => &["default", "dense", "sortable", "loading", "empty"],
        UiComponentCategory::Disclosure => &["single", "multiple", "compact", "disabled"],
        UiComponentCategory::Display => &["default", "compact", "media", "selected"],
        UiComponentCategory::Feedback => &["default", "success", "warning", "danger", "loading"],
        UiComponentCategory::Form => &["default", "filled", "invalid", "disabled", "readonly"],
        UiComponentCategory::Layout => &["default", "horizontal", "vertical", "contained"],
        UiComponentCategory::Messaging => &["incoming", "outgoing", "system", "compact"],
        UiComponentCategory::Navigation => &["default", "active", "collapsed", "overflow"],
        UiComponentCategory::Overlay => &["modal", "nonmodal", "anchored", "danger"],
        UiComponentCategory::Typography => &["prose", "heading", "inline", "code"],
        UiComponentCategory::Utility => &["ltr", "rtl", "nested"],
    }
}

const fn states_for_state(state: UiStateModel) -> &'static [&'static str] {
    match state {
        UiStateModel::ConsumerDurable => &[
            "controlled",
            "pending persistence",
            "hydrated",
            "invalid boundary input",
            "disabled",
        ],
        UiStateModel::Ephemeral => &["closed", "open", "hovered", "focused", "reduced motion"],
        UiStateModel::Stateless => &["default", "loading", "empty", "disabled", "themed"],
    }
}

const fn accessibility_for_category(category: UiComponentCategory) -> &'static [&'static str] {
    match category {
        UiComponentCategory::Action => &[
            "Use semantic buttons or links.",
            "Expose disabled and pressed state to assistive tech.",
            "Keep visible focus from the focus-ring token.",
        ],
        UiComponentCategory::Data => &[
            "Preserve table, grid, or figure semantics.",
            "Announce sorting, filtering, and loading changes.",
            "Keep row and cell labels stable across hydration.",
        ],
        UiComponentCategory::Disclosure => &[
            "Connect trigger and content with aria-controls.",
            "Expose expanded state from renderer-local state.",
            "Keep keyboard activation on Enter and Space.",
        ],
        UiComponentCategory::Display => &[
            "Prefer semantic content order over visual order.",
            "Provide text alternatives for media.",
            "Keep selected and unavailable states perceivable without color alone.",
        ],
        UiComponentCategory::Feedback => &[
            "Use status or alert semantics based on urgency.",
            "Keep recovery actions reachable by keyboard.",
            "Avoid motion-only status communication.",
        ],
        UiComponentCategory::Form => &[
            "Bind labels, descriptions, and errors to controls.",
            "Expose invalid and required state explicitly.",
            "Do not persist field values inside renderer-local state.",
        ],
        UiComponentCategory::Layout => &[
            "Preserve source order when visual layout changes.",
            "Keep handles and scroll affordances keyboard reachable.",
            "Avoid layout shift when content hydrates.",
        ],
        UiComponentCategory::Messaging => &[
            "Keep sender, timestamp, and delivery state available to assistive tech.",
            "Do not trap focus inside live message regions.",
            "Announce new messages only when consumer opts in.",
        ],
        UiComponentCategory::Navigation => &[
            "Use navigation landmarks where appropriate.",
            "Expose current page or active item.",
            "Keep roving focus predictable in composite menus.",
        ],
        UiComponentCategory::Overlay => &[
            "Trap focus only for modal surfaces.",
            "Restore focus to the trigger on close.",
            "Expose title and description for dialog-like content.",
        ],
        UiComponentCategory::Typography => &[
            "Keep heading hierarchy valid.",
            "Use readable line-height tokens.",
            "Do not encode meaning through size alone.",
        ],
        UiComponentCategory::Utility => &[
            "Apply direction or utility scope without changing semantic order.",
            "Keep nested scopes explicit.",
            "Do not hide framework-specific assumptions in utility components.",
        ],
    }
}

const fn consumer_contract_for(state: UiStateModel) -> &'static str {
    match state {
        UiStateModel::ConsumerDurable => {
            "Consumer owns value, validation, and persistence through crates/state and rs-dean-idb; renderer emits typed intents."
        }
        UiStateModel::Ephemeral => {
            "Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful."
        }
        UiStateModel::Stateless => {
            "Consumer passes display data; renderer is a pure projection of props, theme, and tokens."
        }
    }
}

pub const fn implementation_issue_marker() -> &'static str {
    "Implementation Status"
}

impl ImplementationMaturity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Implemented => "Implemented",
        }
    }
}

impl RenderContract {
    pub const fn label(self) -> &'static str {
        match self {
            Self::SharedLeptosBevy => "Shared Leptos/Bevy",
            Self::LeptosDomOnly => "Leptos DOM only",
        }
    }
}

impl StateContract {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ConsumerDurable => "Consumer durable",
            Self::RendererEphemeral => "Renderer ephemeral",
            Self::Stateless => "Stateless",
        }
    }
}

impl LayoutContract {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ActionRow => "Action row",
            Self::DataSurface => "Data surface",
            Self::DisclosurePanel => "Disclosure panel",
            Self::DisplaySurface => "Display surface",
            Self::FeedbackCallout => "Feedback callout",
            Self::FormControl => "Form control",
            Self::LayoutFrame => "Layout frame",
            Self::MessagingThread => "Messaging thread",
            Self::NavigationRegion => "Navigation region",
            Self::OverlaySurface => "Overlay surface",
            Self::TypographyFlow => "Typography flow",
            Self::UtilityScope => "Utility scope",
        }
    }
}

fn end_user_outcome_for(id: UiComponentId) -> &'static str {
    match id {
        UiComponentId::Accordion => {
            "Users can scan headings and reveal the exact section they need without leaving context."
        }
        UiComponentId::Alert => {
            "Users immediately understand the status, risk, and available recovery action."
        }
        UiComponentId::AlertDialog => {
            "Users must explicitly confirm or cancel a high-risk action before it mutates state."
        }
        UiComponentId::AspectRatio => {
            "Users see media and canvas content in a stable frame before and after hydration."
        }
        UiComponentId::Attachment => {
            "Users can inspect attached files or resources with clear name, type, size, and action affordances."
        }
        UiComponentId::Avatar => {
            "Users can identify people or entities quickly even when images fail to load."
        }
        UiComponentId::Badge => {
            "Users can read compact status and categorization without interrupting the surrounding flow."
        }
        UiComponentId::Breadcrumb => {
            "Users can understand location and move to an ancestor level predictably."
        }
        UiComponentId::Bubble => {
            "Users can distinguish sender, message body, and delivery context in a conversation."
        }
        UiComponentId::Button => {
            "Users can trigger a clear command with strong focus, disabled, loading, and destructive states."
        }
        UiComponentId::ButtonGroup => {
            "Users can choose between adjacent related actions without losing group context."
        }
        UiComponentId::Calendar => {
            "Users can move through dates and select valid dates or ranges with durable ownership by the consumer."
        }
        UiComponentId::Card => {
            "Users can scan a grouped content surface with predictable header, content, and action zones."
        }
        UiComponentId::Carousel => {
            "Users can browse paged content while position and navigation remain understandable."
        }
        UiComponentId::Chart => {
            "Users can interpret themed data series, legends, and tooltips without bespoke color assumptions."
        }
        UiComponentId::Checkbox => {
            "Users can toggle true, false, or mixed state with label and description tied to the control."
        }
        UiComponentId::Collapsible => {
            "Users can show or hide a single region while preserving focus and layout stability."
        }
        UiComponentId::Combobox => {
            "Users can search, filter, and commit an option without losing typed input context."
        }
        UiComponentId::Command => {
            "Users can search available commands and execute a selected action with keyboard-first flow."
        }
        UiComponentId::ContextMenu => {
            "Users can invoke target-specific actions without shifting global navigation state."
        }
        UiComponentId::DataTable => {
            "Users can sort, filter, page, and inspect rows while durable table state stays consumer-owned."
        }
        UiComponentId::DatePicker => {
            "Users can open a calendar from a field, pick a date, and return the chosen value to the consumer."
        }
        UiComponentId::Dialog => "Users can complete a focused workflow in an accessible overlay.",
        UiComponentId::Direction => {
            "Users can render nested left-to-right or right-to-left scopes without duplicating components."
        }
        UiComponentId::Drawer => {
            "Users can access a mobile-friendly task panel without losing the underlying page context."
        }
        UiComponentId::DropdownMenu => {
            "Users can choose a command from a trigger-attached menu with predictable focus movement."
        }
        UiComponentId::Empty => {
            "Users can understand why content is absent and what action can recover the flow."
        }
        UiComponentId::Field => {
            "Users can read label, hint, control, and validation message as one accessible unit."
        }
        UiComponentId::HoverCard => {
            "Users can preview rich supporting information without committing navigation."
        }
        UiComponentId::Input => {
            "Users can enter a single text value with label, status, and validation handled by the consumer."
        }
        UiComponentId::InputGroup => {
            "Users can complete text entry with addons and inline actions in one stable control row."
        }
        UiComponentId::InputOtp => {
            "Users can enter fixed-length codes with clear slot grouping and paste behavior."
        }
        UiComponentId::Item => {
            "Users can scan a reusable content row with media, description, and actions."
        }
        UiComponentId::Kbd => "Users can recognize keyboard shortcuts and key chords inline.",
        UiComponentId::Label => {
            "Users can associate control names and requirement state with the correct field."
        }
        UiComponentId::Marker => {
            "Users can notice unread, positional, or annotation state without disrupting layout."
        }
        UiComponentId::Menubar => {
            "Users can navigate application commands through a horizontal menu system."
        }
        UiComponentId::Message => {
            "Users can read durable message content with sender, metadata, and actions preserved."
        }
        UiComponentId::MessageScroller => {
            "Users can review message history and jump to the latest item when appropriate."
        }
        UiComponentId::NativeSelect => {
            "Users can choose from platform-native options while retaining themed shell styling."
        }
        UiComponentId::NavigationMenu => {
            "Users can move across top-level sections and reveal grouped navigation content."
        }
        UiComponentId::Pagination => {
            "Users can move through result pages with current page and boundary states exposed."
        }
        UiComponentId::Popover => {
            "Users can access lightweight anchored content and dismiss it without modal overhead."
        }
        UiComponentId::Progress => {
            "Users can track task completion or indeterminate work without guessing status."
        }
        UiComponentId::RadioGroup => "Users can select exactly one option from a labeled group.",
        UiComponentId::Resizable => {
            "Users can adjust panel proportions while the consumer decides whether to persist layout."
        }
        UiComponentId::ScrollArea => {
            "Users can scroll contained content with visible overflow affordances."
        }
        UiComponentId::Select => {
            "Users can pick one option from a custom trigger/listbox flow with durable value ownership."
        }
        UiComponentId::Separator => {
            "Users can distinguish related regions without mistaking decoration for content."
        }
        UiComponentId::Sheet => "Users can complete secondary workflows in an edge-attached panel.",
        UiComponentId::Sidebar => {
            "Users can navigate persistent app structure and collapse state without losing context."
        }
        UiComponentId::Skeleton => "Users see the expected layout shape while content loads.",
        UiComponentId::Slider => {
            "Users can select numeric values with clear range, thumb, and disabled states."
        }
        UiComponentId::Sonner => {
            "Users receive a stack of transient notifications with consistent action and dismiss behavior."
        }
        UiComponentId::Spinner => "Users see compact activity feedback when a region is busy.",
        UiComponentId::Switch => {
            "Users can toggle a binary setting and understand whether it is on or off."
        }
        UiComponentId::Table => "Users can scan structured rows and columns with semantic headers.",
        UiComponentId::Tabs => "Users can switch related panels without navigating away.",
        UiComponentId::Textarea => {
            "Users can enter multi-line text with durable value and validation ownership by the consumer."
        }
        UiComponentId::Toast => {
            "Users receive transient feedback with optional action and non-blocking dismissal."
        }
        UiComponentId::Toggle => {
            "Users can press or unpress a single option and perceive the current state."
        }
        UiComponentId::ToggleGroup => {
            "Users can switch one or more related toggles while the group state remains coherent."
        }
        UiComponentId::Tooltip => {
            "Users can discover a concise description for a focused or hovered control."
        }
        UiComponentId::Typography => {
            "Users can read headings, paragraphs, lists, quotes, and code with consistent hierarchy."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SHADCN_COMPONENT_COUNT, UiComponentId, UiStateModel};
    use std::{fs, path::Path};

    #[test]
    fn every_catalog_component_has_an_implementation_recipe() {
        let implementations: Vec<_> = implemented_components().collect();
        assert_eq!(implementations.len(), SHADCN_COMPONENT_COUNT);
        for implementation in implementations {
            assert_eq!(implementation.maturity, ImplementationMaturity::Implemented);
            assert_eq!(implementation.anatomy, implementation.id.anatomy_parts());
            assert!(!implementation.variants.is_empty());
            assert!(!implementation.states.is_empty());
            assert!(!implementation.accessibility.is_empty());
            assert!(!implementation.consumer_contract.is_empty());
            assert!(!implementation.end_user_outcome.is_empty());
        }
    }

    #[test]
    fn implementation_state_contract_matches_catalog_state() {
        for id in UiComponentId::ALL {
            let implementation = component_implementation(id);
            let expected = match id.definition().state {
                UiStateModel::ConsumerDurable => StateContract::ConsumerDurable,
                UiStateModel::Ephemeral => StateContract::RendererEphemeral,
                UiStateModel::Stateless => StateContract::Stateless,
            };
            assert_eq!(implementation.state, expected, "{}", id.definition().name);
        }
    }

    #[test]
    fn issue_files_are_marked_implemented() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../_issues");
        for id in UiComponentId::ALL {
            let path = root.join(id.issue_filename());
            let contents = fs::read_to_string(&path).expect("issue file should be readable");
            assert!(
                contents.contains(implementation_issue_marker()),
                "{} missing implementation marker",
                path.display()
            );
            assert!(
                contents.contains("[x] Shared Rust implementation recipe"),
                "{} missing shared implementation checklist",
                path.display()
            );
        }
    }
}
