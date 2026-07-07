pub mod accordion;
pub mod alert;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod attachment;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod bubble;
pub mod button;
pub mod button_group;
pub mod calendar;
pub mod catalog;
pub mod catalog_components;
mod dom;
pub mod kit;
pub mod spec;
pub mod theme;
pub mod widgets;

#[cfg(feature = "leptos")]
mod components;

pub use accordion::{
    AccordionChange, AccordionIntent, AccordionItem, AccordionMode, AccordionModel, AccordionPart,
    AccordionRenderNode, AccordionState, accordion_dom_id, accordion_render_nodes,
    default_accordion_items, validate_accordion_model,
};
pub use alert::{
    AlertAction, AlertDensity, AlertIntent, AlertModel, AlertPart, AlertRenderNode, AlertTone,
    alert_render_nodes, default_alert_model, validate_alert_model,
};
pub use alert_dialog::{
    AlertDialogButton, AlertDialogChange, AlertDialogIntent, AlertDialogModel, AlertDialogPart,
    AlertDialogRenderNode, AlertDialogSize, AlertDialogState, alert_dialog_dom_id,
    alert_dialog_render_nodes, default_alert_dialog_model, validate_alert_dialog_model,
};
pub use aspect_ratio::{
    AspectRatioFit, AspectRatioModel, AspectRatioPart, AspectRatioRenderNode,
    aspect_ratio_render_nodes, default_aspect_ratio_model, validate_aspect_ratio_model,
};
pub use attachment::{
    AttachmentAction, AttachmentChange, AttachmentIntent, AttachmentKind, AttachmentModel,
    AttachmentPart, AttachmentRenderNode, AttachmentState, attachment_render_nodes,
    default_attachment_model, validate_attachment_model,
};
pub use avatar::{
    AvatarChange, AvatarImage, AvatarIntent, AvatarModel, AvatarPart, AvatarRenderNode, AvatarSize,
    AvatarState, AvatarVisual, avatar_render_nodes, default_avatar_model, validate_avatar_model,
};
pub use badge::{
    BadgeChange, BadgeIntent, BadgeModel, BadgePart, BadgeRenderNode, BadgeSize, BadgeState,
    BadgeTone, BadgeVariant, badge_render_nodes, default_badge_model, validate_badge_model,
};
pub use breadcrumb::{
    BreadcrumbChange, BreadcrumbDensity, BreadcrumbEntry, BreadcrumbIntent, BreadcrumbModel,
    BreadcrumbPart, BreadcrumbRenderNode, BreadcrumbState, breadcrumb_render_nodes,
    default_breadcrumb_model, validate_breadcrumb_model,
};
pub use bubble::{
    BubbleAction, BubbleChange, BubbleIntent, BubbleModel, BubblePart, BubbleRenderNode,
    BubbleSide, BubbleState, bubble_render_nodes, default_bubble_model, validate_bubble_model,
};
pub use button::{
    ButtonChange, ButtonIntent, ButtonKind, ButtonModel, ButtonPart, ButtonRenderNode, ButtonSize,
    ButtonState, ButtonVariant, button_render_nodes, default_button_model, validate_button_model,
};
pub use button_group::{
    ButtonGroupChange, ButtonGroupIntent, ButtonGroupItem, ButtonGroupModel,
    ButtonGroupOrientation, ButtonGroupPart, ButtonGroupRenderNode, ButtonGroupState,
    button_group_render_nodes, default_button_group_model, validate_button_group_model,
};
pub use calendar::{
    CalendarChange, CalendarDate, CalendarIntent, CalendarModel, CalendarPart, CalendarRange,
    CalendarRenderNode, CalendarSelectionMode, CalendarState, calendar_render_nodes,
    default_calendar_model, month_name, month_short_name, validate_calendar_model,
};
pub use catalog::{
    ComponentDefinition, FrameworkMode, SHADCN_COMPONENT_COUNT, SHADCN_COMPONENTS,
    UiComponentCategory, UiComponentId, UiStateModel,
};
pub use catalog_components::*;
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
    UiWidgetIntent, UiWidgetPattern, UiWidgetRenderNode, UiWidgetSlot, UiWidgetSlotKind,
    accordion_widget, alert_dialog_widget, alert_widget, aspect_ratio_widget, attachment_widget,
    avatar_widget, badge_widget, breadcrumb_widget, bubble_widget, button_group_widget,
    button_widget, calendar_widget, card_widget, carousel_widget, chart_widget, checkbox_widget,
    collapsible_widget, combobox_widget, command_widget, context_menu_widget, data_table_widget,
    date_picker_widget, dialog_widget, direction_widget, drawer_widget, dropdown_menu_widget,
    empty_widget, field_widget, hover_card_widget, implemented_widgets, input_group_widget,
    input_otp_widget, input_widget, item_widget, kbd_widget, label_widget, marker_widget,
    menubar_widget, message_scroller_widget, message_widget, native_select_widget,
    navigation_menu_widget, pagination_widget, popover_widget, progress_widget, radio_group_widget,
    resizable_widget, scroll_area_widget, select_widget, separator_widget, sheet_widget,
    sidebar_widget, skeleton_widget, slider_widget, sonner_widget, spinner_widget, switch_widget,
    table_widget, tabs_widget, textarea_widget, toast_widget, toggle_group_widget, toggle_widget,
    tooltip_widget, typography_widget, validate_widget, widget_for_component, widget_render_nodes,
};
