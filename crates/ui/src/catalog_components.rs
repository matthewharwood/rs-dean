use core::marker::PhantomData;

use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{
    UiBlockRole, UiBlockTone, UiComponentId, UiWidgetIntent, UiWidgetPattern, UiWidgetSlot,
    UiWidgetSlotKind, widget_for_component,
};

pub trait CatalogComponentPart: Copy + Eq + 'static {
    const ID: UiComponentId;
    const ALL: &'static [Self];

    fn label(self) -> &'static str;
    fn from_label(label: &str) -> Option<Self>;
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CatalogComponentSlot {
    #[garde(length(min = 1, max = 128))]
    pub part: String,
    #[garde(skip)]
    pub kind: UiWidgetSlotKind,
    #[garde(skip)]
    pub role: UiBlockRole,
    #[garde(skip)]
    pub tone: UiBlockTone,
    #[garde(length(min = 1, max = 160))]
    pub label: String,
    #[garde(length(min = 1, max = 2_000))]
    pub value: String,
    #[garde(length(min = 1, max = 240))]
    pub detail: String,
    #[garde(skip)]
    pub intent: UiWidgetIntent,
    #[garde(skip)]
    pub selected: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Validate)]
pub struct CatalogComponentModel<P: CatalogComponentPart> {
    #[serde(skip)]
    #[garde(skip)]
    part: PhantomData<P>,
    #[garde(skip)]
    pub pattern: UiWidgetPattern,
    #[garde(length(min = 1), dive, custom(component_slots_match_part::<P>))]
    pub slots: Vec<CatalogComponentSlot>,
    #[garde(skip)]
    pub loading: bool,
    #[garde(skip)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize)]
pub struct CatalogComponentState<P: CatalogComponentPart> {
    #[serde(skip)]
    part: PhantomData<P>,
    active_parts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogComponentIntent<P: CatalogComponentPart> {
    Activate { part: P, value: String },
    Close(P),
    Dismiss(P),
    Input { part: P, value: String },
    Navigate { part: P, value: String },
    Open(P),
    Resize(P),
    Scroll(P),
    Select(P),
    Toggle(P),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogComponentChange<P: CatalogComponentPart> {
    Activated { part: P, value: String },
    Closed(P),
    Dismissed(P),
    Input { part: P, value: String },
    Navigated { part: P, value: String },
    Opened(P),
    Resized(P),
    Scrolled(P),
    Selected(P),
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogComponentRenderNode<P: CatalogComponentPart> {
    pub part: P,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub kind: UiWidgetSlotKind,
    pub role: UiBlockRole,
    pub tone: UiBlockTone,
    pub intent: UiWidgetIntent,
    pub selected: bool,
    pub loading: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogAnyRenderNode {
    pub part: String,
    pub value: String,
    pub label: String,
    pub detail: String,
    pub kind: UiWidgetSlotKind,
    pub role: UiBlockRole,
    pub tone: UiBlockTone,
    pub intent: UiWidgetIntent,
    pub selected: bool,
    pub loading: bool,
    pub disabled: bool,
}

impl CatalogComponentSlot {
    pub fn from_widget_slot(slot: &UiWidgetSlot) -> Self {
        Self {
            part: slot.part.to_owned(),
            kind: slot.kind,
            role: slot.role,
            tone: slot.tone,
            label: slot.label.to_owned(),
            value: slot.value.to_owned(),
            detail: slot.detail.to_owned(),
            intent: slot.intent,
            selected: slot.selected,
            disabled: slot.disabled,
        }
    }

    pub const fn selected(mut self) -> Self {
        self.selected = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl<P: CatalogComponentPart> CatalogComponentModel<P> {
    pub fn new(pattern: UiWidgetPattern, slots: Vec<CatalogComponentSlot>) -> Self {
        Self {
            part: PhantomData,
            pattern,
            slots,
            loading: false,
            disabled: false,
        }
    }

    pub fn from_component_id(id: UiComponentId) -> Self {
        assert_eq!(
            id,
            P::ID,
            "invariant: concrete component model id must match its part enum"
        );
        let widget = widget_for_component(id);
        Self::from_widget_slots(widget.pattern, &widget.slots)
    }

    pub fn from_widget_slots(pattern: UiWidgetPattern, slots: &[UiWidgetSlot]) -> Self {
        Self::new(
            pattern,
            slots
                .iter()
                .map(CatalogComponentSlot::from_widget_slot)
                .collect(),
        )
    }

    pub const fn loading(mut self) -> Self {
        self.loading = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn state(&self) -> CatalogComponentState<P> {
        CatalogComponentState::from_model(self)
    }
}

impl<P: CatalogComponentPart> Default for CatalogComponentModel<P> {
    fn default() -> Self {
        default_catalog_component_model::<P>()
    }
}

impl<P: CatalogComponentPart> CatalogComponentState<P> {
    pub fn new(active_parts: Vec<String>) -> Self {
        let mut normalized = Vec::new();
        for part in active_parts {
            if part.is_empty() || normalized.contains(&part) || P::from_label(&part).is_none() {
                continue;
            }
            normalized.push(part);
        }
        Self {
            part: PhantomData,
            active_parts: normalized,
        }
    }

    pub fn from_model(model: &CatalogComponentModel<P>) -> Self {
        Self::new(
            model
                .slots
                .iter()
                .filter(|slot| slot.selected)
                .map(|slot| slot.part.clone())
                .collect(),
        )
    }

    pub fn active_parts(&self) -> &[String] {
        &self.active_parts
    }

    pub fn is_active(&self, part: P) -> bool {
        self.active_parts
            .iter()
            .any(|active| active == part.label())
    }

    pub fn apply(&mut self, intent: CatalogComponentIntent<P>) -> CatalogComponentChange<P> {
        match intent {
            CatalogComponentIntent::Activate { part, value } => {
                CatalogComponentChange::Activated { part, value }
            }
            CatalogComponentIntent::Close(part) => {
                self.deactivate(part, CatalogComponentChange::Closed(part))
            }
            CatalogComponentIntent::Dismiss(part) => {
                self.deactivate(part, CatalogComponentChange::Dismissed(part))
            }
            CatalogComponentIntent::Input { part, value } => {
                CatalogComponentChange::Input { part, value }
            }
            CatalogComponentIntent::Navigate { part, value } => {
                CatalogComponentChange::Navigated { part, value }
            }
            CatalogComponentIntent::Open(part) => {
                self.activate(part, CatalogComponentChange::Opened(part))
            }
            CatalogComponentIntent::Resize(part) => CatalogComponentChange::Resized(part),
            CatalogComponentIntent::Scroll(part) => CatalogComponentChange::Scrolled(part),
            CatalogComponentIntent::Select(part) => {
                self.activate(part, CatalogComponentChange::Selected(part))
            }
            CatalogComponentIntent::Toggle(part) => {
                if self.is_active(part) {
                    self.deactivate(part, CatalogComponentChange::Closed(part))
                } else {
                    self.activate(part, CatalogComponentChange::Opened(part))
                }
            }
        }
    }

    fn activate(
        &mut self,
        part: P,
        change: CatalogComponentChange<P>,
    ) -> CatalogComponentChange<P> {
        if self.is_active(part) {
            CatalogComponentChange::Unchanged
        } else {
            self.active_parts.push(part.label().to_owned());
            change
        }
    }

    fn deactivate(
        &mut self,
        part: P,
        change: CatalogComponentChange<P>,
    ) -> CatalogComponentChange<P> {
        let Some(index) = self
            .active_parts
            .iter()
            .position(|active| active == part.label())
        else {
            return CatalogComponentChange::Unchanged;
        };
        self.active_parts.remove(index);
        change
    }
}

impl<P: CatalogComponentPart> From<CatalogComponentRenderNode<P>> for CatalogAnyRenderNode {
    fn from(node: CatalogComponentRenderNode<P>) -> Self {
        Self {
            part: node.part.label().to_owned(),
            value: node.value,
            label: node.label,
            detail: node.detail,
            kind: node.kind,
            role: node.role,
            tone: node.tone,
            intent: node.intent,
            selected: node.selected,
            loading: node.loading,
            disabled: node.disabled,
        }
    }
}

pub fn default_catalog_component_model<P: CatalogComponentPart>() -> CatalogComponentModel<P> {
    let widget = widget_for_component(P::ID);
    CatalogComponentModel::from_widget_slots(widget.pattern, &widget.slots)
}

pub fn validate_catalog_component_model<P: CatalogComponentPart>(
    model: &CatalogComponentModel<P>,
) -> Result<(), garde::Report> {
    model.validate()
}

pub fn catalog_component_render_nodes<P: CatalogComponentPart>(
    model: &CatalogComponentModel<P>,
) -> Result<Vec<CatalogComponentRenderNode<P>>, garde::Report> {
    validate_catalog_component_model(model)?;
    Ok(model
        .slots
        .iter()
        .map(|slot| {
            let part = P::from_label(&slot.part)
                .expect("invariant: catalog component validation accepts only known parts");
            CatalogComponentRenderNode {
                part,
                value: slot.value.clone(),
                label: slot.label.clone(),
                detail: slot.detail.clone(),
                kind: slot.kind,
                role: slot.role,
                tone: slot.tone,
                intent: slot.intent,
                selected: slot.selected,
                loading: model.loading,
                disabled: model.disabled || slot.disabled,
            }
        })
        .collect())
}

fn component_slots_match_part<P: CatalogComponentPart>(
    slots: &[CatalogComponentSlot],
    _context: &(),
) -> garde::Result {
    let Some(root) = P::ALL.first().copied() else {
        return Err(garde::Error::new("component must define anatomy"));
    };
    if slots.first().map(|slot| slot.part.as_str()) != Some(root.label()) {
        return Err(garde::Error::new(format!(
            "{:?} model must start with root part `{}`",
            P::ID,
            root.label()
        )));
    }

    for slot in slots {
        if P::from_label(&slot.part).is_none() {
            return Err(garde::Error::new(format!(
                "{:?} model has unknown anatomy part `{}`",
                P::ID,
                slot.part
            )));
        }
    }

    for part in P::ALL {
        if !slots.iter().any(|slot| slot.part == part.label()) {
            return Err(garde::Error::new(format!(
                "{:?} model is missing anatomy part `{}`",
                P::ID,
                part.label()
            )));
        }
    }

    Ok(())
}

macro_rules! define_catalog_component {
    (
        $module:ident,
        $id:ident,
        $model:ident,
        $part:ident,
        $render_node:ident,
        $state:ident,
        $intent:ident,
        $change:ident,
        $validate_fn:ident,
        $render_nodes_fn:ident,
        $default_fn:ident,
        [$( $variant:ident => $label:literal ),+ $(,)?]
    ) => {
        pub mod $module {
            use super::*;

            #[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize)]
            #[serde(rename_all = "kebab-case")]
            pub enum $part {
                $( $variant, )+
            }

            impl $part {
                pub const fn label(self) -> &'static str {
                    match self {
                        $( Self::$variant => $label, )+
                    }
                }
            }

            impl CatalogComponentPart for $part {
                const ID: UiComponentId = UiComponentId::$id;
                const ALL: &'static [Self] = &[
                    $( Self::$variant, )+
                ];

                fn label(self) -> &'static str {
                    self.label()
                }

                fn from_label(label: &str) -> Option<Self> {
                    match label {
                        $( $label => Some(Self::$variant), )+
                        _ => None,
                    }
                }
            }

            pub type $model = CatalogComponentModel<$part>;
            pub type $render_node = CatalogComponentRenderNode<$part>;
            pub type $state = CatalogComponentState<$part>;
            pub type $intent = CatalogComponentIntent<$part>;
            pub type $change = CatalogComponentChange<$part>;

            pub fn $default_fn() -> $model {
                default_catalog_component_model::<$part>()
            }

            pub fn $validate_fn(model: &$model) -> Result<(), garde::Report> {
                validate_catalog_component_model(model)
            }

            pub fn $render_nodes_fn(
                model: &$model,
            ) -> Result<Vec<$render_node>, garde::Report> {
                catalog_component_render_nodes(model)
            }

            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn default_model_validates_with_garde() {
                    assert!($validate_fn(&$default_fn()).is_ok());
                }

                #[test]
                fn render_nodes_cover_shadcn_anatomy() {
                    let model = $default_fn();
                    let nodes = $render_nodes_fn(&model).expect("default model should validate");
                    assert_eq!(nodes.first().map(|node| node.part), Some($part::ALL[0]));
                    for part in $part::ALL {
                        assert!(
                            nodes.iter().any(|node| node.part == *part),
                            "missing {}",
                            part.label()
                        );
                    }
                }

                #[test]
                fn garde_rejects_unknown_parts() {
                    let mut model = $default_fn();
                    model.slots[0].part = "UnknownPart".to_owned();
                    assert!($validate_fn(&model).is_err());
                }
            }
        }

        pub use $module::{
            $change, $default_fn, $intent, $model, $part, $render_node, $render_nodes_fn, $state,
            $validate_fn,
        };
    };
}

define_catalog_component!(
    typography,
    Typography,
    TypographyModel,
    TypographyPart,
    TypographyRenderNode,
    TypographyState,
    TypographyIntent,
    TypographyChange,
    validate_typography_model,
    typography_render_nodes,
    default_typography_model,
    [
        Root => "Typography",
        H1 => "TypographyH1",
        H2 => "TypographyH2",
        P => "TypographyP",
        List => "TypographyList",
        Blockquote => "TypographyBlockquote",
    ]
);

pub fn catalog_component_any_render_nodes_for_component(
    id: UiComponentId,
) -> Option<Vec<CatalogAnyRenderNode>> {
    match id {
        UiComponentId::Accordion
        | UiComponentId::Alert
        | UiComponentId::AlertDialog
        | UiComponentId::AspectRatio
        | UiComponentId::Attachment
        | UiComponentId::Avatar
        | UiComponentId::Badge
        | UiComponentId::Breadcrumb
        | UiComponentId::Bubble
        | UiComponentId::Button
        | UiComponentId::ButtonGroup
        | UiComponentId::Calendar
        | UiComponentId::Card
        | UiComponentId::Carousel
        | UiComponentId::Chart
        | UiComponentId::Checkbox
        | UiComponentId::Collapsible
        | UiComponentId::Combobox
        | UiComponentId::Command
        | UiComponentId::ContextMenu
        | UiComponentId::DataTable
        | UiComponentId::DatePicker
        | UiComponentId::Dialog
        | UiComponentId::Direction
        | UiComponentId::Drawer
        | UiComponentId::DropdownMenu
        | UiComponentId::Empty
        | UiComponentId::Field
        | UiComponentId::HoverCard
        | UiComponentId::Input
        | UiComponentId::InputGroup
        | UiComponentId::InputOtp
        | UiComponentId::Item
        | UiComponentId::Kbd
        | UiComponentId::Label
        | UiComponentId::Marker
        | UiComponentId::Menubar
        | UiComponentId::Message
        | UiComponentId::MessageScroller
        | UiComponentId::NativeSelect
        | UiComponentId::NavigationMenu
        | UiComponentId::Pagination
        | UiComponentId::Popover
        | UiComponentId::Progress
        | UiComponentId::RadioGroup
        | UiComponentId::Resizable
        | UiComponentId::ScrollArea
        | UiComponentId::Select
        | UiComponentId::Separator
        | UiComponentId::Sheet
        | UiComponentId::Sidebar
        | UiComponentId::Skeleton
        | UiComponentId::Slider
        | UiComponentId::Sonner
        | UiComponentId::Spinner
        | UiComponentId::Switch
        | UiComponentId::Table
        | UiComponentId::Tabs
        | UiComponentId::Textarea
        | UiComponentId::Toast
        | UiComponentId::Toggle
        | UiComponentId::ToggleGroup
        | UiComponentId::Tooltip => None,
        UiComponentId::Typography => Some(any_nodes(typography_render_nodes(
            &default_typography_model(),
        ))),
    }
}

fn any_nodes<P: CatalogComponentPart>(
    result: Result<Vec<CatalogComponentRenderNode<P>>, garde::Report>,
) -> Vec<CatalogAnyRenderNode> {
    result
        .expect("invariant: generated default catalog model validates")
        .into_iter()
        .map(CatalogAnyRenderNode::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concrete_catalog_covers_all_non_bespoke_components() {
        for id in UiComponentId::ALL {
            let nodes = catalog_component_any_render_nodes_for_component(id);
            if matches!(
                id,
                UiComponentId::Accordion
                    | UiComponentId::Alert
                    | UiComponentId::AlertDialog
                    | UiComponentId::AspectRatio
                    | UiComponentId::Attachment
                    | UiComponentId::Avatar
                    | UiComponentId::Badge
                    | UiComponentId::Breadcrumb
                    | UiComponentId::Bubble
                    | UiComponentId::Button
                    | UiComponentId::ButtonGroup
                    | UiComponentId::Calendar
                    | UiComponentId::Card
                    | UiComponentId::Carousel
                    | UiComponentId::Chart
                    | UiComponentId::Checkbox
                    | UiComponentId::Collapsible
                    | UiComponentId::Combobox
                    | UiComponentId::Command
                    | UiComponentId::ContextMenu
                    | UiComponentId::DataTable
                    | UiComponentId::DatePicker
                    | UiComponentId::Dialog
                    | UiComponentId::Direction
                    | UiComponentId::Drawer
                    | UiComponentId::DropdownMenu
                    | UiComponentId::Empty
                    | UiComponentId::Field
                    | UiComponentId::HoverCard
                    | UiComponentId::Input
                    | UiComponentId::InputGroup
                    | UiComponentId::InputOtp
                    | UiComponentId::Item
                    | UiComponentId::Kbd
                    | UiComponentId::Label
                    | UiComponentId::Marker
                    | UiComponentId::Menubar
                    | UiComponentId::Message
                    | UiComponentId::MessageScroller
                    | UiComponentId::NativeSelect
                    | UiComponentId::NavigationMenu
                    | UiComponentId::Pagination
                    | UiComponentId::Popover
                    | UiComponentId::Progress
                    | UiComponentId::RadioGroup
                    | UiComponentId::Resizable
                    | UiComponentId::ScrollArea
                    | UiComponentId::Select
                    | UiComponentId::Separator
                    | UiComponentId::Sheet
                    | UiComponentId::Sidebar
                    | UiComponentId::Skeleton
                    | UiComponentId::Slider
                    | UiComponentId::Sonner
                    | UiComponentId::Spinner
                    | UiComponentId::Switch
                    | UiComponentId::Table
                    | UiComponentId::Tabs
                    | UiComponentId::Textarea
                    | UiComponentId::Toast
                    | UiComponentId::Toggle
                    | UiComponentId::ToggleGroup
                    | UiComponentId::Tooltip
            ) {
                assert!(nodes.is_none(), "{id:?} has a bespoke implementation");
            } else {
                let nodes = nodes.unwrap_or_else(|| panic!("{id:?} should have concrete nodes"));
                for part in id.anatomy_parts() {
                    assert!(
                        nodes.iter().any(|node| node.part == *part),
                        "{id:?} is missing {part}",
                    );
                }
            }
        }
    }

    #[test]
    fn shared_state_toggles_parts_locally() {
        let mut state = default_typography_model().state();
        assert!(!state.is_active(TypographyPart::Root));
        assert_eq!(
            state.apply(TypographyIntent::Toggle(TypographyPart::Root)),
            TypographyChange::Opened(TypographyPart::Root)
        );
        assert!(state.is_active(TypographyPart::Root));
        assert_eq!(
            state.apply(TypographyIntent::Toggle(TypographyPart::Root)),
            TypographyChange::Closed(TypographyPart::Root)
        );
        assert!(!state.is_active(TypographyPart::Root));
    }
}
