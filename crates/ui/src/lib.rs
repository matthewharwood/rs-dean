pub mod accordion;
pub mod catalog;
pub mod kit;
pub mod spec;
pub mod theme;
pub mod widgets;

#[cfg(feature = "leptos")]
mod components;

pub use accordion::{
    AccordionChange, AccordionIntent, AccordionItem, AccordionMode, AccordionModel, AccordionPart,
    AccordionRenderNode, AccordionState, accordion_dom_id, accordion_render_nodes,
    default_accordion_items,
};
pub use catalog::{
    ComponentDefinition, FrameworkMode, SHADCN_COMPONENT_COUNT, SHADCN_COMPONENTS,
    UiComponentCategory, UiComponentId, UiStateModel,
};
#[cfg(feature = "leptos")]
pub use components::*;
pub use kit::{
    ComponentImplementation, ImplementationMaturity, LayoutContract, RenderContract, StateContract,
    component_implementation, implementation_issue_marker, implemented_components,
};
#[cfg(feature = "bevy")]
pub use spec::bevy_adapter::{BevyUiPrimitive, bevy_primitives_for_component};
pub use spec::{
    UiBlock, UiBlockRole, UiBlockTone, UiComponentSpec, component_spec, detail_for_part,
    role_for_part, tone_for_category, tone_for_role,
};
pub use theme::{ActiveTheme, Oklch, Theme, ThemeChoice, ThemeId, Tone, scale};
pub use widgets::{
    UI_WIDGET_CONSTRUCTOR_COUNT, UI_WIDGET_CONSTRUCTORS, UiWidget, UiWidgetConstructor,
    UiWidgetIntent, UiWidgetPattern, UiWidgetSlot, UiWidgetSlotKind, accordion_widget,
    alert_dialog_widget, alert_widget, aspect_ratio_widget, attachment_widget, avatar_widget,
    badge_widget, breadcrumb_widget, bubble_widget, button_group_widget, button_widget,
    calendar_widget, card_widget, carousel_widget, chart_widget, checkbox_widget,
    collapsible_widget, combobox_widget, command_widget, context_menu_widget, data_table_widget,
    date_picker_widget, dialog_widget, direction_widget, drawer_widget, dropdown_menu_widget,
    empty_widget, field_widget, hover_card_widget, implemented_widgets, input_group_widget,
    input_otp_widget, input_widget, item_widget, kbd_widget, label_widget, marker_widget,
    menubar_widget, message_scroller_widget, message_widget, native_select_widget,
    navigation_menu_widget, pagination_widget, popover_widget, progress_widget, radio_group_widget,
    resizable_widget, scroll_area_widget, select_widget, separator_widget, sheet_widget,
    sidebar_widget, skeleton_widget, slider_widget, sonner_widget, spinner_widget, switch_widget,
    table_widget, tabs_widget, textarea_widget, toast_widget, toggle_group_widget, toggle_widget,
    tooltip_widget, typography_widget, widget_for_component,
};
