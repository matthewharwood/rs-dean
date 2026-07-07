use leptos::prelude::*;

use crate::{
    AccordionIntent, AccordionItem, AccordionMode, AccordionModel, AlertDensity, AlertDialogIntent,
    AlertDialogModel, AlertDialogSize, AlertDialogState, AlertModel, AlertTone, AspectRatioFit,
    AspectRatioModel, AspectRatioPart, AttachmentIntent, AttachmentKind, AttachmentModel,
    AttachmentPart, AvatarIntent, AvatarModel, AvatarPart, AvatarSize, AvatarVisual, BadgeIntent,
    BadgeModel, BadgePart, BadgeSize, BadgeTone, BadgeVariant, BreadcrumbDensity, BreadcrumbEntry,
    BreadcrumbIntent, BreadcrumbModel, BreadcrumbPart, BreadcrumbState, BubbleIntent, BubbleModel,
    BubblePart, BubbleSide, ButtonGroupIntent, ButtonGroupModel, ButtonGroupOrientation,
    ButtonGroupPart, ButtonIntent, ButtonKind, ButtonModel, ButtonPart, ButtonSize, ButtonVariant,
    CalendarIntent, CalendarModel, CalendarPart, CalendarSelectionMode, CardDensity, CardIntent,
    CardModel, CardPart, CardVariant, CarouselDensity, CarouselIntent, CarouselModel, CarouselPart,
    CatalogComponentModel, CatalogComponentPart, CatalogComponentRenderNode, ChartDensity,
    ChartIntent, ChartModel, ChartPart, ChartTone, CheckboxChecked, CheckboxDensity,
    CheckboxIntent, CheckboxModel, CheckboxPart, CollapsibleDensity, CollapsibleIntent,
    CollapsibleModel, CollapsiblePart, ComboboxDensity, ComboboxIntent, ComboboxModel,
    ComboboxPart, CommandDensity, CommandIntent, CommandModel, CommandPart,
    ComponentImplementation, ContextMenuDensity, ContextMenuIntent, ContextMenuModel,
    ContextMenuPart, ContextMenuState, DataTableDensity, DataTableIntent, DataTableModel,
    DataTablePart, DataTableState, DatePickerDensity, DatePickerIntent, DatePickerModel,
    DatePickerPart, DatePickerState, DialogIntent, DialogMode, DialogModel, DialogPart, DialogSize,
    DialogState, DirectionIntent, DirectionModel, DirectionPart, DirectionValue, DrawerIntent,
    DrawerModel, DrawerPart, DrawerSide, DrawerState, DropdownMenuDensity, DropdownMenuIntent,
    DropdownMenuModel, DropdownMenuPart, DropdownMenuState, EmptyDensity, EmptyIntent, EmptyModel,
    EmptyPart, FieldDensity, FieldIntent, FieldModel, FieldPart, HoverCardDensity, HoverCardIntent,
    HoverCardModel, HoverCardPart, InputDensity, InputGroupIntent, InputGroupModel, InputGroupPart,
    InputGroupState, InputIntent, InputModel, InputOtpIntent, InputOtpModel, InputOtpPart,
    InputOtpState, InputPart, InputState, ItemDensity, ItemIntent, ItemModel, ItemPart, ItemState,
    KbdDensity, KbdIntent, KbdModel, KbdPart, KbdState, LabelDensity, LabelIntent, LabelModel,
    LabelPart, LabelRequirement, LabelState, ThemeChoice, ThemeId, UiBlock, UiBlockTone,
    UiComponentId, UiWidgetIntent, UiWidgetPattern, UiWidgetSlotKind, accordion_dom_id,
    alert_dialog_dom_id, aspect_ratio_render_nodes, attachment_render_nodes, avatar_render_nodes,
    badge_render_nodes, breadcrumb_render_nodes, bubble_render_nodes, button_group_render_nodes,
    button_render_nodes, calendar_render_nodes, card_render_nodes, carousel_render_nodes,
    catalog_component_render_nodes, chart_render_nodes, checkbox_render_nodes,
    collapsible_render_nodes, combobox_render_nodes, command_render_nodes,
    component_implementation, component_spec, context_menu_render_nodes, data_table_render_nodes,
    date_picker_render_nodes, default_accordion_items, default_alert_dialog_model,
    default_alert_model, default_aspect_ratio_model, default_attachment_model,
    default_avatar_model, default_badge_model, default_breadcrumb_model, default_bubble_model,
    default_button_group_model, default_button_model, default_calendar_model, default_card_model,
    default_carousel_model, default_chart_model, default_checkbox_model, default_collapsible_model,
    default_combobox_model, default_command_model, default_context_menu_model,
    default_data_table_model, default_date_picker_model, default_dialog_model,
    default_direction_model, default_drawer_model, default_dropdown_menu_model,
    default_empty_model, default_field_model, default_hover_card_model, default_input_group_model,
    default_input_otp_model, default_item_model, default_kbd_model, default_label_model,
    dialog_render_nodes, direction_render_nodes, drawer_render_nodes, dropdown_menu_render_nodes,
    empty_render_nodes, field_render_nodes, hover_card_render_nodes, input_group_render_nodes,
    input_otp_render_nodes, input_render_nodes, item_render_nodes, kbd_render_nodes,
    label_render_nodes, max_data_table_page_index, month_name, validate_accordion_model,
    validate_alert_dialog_model, validate_alert_model, validate_aspect_ratio_model,
    validate_attachment_model, validate_avatar_model, validate_badge_model,
    validate_breadcrumb_model, validate_bubble_model, validate_button_group_model,
    validate_button_model, validate_calendar_model, validate_card_model, validate_carousel_model,
    validate_chart_model, validate_checkbox_model, validate_collapsible_model,
    validate_combobox_model, validate_command_model, validate_context_menu_model,
    validate_data_table_model, validate_date_picker_model, validate_dialog_model,
    validate_direction_model, validate_drawer_model, validate_dropdown_menu_model,
    validate_empty_model, validate_field_model, validate_hover_card_model,
    validate_input_group_model, validate_input_model, validate_input_otp_model,
    validate_item_model, validate_kbd_model, validate_label_model,
};

const HEALTH_CARD: &str =
    "rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-2";
const HEALTH_CARD_EYEBROW: &str = "m-0 text-00 font-7 uppercase text-brand";
const HEALTH_CARD_TITLE: &str = "m-0 mt-2xs text-1 font-7 text-text-1";
const HEALTH_CARD_BODY: &str = "m-0 mt-xs text-0 leading-0 text-text-2";
const THEME_BUTTON: &str = "inline-flex items-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const THEME_BUTTON_LABEL: &str = "text-text-muted";
const THEME_SCOPE: &str = "contents";
const COMPONENT_GALLERY: &str = "grid gap-s sm:grid-cols-2 lg:grid-cols-3";
const COMPONENT_CARD: &str = "grid min-w-0 gap-s rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-2";
const COMPONENT_HEADER: &str = "grid gap-2xs";
const COMPONENT_META: &str = "flex flex-wrap items-center gap-2xs";
const COMPONENT_PILL: &str = "rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-6 uppercase tracking-label text-text-muted";
const COMPONENT_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const COMPONENT_SUMMARY: &str = "m-0 text-0 leading-0 text-text-2";
const COMPONENT_BLOCKS: &str = "grid gap-2xs";
const COMPONENT_RECIPE: &str = "grid gap-2xs border-t border-border-subtle pt-s";
const COMPONENT_RECIPE_TITLE: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const COMPONENT_TAG_ROW: &str = "flex flex-wrap gap-2xs";
const COMPONENT_TAG: &str = "rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-2";
const COMPONENT_DEMO: &str = "grid gap-2xs border-t border-border-subtle pt-s";
const BLOCK_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const BLOCK_DETAIL: &str = "m-0 text-0 leading-0 text-text-2";
const WIDGET_SHELL: &str = "grid min-w-0 gap-s rounded-field bg-surface-1 p-s text-text-1";
const WIDGET_HEADER: &str = "grid gap-2xs";
const WIDGET_EYEBROW: &str = "m-0 text-00 font-7 uppercase tracking-label text-brand";
const WIDGET_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const WIDGET_SUMMARY: &str = "m-0 text-0 leading-0 text-text-2";
const WIDGET_CONTENT: &str = "grid gap-2xs";
const WIDGET_CONTENT_INLINE: &str = "flex flex-wrap items-center gap-2xs";
const WIDGET_SLOT: &str =
    "grid gap-3xs rounded-field border border-border-subtle bg-surface-2 p-xs text-text-1";
const WIDGET_SLOT_ACTIVE: &str =
    "grid gap-3xs rounded-field border border-brand bg-primary-soft p-xs text-text-1";
const WIDGET_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const WIDGET_VALUE: &str = "m-0 text-0 leading-0 text-text-2";
const WIDGET_BUTTON: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const WIDGET_BUTTON_ACTIVE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const WIDGET_INPUT: &str = "min-h-field w-full rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-0 text-text-1 outline-none focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const WIDGET_TEXTAREA: &str = "min-h-2xl w-full resize-y rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 outline-none focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const WIDGET_BADGE: &str = "inline-flex items-center gap-2xs rounded-pill border border-border-subtle bg-surface-2 px-xs py-3xs text-00 font-7 text-text-1";
const WIDGET_MEDIA: &str = "grid min-h-xl place-items-center rounded-field border border-border-subtle bg-surface-2 p-s text-0 font-7 text-text-2";
const WIDGET_AVATAR: &str = "grid size-l place-items-center rounded-pill border border-border-subtle bg-primary-soft text-0 font-7 text-text-1";
const WIDGET_MARKER: &str = "inline-flex size-s items-center justify-center rounded-pill bg-brand text-00 font-7 text-text-on-brand";
const WIDGET_KEY: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-muted bg-surface-2 px-2xs py-3xs font-mono text-00 text-text-1 shadow-1";
const WIDGET_SEPARATOR: &str = "min-h-selector rounded-pill bg-border-subtle";
const WIDGET_TABLE: &str =
    "w-full overflow-hidden rounded-field border border-border-subtle text-left text-0 text-text-1";
const WIDGET_TABLE_CELL: &str = "border-t border-border-subtle px-xs py-2xs";
const WIDGET_PROGRESS: &str = "h-xs w-full rounded-pill accent-brand";
const WIDGET_RANGE: &str = "w-full accent-brand";
const WIDGET_SKELETON: &str = "h-s rounded-field bg-surface-3";
const WIDGET_SPINNER: &str = "size-s rounded-pill border border-border-muted border-t-brand";
const WIDGET_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const ACCORDION_ROOT: &str =
    "grid w-full gap-2xs rounded-box border border-border-subtle bg-surface-1 p-2xs text-text-1";
const ACCORDION_ITEM: &str = "grid gap-0 rounded-field border border-border-faint bg-surface-1";
const ACCORDION_ITEM_OPEN: &str = "grid gap-0 rounded-field border border-brand bg-primary-soft";
const ACCORDION_TRIGGER: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field px-xs py-2xs text-left text-0 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled";
const ACCORDION_TRIGGER_OPEN: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field bg-selected-tint px-xs py-2xs text-left text-0 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled";
const ACCORDION_CONTENT: &str = "grid gap-2xs px-xs pb-xs text-0 leading-0 text-text-2";
const ACCORDION_INDICATOR: &str =
    "grid size-s place-items-center rounded-pill bg-surface-2 text-00 font-7 text-text-muted";
const ACCORDION_EMPTY: &str =
    "rounded-field border border-border-subtle bg-surface-2 p-s text-0 leading-0 text-text-muted";
const ACCORDION_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const ALERT_STANDARD_DEFAULT: &str = "relative grid w-full gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const ALERT_STANDARD_INFO: &str = "relative grid w-full gap-xs rounded-box border border-info bg-info-soft p-s text-text-1 shadow-1";
const ALERT_STANDARD_SUCCESS: &str = "relative grid w-full gap-xs rounded-box border border-success bg-success-soft p-s text-text-1 shadow-1";
const ALERT_STANDARD_WARNING: &str = "relative grid w-full gap-xs rounded-box border border-warning bg-warning-soft p-s text-text-1 shadow-1";
const ALERT_STANDARD_DESTRUCTIVE: &str = "relative grid w-full gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const ALERT_DENSE_DEFAULT: &str = "relative grid w-full gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const ALERT_DENSE_INFO: &str = "relative grid w-full gap-2xs rounded-field border border-info bg-info-soft p-xs text-text-1 shadow-1";
const ALERT_DENSE_SUCCESS: &str = "relative grid w-full gap-2xs rounded-field border border-success bg-success-soft p-xs text-text-1 shadow-1";
const ALERT_DENSE_WARNING: &str = "relative grid w-full gap-2xs rounded-field border border-warning bg-warning-soft p-xs text-text-1 shadow-1";
const ALERT_DENSE_DESTRUCTIVE: &str = "relative grid w-full gap-2xs rounded-field border border-danger bg-error-soft p-xs text-text-1 shadow-1";
const ALERT_ROW: &str = "flex min-w-0 flex-wrap items-start gap-xs";
const ALERT_BODY: &str = "grid min-w-0 flex-1 gap-2xs";
const ALERT_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const ALERT_TITLE_DENSE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const ALERT_DESCRIPTION: &str = "m-0 text-0 leading-0 text-text-2";
const ALERT_DESCRIPTION_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const ALERT_ACTION: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ALERT_MARKER_DEFAULT: &str = "grid size-s shrink-0 place-items-center rounded-pill bg-surface-2 text-00 font-7 text-text-muted";
const ALERT_MARKER_INFO: &str = "grid size-s shrink-0 place-items-center rounded-pill bg-info text-00 font-7 text-text-on-brand";
const ALERT_MARKER_SUCCESS: &str = "grid size-s shrink-0 place-items-center rounded-pill bg-success text-00 font-7 text-text-on-brand";
const ALERT_MARKER_WARNING: &str =
    "grid size-s shrink-0 place-items-center rounded-pill bg-warning text-00 font-7 text-text-1";
const ALERT_MARKER_DESTRUCTIVE: &str = "grid size-s shrink-0 place-items-center rounded-pill bg-danger text-00 font-7 text-text-on-brand";
const ALERT_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const ALERT_DIALOG_ROOT: &str = "grid w-full gap-xs text-text-1";
const ALERT_DIALOG_TRIGGER: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ALERT_DIALOG_OVERLAY: &str =
    "grid w-full place-items-center rounded-box bg-surface-overlay p-s";
const ALERT_DIALOG_CONTENT: &str = "grid w-full gap-s rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-3";
const ALERT_DIALOG_CONTENT_SMALL: &str = "grid w-full gap-xs rounded-box border border-border-subtle bg-surface-elevated p-xs text-text-1 shadow-2";
const ALERT_DIALOG_CONTENT_DESTRUCTIVE: &str = "grid w-full gap-s rounded-box border border-danger bg-surface-elevated p-s text-text-1 shadow-3";
const ALERT_DIALOG_CONTENT_SMALL_DESTRUCTIVE: &str = "grid w-full gap-xs rounded-box border border-danger bg-surface-elevated p-xs text-text-1 shadow-2";
const ALERT_DIALOG_HEADER: &str = "grid gap-2xs";
const ALERT_DIALOG_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const ALERT_DIALOG_TITLE_SMALL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const ALERT_DIALOG_DESCRIPTION: &str = "m-0 text-0 leading-0 text-text-2";
const ALERT_DIALOG_DESCRIPTION_SMALL: &str = "m-0 text-00 leading-0 text-text-2";
const ALERT_DIALOG_FOOTER: &str = "flex flex-wrap items-center justify-end gap-2xs";
const ALERT_DIALOG_ACTION: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ALERT_DIALOG_ACTION_DESTRUCTIVE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-danger bg-error-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-press-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ALERT_DIALOG_CANCEL: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ALERT_DIALOG_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const ASPECT_RATIO_ROOT: &str = "grid w-full gap-2xs text-text-1";
const ASPECT_RATIO_FIGURE: &str = "m-0 grid gap-2xs";
const ASPECT_RATIO_FRAME: &str = "relative grid w-full overflow-hidden rounded-box border border-border-subtle bg-surface-2 p-2xs shadow-1";
const ASPECT_RATIO_MEDIA_COVER: &str =
    "grid h-full w-full place-items-center rounded-field bg-primary-soft p-s text-center";
const ASPECT_RATIO_MEDIA_CONTAIN: &str = "grid h-full w-full place-items-center rounded-field border border-border-faint bg-surface-1 p-s text-center";
const ASPECT_RATIO_MEDIA_LOADING: &str =
    "grid h-full w-full place-items-center rounded-field bg-surface-3 p-s text-center";
const ASPECT_RATIO_MEDIA_DISABLED: &str =
    "grid h-full w-full place-items-center rounded-field bg-surface-2 p-s text-center";
const ASPECT_RATIO_MEDIA_STACK: &str = "grid gap-2xs";
const ASPECT_RATIO_MEDIA_MARKER: &str = "mx-auto grid size-xl place-items-center rounded-field border border-border-subtle bg-surface-elevated text-1 font-7 text-brand shadow-1";
const ASPECT_RATIO_MEDIA_LABEL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const ASPECT_RATIO_MEDIA_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const ASPECT_RATIO_MEDIA_MUTED: &str = "m-0 text-0 font-6 leading-0 text-text-muted";
const ASPECT_RATIO_CAPTION: &str = "flex flex-wrap items-start justify-between gap-2xs";
const ASPECT_RATIO_CAPTION_TEXT: &str = "grid min-w-0 gap-3xs";
const ASPECT_RATIO_LABEL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const ASPECT_RATIO_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const ASPECT_RATIO_BADGE: &str = "inline-flex rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-7 uppercase tracking-label text-text-muted";
const ASPECT_RATIO_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const ATTACHMENT_ROOT: &str = "flex min-w-0 items-center gap-xs rounded-box border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const ATTACHMENT_ROOT_LOADING: &str = "flex min-w-0 items-center gap-xs rounded-box border border-info bg-info-soft p-xs text-text-1 shadow-1";
const ATTACHMENT_ROOT_DISABLED: &str = "flex min-w-0 items-center gap-xs rounded-box border border-border-muted bg-surface-2 p-xs text-text-disabled";
const ATTACHMENT_PREVIEW_PDF: &str = "grid size-xl shrink-0 place-items-center rounded-field bg-error-soft text-00 font-7 text-text-1";
const ATTACHMENT_PREVIEW_IMAGE: &str = "grid size-xl shrink-0 place-items-center rounded-field bg-success-soft text-00 font-7 text-text-1";
const ATTACHMENT_PREVIEW_ARCHIVE: &str = "grid size-xl shrink-0 place-items-center rounded-field bg-warning-soft text-00 font-7 text-text-1";
const ATTACHMENT_PREVIEW_DATA: &str = "grid size-xl shrink-0 place-items-center rounded-field bg-info-soft text-00 font-7 text-text-1";
const ATTACHMENT_PREVIEW_MUTED: &str = "grid size-xl shrink-0 place-items-center rounded-field bg-surface-3 text-00 font-7 text-text-muted";
const ATTACHMENT_BODY: &str = "grid min-w-0 flex-1 gap-3xs";
const ATTACHMENT_TITLE: &str =
    "m-0 overflow-hidden text-ellipsis whitespace-nowrap text-0 font-7 leading-0 text-text-1";
const ATTACHMENT_TITLE_DISABLED: &str = "m-0 overflow-hidden text-ellipsis whitespace-nowrap text-0 font-7 leading-0 text-text-disabled";
const ATTACHMENT_META: &str = "m-0 text-00 leading-0 text-text-2";
const ATTACHMENT_META_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const ATTACHMENT_ACTION: &str = "inline-flex min-h-field shrink-0 items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ATTACHMENT_ACTION_ACTIVE: &str = "inline-flex min-h-field shrink-0 items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ATTACHMENT_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const AVATAR_ROOT_SMALL: &str = "m-0 grid size-l place-items-center overflow-hidden rounded-pill border border-border-subtle bg-surface-2 text-text-1 shadow-1";
const AVATAR_ROOT_MEDIUM: &str = "m-0 grid size-xl place-items-center overflow-hidden rounded-pill border border-border-subtle bg-surface-2 text-text-1 shadow-1";
const AVATAR_ROOT_LARGE: &str = "m-0 grid size-2xl place-items-center overflow-hidden rounded-pill border border-border-subtle bg-surface-2 text-text-1 shadow-2";
const AVATAR_ROOT_DISABLED: &str = "m-0 grid size-xl place-items-center overflow-hidden rounded-pill border border-border-muted bg-surface-3 text-text-disabled";
const AVATAR_IMAGE: &str = "h-full w-full object-cover";
const AVATAR_FALLBACK: &str =
    "grid h-full w-full place-items-center bg-primary-soft text-0 font-7 text-text-1";
const AVATAR_FALLBACK_SMALL: &str =
    "grid h-full w-full place-items-center bg-primary-soft text-00 font-7 text-text-1";
const AVATAR_FALLBACK_LARGE: &str =
    "grid h-full w-full place-items-center bg-primary-soft text-1 font-7 text-text-1";
const AVATAR_FALLBACK_MUTED: &str =
    "grid h-full w-full place-items-center bg-surface-3 text-0 font-7 text-text-muted";
const AVATAR_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const BADGE_BASE: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-pill border transition-colors hover:bg-hover-tint";
const BADGE_SIZE_SMALL: &str = "min-h-s gap-3xs px-2xs py-3xs text-00 font-7 leading-0";
const BADGE_SIZE_MEDIUM: &str = "min-h-field gap-2xs px-xs py-2xs text-0 font-7 leading-0";
const BADGE_SOFT_DEFAULT: &str = "border-border-subtle bg-surface-2 text-text-1";
const BADGE_SOFT_BRAND: &str = "border-brand bg-primary-soft text-text-1";
const BADGE_SOFT_INFO: &str = "border-info bg-info-soft text-text-1";
const BADGE_SOFT_SUCCESS: &str = "border-success bg-success-soft text-text-1";
const BADGE_SOFT_WARNING: &str = "border-warning bg-warning-soft text-text-1";
const BADGE_SOFT_DESTRUCTIVE: &str = "border-danger bg-error-soft text-text-1";
const BADGE_SOFT_MUTED: &str = "border-border-muted bg-surface-3 text-text-muted";
const BADGE_SOLID_DEFAULT: &str = "border-border-strong bg-surface-3 text-text-1";
const BADGE_SOLID_BRAND: &str = "border-brand bg-brand text-text-on-brand";
const BADGE_SOLID_INFO: &str = "border-info bg-info text-text-on-brand";
const BADGE_SOLID_SUCCESS: &str = "border-success bg-success text-text-on-brand";
const BADGE_SOLID_WARNING: &str = "border-warning bg-warning text-text-1";
const BADGE_SOLID_DESTRUCTIVE: &str = "border-danger bg-danger text-text-on-brand";
const BADGE_SOLID_MUTED: &str = "border-border-muted bg-surface-3 text-text-muted";
const BADGE_OUTLINE_DEFAULT: &str = "border-border-strong bg-surface-1 text-text-1";
const BADGE_OUTLINE_BRAND: &str = "border-brand bg-surface-1 text-brand";
const BADGE_OUTLINE_INFO: &str = "border-info bg-surface-1 text-info";
const BADGE_OUTLINE_SUCCESS: &str = "border-success bg-surface-1 text-success";
const BADGE_OUTLINE_WARNING: &str = "border-warning bg-surface-1 text-warning";
const BADGE_OUTLINE_DESTRUCTIVE: &str = "border-danger bg-surface-1 text-danger";
const BADGE_OUTLINE_MUTED: &str = "border-border-muted bg-surface-1 text-text-muted";
const BADGE_DISABLED: &str = "opacity-disabled";
const BADGE_HIGHLIGHTED: &str = "shadow-1";
const BADGE_ICON: &str = "inline-flex min-w-s justify-center text-00 font-7 leading-0";
const BADGE_TEXT: &str = "truncate";
const BADGE_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const BREADCRUMB_ROOT: &str = "w-full min-w-0 text-text-1";
const BREADCRUMB_ROOT_DISABLED: &str = "w-full min-w-0 text-text-disabled";
const BREADCRUMB_LIST: &str =
    "m-0 flex min-w-0 list-none flex-wrap items-center gap-2xs p-0 text-0 leading-0";
const BREADCRUMB_LIST_DENSE: &str =
    "m-0 flex min-w-0 list-none flex-wrap items-center gap-3xs p-0 text-00 leading-0";
const BREADCRUMB_ITEM: &str = "inline-flex min-w-0 items-center gap-2xs";
const BREADCRUMB_LINK: &str = "inline-flex min-h-s max-w-full items-center rounded-field px-2xs py-3xs text-text-2 transition-colors hover:bg-hover-tint hover:text-text-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const BREADCRUMB_LINK_ACTIVE: &str = "inline-flex min-h-s max-w-full items-center rounded-field bg-selected-tint px-2xs py-3xs text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const BREADCRUMB_LINK_DISABLED: &str =
    "inline-flex min-h-s max-w-full items-center rounded-field px-2xs py-3xs text-text-disabled";
const BREADCRUMB_PAGE: &str = "max-w-full truncate text-text-1 font-7";
const BREADCRUMB_PAGE_DISABLED: &str = "max-w-full truncate text-text-disabled font-7";
const BREADCRUMB_SEPARATOR: &str = "text-text-muted";
const BREADCRUMB_LOADING: &str =
    "inline-flex min-h-s items-center rounded-field bg-surface-3 px-xs py-3xs text-text-muted";
const BREADCRUMB_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const BUBBLE_ROOT_INCOMING: &str = "flex w-full min-w-0 items-start gap-xs text-text-1";
const BUBBLE_ROOT_OUTGOING: &str =
    "flex w-full min-w-0 flex-row-reverse items-start gap-xs text-text-1";
const BUBBLE_ROOT_SYSTEM: &str = "flex w-full min-w-0 justify-center text-text-1";
const BUBBLE_ROOT_DISABLED: &str = "flex w-full min-w-0 items-start gap-xs text-text-disabled";
const BUBBLE_AVATAR: &str = "grid size-l shrink-0 place-items-center rounded-pill border border-border-subtle bg-primary-soft text-00 font-7 text-text-1";
const BUBBLE_AVATAR_OUTGOING: &str = "grid size-l shrink-0 place-items-center rounded-pill border border-brand bg-brand text-00 font-7 text-text-on-brand";
const BUBBLE_AVATAR_SYSTEM: &str = "hidden";
const BUBBLE_PANEL_INCOMING: &str =
    "grid max-w-2xl gap-2xs rounded-box border border-border-subtle bg-surface-1 p-s shadow-1";
const BUBBLE_PANEL_OUTGOING: &str =
    "grid max-w-2xl gap-2xs rounded-box border border-brand bg-primary-soft p-s shadow-1";
const BUBBLE_PANEL_SYSTEM: &str =
    "grid max-w-2xl gap-2xs rounded-box border border-border-subtle bg-surface-2 p-s";
const BUBBLE_PANEL_LOADING: &str =
    "grid max-w-2xl gap-2xs rounded-box border border-info bg-info-soft p-s shadow-1";
const BUBBLE_PANEL_DISABLED: &str =
    "grid max-w-2xl gap-2xs rounded-box border border-border-muted bg-surface-2 p-s";
const BUBBLE_CONTENT: &str = "m-0 text-0 leading-0 text-text-1";
const BUBBLE_CONTENT_DISABLED: &str = "m-0 text-0 leading-0 text-text-disabled";
const BUBBLE_META: &str = "m-0 text-00 leading-0 text-text-muted";
const BUBBLE_META_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const BUBBLE_ACTIONS: &str = "flex flex-wrap items-center gap-2xs pt-2xs";
const BUBBLE_ACTION: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const BUBBLE_ACTION_ACTIVE: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-brand bg-selected-tint px-2xs py-3xs text-00 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const BUBBLE_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const BUTTON_BASE: &str = "inline-flex max-w-full items-center justify-center whitespace-nowrap rounded-field border transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const BUTTON_SIZE_SMALL: &str = "min-h-s gap-2xs px-2xs py-3xs text-00 font-7 leading-0";
const BUTTON_SIZE_MEDIUM: &str = "min-h-field gap-2xs px-xs py-2xs text-0 font-7 leading-0";
const BUTTON_SIZE_LARGE: &str = "min-h-field gap-xs px-s py-xs text-1 font-7 leading-2";
const BUTTON_SIZE_ICON: &str = "size-l gap-0 p-0 text-0 font-7 leading-0";
const BUTTON_PRIMARY: &str = "border-brand bg-brand text-text-on-brand hover:bg-selected-tint";
const BUTTON_SECONDARY: &str = "border-border-strong bg-surface-2 text-text-1 hover:bg-hover-tint";
const BUTTON_DESTRUCTIVE: &str = "border-danger bg-danger text-text-on-brand hover:bg-press-tint";
const BUTTON_OUTLINE: &str = "border-border-strong bg-surface-1 text-text-1 hover:bg-hover-tint";
const BUTTON_GHOST: &str = "border-border-faint bg-surface-1 text-text-1 hover:bg-hover-tint";
const BUTTON_LINK: &str = "border-border-faint bg-surface-1 text-link hover:bg-hover-tint";
const BUTTON_PRESSED: &str = "shadow-1";
const BUTTON_BLOCKED: &str = "opacity-disabled";
const BUTTON_ICON: &str = "inline-flex min-w-s justify-center text-00 font-7 leading-0";
const BUTTON_LABEL: &str = "truncate";
const BUTTON_LABEL_ICON: &str = "sr-only";
const BUTTON_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const BUTTON_GROUP_ROOT_HORIZONTAL: &str = "inline-flex max-w-full items-stretch overflow-hidden rounded-field border border-border-strong bg-surface-1 p-3xs text-text-1 shadow-1";
const BUTTON_GROUP_ROOT_VERTICAL: &str = "inline-grid max-w-full overflow-hidden rounded-field border border-border-strong bg-surface-1 p-3xs text-text-1 shadow-1";
const BUTTON_GROUP_ROOT_DISABLED: &str = "opacity-disabled";
const BUTTON_GROUP_ITEM_BASE: &str = "inline-flex max-w-full items-center justify-center whitespace-nowrap transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const BUTTON_GROUP_ITEM_SMALL: &str = "min-h-s gap-2xs px-2xs py-3xs text-00 font-7 leading-0";
const BUTTON_GROUP_ITEM_MEDIUM: &str = "min-h-field gap-2xs px-xs py-2xs text-0 font-7 leading-0";
const BUTTON_GROUP_ITEM_LARGE: &str = "min-h-field gap-xs px-s py-xs text-1 font-7 leading-2";
const BUTTON_GROUP_ITEM_ICON: &str = "size-l gap-0 p-0 text-0 font-7 leading-0";
const BUTTON_GROUP_ITEM_IDLE: &str = "bg-surface-1 text-text-1 hover:bg-hover-tint";
const BUTTON_GROUP_ITEM_SELECTED_PRIMARY: &str = "bg-brand text-text-on-brand hover:bg-brand";
const BUTTON_GROUP_ITEM_SELECTED_SECONDARY: &str =
    "bg-selected-tint text-text-1 hover:bg-selected-tint";
const BUTTON_GROUP_ITEM_SELECTED_DESTRUCTIVE: &str = "bg-danger text-text-on-brand hover:bg-danger";
const BUTTON_GROUP_ITEM_SELECTED_OUTLINE: &str =
    "bg-primary-soft text-text-1 hover:bg-selected-tint";
const BUTTON_GROUP_ITEM_SELECTED_GHOST: &str = "bg-hover-tint text-text-1 hover:bg-hover-tint";
const BUTTON_GROUP_ITEM_SELECTED_LINK: &str = "bg-selected-tint text-link hover:bg-selected-tint";
const BUTTON_GROUP_ITEM_DISABLED: &str = "text-text-disabled";
const BUTTON_GROUP_ICON: &str = "inline-flex min-w-s justify-center text-00 font-7 leading-0";
const BUTTON_GROUP_LABEL: &str = "truncate";
const BUTTON_GROUP_LABEL_ICON: &str = "sr-only";
const BUTTON_GROUP_SEPARATOR_HORIZONTAL: &str = "w-selector bg-border-subtle";
const BUTTON_GROUP_SEPARATOR_VERTICAL: &str = "h-selector bg-border-subtle";
const BUTTON_GROUP_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const CALENDAR_ROOT: &str = "grid w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const CALENDAR_ROOT_DISABLED: &str = "grid w-full max-w-md gap-s rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const CALENDAR_HEADER: &str = "flex items-center justify-between gap-xs";
const CALENDAR_NAV: &str = "inline-flex size-l items-center justify-center rounded-field border border-border-strong bg-surface-2 text-0 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CALENDAR_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const CALENDAR_GRID: &str = "grid grid-cols-7 gap-2xs";
const CALENDAR_WEEKDAY: &str =
    "grid min-h-s place-items-center text-00 font-7 uppercase text-text-muted";
const CALENDAR_DAY: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-border-faint bg-surface-1 px-2xs py-3xs text-0 font-6 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled";
const CALENDAR_DAY_SELECTED: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-brand bg-brand px-2xs py-3xs text-0 font-7 leading-0 text-text-on-brand transition-colors hover:bg-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CALENDAR_DAY_RANGE: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-brand bg-primary-soft px-2xs py-3xs text-0 font-7 leading-0 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CALENDAR_DAY_OUTSIDE: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-border-faint bg-surface-2 px-2xs py-3xs text-0 font-6 leading-0 text-text-muted disabled:text-text-disabled";
const CALENDAR_RANGE: &str =
    "rounded-field border border-border-subtle bg-surface-2 p-xs text-0 leading-0 text-text-2";
const CALENDAR_RANGE_ACTIVE: &str =
    "rounded-field border border-brand bg-primary-soft p-xs text-0 font-7 leading-0 text-text-1";
const CALENDAR_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const CARD_STANDARD_ELEVATED: &str = "grid w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-2";
const CARD_STANDARD_OUTLINE: &str = "grid w-full max-w-md gap-s rounded-box border border-border-strong bg-surface-1 p-s text-text-1 shadow-1";
const CARD_STANDARD_GHOST: &str = "grid w-full max-w-md gap-s rounded-box border border-border-faint bg-surface-1 p-s text-text-1";
const CARD_DENSE_ELEVATED: &str = "grid w-full max-w-md gap-xs rounded-field border border-border-subtle bg-surface-elevated p-xs text-text-1 shadow-1";
const CARD_DENSE_OUTLINE: &str = "grid w-full max-w-md gap-xs rounded-field border border-border-strong bg-surface-1 p-xs text-text-1 shadow-1";
const CARD_DENSE_GHOST: &str = "grid w-full max-w-md gap-xs rounded-field border border-border-faint bg-surface-1 p-xs text-text-1";
const CARD_DISABLED: &str = "grid w-full max-w-md gap-s rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const CARD_HEADER: &str = "grid gap-2xs";
const CARD_HEADER_DENSE: &str = "grid gap-3xs";
const CARD_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const CARD_TITLE_DENSE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const CARD_DESCRIPTION: &str = "m-0 text-0 leading-0 text-text-2";
const CARD_DESCRIPTION_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const CARD_CONTENT: &str =
    "rounded-field border border-border-subtle bg-surface-2 p-s text-0 leading-0 text-text-1";
const CARD_CONTENT_DENSE: &str =
    "rounded-field border border-border-subtle bg-surface-2 p-xs text-00 leading-0 text-text-1";
const CARD_FOOTER: &str =
    "flex flex-wrap items-center justify-between gap-xs border-t border-border-subtle pt-s";
const CARD_FOOTER_DENSE: &str =
    "flex flex-wrap items-center justify-between gap-2xs border-t border-border-subtle pt-xs";
const CARD_FOOTER_TEXT: &str = "m-0 text-00 font-6 uppercase tracking-label text-text-muted";
const CARD_ACTION: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CARD_ACTION_ACTIVE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-selected-tint px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CARD_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const CAROUSEL_ROOT: &str = "grid w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const CAROUSEL_ROOT_DENSE: &str = "grid w-full max-w-md gap-xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const CAROUSEL_ROOT_DISABLED: &str = "grid w-full max-w-md gap-s rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const CAROUSEL_CONTENT: &str = "grid gap-xs overflow-hidden";
const CAROUSEL_CONTENT_DENSE: &str = "grid gap-2xs overflow-hidden";
const CAROUSEL_ITEM: &str = "grid min-h-xl gap-2xs rounded-field border border-border-subtle bg-surface-2 p-s text-text-1 transition-colors";
const CAROUSEL_ITEM_SELECTED: &str = "grid min-h-xl gap-2xs rounded-field border border-brand bg-primary-soft p-s text-text-1 shadow-1 transition-colors";
const CAROUSEL_ITEM_DISABLED: &str = "grid min-h-xl gap-2xs rounded-field border border-border-muted bg-surface-2 p-s text-text-disabled";
const CAROUSEL_ITEM_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const CAROUSEL_ITEM_TITLE_DENSE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const CAROUSEL_ITEM_DETAIL: &str = "m-0 text-0 leading-0 text-text-2";
const CAROUSEL_ITEM_DETAIL_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const CAROUSEL_CONTROLS: &str = "flex items-center justify-between gap-xs";
const CAROUSEL_NAV: &str = "inline-flex size-l items-center justify-center rounded-field border border-border-strong bg-surface-2 text-0 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CAROUSEL_INDICATOR: &str = "rounded-pill border border-border-subtle bg-surface-2 px-xs py-3xs text-00 font-7 text-text-muted";
const CAROUSEL_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const CHART_ROOT: &str = "grid w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const CHART_ROOT_DENSE: &str = "grid w-full max-w-md gap-xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const CHART_ROOT_DISABLED: &str = "grid w-full max-w-md gap-s rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const CHART_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const CHART_TITLE_DENSE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const CHART_SERIES_LIST: &str = "grid gap-xs";
const CHART_SERIES_LIST_DENSE: &str = "grid gap-2xs";
const CHART_SERIES: &str = "grid gap-2xs rounded-field border border-border-subtle bg-surface-2 p-xs text-left transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CHART_SERIES_SELECTED: &str = "grid gap-2xs rounded-field border border-brand bg-primary-soft p-xs text-left shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CHART_SERIES_LABEL: &str = "flex items-center justify-between gap-xs text-00 font-7 uppercase tracking-label text-text-muted";
const CHART_TRACK: &str = "h-s overflow-hidden rounded-pill bg-surface-3";
const CHART_BAR_BRAND: &str = "block h-full rounded-pill bg-brand";
const CHART_BAR_INFO: &str = "block h-full rounded-pill bg-info";
const CHART_BAR_SUCCESS: &str = "block h-full rounded-pill bg-success";
const CHART_BAR_WARNING: &str = "block h-full rounded-pill bg-warning";
const CHART_BAR_DANGER: &str = "block h-full rounded-pill bg-danger";
const CHART_BAR_MUTED: &str = "block h-full rounded-pill bg-border-strong";
const CHART_LEGEND: &str = "flex flex-wrap gap-2xs";
const CHART_LEGEND_ITEM: &str = "rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-2";
const CHART_LEGEND_SELECTED: &str =
    "rounded-pill border border-brand bg-selected-tint px-2xs py-3xs text-00 font-7 text-text-1";
const CHART_TOOLTIP: &str = "rounded-field border border-border-subtle bg-surface-elevated p-xs text-0 leading-0 text-text-1 shadow-1";
const CHART_AXIS: &str =
    "border-t border-border-subtle pt-2xs text-00 font-6 uppercase tracking-label text-text-muted";
const CHART_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const CHECKBOX_ROOT: &str = "flex w-full max-w-md items-start gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const CHECKBOX_ROOT_DENSE: &str = "flex w-full max-w-md items-start gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const CHECKBOX_ROOT_INVALID: &str = "flex w-full max-w-md items-start gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const CHECKBOX_ROOT_DISABLED: &str = "flex w-full max-w-md items-start gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const CHECKBOX_CONTROL: &str = "inline-grid size-s shrink-0 place-items-center rounded-selector border border-border-strong bg-surface-1 text-00 font-7 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CHECKBOX_CONTROL_CHECKED: &str = "inline-grid size-s shrink-0 place-items-center rounded-selector border border-brand bg-brand text-00 font-7 leading-0 text-text-on-brand transition-colors hover:bg-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CHECKBOX_CONTROL_INDETERMINATE: &str = "inline-grid size-s shrink-0 place-items-center rounded-selector border border-brand bg-primary-soft text-00 font-7 leading-0 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CHECKBOX_CONTROL_INVALID: &str = "inline-grid size-s shrink-0 place-items-center rounded-selector border border-danger bg-error-soft text-00 font-7 leading-0 text-text-1 transition-colors hover:bg-press-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CHECKBOX_CONTROL_DISABLED: &str = "inline-grid size-s shrink-0 place-items-center rounded-selector border border-border-muted bg-surface-3 text-00 font-7 leading-0 text-text-disabled";
const CHECKBOX_TEXT: &str = "grid min-w-0 gap-3xs";
const CHECKBOX_LABEL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const CHECKBOX_LABEL_DISABLED: &str = "m-0 text-0 font-7 leading-0 text-text-disabled";
const CHECKBOX_DESCRIPTION: &str = "m-0 text-00 leading-0 text-text-2";
const CHECKBOX_DESCRIPTION_INVALID: &str = "m-0 text-00 font-6 leading-0 text-danger";
const CHECKBOX_DESCRIPTION_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const CHECKBOX_REQUIRED: &str = "text-danger";
const CHECKBOX_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const COLLAPSIBLE_ROOT: &str = "grid w-full max-w-md gap-0 overflow-hidden rounded-box border border-border-subtle bg-surface-1 text-text-1 shadow-1";
const COLLAPSIBLE_ROOT_DENSE: &str = "grid w-full max-w-md gap-0 overflow-hidden rounded-field border border-border-subtle bg-surface-1 text-text-1 shadow-1";
const COLLAPSIBLE_ROOT_DISABLED: &str = "grid w-full max-w-md gap-0 overflow-hidden rounded-box border border-border-muted bg-surface-2 text-text-disabled";
const COLLAPSIBLE_TRIGGER: &str = "flex min-h-field w-full items-center justify-between gap-xs px-s py-xs text-left text-0 font-7 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COLLAPSIBLE_TRIGGER_DENSE: &str = "flex min-h-field w-full items-center justify-between gap-2xs px-xs py-2xs text-left text-00 font-7 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COLLAPSIBLE_TRIGGER_OPEN: &str = "flex min-h-field w-full items-center justify-between gap-xs bg-selected-tint px-s py-xs text-left text-0 font-7 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COLLAPSIBLE_TRIGGER_DISABLED: &str = "flex min-h-field w-full items-center justify-between gap-xs px-s py-xs text-left text-0 font-7 leading-0 text-text-disabled disabled:opacity-disabled";
const COLLAPSIBLE_INDICATOR: &str = "grid size-s shrink-0 place-items-center rounded-pill bg-surface-2 text-00 font-7 text-text-muted";
const COLLAPSIBLE_CONTENT: &str =
    "border-t border-border-subtle bg-surface-1 p-s text-0 leading-0 text-text-2";
const COLLAPSIBLE_CONTENT_DENSE: &str =
    "border-t border-border-subtle bg-surface-1 p-xs text-00 leading-0 text-text-2";
const COLLAPSIBLE_CONTENT_HIDDEN: &str = "hidden";
const COLLAPSIBLE_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const COMBOBOX_ROOT: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const COMBOBOX_ROOT_DENSE: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const COMBOBOX_ROOT_DISABLED: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const COMBOBOX_INPUT: &str = "min-h-field w-full rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COMBOBOX_INPUT_DENSE: &str = "min-h-s w-full rounded-field border border-border-strong bg-surface-1 px-2xs py-3xs text-00 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COMBOBOX_LIST: &str = "grid max-h-4xl gap-2xs overflow-auto rounded-field border border-border-subtle bg-surface-2 p-2xs";
const COMBOBOX_LIST_HIDDEN: &str = "hidden";
const COMBOBOX_OPTION: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-faint bg-surface-1 px-xs py-2xs text-left text-0 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COMBOBOX_OPTION_SELECTED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-left text-0 font-7 leading-0 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COMBOBOX_OPTION_DISABLED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-left text-0 leading-0 text-text-disabled disabled:opacity-disabled";
const COMBOBOX_EMPTY: &str =
    "rounded-field border border-border-subtle bg-surface-1 p-xs text-0 leading-0 text-text-muted";
const COMBOBOX_META: &str = "text-00 font-6 uppercase tracking-label text-text-muted";
const COMBOBOX_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const COMMAND_ROOT: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const COMMAND_ROOT_DENSE: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const COMMAND_ROOT_DISABLED: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const COMMAND_INPUT: &str = "min-h-field w-full rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COMMAND_INPUT_DENSE: &str = "min-h-s w-full rounded-field border border-border-strong bg-surface-1 px-2xs py-3xs text-00 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COMMAND_LIST: &str = "grid max-h-4xl gap-2xs overflow-auto rounded-field border border-border-subtle bg-surface-2 p-2xs";
const COMMAND_LIST_HIDDEN: &str = "hidden";
const COMMAND_GROUP: &str =
    "grid gap-2xs rounded-field border border-border-faint bg-surface-1 p-2xs";
const COMMAND_GROUP_LABEL: &str =
    "px-2xs pt-2xs text-00 font-7 uppercase tracking-label text-text-muted";
const COMMAND_ITEM: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-faint bg-surface-1 px-xs py-2xs text-left text-0 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COMMAND_ITEM_HIGHLIGHTED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-brand bg-selected-tint px-xs py-2xs text-left text-0 leading-0 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COMMAND_ITEM_SELECTED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-left text-0 font-7 leading-0 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const COMMAND_ITEM_DISABLED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-left text-0 leading-0 text-text-disabled disabled:opacity-disabled";
const COMMAND_ITEM_BODY: &str = "grid min-w-0 gap-3xs";
const COMMAND_ITEM_LABEL: &str = "m-0 text-0 font-7 leading-0";
const COMMAND_ITEM_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const COMMAND_SHORTCUT: &str = "rounded-field border border-border-muted bg-surface-2 px-2xs py-3xs font-mono text-00 text-text-muted shadow-1";
const COMMAND_EMPTY: &str =
    "rounded-field border border-border-subtle bg-surface-1 p-xs text-0 leading-0 text-text-muted";
const COMMAND_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const CONTEXT_MENU_ROOT: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const CONTEXT_MENU_ROOT_DENSE: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const CONTEXT_MENU_ROOT_DISABLED: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const CONTEXT_MENU_TRIGGER: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CONTEXT_MENU_TRIGGER_DENSE: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CONTEXT_MENU_CONTENT: &str = "grid gap-2xs rounded-field border border-border-subtle bg-surface-elevated p-2xs text-text-1 shadow-2";
const CONTEXT_MENU_CONTENT_HIDDEN: &str = "hidden";
const CONTEXT_MENU_ITEM: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-faint bg-surface-1 px-xs py-2xs text-left text-0 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CONTEXT_MENU_ITEM_ACTIVE: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-brand bg-selected-tint px-xs py-2xs text-left text-0 leading-0 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CONTEXT_MENU_ITEM_SELECTED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-left text-0 font-7 leading-0 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CONTEXT_MENU_ITEM_DANGER: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-danger bg-error-soft px-xs py-2xs text-left text-0 font-7 leading-0 text-text-1 transition-colors hover:bg-press-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CONTEXT_MENU_ITEM_DISABLED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-left text-0 leading-0 text-text-disabled disabled:opacity-disabled";
const CONTEXT_MENU_ITEM_BODY: &str = "grid min-w-0 gap-3xs";
const CONTEXT_MENU_ITEM_LABEL: &str = "m-0 text-0 font-7 leading-0";
const CONTEXT_MENU_ITEM_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const CONTEXT_MENU_SHORTCUT: &str = "rounded-field border border-border-muted bg-surface-2 px-2xs py-3xs font-mono text-00 text-text-muted shadow-1";
const CONTEXT_MENU_SEPARATOR: &str = "h-3xs rounded-pill bg-border-subtle";
const CONTEXT_MENU_SUBMENU: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-faint bg-surface-1 px-xs py-2xs text-left text-0 font-7 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CONTEXT_MENU_SUBMENU_OPEN: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-brand bg-selected-tint px-xs py-2xs text-left text-0 font-7 leading-0 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const CONTEXT_MENU_SUBMENU_DISABLED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-left text-0 font-7 leading-0 text-text-disabled disabled:opacity-disabled";
const CONTEXT_MENU_SUBMENU_MARKER: &str = "text-00 font-7 text-text-muted";
const CONTEXT_MENU_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const DATA_TABLE_ROOT: &str = "grid w-full max-w-2xl gap-2xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const DATA_TABLE_ROOT_DENSE: &str = "grid w-full max-w-2xl gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const DATA_TABLE_ROOT_DISABLED: &str = "grid w-full max-w-2xl gap-2xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const DATA_TABLE_TOOLBAR: &str = "flex flex-wrap items-center justify-between gap-2xs";
const DATA_TABLE_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const DATA_TABLE_FILTER: &str = "min-h-field min-w-0 flex-1 rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DATA_TABLE_FILTER_DENSE: &str = "min-h-s min-w-0 flex-1 rounded-field border border-border-strong bg-surface-1 px-2xs py-3xs text-00 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DATA_TABLE_FRAME: &str =
    "w-full overflow-hidden rounded-field border border-border-subtle bg-surface-1";
const DATA_TABLE_TABLE: &str = "w-full border-collapse text-left text-0 text-text-1";
const DATA_TABLE_HEADER: &str = "border-b border-border-subtle bg-surface-2 px-xs py-2xs text-left text-00 font-7 uppercase tracking-label text-text-muted";
const DATA_TABLE_HEADER_SORTED: &str = "border-b border-brand bg-primary-soft px-xs py-2xs text-left text-00 font-7 uppercase tracking-label text-text-1";
const DATA_TABLE_HEADER_BUTTON: &str = "flex w-full items-center justify-between gap-2xs rounded-field text-left text-00 font-7 uppercase tracking-label focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled";
const DATA_TABLE_SORT_MARKER: &str = "text-00 font-7 text-text-muted";
const DATA_TABLE_ROW: &str =
    "border-b border-border-faint bg-surface-1 transition-colors hover:bg-hover-tint";
const DATA_TABLE_ROW_SELECTED: &str =
    "border-b border-brand bg-primary-soft transition-colors hover:bg-selected-tint";
const DATA_TABLE_ROW_DISABLED: &str =
    "border-b border-border-muted bg-surface-2 text-text-disabled";
const DATA_TABLE_CELL: &str = "px-xs py-2xs text-0 leading-0 text-text-1";
const DATA_TABLE_CELL_DENSE: &str = "px-2xs py-3xs text-00 leading-0 text-text-1";
const DATA_TABLE_EMPTY: &str = "px-xs py-s text-center text-0 leading-0 text-text-muted";
const DATA_TABLE_PAGINATION: &str = "flex flex-wrap items-center justify-between gap-2xs";
const DATA_TABLE_PAGE_LABEL: &str = "m-0 text-00 font-6 uppercase tracking-label text-text-muted";
const DATA_TABLE_PAGE_BUTTON: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DATA_TABLE_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const DATE_PICKER_ROOT: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const DATE_PICKER_ROOT_DENSE: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const DATE_PICKER_ROOT_DISABLED: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const DATE_PICKER_TRIGGER: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-left text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DATE_PICKER_TRIGGER_DENSE: &str = "flex min-h-s w-full items-center justify-between gap-2xs rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-left text-00 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DATE_PICKER_TRIGGER_OPEN: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-left text-0 font-7 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DATE_PICKER_VALUE: &str = "truncate text-text-1";
const DATE_PICKER_VALUE_EMPTY: &str = "truncate text-text-muted";
const DATE_PICKER_MARKER: &str = "grid size-s shrink-0 place-items-center rounded-pill bg-surface-3 text-00 font-7 text-text-muted";
const DATE_PICKER_POPOVER: &str =
    "grid gap-2xs rounded-field border border-border-subtle bg-surface-elevated p-xs shadow-2";
const DATE_PICKER_POPOVER_HIDDEN: &str = "hidden";
const DATE_PICKER_POPOVER_HEADER: &str = "flex items-center justify-between gap-2xs";
const DATE_PICKER_CLEAR: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DATE_PICKER_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const DIALOG_ROOT: &str = "grid w-full gap-2xs text-text-1";
const DIALOG_ROOT_DISABLED: &str = "grid w-full gap-2xs text-text-disabled";
const DIALOG_TRIGGER: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DIALOG_TRIGGER_OPEN: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DIALOG_OVERLAY_MODAL: &str =
    "grid w-full place-items-center rounded-box bg-surface-overlay p-s";
const DIALOG_OVERLAY_NON_MODAL: &str =
    "grid w-full place-items-center rounded-box border border-border-subtle bg-surface-2 p-s";
const DIALOG_CONTENT: &str = "grid w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-3";
const DIALOG_CONTENT_SMALL: &str = "grid w-full max-w-md gap-xs rounded-box border border-border-subtle bg-surface-elevated p-xs text-text-1 shadow-2";
const DIALOG_HEADER: &str = "grid gap-2xs";
const DIALOG_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const DIALOG_TITLE_SMALL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const DIALOG_DESCRIPTION: &str = "m-0 text-0 leading-0 text-text-2";
const DIALOG_DESCRIPTION_SMALL: &str = "m-0 text-00 leading-0 text-text-2";
const DIALOG_BODY: &str = "m-0 text-0 leading-0 text-text-2";
const DIALOG_FOOTER: &str = "flex flex-wrap items-center justify-end gap-2xs";
const DIALOG_ACTION: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DIALOG_ACTION_SECONDARY: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DIALOG_ACTION_ACTIVE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-selected-tint px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DIALOG_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const DIRECTION_ROOT: &str = "grid w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-1 p-s text-start text-text-1 shadow-1";
const DIRECTION_ROOT_DISABLED: &str = "grid w-full max-w-md gap-s rounded-box border border-border-muted bg-surface-2 p-s text-start text-text-disabled shadow-1";
const DIRECTION_HEADER: &str = "grid gap-2xs";
const DIRECTION_EYEBROW: &str = "m-0 text-00 font-7 uppercase tracking-label text-brand";
const DIRECTION_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const DIRECTION_DETAIL: &str = "m-0 text-0 leading-0 text-text-2";
const DIRECTION_ACTIONS: &str = "flex flex-wrap items-center gap-2xs";
const DIRECTION_ACTION: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DIRECTION_ACTION_ACTIVE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DIRECTION_SCOPE: &str =
    "grid gap-xs rounded-field border border-border-subtle bg-surface-2 p-xs text-start";
const DIRECTION_SCOPE_ACTIVE: &str =
    "grid gap-xs rounded-field border border-brand bg-selected-tint p-xs text-start";
const DIRECTION_SCOPE_DISABLED: &str = "grid gap-xs rounded-field border border-border-muted bg-surface-2 p-xs text-start text-text-disabled";
const DIRECTION_CONTENT: &str = "grid gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-start text-text-1";
const DIRECTION_CONTENT_RTL: &str =
    "grid gap-2xs rounded-field border border-brand bg-primary-soft p-xs text-start text-text-1";
const DIRECTION_BADGE: &str = "inline-flex w-fit items-center rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-7 uppercase tracking-label text-text-muted";
const DIRECTION_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const DRAWER_ROOT: &str = "grid w-full gap-2xs text-text-1";
const DRAWER_ROOT_DISABLED: &str = "grid w-full gap-2xs text-text-disabled";
const DRAWER_TRIGGER: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DRAWER_TRIGGER_OPEN: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DRAWER_OVERLAY_BOTTOM: &str =
    "fixed inset-0 z-50 grid items-end justify-items-center bg-overlay p-s text-text-1";
const DRAWER_OVERLAY_TOP: &str =
    "fixed inset-0 z-50 grid items-start justify-items-center bg-overlay p-s text-text-1";
const DRAWER_OVERLAY_RIGHT: &str =
    "fixed inset-0 z-50 grid items-stretch justify-items-end bg-overlay p-s text-text-1";
const DRAWER_OVERLAY_LEFT: &str =
    "fixed inset-0 z-50 grid items-stretch justify-items-start bg-overlay p-s text-text-1";
const DRAWER_CONTENT_VERTICAL: &str = "grid w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-3";
const DRAWER_CONTENT_SIDE: &str = "grid h-full w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-3";
const DRAWER_HANDLE: &str =
    "mx-auto h-2xs w-l rounded-pill border border-border-subtle bg-surface-3";
const DRAWER_HANDLE_ACTIVE: &str = "mx-auto h-2xs w-xl rounded-pill border border-brand bg-brand";
const DRAWER_HEADER: &str = "grid gap-2xs";
const DRAWER_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const DRAWER_DESCRIPTION: &str = "m-0 text-0 leading-0 text-text-2";
const DRAWER_BODY: &str = "m-0 text-0 leading-0 text-text-2";
const DRAWER_FOOTER: &str = "flex flex-wrap items-center justify-end gap-2xs";
const DRAWER_ACTION: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DRAWER_ACTION_SECONDARY: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DRAWER_ACTION_ACTIVE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-selected-tint px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DRAWER_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const DROPDOWN_MENU_ROOT: &str = "relative grid w-full max-w-md gap-2xs text-text-1";
const DROPDOWN_MENU_ROOT_DENSE: &str = "relative grid w-full max-w-md gap-3xs text-text-1";
const DROPDOWN_MENU_ROOT_DISABLED: &str =
    "relative grid w-full max-w-md gap-2xs text-text-disabled";
const DROPDOWN_MENU_TRIGGER: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DROPDOWN_MENU_TRIGGER_DENSE: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DROPDOWN_MENU_TRIGGER_OPEN: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const DROPDOWN_MENU_CONTENT: &str = "absolute start-0 top-full z-50 mt-2xs grid w-full gap-3xs rounded-box border border-border-subtle bg-surface-elevated p-2xs text-text-1 shadow-3";
const DROPDOWN_MENU_CONTENT_DENSE: &str = "absolute start-0 top-full z-50 mt-2xs grid w-full gap-3xs rounded-field border border-border-subtle bg-surface-elevated p-3xs text-text-1 shadow-2";
const DROPDOWN_MENU_CONTENT_HIDDEN: &str = "hidden";
const DROPDOWN_MENU_ITEM: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field px-xs py-2xs text-left text-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled";
const DROPDOWN_MENU_ITEM_DENSE: &str = "flex min-h-s w-full items-center justify-between gap-2xs rounded-field px-2xs py-3xs text-left text-00 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled";
const DROPDOWN_MENU_ITEM_ACTIVE: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field bg-selected-tint px-xs py-2xs text-left text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const DROPDOWN_MENU_ITEM_SELECTED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-left text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const DROPDOWN_MENU_ITEM_DESTRUCTIVE: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field bg-error-soft px-xs py-2xs text-left text-0 font-7 text-text-1 transition-colors hover:bg-press-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const DROPDOWN_MENU_ITEM_DISABLED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field px-xs py-2xs text-left text-0 text-text-disabled opacity-disabled";
const DROPDOWN_MENU_ITEM_BODY: &str = "grid min-w-0 gap-3xs";
const DROPDOWN_MENU_ITEM_LABEL: &str = "truncate";
const DROPDOWN_MENU_ITEM_DETAIL: &str = "truncate text-00 leading-0 text-text-muted";
const DROPDOWN_MENU_SHORTCUT: &str = "shrink-0 rounded-field border border-border-muted bg-surface-2 px-2xs py-3xs font-mono text-00 text-text-muted";
const DROPDOWN_MENU_LABEL: &str =
    "px-xs py-3xs text-00 font-7 uppercase tracking-label text-text-muted";
const DROPDOWN_MENU_SEPARATOR: &str = "my-3xs h-3xs rounded-pill bg-border-subtle";
const DROPDOWN_MENU_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const EMPTY_ROOT: &str = "grid w-full max-w-md justify-items-center gap-s rounded-box border border-border-subtle bg-surface-1 p-m text-center text-text-1 shadow-1";
const EMPTY_ROOT_DENSE: &str = "grid w-full max-w-md justify-items-center gap-xs rounded-field border border-border-subtle bg-surface-1 p-s text-center text-text-1 shadow-1";
const EMPTY_ROOT_LOADING: &str = "grid w-full max-w-md justify-items-center gap-s rounded-box border border-info bg-info-soft p-m text-center text-text-1 shadow-1";
const EMPTY_ROOT_DISABLED: &str = "grid w-full max-w-md justify-items-center gap-s rounded-box border border-border-muted bg-surface-2 p-m text-center text-text-disabled";
const EMPTY_HEADER: &str = "grid justify-items-center gap-2xs";
const EMPTY_HEADER_DENSE: &str = "grid justify-items-center gap-3xs";
const EMPTY_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const EMPTY_TITLE_DENSE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const EMPTY_DESCRIPTION: &str = "m-0 text-0 leading-0 text-text-2";
const EMPTY_DESCRIPTION_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const EMPTY_CONTENT: &str = "grid w-full justify-items-center gap-2xs rounded-field border border-border-faint bg-surface-2 p-s";
const EMPTY_CONTENT_DENSE: &str = "grid w-full justify-items-center gap-3xs rounded-field border border-border-faint bg-surface-2 p-xs";
const EMPTY_MARKER: &str = "grid size-xl place-items-center rounded-field border border-border-subtle bg-primary-soft text-0 font-7 text-brand";
const EMPTY_MARKER_DENSE: &str = "grid size-l place-items-center rounded-field border border-border-subtle bg-primary-soft text-00 font-7 text-brand";
const EMPTY_CONTENT_TEXT: &str = "m-0 text-0 leading-0 text-text-2";
const EMPTY_CONTENT_TEXT_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const EMPTY_ACTION: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const EMPTY_ACTION_DENSE: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-2xs py-3xs text-00 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const EMPTY_ACTION_ACTIVE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-selected-tint px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const EMPTY_ACTION_DISABLED: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 text-text-disabled opacity-disabled";
const EMPTY_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const FIELD_ROOT: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const FIELD_ROOT_DENSE: &str = "grid w-full max-w-md gap-3xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const FIELD_ROOT_INVALID: &str = "grid w-full max-w-md gap-2xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const FIELD_ROOT_LOADING: &str = "grid w-full max-w-md gap-2xs rounded-box border border-info bg-info-soft p-s text-text-1 shadow-1";
const FIELD_ROOT_DISABLED: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled";
const FIELD_LABEL: &str = "m-0 flex items-center gap-2xs text-0 font-7 leading-0 text-text-1";
const FIELD_LABEL_DENSE: &str =
    "m-0 flex items-center gap-2xs text-00 font-7 leading-0 text-text-1";
const FIELD_LABEL_DISABLED: &str =
    "m-0 flex items-center gap-2xs text-0 font-7 leading-0 text-text-disabled";
const FIELD_REQUIRED: &str = "text-danger";
const FIELD_CONTROL: &str = "min-h-field w-full rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const FIELD_CONTROL_DENSE: &str = "min-h-s w-full rounded-field border border-border-strong bg-surface-1 px-2xs py-3xs text-00 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const FIELD_CONTROL_FOCUSED: &str = "min-h-field w-full rounded-field border border-brand bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const FIELD_CONTROL_DENSE_FOCUSED: &str = "min-h-s w-full rounded-field border border-brand bg-surface-1 px-2xs py-3xs text-00 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const FIELD_CONTROL_INVALID: &str = "min-h-field w-full rounded-field border border-danger bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const FIELD_CONTROL_DENSE_INVALID: &str = "min-h-s w-full rounded-field border border-danger bg-surface-1 px-2xs py-3xs text-00 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const FIELD_CONTROL_DISABLED: &str = "min-h-field w-full rounded-field border border-border-muted bg-surface-3 px-xs py-2xs text-0 leading-0 text-text-disabled outline-none opacity-disabled";
const FIELD_CONTROL_DENSE_DISABLED: &str = "min-h-s w-full rounded-field border border-border-muted bg-surface-3 px-2xs py-3xs text-00 leading-0 text-text-disabled outline-none opacity-disabled";
const FIELD_DESCRIPTION: &str = "m-0 text-00 leading-0 text-text-2";
const FIELD_DESCRIPTION_DENSE: &str = "m-0 text-00 leading-0 text-text-muted";
const FIELD_DESCRIPTION_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const FIELD_ERROR_TEXT: &str = "m-0 text-00 font-6 leading-0 text-danger";
const FIELD_ERROR_HIDDEN: &str = "hidden";
const FIELD_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const HOVER_CARD_ROOT: &str = "relative grid w-full max-w-md gap-2xs text-text-1";
const HOVER_CARD_ROOT_DISABLED: &str = "relative grid w-full max-w-md gap-2xs text-text-disabled";
const HOVER_CARD_TRIGGER: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const HOVER_CARD_TRIGGER_DENSE: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const HOVER_CARD_TRIGGER_OPEN: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 shadow-2 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const HOVER_CARD_TRIGGER_DENSE_OPEN: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-2xs py-3xs text-00 font-7 text-text-1 shadow-2 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const HOVER_CARD_TRIGGER_DISABLED: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 text-text-disabled opacity-disabled";
const HOVER_CARD_CONTENT: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-xs rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-3";
const HOVER_CARD_CONTENT_DENSE: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-2xs rounded-field border border-border-subtle bg-surface-elevated p-xs text-text-1 shadow-2";
const HOVER_CARD_CONTENT_LOADING: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-xs rounded-box border border-info bg-info-soft p-s text-text-1 shadow-2";
const HOVER_CARD_CONTENT_DISABLED: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled shadow-1";
const HOVER_CARD_CONTENT_HIDDEN: &str = "hidden";
const HOVER_CARD_META: &str = "m-0 text-00 font-7 uppercase tracking-label text-brand";
const HOVER_CARD_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const HOVER_CARD_TITLE_DENSE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const HOVER_CARD_DETAIL: &str = "m-0 text-0 leading-0 text-text-2";
const HOVER_CARD_DETAIL_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const HOVER_CARD_ARROW: &str = "grid size-s place-items-center rounded-field border border-border-subtle bg-surface-elevated text-00 font-7 text-text-muted shadow-1";
const HOVER_CARD_ARROW_HIDDEN: &str = "hidden";
const HOVER_CARD_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const INPUT_ROOT: &str = "grid w-full max-w-md gap-2xs text-text-1";
const INPUT_ROOT_DISABLED: &str = "grid w-full max-w-md gap-2xs text-text-disabled";
const INPUT_ROW: &str = "flex min-h-field w-full items-stretch overflow-hidden rounded-field border border-border-strong bg-surface-1 text-text-1 shadow-1 transition-colors focus-within:border-brand focus-within:outline-2 focus-within:outline-offset-2 focus-within:outline-focus-ring";
const INPUT_ROW_DENSE: &str = "flex min-h-s w-full items-stretch overflow-hidden rounded-field border border-border-strong bg-surface-1 text-text-1 shadow-1 transition-colors focus-within:border-brand focus-within:outline-2 focus-within:outline-offset-2 focus-within:outline-focus-ring";
const INPUT_ROW_FOCUSED: &str = "flex min-h-field w-full items-stretch overflow-hidden rounded-field border border-brand bg-surface-1 text-text-1 shadow-2 transition-colors";
const INPUT_ROW_DENSE_FOCUSED: &str = "flex min-h-s w-full items-stretch overflow-hidden rounded-field border border-brand bg-surface-1 text-text-1 shadow-2 transition-colors";
const INPUT_ROW_INVALID: &str = "flex min-h-field w-full items-stretch overflow-hidden rounded-field border border-danger bg-error-soft text-text-1 shadow-1 transition-colors";
const INPUT_ROW_DENSE_INVALID: &str = "flex min-h-s w-full items-stretch overflow-hidden rounded-field border border-danger bg-error-soft text-text-1 shadow-1 transition-colors";
const INPUT_ROW_LOADING: &str = "flex min-h-field w-full items-stretch overflow-hidden rounded-field border border-info bg-info-soft text-text-1 shadow-1";
const INPUT_ROW_DISABLED: &str = "flex min-h-field w-full items-stretch overflow-hidden rounded-field border border-border-muted bg-surface-2 text-text-disabled opacity-disabled";
const INPUT_PREFIX: &str = "inline-flex items-center border-r border-border-subtle bg-surface-2 px-xs text-0 font-6 text-text-muted";
const INPUT_PREFIX_DENSE: &str = "inline-flex items-center border-r border-border-subtle bg-surface-2 px-2xs text-00 font-6 text-text-muted";
const INPUT_PREFIX_HIDDEN: &str = "hidden";
const INPUT_CONTROL: &str = "min-w-0 flex-1 bg-transparent px-xs py-2xs text-0 leading-0 text-text-1 outline-none placeholder:text-text-muted disabled:text-text-disabled";
const INPUT_CONTROL_DENSE: &str = "min-w-0 flex-1 bg-transparent px-2xs py-3xs text-00 leading-0 text-text-1 outline-none placeholder:text-text-muted disabled:text-text-disabled";
const INPUT_CONTROL_DISABLED: &str =
    "min-w-0 flex-1 bg-transparent px-xs py-2xs text-0 leading-0 text-text-disabled outline-none";
const INPUT_SUFFIX: &str = "inline-flex shrink-0 items-center justify-center border-l border-border-subtle bg-surface-2 px-xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const INPUT_SUFFIX_DENSE: &str = "inline-flex shrink-0 items-center justify-center border-l border-border-subtle bg-surface-2 px-2xs text-00 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const INPUT_SUFFIX_ACTIVE: &str = "inline-flex shrink-0 items-center justify-center border-l border-brand bg-primary-soft px-xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const INPUT_SUFFIX_DISABLED: &str = "inline-flex shrink-0 items-center justify-center border-l border-border-muted bg-surface-2 px-xs text-0 font-6 text-text-disabled opacity-disabled";
const INPUT_SUFFIX_HIDDEN: &str = "hidden";
const INPUT_ERROR_TEXT: &str = "m-0 text-00 font-6 leading-0 text-danger";
const INPUT_ERROR_HIDDEN: &str = "hidden";
const INPUT_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const INPUT_OTP_GROUP: &str = "flex flex-wrap items-center gap-2xs";
const INPUT_OTP_SLOT: &str = "grid size-l place-items-center rounded-field border border-border-strong bg-surface-1 text-center text-1 font-7 leading-2 text-text-1 shadow-1 transition-colors focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const INPUT_OTP_SLOT_DENSE: &str = "grid size-s place-items-center rounded-field border border-border-strong bg-surface-1 text-center text-0 font-7 leading-0 text-text-1 shadow-1 transition-colors focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const INPUT_OTP_SLOT_FILLED: &str = "grid size-l place-items-center rounded-field border border-brand bg-primary-soft text-center text-1 font-7 leading-2 text-text-1 shadow-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const INPUT_OTP_SLOT_DENSE_FILLED: &str = "grid size-s place-items-center rounded-field border border-brand bg-primary-soft text-center text-0 font-7 leading-0 text-text-1 shadow-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const INPUT_OTP_SLOT_FOCUSED: &str = "grid size-l place-items-center rounded-field border border-brand bg-surface-1 text-center text-1 font-7 leading-2 text-text-1 shadow-2 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const INPUT_OTP_SLOT_DENSE_FOCUSED: &str = "grid size-s place-items-center rounded-field border border-brand bg-surface-1 text-center text-0 font-7 leading-0 text-text-1 shadow-2 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const INPUT_OTP_SLOT_INVALID: &str = "grid size-l place-items-center rounded-field border border-danger bg-error-soft text-center text-1 font-7 leading-2 text-text-1 shadow-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const INPUT_OTP_SLOT_DENSE_INVALID: &str = "grid size-s place-items-center rounded-field border border-danger bg-error-soft text-center text-0 font-7 leading-0 text-text-1 shadow-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const INPUT_OTP_SLOT_LOADING: &str = "grid size-l place-items-center rounded-field border border-info bg-info-soft text-center text-1 font-7 leading-2 text-text-1 shadow-1";
const INPUT_OTP_SLOT_DISABLED: &str = "grid size-l place-items-center rounded-field border border-border-muted bg-surface-2 text-center text-1 font-7 leading-2 text-text-disabled opacity-disabled";
const INPUT_OTP_SEPARATOR: &str =
    "inline-flex min-h-field items-center px-2xs text-1 font-7 text-text-muted";
const INPUT_OTP_SEPARATOR_DENSE: &str =
    "inline-flex min-h-s items-center px-3xs text-0 font-7 text-text-muted";
const INPUT_OTP_SEPARATOR_HIDDEN: &str = "hidden";
const ITEM_ROOT: &str = "flex w-full max-w-md items-start gap-xs rounded-box border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const ITEM_ROOT_DENSE: &str = "flex w-full max-w-md items-start gap-2xs rounded-field border border-border-subtle bg-surface-1 p-2xs text-text-1 shadow-1";
const ITEM_ROOT_INVALID: &str = "flex w-full max-w-md items-start gap-xs rounded-box border border-danger bg-error-soft p-xs text-text-1 shadow-1";
const ITEM_ROOT_LOADING: &str = "flex w-full max-w-md items-start gap-xs rounded-box border border-info bg-info-soft p-xs text-text-1 shadow-1";
const ITEM_ROOT_DISABLED: &str = "flex w-full max-w-md items-start gap-xs rounded-box border border-border-muted bg-surface-2 p-xs text-text-disabled";
const ITEM_MEDIA: &str = "grid size-xl shrink-0 place-items-center rounded-field border border-border-subtle bg-primary-soft text-00 font-7 text-brand";
const ITEM_MEDIA_DENSE: &str = "grid size-l shrink-0 place-items-center rounded-field border border-border-subtle bg-primary-soft text-00 font-7 text-brand";
const ITEM_MEDIA_DISABLED: &str = "grid size-xl shrink-0 place-items-center rounded-field border border-border-muted bg-surface-3 text-00 font-7 text-text-disabled";
const ITEM_MEDIA_HIDDEN: &str = "hidden";
const ITEM_CONTENT: &str = "grid min-w-0 flex-1 gap-3xs";
const ITEM_CONTENT_DENSE: &str = "grid min-w-0 flex-1 gap-0";
const ITEM_TITLE: &str = "m-0 truncate text-0 font-7 leading-0 text-text-1";
const ITEM_TITLE_DENSE: &str = "m-0 truncate text-00 font-7 leading-0 text-text-1";
const ITEM_TITLE_DISABLED: &str = "m-0 truncate text-0 font-7 leading-0 text-text-disabled";
const ITEM_DESCRIPTION: &str = "m-0 text-00 leading-0 text-text-2";
const ITEM_DESCRIPTION_DENSE: &str = "m-0 text-00 leading-0 text-text-muted";
const ITEM_DESCRIPTION_INVALID: &str = "m-0 text-00 font-6 leading-0 text-danger";
const ITEM_DESCRIPTION_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const ITEM_ACTIONS: &str = "flex shrink-0 flex-wrap items-center justify-end gap-2xs";
const ITEM_ACTIONS_HIDDEN: &str = "hidden";
const ITEM_ACTION: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-7 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ITEM_ACTION_DENSE: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ITEM_ACTION_ACTIVE: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-brand bg-selected-tint px-2xs py-3xs text-00 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const ITEM_ACTION_DISABLED: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-muted bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-disabled opacity-disabled";
const ITEM_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const KBD_ROOT: &str = "inline-flex max-w-full items-center rounded-field border border-border-subtle bg-surface-1 p-2xs text-text-1 shadow-1";
const KBD_ROOT_DENSE: &str = "inline-flex max-w-full items-center rounded-field border border-border-subtle bg-surface-1 p-3xs text-text-1 shadow-1";
const KBD_ROOT_INVALID: &str = "inline-flex max-w-full items-center rounded-field border border-danger bg-error-soft p-2xs text-text-1 shadow-1";
const KBD_ROOT_LOADING: &str = "inline-flex max-w-full items-center rounded-field border border-info bg-info-soft p-2xs text-text-1 shadow-1";
const KBD_ROOT_DISABLED: &str = "inline-flex max-w-full items-center rounded-field border border-border-muted bg-surface-2 p-2xs text-text-disabled";
const KBD_CHORD: &str = "inline-flex max-w-full items-center gap-2xs";
const KBD_CHORD_DENSE: &str = "inline-flex max-w-full items-center gap-3xs";
const KBD_KEY: &str = "inline-flex min-h-s min-w-s items-center justify-center rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs font-mono text-00 font-7 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const KBD_KEY_DENSE: &str = "inline-flex min-h-s min-w-s items-center justify-center rounded-field border border-border-strong bg-surface-2 px-3xs py-3xs font-mono text-00 font-6 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const KBD_KEY_FOCUSED: &str = "inline-flex min-h-s min-w-s items-center justify-center rounded-field border border-brand bg-primary-soft px-2xs py-3xs font-mono text-00 font-7 leading-0 text-text-1 shadow-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const KBD_KEY_INVALID: &str = "inline-flex min-h-s min-w-s items-center justify-center rounded-field border border-danger bg-error-soft px-2xs py-3xs font-mono text-00 font-7 leading-0 text-text-1 shadow-1";
const KBD_KEY_LOADING: &str = "inline-flex min-h-s min-w-s items-center justify-center rounded-field border border-info bg-info-soft px-2xs py-3xs font-mono text-00 font-7 leading-0 text-text-1";
const KBD_KEY_DISABLED: &str = "inline-flex min-h-s min-w-s items-center justify-center rounded-field border border-border-muted bg-surface-3 px-2xs py-3xs font-mono text-00 font-6 leading-0 text-text-disabled opacity-disabled";
const KBD_SEPARATOR: &str = "text-00 font-6 leading-0 text-text-muted";
const KBD_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const LABEL_ROOT: &str =
    "inline-flex max-w-full items-center gap-2xs rounded-field text-text-1 transition-colors";
const LABEL_ROOT_DENSE: &str =
    "inline-flex max-w-full items-center gap-3xs rounded-field text-text-1 transition-colors";
const LABEL_ROOT_ACTIVE: &str = "inline-flex max-w-full items-center gap-2xs rounded-field bg-selected-tint text-text-1 transition-colors";
const LABEL_ROOT_INVALID: &str =
    "inline-flex max-w-full items-center gap-2xs rounded-field text-danger transition-colors";
const LABEL_ROOT_LOADING: &str =
    "inline-flex max-w-full items-center gap-2xs rounded-field text-info transition-colors";
const LABEL_ROOT_DISABLED: &str =
    "inline-flex max-w-full items-center gap-2xs rounded-field text-text-disabled opacity-disabled";
const LABEL_TEXT: &str = "m-0 text-0 font-7 leading-0 text-inherit";
const LABEL_TEXT_DENSE: &str = "m-0 text-00 font-7 leading-0 text-inherit";
const LABEL_TEXT_DISABLED: &str = "m-0 text-0 font-7 leading-0 text-text-disabled";
const LABEL_REQUIREMENT_REQUIRED: &str = "text-00 font-7 leading-0 text-danger";
const LABEL_REQUIREMENT_OPTIONAL: &str = "rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-6 leading-0 text-text-muted";
const LABEL_REQUIREMENT_DENSE_REQUIRED: &str = "text-00 font-7 leading-0 text-danger";
const LABEL_REQUIREMENT_DENSE_OPTIONAL: &str = "rounded-pill border border-border-subtle bg-surface-2 px-3xs py-3xs text-00 font-6 leading-0 text-text-muted";
const LABEL_REQUIREMENT_HIDDEN: &str = "hidden";
const LABEL_REQUIREMENT_DISABLED: &str = "text-00 font-6 leading-0 text-text-disabled";
const LABEL_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";

#[derive(Clone)]
struct WidgetViewNode {
    part: String,
    kind: UiWidgetSlotKind,
    label: String,
    value: String,
    intent: UiWidgetIntent,
    selected: bool,
    disabled: bool,
}

impl<P: CatalogComponentPart> From<CatalogComponentRenderNode<P>> for WidgetViewNode {
    fn from(node: CatalogComponentRenderNode<P>) -> Self {
        Self {
            part: node.part.label().to_owned(),
            kind: node.kind,
            label: node.label,
            value: node.value,
            intent: node.intent,
            selected: node.selected,
            disabled: node.disabled,
        }
    }
}

#[component]
pub fn HealthCard(title: &'static str, body: &'static str) -> impl IntoView {
    view! {
        <section class=HEALTH_CARD>
            <p class=HEALTH_CARD_EYEBROW>"rs-dean"</p>
            <h2 class=HEALTH_CARD_TITLE>{title}</h2>
            <p class=HEALTH_CARD_BODY>{body}</p>
        </section>
    }
}

#[component]
pub fn ShadcnComponentGallery() -> impl IntoView {
    view! {
        <section class=COMPONENT_GALLERY data-story-id="shadcn-component-gallery">
            {UiComponentId::ALL
                .into_iter()
                .map(|id| view! { <ShadcnComponentPreview id /> })
                .collect_view()}
        </section>
    }
}

#[component]
pub fn ShadcnComponentPreview(id: UiComponentId) -> impl IntoView {
    let spec = component_spec(id);
    let implementation = component_implementation(id);
    let definition = spec.definition;
    let category = definition.category.label();
    let framework = definition.framework.label();
    let state = definition.state.label();
    let slug = definition.slug;
    let name = definition.name;
    let summary = definition.summary;

    view! {
        <article class=COMPONENT_CARD data-component-id=slug>
            <header class=COMPONENT_HEADER>
                <div class=COMPONENT_META>
                    <span class=COMPONENT_PILL>{category}</span>
                    <span class=COMPONENT_PILL>{framework}</span>
                    <span class=COMPONENT_PILL>{state}</span>
                </div>
                <h2 class=COMPONENT_TITLE>{name}</h2>
                <p class=COMPONENT_SUMMARY>{summary}</p>
            </header>
            <div class=COMPONENT_BLOCKS>
                {spec.blocks.into_iter().map(component_block).collect_view()}
            </div>
            {implementation_recipe(implementation)}
            <section class=COMPONENT_DEMO>
                <h3 class=COMPONENT_RECIPE_TITLE>"Live component"</h3>
                <ComponentDemo id />
            </section>
        </article>
    }
}

fn implementation_recipe(implementation: ComponentImplementation) -> impl IntoView {
    view! {
        <section class=COMPONENT_RECIPE>
            <h3 class=COMPONENT_RECIPE_TITLE>"Implementation"</h3>
            <p class=BLOCK_DETAIL>{implementation.end_user_outcome}</p>
            <p class=BLOCK_DETAIL>{implementation.consumer_contract}</p>
            <div class=COMPONENT_META>
                <span class=COMPONENT_PILL>{implementation.maturity.label()}</span>
                <span class=COMPONENT_PILL>{implementation.render.label()}</span>
                <span class=COMPONENT_PILL>{implementation.state.label()}</span>
                <span class=COMPONENT_PILL>{implementation.layout.label()}</span>
            </div>
            <TokenList title="Variants" values=implementation.variants />
            <TokenList title="States" values=implementation.states />
            <TokenList title="Accessibility" values=implementation.accessibility />
        </section>
    }
}

#[component]
fn TokenList(title: &'static str, values: &'static [&'static str]) -> impl IntoView {
    view! {
        <div class="grid gap-2xs">
            <h4 class=COMPONENT_RECIPE_TITLE>{title}</h4>
            <div class=COMPONENT_TAG_ROW>
                {values
                    .iter()
                    .map(|value| view! { <span class=COMPONENT_TAG>{*value}</span> })
                    .collect_view()}
            </div>
        </div>
    }
}

fn component_block(block: UiBlock) -> impl IntoView {
    view! {
        <div class=block_class(block.tone) data-ui-role=block.role.slug()>
            <p class=BLOCK_LABEL>{block.label}</p>
            <p class=BLOCK_DETAIL>{block.detail}</p>
        </div>
    }
}

fn block_class(tone: UiBlockTone) -> &'static str {
    match tone {
        UiBlockTone::Accent => {
            "grid min-h-field gap-3xs rounded-field border border-accent bg-accent-soft p-s"
        }
        UiBlockTone::Brand => {
            "grid min-h-field gap-3xs rounded-field border border-brand bg-primary-soft p-s"
        }
        UiBlockTone::Danger => {
            "grid min-h-field gap-3xs rounded-field border border-danger bg-error-soft p-s"
        }
        UiBlockTone::Info => {
            "grid min-h-field gap-3xs rounded-field border border-info bg-info-soft p-s"
        }
        UiBlockTone::Muted => {
            "grid min-h-field gap-3xs rounded-field border border-border-subtle bg-surface-2 p-s"
        }
        UiBlockTone::Surface => {
            "grid min-h-field gap-3xs rounded-field border border-border-subtle bg-surface-1 p-s"
        }
        UiBlockTone::Success => {
            "grid min-h-field gap-3xs rounded-field border border-success bg-success-soft p-s"
        }
        UiBlockTone::Warning => {
            "grid min-h-field gap-3xs rounded-field border border-warning bg-warning-soft p-s"
        }
    }
}

#[component]
pub fn ComponentDemo(id: UiComponentId) -> AnyView {
    match id {
        UiComponentId::Accordion => view! { <Accordion /> }.into_any(),
        UiComponentId::Alert => view! { <Alert /> }.into_any(),
        UiComponentId::AlertDialog => view! { <AlertDialog /> }.into_any(),
        UiComponentId::AspectRatio => view! { <AspectRatio /> }.into_any(),
        UiComponentId::Attachment => view! { <Attachment /> }.into_any(),
        UiComponentId::Avatar => view! { <Avatar /> }.into_any(),
        UiComponentId::Badge => view! { <Badge /> }.into_any(),
        UiComponentId::Breadcrumb => view! { <Breadcrumb /> }.into_any(),
        UiComponentId::Bubble => view! { <Bubble /> }.into_any(),
        UiComponentId::Button => view! { <Button /> }.into_any(),
        UiComponentId::ButtonGroup => view! { <ButtonGroup /> }.into_any(),
        UiComponentId::Calendar => view! { <Calendar /> }.into_any(),
        UiComponentId::Card => view! { <Card /> }.into_any(),
        UiComponentId::Carousel => view! { <Carousel /> }.into_any(),
        UiComponentId::Chart => view! { <Chart /> }.into_any(),
        UiComponentId::Checkbox => view! { <Checkbox /> }.into_any(),
        UiComponentId::Collapsible => view! { <Collapsible /> }.into_any(),
        UiComponentId::Combobox => view! { <Combobox /> }.into_any(),
        UiComponentId::Command => view! { <Command /> }.into_any(),
        UiComponentId::ContextMenu => view! { <ContextMenu /> }.into_any(),
        UiComponentId::DataTable => view! { <DataTable /> }.into_any(),
        UiComponentId::DatePicker => view! { <DatePicker /> }.into_any(),
        UiComponentId::Dialog => view! { <Dialog /> }.into_any(),
        UiComponentId::Direction => view! { <Direction /> }.into_any(),
        UiComponentId::Drawer => view! { <Drawer /> }.into_any(),
        UiComponentId::DropdownMenu => view! { <DropdownMenu /> }.into_any(),
        UiComponentId::Empty => view! { <Empty /> }.into_any(),
        UiComponentId::Field => view! { <Field /> }.into_any(),
        UiComponentId::HoverCard => view! { <HoverCard /> }.into_any(),
        UiComponentId::Input => view! { <Input /> }.into_any(),
        UiComponentId::InputGroup => view! { <InputGroup /> }.into_any(),
        UiComponentId::InputOtp => view! { <InputOtp /> }.into_any(),
        UiComponentId::Item => view! { <Item /> }.into_any(),
        UiComponentId::Kbd => view! { <Kbd /> }.into_any(),
        UiComponentId::Label => view! { <Label /> }.into_any(),
        UiComponentId::Marker => view! { <Marker /> }.into_any(),
        UiComponentId::Menubar => view! { <Menubar /> }.into_any(),
        UiComponentId::Message => view! { <Message /> }.into_any(),
        UiComponentId::MessageScroller => view! { <MessageScroller /> }.into_any(),
        UiComponentId::NativeSelect => view! { <NativeSelect /> }.into_any(),
        UiComponentId::NavigationMenu => view! { <NavigationMenu /> }.into_any(),
        UiComponentId::Pagination => view! { <Pagination /> }.into_any(),
        UiComponentId::Popover => view! { <Popover /> }.into_any(),
        UiComponentId::Progress => view! { <Progress /> }.into_any(),
        UiComponentId::RadioGroup => view! { <RadioGroup /> }.into_any(),
        UiComponentId::Resizable => view! { <Resizable /> }.into_any(),
        UiComponentId::ScrollArea => view! { <ScrollArea /> }.into_any(),
        UiComponentId::Select => view! { <Select /> }.into_any(),
        UiComponentId::Separator => view! { <Separator /> }.into_any(),
        UiComponentId::Sheet => view! { <Sheet /> }.into_any(),
        UiComponentId::Sidebar => view! { <Sidebar /> }.into_any(),
        UiComponentId::Skeleton => view! { <Skeleton /> }.into_any(),
        UiComponentId::Slider => view! { <Slider /> }.into_any(),
        UiComponentId::Sonner => view! { <Sonner /> }.into_any(),
        UiComponentId::Spinner => view! { <Spinner /> }.into_any(),
        UiComponentId::Switch => view! { <Switch /> }.into_any(),
        UiComponentId::Table => view! { <Table /> }.into_any(),
        UiComponentId::Tabs => view! { <Tabs /> }.into_any(),
        UiComponentId::Textarea => view! { <Textarea /> }.into_any(),
        UiComponentId::Toast => view! { <Toast /> }.into_any(),
        UiComponentId::Toggle => view! { <Toggle /> }.into_any(),
        UiComponentId::ToggleGroup => view! { <ToggleGroup /> }.into_any(),
        UiComponentId::Tooltip => view! { <Tooltip /> }.into_any(),
        UiComponentId::Typography => view! { <Typography /> }.into_any(),
    }
}

fn render_catalog_component<P: CatalogComponentPart>(model: CatalogComponentModel<P>) -> AnyView {
    let id = P::ID;
    let slug = id.definition().slug;
    let pattern = model.pattern.label();
    let nodes = match catalog_component_render_nodes(&model) {
        Ok(nodes) => nodes,
        Err(report) => {
            let message = format!("{} validation failed: {report}", id.definition().name);
            return view! {
                <section class=WIDGET_SHELL data-ui-widget=slug data-ui-state="invalid">
                    <p class=WIDGET_ERROR role="alert">{message}</p>
                </section>
            }
            .into_any();
        }
    };
    let root = nodes
        .first()
        .cloned()
        .map(WidgetViewNode::from)
        .expect("invariant: validated catalog components always include a root node");
    let content_class = content_class(model.pattern);
    let slots = nodes
        .into_iter()
        .skip(1)
        .map(WidgetViewNode::from)
        .collect::<Vec<_>>();

    view! {
        <section class=widget_class(model.pattern) data-ui-widget=slug data-ui-pattern=pattern>
            <header class=WIDGET_HEADER>
                <p class=WIDGET_EYEBROW>{pattern}</p>
                <h3 class=WIDGET_TITLE>{root.label}</h3>
                <p class=WIDGET_SUMMARY>{root.value}</p>
            </header>
            <div class=content_class>
                {slots.into_iter().map(slot_view).collect_view()}
            </div>
        </section>
    }
    .into_any()
}

fn slot_view(slot: WidgetViewNode) -> AnyView {
    let part = slot.part.clone();
    let kind = slot.kind.label();
    let intent = slot.intent.label();
    let selected = slot.selected.to_string();
    let disabled = slot.disabled;
    let slot_class = slot_class(&slot);

    match slot.kind {
        UiWidgetSlotKind::Avatar => view! {
            <div class=WIDGET_SLOT data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_AVATAR aria-hidden="true">{slot.label.clone()}</span>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Badge => view! {
            <span class=WIDGET_BADGE data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                {slot.label.clone()}
            </span>
        }.into_any(),
        UiWidgetSlotKind::Button | UiWidgetSlotKind::IconButton => view! {
            <button type="button" class=button_class(&slot) data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-pressed=selected disabled=disabled>
                {slot.label.clone()}
            </button>
        }.into_any(),
        UiWidgetSlotKind::Cell => view! {
            <div role="cell" class=WIDGET_TABLE_CELL data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                {slot.label.clone()}
            </div>
        }.into_any(),
        UiWidgetSlotKind::Chart => view! {
            <div class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <p class=WIDGET_LABEL>{slot.label.clone()}</p>
                <div class="flex items-end gap-2xs" aria-hidden="true">
                    <span class="h-xs w-s rounded-field bg-brand"></span>
                    <span class="h-m w-s rounded-field bg-success"></span>
                    <span class="h-l w-s rounded-field bg-accent"></span>
                </div>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Checkbox => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <input type="checkbox" class="size-s rounded-selector border border-border-strong accent-brand" checked=slot.selected disabled=disabled />
                <span class=WIDGET_VALUE>{slot.value.clone()}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Description | UiWidgetSlotKind::Text => view! {
            <div class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <p class=WIDGET_LABEL>{slot.label.clone()}</p>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Header | UiWidgetSlotKind::Title => view! {
            <div class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <h4 class=WIDGET_TITLE>{slot.label.clone()}</h4>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Input => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <input class=WIDGET_INPUT value=slot.value.clone() aria-label=slot.label.clone() disabled=disabled />
            </label>
        }.into_any(),
        UiWidgetSlotKind::Key => view! {
            <kbd class=WIDGET_KEY data-ui-part=part data-ui-kind=kind data-ui-intent=intent>{slot.label.clone()}</kbd>
        }.into_any(),
        UiWidgetSlotKind::Link => view! {
            <a class=button_class(&slot) href="#" data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-current=selected>
                {slot.label.clone()}
            </a>
        }.into_any(),
        UiWidgetSlotKind::List => view! {
            <div role="list" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <p class=WIDGET_LABEL>{slot.label.clone()}</p>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::ListItem | UiWidgetSlotKind::Option => view! {
            <div role="option" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                <p class=WIDGET_LABEL>{slot.label.clone()}</p>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Marker => view! {
            <span class=WIDGET_MARKER data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                {slot.label.clone()}
            </span>
        }.into_any(),
        UiWidgetSlotKind::Media => view! {
            <figure class=WIDGET_MEDIA data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span>{slot.label.clone()}</span>
                <figcaption class=WIDGET_VALUE>{slot.value.clone()}</figcaption>
            </figure>
        }.into_any(),
        UiWidgetSlotKind::Overlay | UiWidgetSlotKind::Panel | UiWidgetSlotKind::Section => view! {
            <section class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                <p class=WIDGET_LABEL>{slot.label.clone()}</p>
                <p class=WIDGET_VALUE>{slot.value.clone()}</p>
            </section>
        }.into_any(),
        UiWidgetSlotKind::Progress => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <progress class=WIDGET_PROGRESS value="64" max="100">{slot.value.clone()}</progress>
                <span class=WIDGET_VALUE>{slot.value.clone()}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Radio => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <input type="radio" class="size-s border border-border-strong accent-brand" checked=slot.selected disabled=disabled />
                <span class=WIDGET_VALUE>{slot.value.clone()}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Row => view! {
            <div role="row" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-selected=selected>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <span class=WIDGET_VALUE>{slot.value.clone()}</span>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Select => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <select class=WIDGET_INPUT aria-label=slot.label.clone() disabled=disabled>
                    <option selected=slot.selected>{slot.value.clone()}</option>
                    <option>"Light"</option>
                    <option>"Dark"</option>
                </select>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Separator => view! {
            <div class="grid gap-2xs" data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_SEPARATOR role="separator" aria-label=slot.value.clone()></span>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Skeleton => view! {
            <div class="grid gap-2xs" data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-busy="true">
                <span class=WIDGET_SKELETON></span>
                <span class="h-xs rounded-field bg-surface-3"></span>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Slider => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <input type="range" class=WIDGET_RANGE min="0" max="100" value="72" aria-label=slot.label.clone() disabled=disabled />
                <span class=WIDGET_VALUE>{slot.value.clone()}</span>
            </label>
        }.into_any(),
        UiWidgetSlotKind::Spinner => view! {
            <div role="status" class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_SPINNER aria-hidden="true"></span>
                <span class=WIDGET_VALUE>{slot.label.clone()}</span>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Switch => view! {
            <button type="button" role="switch" class=button_class(&slot) data-ui-part=part data-ui-kind=kind data-ui-intent=intent aria-checked=selected disabled=disabled>
                {slot.label.clone()}
            </button>
        }.into_any(),
        UiWidgetSlotKind::Table => view! {
            <div role="table" class=WIDGET_TABLE data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <div role="row" class="grid grid-cols-2">
                    <span role="cell" class=WIDGET_TABLE_CELL>{slot.label.clone()}</span>
                    <span role="cell" class=WIDGET_TABLE_CELL>{slot.value.clone()}</span>
                </div>
            </div>
        }.into_any(),
        UiWidgetSlotKind::Textarea => view! {
            <label class=slot_class data-ui-part=part data-ui-kind=kind data-ui-intent=intent>
                <span class=WIDGET_LABEL>{slot.label.clone()}</span>
                <textarea class=WIDGET_TEXTAREA aria-label=slot.label.clone() disabled=disabled>{slot.value.clone()}</textarea>
            </label>
        }.into_any(),
    }
}

fn widget_class(pattern: UiWidgetPattern) -> &'static str {
    match pattern {
        UiWidgetPattern::Action | UiWidgetPattern::Navigation => {
            "grid min-w-0 gap-s rounded-field bg-surface-1 p-s text-text-1"
        }
        UiWidgetPattern::Overlay => {
            "grid min-w-0 gap-s rounded-field bg-surface-elevated p-s text-text-1 shadow-2"
        }
        UiWidgetPattern::Callout | UiWidgetPattern::Feedback => {
            "grid min-w-0 gap-s rounded-field bg-warning-soft p-s text-text-1"
        }
        UiWidgetPattern::Data
        | UiWidgetPattern::Disclosure
        | UiWidgetPattern::Display
        | UiWidgetPattern::Form
        | UiWidgetPattern::Layout
        | UiWidgetPattern::Messaging
        | UiWidgetPattern::Typography
        | UiWidgetPattern::Utility => WIDGET_SHELL,
    }
}

fn content_class(pattern: UiWidgetPattern) -> &'static str {
    match pattern {
        UiWidgetPattern::Action | UiWidgetPattern::Navigation => WIDGET_CONTENT_INLINE,
        UiWidgetPattern::Data => "grid gap-2xs overflow-hidden",
        _ => WIDGET_CONTENT,
    }
}

fn slot_class(slot: &WidgetViewNode) -> &'static str {
    if slot.selected {
        WIDGET_SLOT_ACTIVE
    } else {
        WIDGET_SLOT
    }
}

fn button_class(slot: &WidgetViewNode) -> &'static str {
    if slot.selected {
        WIDGET_BUTTON_ACTIVE
    } else {
        WIDGET_BUTTON
    }
}

macro_rules! catalog_component {
    ($name:ident, $model:ty, $default:path) => {
        #[component]
        pub fn $name(#[prop(optional, default = $default())] model: $model) -> AnyView {
            render_catalog_component(model)
        }
    };
}

#[component]
pub fn Alert(#[prop(optional, default = default_alert_model())] model: AlertModel) -> AnyView {
    if let Err(report) = validate_alert_model(&model) {
        let message = format!("Alert validation failed: {report}");
        return view! {
            <section class=ALERT_STANDARD_DESTRUCTIVE data-ui-component="alert" data-ui-state="invalid" role="alert">
                <p class=ALERT_ERROR>{message}</p>
            </section>
        }
        .into_any();
    }

    let root_class = alert_root_class(model.tone, model.density);
    let marker_class = alert_marker_class(model.tone);
    let marker_label = alert_marker_label(model.tone);
    let title_class = alert_title_class(model.density);
    let description_class = alert_description_class(model.density);
    let role = alert_role(model.tone);
    let state = alert_state(&model);
    let title = model.title;
    let description = model.description;
    let tone = model.tone.label();
    let density = model.density.label();
    let loading = model.loading;
    let disabled = model.disabled;
    let action = model.action;

    view! {
        <section
            class=root_class
            data-ui-component="alert"
            data-ui-part="Alert"
            data-ui-tone=tone
            data-ui-density=density
            data-ui-state=state
            role=role
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
        >
            <div class=ALERT_ROW>
                <span class=marker_class aria-hidden="true">{marker_label}</span>
                <div class=ALERT_BODY>
                    <h3 class=title_class data-ui-part="AlertTitle">{title}</h3>
                    <p class=description_class data-ui-part="AlertDescription">{description}</p>
                </div>
                {if let Some(action) = action {
                    let action_disabled = disabled || loading || action.disabled;
                    let action_state = if action_disabled { "disabled" } else { "ready" };
                    let label = if loading {
                        "Loading".to_owned()
                    } else {
                        action.label
                    };

                    view! {
                        <button
                            type="button"
                            class=ALERT_ACTION
                            data-ui-part="AlertAction"
                            data-ui-intent="activate"
                            data-ui-action=action.value
                            data-ui-state=action_state
                            disabled=action_disabled
                        >
                            {label}
                        </button>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }}
            </div>
        </section>
    }
    .into_any()
}

fn alert_root_class(tone: AlertTone, density: AlertDensity) -> &'static str {
    match (density, tone) {
        (AlertDensity::Standard, AlertTone::Default) => ALERT_STANDARD_DEFAULT,
        (AlertDensity::Standard, AlertTone::Info) => ALERT_STANDARD_INFO,
        (AlertDensity::Standard, AlertTone::Success) => ALERT_STANDARD_SUCCESS,
        (AlertDensity::Standard, AlertTone::Warning) => ALERT_STANDARD_WARNING,
        (AlertDensity::Standard, AlertTone::Destructive) => ALERT_STANDARD_DESTRUCTIVE,
        (AlertDensity::Dense, AlertTone::Default) => ALERT_DENSE_DEFAULT,
        (AlertDensity::Dense, AlertTone::Info) => ALERT_DENSE_INFO,
        (AlertDensity::Dense, AlertTone::Success) => ALERT_DENSE_SUCCESS,
        (AlertDensity::Dense, AlertTone::Warning) => ALERT_DENSE_WARNING,
        (AlertDensity::Dense, AlertTone::Destructive) => ALERT_DENSE_DESTRUCTIVE,
    }
}

const fn alert_marker_class(tone: AlertTone) -> &'static str {
    match tone {
        AlertTone::Default => ALERT_MARKER_DEFAULT,
        AlertTone::Info => ALERT_MARKER_INFO,
        AlertTone::Success => ALERT_MARKER_SUCCESS,
        AlertTone::Warning => ALERT_MARKER_WARNING,
        AlertTone::Destructive => ALERT_MARKER_DESTRUCTIVE,
    }
}

const fn alert_marker_label(tone: AlertTone) -> &'static str {
    match tone {
        AlertTone::Default | AlertTone::Info => "i",
        AlertTone::Success => "+",
        AlertTone::Warning | AlertTone::Destructive => "!",
    }
}

const fn alert_title_class(density: AlertDensity) -> &'static str {
    match density {
        AlertDensity::Standard => ALERT_TITLE,
        AlertDensity::Dense => ALERT_TITLE_DENSE,
    }
}

const fn alert_description_class(density: AlertDensity) -> &'static str {
    match density {
        AlertDensity::Standard => ALERT_DESCRIPTION,
        AlertDensity::Dense => ALERT_DESCRIPTION_DENSE,
    }
}

const fn alert_role(tone: AlertTone) -> &'static str {
    match tone {
        AlertTone::Warning | AlertTone::Destructive => "alert",
        AlertTone::Default | AlertTone::Info | AlertTone::Success => "status",
    }
}

const fn alert_state(model: &AlertModel) -> &'static str {
    if model.loading {
        "loading"
    } else if model.disabled {
        "disabled"
    } else {
        "ready"
    }
}

#[component]
pub fn AlertDialog(
    #[prop(optional, default = default_alert_dialog_model())] model: AlertDialogModel,
    #[prop(optional, default = false)] default_open: bool,
) -> AnyView {
    if let Err(report) = validate_alert_dialog_model(&model) {
        let message = format!("AlertDialog validation failed: {report}");
        return view! {
            <section class=ALERT_DIALOG_ROOT data-ui-component="alert-dialog" data-ui-state="invalid">
                <p class=ALERT_DIALOG_ERROR role="alert">{message}</p>
            </section>
        }
        .into_any();
    }

    let initial_state = if default_open {
        AlertDialogState::open()
    } else {
        AlertDialogState::closed()
    };
    let (state, set_state) = signal(initial_state);
    let is_open = Memo::new(move |_| state.with(|state| state.is_open()));
    let trigger_label = model.trigger_label;
    let title = model.title;
    let description = model.description;
    let title_id = alert_dialog_dom_id("alert-dialog-title", &model.action.value);
    let description_id = alert_dialog_dom_id("alert-dialog-description", &model.action.value);
    let content_class = alert_dialog_content_class(model.size, model.destructive);
    let title_class = alert_dialog_title_class(model.size);
    let description_class = alert_dialog_description_class(model.size);
    let action_class = alert_dialog_action_class(model.destructive);
    let action_value = model.action.value;
    let action_label = if model.loading {
        "Working".to_owned()
    } else {
        model.action.label
    };
    let action_disabled = model.disabled || model.loading || model.action.disabled;
    let cancel_value = model.cancel.value;
    let cancel_label = model.cancel.label;
    let cancel_disabled = model.disabled || model.loading || model.cancel.disabled;
    let disabled = model.disabled;
    let loading = model.loading;
    let size = model.size.label();
    let tone = if model.destructive {
        "destructive"
    } else {
        "default"
    };
    let action_state = if action_disabled { "disabled" } else { "ready" };
    let cancel_state = if cancel_disabled { "disabled" } else { "ready" };

    let open_dialog = move |_| {
        if disabled {
            return;
        }
        set_state.update(|state| {
            let _ = state.apply(AlertDialogIntent::Open);
        });
    };
    let is_open_for_trigger = is_open;
    let is_open_for_root = is_open;
    let is_open_for_overlay = is_open;

    view! {
        <section
            class=ALERT_DIALOG_ROOT
            data-ui-component="alert-dialog"
            data-ui-size=size
            data-ui-tone=tone
            data-ui-state=move || {
                if is_open_for_root.get() {
                    "open"
                } else if disabled {
                    "disabled"
                } else {
                    "closed"
                }
            }
            aria-busy=loading.to_string()
        >
            <button
                type="button"
                class=ALERT_DIALOG_TRIGGER
                data-ui-part="AlertDialogTrigger"
                aria-haspopup="dialog"
                aria-expanded=move || is_open_for_trigger.get().to_string()
                disabled=disabled
                on:click=open_dialog
            >
                {trigger_label}
            </button>
            {move || {
                if is_open_for_overlay.get() {
                    let action_value_for_click = action_value.clone();
                    let confirm_dialog = move |_| {
                        if action_disabled {
                            return;
                        }
                        set_state.update(|state| {
                            let _ = state.apply(AlertDialogIntent::Confirm(
                                action_value_for_click.clone(),
                            ));
                        });
                    };
                    let cancel_dialog = move |_| {
                        if cancel_disabled {
                            return;
                        }
                        set_state.update(|state| {
                            let _ = state.apply(AlertDialogIntent::Cancel);
                        });
                    };

                    view! {
                        <div class=ALERT_DIALOG_OVERLAY data-ui-part="AlertDialog">
                            <section
                                class=content_class
                                role="dialog"
                                aria-modal="true"
                                aria-labelledby=title_id.clone()
                                aria-describedby=description_id.clone()
                                data-ui-part="AlertDialogContent"
                            >
                                <header class=ALERT_DIALOG_HEADER data-ui-part="AlertDialogHeader">
                                    <h3 id=title_id.clone() class=title_class>{title.clone()}</h3>
                                    <p id=description_id.clone() class=description_class>{description.clone()}</p>
                                </header>
                                <footer class=ALERT_DIALOG_FOOTER data-ui-part="AlertDialogFooter">
                                    <button
                                        type="button"
                                        class=ALERT_DIALOG_CANCEL
                                        data-ui-part="AlertDialogCancel"
                                        data-ui-intent="close"
                                        data-ui-action=cancel_value.clone()
                                        data-ui-state=cancel_state
                                        disabled=cancel_disabled
                                        on:click=cancel_dialog
                                    >
                                        {cancel_label.clone()}
                                    </button>
                                    <button
                                        type="button"
                                        class=action_class
                                        data-ui-part="AlertDialogAction"
                                        data-ui-intent="activate"
                                        data-ui-action=action_value.clone()
                                        data-ui-state=action_state
                                        disabled=action_disabled
                                        on:click=confirm_dialog
                                    >
                                        {action_label.clone()}
                                    </button>
                                </footer>
                            </section>
                        </div>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}
        </section>
    }
    .into_any()
}

const fn alert_dialog_content_class(size: AlertDialogSize, destructive: bool) -> &'static str {
    match (size, destructive) {
        (AlertDialogSize::Default, false) => ALERT_DIALOG_CONTENT,
        (AlertDialogSize::Default, true) => ALERT_DIALOG_CONTENT_DESTRUCTIVE,
        (AlertDialogSize::Small, false) => ALERT_DIALOG_CONTENT_SMALL,
        (AlertDialogSize::Small, true) => ALERT_DIALOG_CONTENT_SMALL_DESTRUCTIVE,
    }
}

const fn alert_dialog_title_class(size: AlertDialogSize) -> &'static str {
    match size {
        AlertDialogSize::Default => ALERT_DIALOG_TITLE,
        AlertDialogSize::Small => ALERT_DIALOG_TITLE_SMALL,
    }
}

const fn alert_dialog_description_class(size: AlertDialogSize) -> &'static str {
    match size {
        AlertDialogSize::Default => ALERT_DIALOG_DESCRIPTION,
        AlertDialogSize::Small => ALERT_DIALOG_DESCRIPTION_SMALL,
    }
}

const fn alert_dialog_action_class(destructive: bool) -> &'static str {
    if destructive {
        ALERT_DIALOG_ACTION_DESTRUCTIVE
    } else {
        ALERT_DIALOG_ACTION
    }
}

#[component]
pub fn Accordion(
    #[prop(optional, default = default_accordion_items())] items: Vec<AccordionItem>,
    #[prop(optional, default = AccordionMode::Single)] mode: AccordionMode,
    #[prop(optional, default = vec!["tokens".to_owned()])] default_open: Vec<String>,
) -> AnyView {
    let model = AccordionModel::new(mode, items).with_default_open(default_open);
    if let Err(report) = validate_accordion_model(&model) {
        let message = format!("Accordion validation failed: {report}");
        return view! {
            <section class=ACCORDION_ROOT data-ui-component="accordion" data-ui-state="invalid">
                <p class=ACCORDION_ERROR role="alert">{message}</p>
            </section>
        }
        .into_any();
    }

    let (state, set_state) = signal(model.state());
    let items = model.items;

    view! {
        <section class=ACCORDION_ROOT data-ui-component="accordion" data-ui-mode=mode.label()>
            {if items.is_empty() {
                view! {
                    <p class=ACCORDION_EMPTY data-ui-part="AccordionEmpty">
                        "No accordion sections are available."
                    </p>
                }
                    .into_any()
            } else {
                items
                    .into_iter()
                    .map(move |item| {
                        let value = item.value.clone();
                        let title = item.title;
                        let content = item.content;
                        let disabled = item.disabled;
                        let trigger_id = accordion_dom_id("accordion-trigger", &value);
                        let panel_id = accordion_dom_id("accordion-panel", &value);
                        let open_value = value.clone();
                        let is_open = Memo::new(move |_| {
                            state.with(|state| state.is_open(&open_value))
                        });
                        let toggle_value = value.clone();
                        let on_click = move |_| {
                            if disabled {
                                return;
                            }
                            set_state.update(|state| {
                                let _ = state.apply(AccordionIntent::Toggle(toggle_value.clone()));
                            });
                        };
                        let is_open_for_item = is_open;
                        let is_open_for_trigger = is_open;
                        let is_open_for_indicator = is_open;
                        let is_open_for_hidden = is_open;

                        view! {
                            <article
                                class=move || {
                                    if is_open_for_item.get() {
                                        ACCORDION_ITEM_OPEN
                                    } else {
                                        ACCORDION_ITEM
                                    }
                                }
                                data-ui-part="AccordionItem"
                                data-value=value
                                data-state=move || {
                                    if is_open_for_trigger.get() {
                                        "open"
                                    } else {
                                        "closed"
                                    }
                                }
                            >
                                <h3 class="m-0">
                                    <button
                                        id=trigger_id.clone()
                                        type="button"
                                        class=move || {
                                            if is_open_for_trigger.get() {
                                                ACCORDION_TRIGGER_OPEN
                                            } else {
                                                ACCORDION_TRIGGER
                                            }
                                        }
                                        aria-expanded=move || {
                                            is_open_for_trigger.get().to_string()
                                        }
                                        aria-controls=panel_id.clone()
                                        aria-disabled=disabled.to_string()
                                        disabled=disabled
                                        on:click=on_click
                                    >
                                        <span>{title}</span>
                                        <span class=ACCORDION_INDICATOR aria-hidden="true">
                                            {move || if is_open_for_indicator.get() { "-" } else { "+" }}
                                        </span>
                                    </button>
                                </h3>
                                <div
                                    id=panel_id
                                    role="region"
                                    class=ACCORDION_CONTENT
                                    aria-labelledby=trigger_id
                                    hidden=move || !is_open_for_hidden.get()
                                    data-ui-part="AccordionContent"
                                >
                                    <p class="m-0">{content}</p>
                                </div>
                            </article>
                        }
                    })
                    .collect_view()
                    .into_any()
            }}
        </section>
    }
    .into_any()
}

#[component]
pub fn AspectRatio(
    #[prop(optional, default = default_aspect_ratio_model())] model: AspectRatioModel,
) -> AnyView {
    if let Err(report) = validate_aspect_ratio_model(&model) {
        let message = format!("AspectRatio validation failed: {report}");
        return view! {
            <section class=ASPECT_RATIO_ROOT data-ui-component="aspect-ratio" data-ui-state="invalid">
                <p class=ASPECT_RATIO_ERROR role="alert">{message}</p>
            </section>
        }
        .into_any();
    }

    let nodes = aspect_ratio_render_nodes(&model);
    let frame = nodes
        .iter()
        .find(|node| node.part == AspectRatioPart::Frame)
        .expect("invariant: aspect ratio render nodes include frame");
    let media = nodes
        .iter()
        .find(|node| node.part == AspectRatioPart::Media)
        .expect("invariant: aspect ratio render nodes include media");
    let frame_label = frame.label.clone();
    let frame_detail = frame.detail.clone();
    let media_label = media.label.clone();
    let media_detail = media.detail.clone();
    let ratio_label = model.ratio_label();
    let frame_style = model.aspect_ratio_style();
    let fit = model.fit.label();
    let media_class = aspect_ratio_media_class(model.fit, model.loading, model.disabled);
    let state = aspect_ratio_state(&model);
    let loading = model.loading;
    let disabled = model.disabled;

    view! {
        <section
            class=ASPECT_RATIO_ROOT
            data-ui-component="aspect-ratio"
            data-ui-state=state
            data-ui-ratio=ratio_label.clone()
            data-ui-fit=fit
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
        >
            <figure class=ASPECT_RATIO_FIGURE data-ui-part="AspectRatio">
                <div
                    class=ASPECT_RATIO_FRAME
                    style=frame_style
                    data-ui-part="AspectRatioFrame"
                    data-ui-ratio=ratio_label.clone()
                    aria-label=frame_label.clone()
                >
                    <div class=media_class data-ui-part="AspectRatioMedia" data-ui-fit=fit>
                        {if loading {
                            view! {
                                <p class=ASPECT_RATIO_MEDIA_MUTED role="status">
                                    "Loading media"
                                </p>
                            }
                                .into_any()
                        } else {
                            view! {
                                <div class=ASPECT_RATIO_MEDIA_STACK>
                                    <span class=ASPECT_RATIO_MEDIA_MARKER aria-hidden="true">
                                        {ratio_label.clone()}
                                    </span>
                                    <p class=ASPECT_RATIO_MEDIA_LABEL>{media_label}</p>
                                    <p class=ASPECT_RATIO_MEDIA_DETAIL>{media_detail}</p>
                                </div>
                            }
                                .into_any()
                        }}
                    </div>
                </div>
                <figcaption class=ASPECT_RATIO_CAPTION>
                    <span class=ASPECT_RATIO_CAPTION_TEXT>
                        <span class=ASPECT_RATIO_LABEL>{frame_label}</span>
                        <span class=ASPECT_RATIO_DETAIL>{frame_detail}</span>
                    </span>
                    <span class=ASPECT_RATIO_BADGE>{ratio_label.clone()}</span>
                </figcaption>
            </figure>
        </section>
    }
    .into_any()
}

const fn aspect_ratio_media_class(
    fit: AspectRatioFit,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        ASPECT_RATIO_MEDIA_DISABLED
    } else if loading {
        ASPECT_RATIO_MEDIA_LOADING
    } else {
        match fit {
            AspectRatioFit::Cover => ASPECT_RATIO_MEDIA_COVER,
            AspectRatioFit::Contain => ASPECT_RATIO_MEDIA_CONTAIN,
        }
    }
}

const fn aspect_ratio_state(model: &AspectRatioModel) -> &'static str {
    if model.disabled {
        "disabled"
    } else if model.loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Attachment(
    #[prop(optional, default = default_attachment_model())] model: AttachmentModel,
) -> AnyView {
    if let Err(report) = validate_attachment_model(&model) {
        let message = format!("Attachment validation failed: {report}");
        return view! {
            <section class=ATTACHMENT_ROOT data-ui-component="attachment" data-ui-state="invalid">
                <p class=ATTACHMENT_ERROR role="alert">{message}</p>
            </section>
        }
        .into_any();
    }

    let nodes = attachment_render_nodes(&model);
    let preview = nodes
        .iter()
        .find(|node| node.part == AttachmentPart::Preview)
        .expect("invariant: attachment render nodes include preview");
    let title = nodes
        .iter()
        .find(|node| node.part == AttachmentPart::Title)
        .expect("invariant: attachment render nodes include title");
    let meta = nodes
        .iter()
        .find(|node| node.part == AttachmentPart::Meta)
        .expect("invariant: attachment render nodes include metadata");
    let preview_label = preview.label.clone();
    let title_value = title.value.clone();
    let meta_value = meta.value.clone();
    let root_class = attachment_root_class(model.loading, model.disabled);
    let preview_class = attachment_preview_class(model.kind, model.loading, model.disabled);
    let title_class = attachment_title_class(model.disabled);
    let meta_class = attachment_meta_class(model.disabled);
    let state_label = attachment_state(&model);
    let kind = model.kind.label();
    let loading = model.loading;
    let disabled = model.disabled;
    let (state, set_state) = signal(model.state());
    let action = model.action;

    view! {
        <section
            class=root_class
            data-ui-component="attachment"
            data-ui-part="Attachment"
            data-ui-kind=kind
            data-ui-state=state_label
            role="group"
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
        >
            <span class=preview_class data-ui-part="AttachmentPreview" aria-hidden="true">
                {if loading { "...".to_owned() } else { preview_label }}
            </span>
            <span class=ATTACHMENT_BODY>
                <span class=title_class data-ui-part="AttachmentTitle">{title_value}</span>
                <span class=meta_class data-ui-part="AttachmentMeta">
                    {if loading { "Preparing attachment".to_owned() } else { meta_value }}
                </span>
            </span>
            {if let Some(action) = action {
                let action_disabled = disabled || loading || action.disabled;
                let action_label = if loading {
                    "Loading".to_owned()
                } else {
                    action.label
                };
                let action_state = if action_disabled { "disabled" } else { "ready" };
                let action_value = action.value;
                let action_value_for_memo = action_value.clone();
                let action_value_for_click = action_value.clone();
                let is_active = Memo::new(move |_| {
                    state.with(|state| {
                        state.activated_value() == Some(action_value_for_memo.as_str())
                    })
                });
                let is_active_for_class = is_active;
                let is_active_for_pressed = is_active;
                let on_click = move |_| {
                    if action_disabled {
                        return;
                    }
                    set_state.update(|state| {
                        let _ = state.apply(AttachmentIntent::Activate(
                            action_value_for_click.clone(),
                        ));
                    });
                };

                view! {
                    <button
                        type="button"
                        class=move || {
                            if is_active_for_class.get() {
                                ATTACHMENT_ACTION_ACTIVE
                            } else {
                                ATTACHMENT_ACTION
                            }
                        }
                        data-ui-part="AttachmentAction"
                        data-ui-intent="activate"
                        data-ui-action=action_value
                        data-ui-state=action_state
                        aria-pressed=move || is_active_for_pressed.get().to_string()
                        disabled=action_disabled
                        on:click=on_click
                    >
                        {action_label}
                    </button>
                }
                    .into_any()
            } else {
                ().into_any()
            }}
        </section>
    }
    .into_any()
}

const fn attachment_root_class(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        ATTACHMENT_ROOT_DISABLED
    } else if loading {
        ATTACHMENT_ROOT_LOADING
    } else {
        ATTACHMENT_ROOT
    }
}

const fn attachment_preview_class(
    kind: AttachmentKind,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if loading || disabled {
        ATTACHMENT_PREVIEW_MUTED
    } else {
        match kind {
            AttachmentKind::Pdf => ATTACHMENT_PREVIEW_PDF,
            AttachmentKind::Image => ATTACHMENT_PREVIEW_IMAGE,
            AttachmentKind::Archive => ATTACHMENT_PREVIEW_ARCHIVE,
            AttachmentKind::Data => ATTACHMENT_PREVIEW_DATA,
        }
    }
}

const fn attachment_title_class(disabled: bool) -> &'static str {
    if disabled {
        ATTACHMENT_TITLE_DISABLED
    } else {
        ATTACHMENT_TITLE
    }
}

const fn attachment_meta_class(disabled: bool) -> &'static str {
    if disabled {
        ATTACHMENT_META_DISABLED
    } else {
        ATTACHMENT_META
    }
}

const fn attachment_state(model: &AttachmentModel) -> &'static str {
    if model.disabled {
        "disabled"
    } else if model.loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Avatar(#[prop(optional, default = default_avatar_model())] model: AvatarModel) -> AnyView {
    if let Err(report) = validate_avatar_model(&model) {
        let message = format!("Avatar validation failed: {report}");
        return view! {
            <section class=AVATAR_ROOT_MEDIUM data-ui-component="avatar" data-ui-state="invalid">
                <p class=AVATAR_ERROR role="alert">{message}</p>
            </section>
        }
        .into_any();
    }

    let initial_state = model.state();
    let nodes = avatar_render_nodes(&model, initial_state);
    let image = nodes
        .iter()
        .find(|node| node.part == AvatarPart::Image)
        .expect("invariant: avatar render nodes include image");
    let fallback = nodes
        .iter()
        .find(|node| node.part == AvatarPart::Fallback)
        .expect("invariant: avatar render nodes include fallback");
    let image_src = image.value.clone();
    let image_alt = image.label.clone();
    let fallback_value = fallback.value.clone();
    let root_label = model.name;
    let root_class = avatar_root_class(model.size, model.disabled);
    let fallback_class = avatar_fallback_class(model.size, model.loading, model.disabled);
    let size = model.size.label();
    let loading = model.loading;
    let disabled = model.disabled;
    let has_image = model.image.is_some();
    let state_label = avatar_state_label(loading, disabled);
    let (state, set_state) = signal(initial_state);
    let visual = Memo::new(move |_| state.with(|state| state.visual()));
    let visual_for_root = visual;
    let visual_for_content = visual;

    view! {
        <figure
            class=root_class
            data-ui-component="avatar"
            data-ui-part="Avatar"
            data-ui-size=size
            data-ui-state=state_label
            data-ui-visual=move || visual_for_root.get().label()
            aria-label=root_label.clone()
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
        >
            {move || {
                if loading || disabled || !has_image || visual_for_content.get() == AvatarVisual::Fallback {
                    view! {
                        <span class=fallback_class data-ui-part="AvatarFallback" aria-hidden="true">
                            {if loading { "...".to_owned() } else { fallback_value.clone() }}
                        </span>
                    }
                        .into_any()
                } else {
                    let set_image_loaded = set_state;
                    let set_image_failed = set_state;
                    view! {
                        <img
                            class=AVATAR_IMAGE
                            data-ui-part="AvatarImage"
                            src=image_src.clone()
                            alt=image_alt.clone()
                            on:load=move |_| {
                                set_image_loaded.update(|state| {
                                    let _ = state.apply(AvatarIntent::ImageLoaded);
                                });
                            }
                            on:error=move |_| {
                                set_image_failed.update(|state| {
                                    let _ = state.apply(AvatarIntent::ImageFailed);
                                });
                            }
                        />
                    }
                        .into_any()
                }
            }}
        </figure>
    }
    .into_any()
}

const fn avatar_root_class(size: AvatarSize, disabled: bool) -> &'static str {
    if disabled {
        AVATAR_ROOT_DISABLED
    } else {
        match size {
            AvatarSize::Small => AVATAR_ROOT_SMALL,
            AvatarSize::Medium => AVATAR_ROOT_MEDIUM,
            AvatarSize::Large => AVATAR_ROOT_LARGE,
        }
    }
}

const fn avatar_fallback_class(size: AvatarSize, loading: bool, disabled: bool) -> &'static str {
    if loading || disabled {
        AVATAR_FALLBACK_MUTED
    } else {
        match size {
            AvatarSize::Small => AVATAR_FALLBACK_SMALL,
            AvatarSize::Medium => AVATAR_FALLBACK,
            AvatarSize::Large => AVATAR_FALLBACK_LARGE,
        }
    }
}

const fn avatar_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Badge(#[prop(optional, default = default_badge_model())] model: BadgeModel) -> AnyView {
    if let Err(report) = validate_badge_model(&model) {
        let message = format!("Badge validation failed: {report}");
        return view! {
            <span class=BADGE_ERROR data-ui-component="badge" data-ui-state="invalid" role="alert">
                {message}
            </span>
        }
        .into_any();
    }

    let initial_state = model.state();
    let nodes = badge_render_nodes(&model, initial_state);
    let icon = nodes
        .iter()
        .find(|node| node.part == BadgePart::Icon)
        .expect("invariant: badge render nodes include icon");
    let text = nodes
        .iter()
        .find(|node| node.part == BadgePart::Text)
        .expect("invariant: badge render nodes include text");
    let icon_value = icon.value.clone();
    let text_value = text.value.clone();
    let text_label = text.label.clone();
    let has_icon = model.icon.is_some();
    let loading = model.loading;
    let disabled = model.disabled;
    let size = model.size;
    let tone = model.tone;
    let variant = model.variant;
    let (state, set_state) = signal(initial_state);
    let highlighted = Memo::new(move |_| state.with(|state| state.is_highlighted()));

    view! {
        <span
            class=move || badge_root_class(size, tone, variant, disabled, highlighted.get())
            data-ui-component="badge"
            data-ui-part="Badge"
            data-ui-size=size.label()
            data-ui-tone=tone.label()
            data-ui-variant=variant.label()
            data-ui-state=badge_state_label(loading, disabled)
            data-ui-highlighted=move || highlighted.get().to_string()
            role="status"
            aria-label=text_label.clone()
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
            on:mouseenter=move |_| {
                if !disabled && !loading {
                    set_state.update(|state| {
                        let _ = state.apply(BadgeIntent::Highlight);
                    });
                }
            }
            on:mouseleave=move |_| {
                set_state.update(|state| {
                    let _ = state.apply(BadgeIntent::ClearHighlight);
                });
            }
        >
            {move || {
                if loading {
                    view! { <span class=BADGE_ICON data-ui-part="BadgeIcon" aria-hidden="true">"..."</span> }
                        .into_any()
                } else if has_icon {
                    view! { <span class=BADGE_ICON data-ui-part="BadgeIcon" aria-hidden="true">{icon_value.clone()}</span> }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}
            <span class=BADGE_TEXT data-ui-part="BadgeText">
                {move || if loading { "Loading".to_owned() } else { text_value.clone() }}
            </span>
        </span>
    }
    .into_any()
}

fn badge_root_class(
    size: BadgeSize,
    tone: BadgeTone,
    variant: BadgeVariant,
    disabled: bool,
    highlighted: bool,
) -> String {
    let size_class = match size {
        BadgeSize::Small => BADGE_SIZE_SMALL,
        BadgeSize::Medium => BADGE_SIZE_MEDIUM,
    };
    let tone_class = badge_tone_class(tone, variant);
    let disabled_class = if disabled { BADGE_DISABLED } else { "" };
    let highlighted_class = if highlighted { BADGE_HIGHLIGHTED } else { "" };
    format!("{BADGE_BASE} {size_class} {tone_class} {disabled_class} {highlighted_class}")
}

const fn badge_tone_class(tone: BadgeTone, variant: BadgeVariant) -> &'static str {
    match variant {
        BadgeVariant::Soft => match tone {
            BadgeTone::Default => BADGE_SOFT_DEFAULT,
            BadgeTone::Brand => BADGE_SOFT_BRAND,
            BadgeTone::Info => BADGE_SOFT_INFO,
            BadgeTone::Success => BADGE_SOFT_SUCCESS,
            BadgeTone::Warning => BADGE_SOFT_WARNING,
            BadgeTone::Destructive => BADGE_SOFT_DESTRUCTIVE,
            BadgeTone::Muted => BADGE_SOFT_MUTED,
        },
        BadgeVariant::Solid => match tone {
            BadgeTone::Default => BADGE_SOLID_DEFAULT,
            BadgeTone::Brand => BADGE_SOLID_BRAND,
            BadgeTone::Info => BADGE_SOLID_INFO,
            BadgeTone::Success => BADGE_SOLID_SUCCESS,
            BadgeTone::Warning => BADGE_SOLID_WARNING,
            BadgeTone::Destructive => BADGE_SOLID_DESTRUCTIVE,
            BadgeTone::Muted => BADGE_SOLID_MUTED,
        },
        BadgeVariant::Outline => match tone {
            BadgeTone::Default => BADGE_OUTLINE_DEFAULT,
            BadgeTone::Brand => BADGE_OUTLINE_BRAND,
            BadgeTone::Info => BADGE_OUTLINE_INFO,
            BadgeTone::Success => BADGE_OUTLINE_SUCCESS,
            BadgeTone::Warning => BADGE_OUTLINE_WARNING,
            BadgeTone::Destructive => BADGE_OUTLINE_DESTRUCTIVE,
            BadgeTone::Muted => BADGE_OUTLINE_MUTED,
        },
    }
}

const fn badge_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

struct BreadcrumbViewContext {
    entry_count: usize,
    separator: String,
    loading: bool,
    disabled: bool,
    state: ReadSignal<BreadcrumbState>,
    set_state: WriteSignal<BreadcrumbState>,
}

#[component]
pub fn Breadcrumb(
    #[prop(optional, default = default_breadcrumb_model())] model: BreadcrumbModel,
) -> AnyView {
    if let Err(report) = validate_breadcrumb_model(&model) {
        let message = format!("Breadcrumb validation failed: {report}");
        return view! {
            <nav class=BREADCRUMB_ROOT data-ui-component="breadcrumb" data-ui-state="invalid" aria-label="Breadcrumb">
                <p class=BREADCRUMB_ERROR role="alert">{message}</p>
            </nav>
        }
        .into_any();
    }

    let initial_state = model.state();
    let nodes = breadcrumb_render_nodes(&model, &initial_state);
    let root = nodes
        .iter()
        .find(|node| node.part == BreadcrumbPart::Root)
        .expect("invariant: breadcrumb render nodes include root");
    let list = nodes
        .iter()
        .find(|node| node.part == BreadcrumbPart::List)
        .expect("invariant: breadcrumb render nodes include list");
    let root_label = root.label.clone();
    let list_label = list.label.clone();
    let density = model.density;
    let entries = model.entries;
    let entry_count = entries.len();
    let separator = model.separator;
    let loading = model.loading;
    let disabled = model.disabled;
    let (state, set_state) = signal(initial_state);
    let context = BreadcrumbViewContext {
        entry_count,
        separator,
        loading,
        disabled,
        state,
        set_state,
    };

    view! {
        <nav
            class=breadcrumb_root_class(disabled)
            data-ui-component="breadcrumb"
            data-ui-part="Breadcrumb"
            data-ui-density=density.label()
            data-ui-state=breadcrumb_state_label(loading, disabled)
            aria-label=root_label
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
        >
            <ol class=breadcrumb_list_class(density) data-ui-part="BreadcrumbList" aria-label=list_label>
                {entries
                    .into_iter()
                    .enumerate()
                    .map(|(index, entry)| breadcrumb_entry_view(index, entry, &context))
                    .collect_view()}
            </ol>
        </nav>
    }
    .into_any()
}

fn breadcrumb_entry_view(
    index: usize,
    entry: BreadcrumbEntry,
    context: &BreadcrumbViewContext,
) -> AnyView {
    let current = index + 1 == context.entry_count;
    let label = entry.label.clone();
    let value = entry.value();
    let href = entry.href.clone().unwrap_or_else(|| "#".to_owned());
    let item_disabled = context.disabled || context.loading || entry.disabled;
    let link_disabled = item_disabled || entry.href.is_none();
    let value_for_focus = value.clone();
    let value_for_click = value.clone();
    let value_for_class = value.clone();
    let loading = context.loading;
    let separator = context.separator.clone();
    let state = context.state;
    let set_state = context.set_state;

    view! {
        <li
            class=BREADCRUMB_ITEM
            data-ui-part="BreadcrumbItem"
            data-ui-index=index.to_string()
            data-ui-current=current.to_string()
        >
            {if current {
                view! {
                    <span
                        class=breadcrumb_page_class(item_disabled)
                        data-ui-part="BreadcrumbPage"
                        aria-current="page"
                        aria-disabled=item_disabled.to_string()
                    >
                        {if loading { "Loading".to_owned() } else { label.clone() }}
                    </span>
                }
                    .into_any()
            } else if link_disabled {
                view! {
                    <span
                        class=breadcrumb_disabled_link_class(loading)
                        data-ui-part="BreadcrumbLink"
                        data-ui-intent="navigate"
                        aria-disabled="true"
                    >
                        {if loading { "Loading".to_owned() } else { label.clone() }}
                    </span>
                }
                    .into_any()
            } else {
                view! {
                    <a
                        class=move || breadcrumb_link_class(state.with(|state| state.is_active(&value_for_class)))
                        href=href
                        data-ui-part="BreadcrumbLink"
                        data-ui-intent="navigate"
                        on:mouseenter=move |_| {
                            set_state.update(|state| {
                                let _ = state.apply(BreadcrumbIntent::Focus(value_for_focus.clone()));
                            });
                        }
                        on:mouseleave=move |_| {
                            set_state.update(|state| {
                                let _ = state.apply(BreadcrumbIntent::Clear);
                            });
                        }
                        on:focus=move |_| {
                            set_state.update(|state| {
                                let _ = state.apply(BreadcrumbIntent::Focus(value.clone()));
                            });
                        }
                        on:blur=move |_| {
                            set_state.update(|state| {
                                let _ = state.apply(BreadcrumbIntent::Clear);
                            });
                        }
                        on:click=move |_| {
                            set_state.update(|state| {
                                let _ = state.apply(BreadcrumbIntent::Navigate(value_for_click.clone()));
                            });
                        }
                    >
                        {label.clone()}
                    </a>
                }
                    .into_any()
            }}
            {if current {
                ().into_any()
            } else {
                view! {
                    <span class=BREADCRUMB_SEPARATOR data-ui-part="BreadcrumbSeparator" aria-hidden="true">
                        {separator}
                    </span>
                }
                    .into_any()
            }}
        </li>
    }
    .into_any()
}

const fn breadcrumb_root_class(disabled: bool) -> &'static str {
    if disabled {
        BREADCRUMB_ROOT_DISABLED
    } else {
        BREADCRUMB_ROOT
    }
}

const fn breadcrumb_list_class(density: BreadcrumbDensity) -> &'static str {
    match density {
        BreadcrumbDensity::Standard => BREADCRUMB_LIST,
        BreadcrumbDensity::Dense => BREADCRUMB_LIST_DENSE,
    }
}

const fn breadcrumb_link_class(active: bool) -> &'static str {
    if active {
        BREADCRUMB_LINK_ACTIVE
    } else {
        BREADCRUMB_LINK
    }
}

const fn breadcrumb_disabled_link_class(loading: bool) -> &'static str {
    if loading {
        BREADCRUMB_LOADING
    } else {
        BREADCRUMB_LINK_DISABLED
    }
}

const fn breadcrumb_page_class(disabled: bool) -> &'static str {
    if disabled {
        BREADCRUMB_PAGE_DISABLED
    } else {
        BREADCRUMB_PAGE
    }
}

const fn breadcrumb_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Bubble(#[prop(optional, default = default_bubble_model())] model: BubbleModel) -> AnyView {
    if let Err(report) = validate_bubble_model(&model) {
        let message = format!("Bubble validation failed: {report}");
        return view! {
            <article class=BUBBLE_ROOT_INCOMING data-ui-component="bubble" data-ui-state="invalid">
                <p class=BUBBLE_ERROR role="alert">{message}</p>
            </article>
        }
        .into_any();
    }

    let initial_state = model.state();
    let nodes = bubble_render_nodes(&model, &initial_state);
    let avatar = nodes
        .iter()
        .find(|node| node.part == BubblePart::Avatar)
        .expect("invariant: bubble render nodes include avatar");
    let content = nodes
        .iter()
        .find(|node| node.part == BubblePart::Content)
        .expect("invariant: bubble render nodes include content");
    let meta = nodes
        .iter()
        .find(|node| node.part == BubblePart::Meta)
        .expect("invariant: bubble render nodes include meta");
    let action_nodes = nodes
        .iter()
        .filter(|node| node.part == BubblePart::Actions)
        .cloned()
        .collect::<Vec<_>>();
    let avatar_value = avatar.value.clone();
    let avatar_label = avatar.label.clone();
    let content_value = content.value.clone();
    let meta_value = meta.value.clone();
    let side = model.side;
    let loading = model.loading;
    let disabled = model.disabled;
    let (state, set_state) = signal(initial_state);

    view! {
        <article
            class=bubble_root_class(side, disabled)
            data-ui-component="bubble"
            data-ui-part="Bubble"
            data-ui-side=side.label()
            data-ui-state=bubble_state_label(loading, disabled)
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
        >
            <span class=bubble_avatar_class(side) data-ui-part="BubbleAvatar" aria-label=avatar_label>
                {avatar_value}
            </span>
            <div class=bubble_panel_class(side, loading, disabled)>
                <p class=bubble_content_class(disabled) data-ui-part="BubbleContent">
                    {if loading { "Loading message".to_owned() } else { content_value }}
                </p>
                <p class=bubble_meta_class(disabled) data-ui-part="BubbleMeta">
                    {if loading { "Pending".to_owned() } else { meta_value }}
                </p>
                <div class=BUBBLE_ACTIONS data-ui-part="BubbleActions">
                    {action_nodes
                        .into_iter()
                        .map(|node| {
                            let value = node.value.clone();
                            let value_for_class = value.clone();
                            let value_for_click = value.clone();
                            let button_disabled = node.disabled || loading || disabled;
                            let label = if loading {
                                "Loading".to_owned()
                            } else {
                                node.label.clone()
                            };
                            view! {
                                <button
                                    type="button"
                                    class=move || bubble_action_class(state.with(|state| state.is_active(&value_for_class)))
                                    data-ui-action=value.clone()
                                    data-ui-intent="activate"
                                    disabled=button_disabled
                                    on:click=move |_| {
                                        if !button_disabled {
                                            set_state.update(|state| {
                                                let _ = state.apply(BubbleIntent::Activate(value_for_click.clone()));
                                            });
                                        }
                                    }
                                >
                                    {label}
                                </button>
                            }
                            .into_any()
                        })
                        .collect_view()}
                </div>
            </div>
        </article>
    }
    .into_any()
}

const fn bubble_root_class(side: BubbleSide, disabled: bool) -> &'static str {
    if disabled {
        BUBBLE_ROOT_DISABLED
    } else {
        match side {
            BubbleSide::Incoming => BUBBLE_ROOT_INCOMING,
            BubbleSide::Outgoing => BUBBLE_ROOT_OUTGOING,
            BubbleSide::System => BUBBLE_ROOT_SYSTEM,
        }
    }
}

const fn bubble_avatar_class(side: BubbleSide) -> &'static str {
    match side {
        BubbleSide::Incoming => BUBBLE_AVATAR,
        BubbleSide::Outgoing => BUBBLE_AVATAR_OUTGOING,
        BubbleSide::System => BUBBLE_AVATAR_SYSTEM,
    }
}

const fn bubble_panel_class(side: BubbleSide, loading: bool, disabled: bool) -> &'static str {
    if disabled {
        BUBBLE_PANEL_DISABLED
    } else if loading {
        BUBBLE_PANEL_LOADING
    } else {
        match side {
            BubbleSide::Incoming => BUBBLE_PANEL_INCOMING,
            BubbleSide::Outgoing => BUBBLE_PANEL_OUTGOING,
            BubbleSide::System => BUBBLE_PANEL_SYSTEM,
        }
    }
}

const fn bubble_content_class(disabled: bool) -> &'static str {
    if disabled {
        BUBBLE_CONTENT_DISABLED
    } else {
        BUBBLE_CONTENT
    }
}

const fn bubble_meta_class(disabled: bool) -> &'static str {
    if disabled {
        BUBBLE_META_DISABLED
    } else {
        BUBBLE_META
    }
}

const fn bubble_action_class(active: bool) -> &'static str {
    if active {
        BUBBLE_ACTION_ACTIVE
    } else {
        BUBBLE_ACTION
    }
}

const fn bubble_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Button(#[prop(optional, default = default_button_model())] model: ButtonModel) -> AnyView {
    if let Err(report) = validate_button_model(&model) {
        let message = format!("Button validation failed: {report}");
        return view! {
            <div class=BUTTON_ERROR data-ui-component="button" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let initial_state = model.state();
    let nodes = button_render_nodes(&model, initial_state);
    let root = nodes
        .iter()
        .find(|node| node.part == ButtonPart::Root)
        .expect("invariant: button render nodes include root");
    let icon = nodes
        .iter()
        .find(|node| node.part == ButtonPart::Icon)
        .expect("invariant: button render nodes include icon");
    let label = nodes
        .iter()
        .find(|node| node.part == ButtonPart::Label)
        .expect("invariant: button render nodes include label");
    let kind = model.kind;
    let variant = model.variant;
    let size = model.size;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let root_value = root.value.clone();
    let label_value = if loading {
        "Loading".to_owned()
    } else {
        label.value.clone()
    };
    let icon_value = if loading {
        Some("...".to_owned())
    } else if model.icon.is_some() {
        Some(icon.value.clone())
    } else {
        None
    };
    let has_visible_icon = icon_value.is_some();
    let icon_label = icon.label.clone();
    let href = model.href.clone().unwrap_or_else(|| "#".to_owned());
    let href = if blocked { "#".to_owned() } else { href };
    let button_type = kind.button_type();
    let state_label = button_state_label(loading, disabled);
    let (state, set_state) = signal(initial_state);
    let pressed = Memo::new(move |_| state.with(|state| state.is_pressed()));

    if kind == ButtonKind::Link {
        let value_for_data = root_value.clone();
        let value_for_activate = root_value.clone();
        view! {
            <a
                class=move || button_root_class(size, variant, pressed.get(), blocked)
                href=href
                data-ui-component="button"
                data-ui-part=ButtonPart::Root.label()
                data-ui-kind=kind.label()
                data-ui-variant=variant.label()
                data-ui-size=size.label()
                data-ui-state=state_label
                data-ui-value=value_for_data
                aria-disabled=blocked.to_string()
                aria-busy=loading.to_string()
                aria-pressed=move || pressed.get().to_string()
                on:mousedown=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ButtonIntent::Press);
                        });
                    }
                }
                on:mouseup=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ButtonIntent::Release);
                        });
                    }
                }
                on:mouseleave=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ButtonIntent::Release);
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ButtonIntent::Release);
                        });
                    }
                }
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ButtonIntent::Activate(value_for_activate.clone()));
                        });
                    }
                }
            >
                {button_icon_view(icon_value.clone(), icon_label.clone())}
                <span class=button_label_class(size, has_visible_icon) data-ui-part=ButtonPart::Label.label()>
                    {label_value.clone()}
                </span>
            </a>
        }
        .into_any()
    } else {
        let value_for_data = root_value.clone();
        let value_for_activate = root_value.clone();
        view! {
            <button
                type=button_type
                class=move || button_root_class(size, variant, pressed.get(), blocked)
                data-ui-component="button"
                data-ui-part=ButtonPart::Root.label()
                data-ui-kind=kind.label()
                data-ui-variant=variant.label()
                data-ui-size=size.label()
                data-ui-state=state_label
                data-ui-value=value_for_data
                aria-busy=loading.to_string()
                aria-pressed=move || pressed.get().to_string()
                disabled=blocked
                on:mousedown=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ButtonIntent::Press);
                        });
                    }
                }
                on:mouseup=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ButtonIntent::Release);
                        });
                    }
                }
                on:mouseleave=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ButtonIntent::Release);
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ButtonIntent::Release);
                        });
                    }
                }
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ButtonIntent::Activate(value_for_activate.clone()));
                        });
                    }
                }
            >
                {button_icon_view(icon_value.clone(), icon_label.clone())}
                <span class=button_label_class(size, has_visible_icon) data-ui-part=ButtonPart::Label.label()>
                    {label_value.clone()}
                </span>
            </button>
        }
        .into_any()
    }
}

fn button_icon_view(icon: Option<String>, label: String) -> AnyView {
    icon.map(|icon| {
        view! {
            <span class=BUTTON_ICON data-ui-part=ButtonPart::Icon.label() aria-label=label>
                {icon}
            </span>
        }
        .into_any()
    })
    .unwrap_or_else(|| ().into_any())
}

fn button_root_class(
    size: ButtonSize,
    variant: ButtonVariant,
    pressed: bool,
    blocked: bool,
) -> String {
    format!(
        "{BUTTON_BASE} {} {} {} {}",
        button_size_class(size),
        button_variant_class(variant),
        if pressed { BUTTON_PRESSED } else { "" },
        if blocked { BUTTON_BLOCKED } else { "" },
    )
}

const fn button_size_class(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Small => BUTTON_SIZE_SMALL,
        ButtonSize::Medium => BUTTON_SIZE_MEDIUM,
        ButtonSize::Large => BUTTON_SIZE_LARGE,
        ButtonSize::Icon => BUTTON_SIZE_ICON,
    }
}

const fn button_variant_class(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Primary => BUTTON_PRIMARY,
        ButtonVariant::Secondary => BUTTON_SECONDARY,
        ButtonVariant::Destructive => BUTTON_DESTRUCTIVE,
        ButtonVariant::Outline => BUTTON_OUTLINE,
        ButtonVariant::Ghost => BUTTON_GHOST,
        ButtonVariant::Link => BUTTON_LINK,
    }
}

const fn button_label_class(size: ButtonSize, has_visible_icon: bool) -> &'static str {
    if matches!(size, ButtonSize::Icon) && has_visible_icon {
        BUTTON_LABEL_ICON
    } else {
        BUTTON_LABEL
    }
}

const fn button_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn ButtonGroup(
    #[prop(optional, default = default_button_group_model())] model: ButtonGroupModel,
) -> AnyView {
    if let Err(report) = validate_button_group_model(&model) {
        let message = format!("Button Group validation failed: {report}");
        return view! {
            <div class=BUTTON_GROUP_ERROR data-ui-component="button-group" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let initial_state = model.state();
    let nodes = button_group_render_nodes(&model, &initial_state);
    let root = nodes
        .iter()
        .find(|node| node.part == ButtonGroupPart::Root)
        .expect("invariant: button group render nodes include root");
    let items = nodes
        .iter()
        .filter(|node| node.part == ButtonGroupPart::Item)
        .cloned()
        .collect::<Vec<_>>();
    let item_count = items.len();
    let orientation = model.orientation;
    let variant = model.variant;
    let size = model.size;
    let loading = model.loading;
    let disabled = model.disabled;
    let root_value = root.value.clone();
    let state_label = button_group_state_label(loading, disabled);
    let (state, set_state) = signal(initial_state);

    view! {
        <div
            class=button_group_root_class(orientation, loading || disabled)
            data-ui-component="button-group"
            data-ui-part=ButtonGroupPart::Root.label()
            data-ui-orientation=orientation.label()
            data-ui-variant=variant.label()
            data-ui-size=size.label()
            data-ui-state=state_label
            data-ui-value=root_value
            role="group"
            aria-orientation=orientation.label()
            aria-disabled=(loading || disabled).to_string()
            aria-busy=loading.to_string()
        >
            {items
                .into_iter()
                .enumerate()
                .map(|(position, node)| {
                    let value = node.value.clone();
                    let value_for_class = value.clone();
                    let value_for_click = value.clone();
                    let label = if loading {
                        "Loading".to_owned()
                    } else {
                        node.label.clone()
                    };
                    let icon = if loading {
                        Some("...".to_owned())
                    } else {
                        node.icon.clone()
                    };
                    let has_visible_icon = icon.is_some();
                    let item_disabled = node.disabled || loading || disabled;
                    view! {
                        <>
                            <button
                                type="button"
                                class=move || button_group_item_class(
                                    variant,
                                    size,
                                    state.with(|state| state.is_selected(&value_for_class)),
                                    item_disabled,
                                )
                                data-ui-part=ButtonGroupPart::Item.label()
                                data-ui-value=value.clone()
                                data-ui-index=node.index.to_string()
                                aria-pressed=move || state.with(|state| state.is_selected(&value)).to_string()
                                disabled=item_disabled
                                on:click=move |_| {
                                    if !item_disabled {
                                        set_state.update(|state| {
                                            let _ = state.apply(ButtonGroupIntent::Select(value_for_click.clone()));
                                        });
                                    }
                                }
                            >
                                {button_group_icon_view(icon.clone())}
                                <span class=button_group_label_class(size, has_visible_icon) data-ui-part="ButtonGroupItemLabel">
                                    {label.clone()}
                                </span>
                            </button>
                            {if position + 1 == item_count {
                                ().into_any()
                            } else {
                                view! {
                                    <span
                                        class=button_group_separator_class(orientation)
                                        data-ui-part=ButtonGroupPart::Separator.label()
                                        role="separator"
                                        aria-orientation=orientation.label()
                                    ></span>
                                }
                                .into_any()
                            }}
                        </>
                    }
                    .into_any()
                })
                .collect_view()}
        </div>
    }
    .into_any()
}

fn button_group_icon_view(icon: Option<String>) -> AnyView {
    icon.map(|icon| {
        view! {
            <span class=BUTTON_GROUP_ICON aria-hidden="true">
                {icon}
            </span>
        }
        .into_any()
    })
    .unwrap_or_else(|| ().into_any())
}

fn button_group_root_class(orientation: ButtonGroupOrientation, blocked: bool) -> String {
    format!(
        "{} {}",
        match orientation {
            ButtonGroupOrientation::Horizontal => BUTTON_GROUP_ROOT_HORIZONTAL,
            ButtonGroupOrientation::Vertical => BUTTON_GROUP_ROOT_VERTICAL,
        },
        if blocked {
            BUTTON_GROUP_ROOT_DISABLED
        } else {
            ""
        },
    )
}

fn button_group_item_class(
    variant: ButtonVariant,
    size: ButtonSize,
    selected: bool,
    disabled: bool,
) -> String {
    format!(
        "{BUTTON_GROUP_ITEM_BASE} {} {} {}",
        button_group_item_size_class(size),
        button_group_item_variant_class(variant, selected),
        if disabled {
            BUTTON_GROUP_ITEM_DISABLED
        } else {
            ""
        },
    )
}

const fn button_group_item_size_class(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Small => BUTTON_GROUP_ITEM_SMALL,
        ButtonSize::Medium => BUTTON_GROUP_ITEM_MEDIUM,
        ButtonSize::Large => BUTTON_GROUP_ITEM_LARGE,
        ButtonSize::Icon => BUTTON_GROUP_ITEM_ICON,
    }
}

const fn button_group_item_variant_class(variant: ButtonVariant, selected: bool) -> &'static str {
    if !selected {
        return BUTTON_GROUP_ITEM_IDLE;
    }
    match variant {
        ButtonVariant::Primary => BUTTON_GROUP_ITEM_SELECTED_PRIMARY,
        ButtonVariant::Secondary => BUTTON_GROUP_ITEM_SELECTED_SECONDARY,
        ButtonVariant::Destructive => BUTTON_GROUP_ITEM_SELECTED_DESTRUCTIVE,
        ButtonVariant::Outline => BUTTON_GROUP_ITEM_SELECTED_OUTLINE,
        ButtonVariant::Ghost => BUTTON_GROUP_ITEM_SELECTED_GHOST,
        ButtonVariant::Link => BUTTON_GROUP_ITEM_SELECTED_LINK,
    }
}

const fn button_group_label_class(size: ButtonSize, has_visible_icon: bool) -> &'static str {
    if matches!(size, ButtonSize::Icon) && has_visible_icon {
        BUTTON_GROUP_LABEL_ICON
    } else {
        BUTTON_GROUP_LABEL
    }
}

const fn button_group_separator_class(orientation: ButtonGroupOrientation) -> &'static str {
    match orientation {
        ButtonGroupOrientation::Horizontal => BUTTON_GROUP_SEPARATOR_HORIZONTAL,
        ButtonGroupOrientation::Vertical => BUTTON_GROUP_SEPARATOR_VERTICAL,
    }
}

const fn button_group_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Calendar(
    #[prop(optional, default = default_calendar_model())] model: CalendarModel,
) -> AnyView {
    if let Err(report) = validate_calendar_model(&model) {
        let message = format!("Calendar validation failed: {report}");
        return view! {
            <div class=CALENDAR_ERROR data-ui-component="calendar" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let mode = model.mode;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let model_for_days = model.clone();
    let model_for_range = model.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=calendar_root_class(disabled)
            data-ui-component="calendar"
            data-ui-part=CalendarPart::Root.label()
            data-ui-mode=mode.label()
            data-ui-state=calendar_state_label(loading, disabled)
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <header class=CALENDAR_HEADER data-ui-part=CalendarPart::Header.label()>
                <button
                    type="button"
                    class=CALENDAR_NAV
                    aria-label="Previous month"
                    disabled=blocked
                    on:click=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(CalendarIntent::PreviousMonth);
                            });
                        }
                    }
                >
                    "<"
                </button>
                <h3 class=CALENDAR_TITLE>
                    {move || {
                        state.with(|state| {
                            format!("{} {}", month_name(state.visible_month()), state.visible_year())
                        })
                    }}
                </h3>
                <button
                    type="button"
                    class=CALENDAR_NAV
                    aria-label="Next month"
                    disabled=blocked
                    on:click=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(CalendarIntent::NextMonth);
                            });
                        }
                    }
                >
                    ">"
                </button>
            </header>
            <div class=CALENDAR_GRID data-ui-part=CalendarPart::Grid.label() role="grid">
                {["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
                    .into_iter()
                    .map(|weekday| {
                        view! {
                            <span class=CALENDAR_WEEKDAY role="columnheader">{weekday}</span>
                        }
                    })
                    .collect_view()}
                {move || {
                    state.with(|state| {
                        calendar_render_nodes(&model_for_days, state)
                            .into_iter()
                            .filter(|node| node.part == CalendarPart::Day)
                            .map(|node| {
                                let date = node
                                    .date
                                    .expect("invariant: calendar day nodes include a date");
                                let date_value = date.value();
                                let detail = node.detail.clone();
                                let selected = node.selected;
                                let item_disabled = node.disabled || blocked;
                                let intent = match mode {
                                    CalendarSelectionMode::Single => CalendarIntent::Select(date),
                                    CalendarSelectionMode::Range => CalendarIntent::SelectRange(date),
                                };
                                view! {
                                    <button
                                        type="button"
                                        class=calendar_day_class(
                                            node.current_month,
                                            selected,
                                            node.in_range,
                                        )
                                        data-ui-part=CalendarPart::Day.label()
                                        data-ui-value=date_value
                                        data-ui-index=node.index.to_string()
                                        aria-label=detail
                                        aria-pressed=selected.to_string()
                                        disabled=item_disabled
                                        on:click=move |_| {
                                            if !item_disabled {
                                                let intent = intent.clone();
                                                set_state.update(|state| {
                                                    let _ = state.apply(intent);
                                                });
                                            }
                                        }
                                    >
                                        {node.label.clone()}
                                    </button>
                                }
                            })
                            .collect_view()
                    })
                }}
            </div>
            {move || {
                state.with(|state| {
                    let range = calendar_render_nodes(&model_for_range, state)
                        .into_iter()
                        .find(|node| node.part == CalendarPart::Range)
                        .expect("invariant: calendar render nodes include range");
                    view! {
                        <p class=calendar_range_class(range.selected) data-ui-part=CalendarPart::Range.label()>
                            {range.detail}
                        </p>
                    }
                    .into_any()
                })
            }}
        </section>
    }
    .into_any()
}

const fn calendar_root_class(disabled: bool) -> &'static str {
    if disabled {
        CALENDAR_ROOT_DISABLED
    } else {
        CALENDAR_ROOT
    }
}

const fn calendar_day_class(current_month: bool, selected: bool, in_range: bool) -> &'static str {
    if selected {
        CALENDAR_DAY_SELECTED
    } else if in_range {
        CALENDAR_DAY_RANGE
    } else if current_month {
        CALENDAR_DAY
    } else {
        CALENDAR_DAY_OUTSIDE
    }
}

const fn calendar_range_class(selected: bool) -> &'static str {
    if selected {
        CALENDAR_RANGE_ACTIVE
    } else {
        CALENDAR_RANGE
    }
}

const fn calendar_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Card(#[prop(optional, default = default_card_model())] model: CardModel) -> AnyView {
    if let Err(report) = validate_card_model(&model) {
        let message = format!("Card validation failed: {report}");
        return view! {
            <div class=CARD_ERROR data-ui-component="card" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = card_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == CardPart::Root)
        .expect("invariant: card render nodes include root")
        .clone();
    let header = nodes
        .iter()
        .find(|node| node.part == CardPart::Header)
        .expect("invariant: card render nodes include header")
        .clone();
    let title = nodes
        .iter()
        .find(|node| node.part == CardPart::Title)
        .expect("invariant: card render nodes include title")
        .clone();
    let description = nodes
        .iter()
        .find(|node| node.part == CardPart::Description)
        .expect("invariant: card render nodes include description")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == CardPart::Content)
        .expect("invariant: card render nodes include content")
        .clone();
    let footer = nodes
        .iter()
        .find(|node| node.part == CardPart::Footer)
        .expect("invariant: card render nodes include footer")
        .clone();
    let (state, set_state) = signal(state_model);
    let footer_disabled = footer.disabled || blocked;
    let action_value = footer.value.clone();
    let action_label = footer.detail.clone();
    let action_view = if footer.actionable {
        view! {
            <button
                type="button"
                class=move || {
                    state.with(|state| {
                        card_action_class(state.is_active(CardPart::Footer), footer_disabled)
                    })
                }
                data-ui-part=CardPart::Footer.label()
                data-ui-value=action_value.clone()
                aria-pressed=move || {
                    state.with(|state| state.is_active(CardPart::Footer).to_string())
                }
                disabled=footer_disabled
                on:focus=move |_| {
                    if !footer_disabled {
                        set_state.update(|state| {
                            let _ = state.apply(CardIntent::Focus(CardPart::Footer));
                        });
                    }
                }
                on:blur=move |_| {
                    if !footer_disabled {
                        set_state.update(|state| {
                            let _ = state.apply(CardIntent::Blur(CardPart::Footer));
                        });
                    }
                }
                on:click=move |_| {
                    if !footer_disabled {
                        let value = action_value.clone();
                        set_state.update(|state| {
                            let _ = state.apply(CardIntent::ActivateFooter(value));
                        });
                    }
                }
            >
                {action_label}
            </button>
        }
        .into_any()
    } else {
        ().into_any()
    };

    view! {
        <article
            class=card_root_class(root.variant, root.density, disabled)
            data-ui-component="card"
            data-ui-part=CardPart::Root.label()
            data-ui-variant=root.variant.label()
            data-ui-density=root.density.label()
            data-ui-state=card_state_label(loading, disabled)
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <header
                class=card_header_class(root.density)
                data-ui-part=header.part.label()
            >
                <h3 class=card_title_class(root.density) data-ui-part=title.part.label()>
                    {title.label}
                </h3>
                <p class=card_description_class(root.density) data-ui-part=description.part.label()>
                    {description.detail}
                </p>
            </header>
            <div class=card_content_class(root.density) data-ui-part=content.part.label()>
                {content.detail}
            </div>
            <footer class=card_footer_class(root.density) data-ui-part=footer.part.label()>
                <p class=CARD_FOOTER_TEXT>{footer.label}</p>
                {action_view}
            </footer>
        </article>
    }
    .into_any()
}

const fn card_root_class(
    variant: CardVariant,
    density: CardDensity,
    disabled: bool,
) -> &'static str {
    if disabled {
        return CARD_DISABLED;
    }
    match (density, variant) {
        (CardDensity::Standard, CardVariant::Elevated) => CARD_STANDARD_ELEVATED,
        (CardDensity::Standard, CardVariant::Outline) => CARD_STANDARD_OUTLINE,
        (CardDensity::Standard, CardVariant::Ghost) => CARD_STANDARD_GHOST,
        (CardDensity::Dense, CardVariant::Elevated) => CARD_DENSE_ELEVATED,
        (CardDensity::Dense, CardVariant::Outline) => CARD_DENSE_OUTLINE,
        (CardDensity::Dense, CardVariant::Ghost) => CARD_DENSE_GHOST,
    }
}

const fn card_header_class(density: CardDensity) -> &'static str {
    match density {
        CardDensity::Standard => CARD_HEADER,
        CardDensity::Dense => CARD_HEADER_DENSE,
    }
}

const fn card_title_class(density: CardDensity) -> &'static str {
    match density {
        CardDensity::Standard => CARD_TITLE,
        CardDensity::Dense => CARD_TITLE_DENSE,
    }
}

const fn card_description_class(density: CardDensity) -> &'static str {
    match density {
        CardDensity::Standard => CARD_DESCRIPTION,
        CardDensity::Dense => CARD_DESCRIPTION_DENSE,
    }
}

const fn card_content_class(density: CardDensity) -> &'static str {
    match density {
        CardDensity::Standard => CARD_CONTENT,
        CardDensity::Dense => CARD_CONTENT_DENSE,
    }
}

const fn card_footer_class(density: CardDensity) -> &'static str {
    match density {
        CardDensity::Standard => CARD_FOOTER,
        CardDensity::Dense => CARD_FOOTER_DENSE,
    }
}

const fn card_action_class(active: bool, disabled: bool) -> &'static str {
    if active && !disabled {
        CARD_ACTION_ACTIVE
    } else {
        CARD_ACTION
    }
}

const fn card_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Carousel(
    #[prop(optional, default = default_carousel_model())] model: CarouselModel,
) -> AnyView {
    if let Err(report) = validate_carousel_model(&model) {
        let message = format!("Carousel validation failed: {report}");
        return view! {
            <div class=CAROUSEL_ERROR data-ui-component="carousel" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let item_model = model.clone();
    let previous_model = model.clone();
    let next_model = model.clone();
    let indicator_model = model.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=carousel_root_class(density, disabled)
            data-ui-component="carousel"
            data-ui-part=CarouselPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=carousel_state_label(loading, disabled)
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <div
                class=carousel_content_class(density)
                data-ui-part=CarouselPart::Content.label()
                role="list"
            >
                {move || {
                    state.with(|state| {
                        carousel_render_nodes(&item_model, state)
                            .into_iter()
                            .filter(|node| node.part == CarouselPart::Item)
                            .map(|node| {
                                view! {
                                    <article
                                        class=carousel_item_class(node.selected, node.disabled)
                                        data-ui-part=CarouselPart::Item.label()
                                        data-ui-value=node.value
                                        data-ui-index=node.index.to_string()
                                        aria-current=node.selected.to_string()
                                        aria-disabled=node.disabled.to_string()
                                        role="listitem"
                                    >
                                        <h3 class=carousel_item_title_class(node.density)>
                                            {node.label}
                                        </h3>
                                        <p class=carousel_item_detail_class(node.density)>
                                            {node.detail}
                                        </p>
                                    </article>
                                }
                            })
                            .collect_view()
                    })
                }}
            </div>
            <div class=CAROUSEL_CONTROLS>
                <button
                    type="button"
                    class=CAROUSEL_NAV
                    data-ui-part=CarouselPart::Previous.label()
                    aria-label="Previous slide"
                    disabled=move || {
                        state.with(|state| {
                            carousel_control_disabled(
                                &previous_model,
                                state,
                                CarouselPart::Previous,
                            )
                        })
                    }
                    on:click=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(CarouselIntent::Previous);
                            });
                        }
                    }
                >
                    "<"
                </button>
                <span
                    class=CAROUSEL_INDICATOR
                    data-ui-part=CarouselPart::Indicator.label()
                    aria-live="polite"
                >
                    {move || {
                        state.with(|state| carousel_indicator_label(&indicator_model, state))
                    }}
                </span>
                <button
                    type="button"
                    class=CAROUSEL_NAV
                    data-ui-part=CarouselPart::Next.label()
                    aria-label="Next slide"
                    disabled=move || {
                        state.with(|state| {
                            carousel_control_disabled(&next_model, state, CarouselPart::Next)
                        })
                    }
                    on:click=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(CarouselIntent::Next);
                            });
                        }
                    }
                >
                    ">"
                </button>
            </div>
        </section>
    }
    .into_any()
}

const fn carousel_root_class(density: CarouselDensity, disabled: bool) -> &'static str {
    if disabled {
        return CAROUSEL_ROOT_DISABLED;
    }
    match density {
        CarouselDensity::Standard => CAROUSEL_ROOT,
        CarouselDensity::Dense => CAROUSEL_ROOT_DENSE,
    }
}

const fn carousel_content_class(density: CarouselDensity) -> &'static str {
    match density {
        CarouselDensity::Standard => CAROUSEL_CONTENT,
        CarouselDensity::Dense => CAROUSEL_CONTENT_DENSE,
    }
}

const fn carousel_item_class(selected: bool, disabled: bool) -> &'static str {
    if disabled {
        CAROUSEL_ITEM_DISABLED
    } else if selected {
        CAROUSEL_ITEM_SELECTED
    } else {
        CAROUSEL_ITEM
    }
}

const fn carousel_item_title_class(density: CarouselDensity) -> &'static str {
    match density {
        CarouselDensity::Standard => CAROUSEL_ITEM_TITLE,
        CarouselDensity::Dense => CAROUSEL_ITEM_TITLE_DENSE,
    }
}

const fn carousel_item_detail_class(density: CarouselDensity) -> &'static str {
    match density {
        CarouselDensity::Standard => CAROUSEL_ITEM_DETAIL,
        CarouselDensity::Dense => CAROUSEL_ITEM_DETAIL_DENSE,
    }
}

fn carousel_control_disabled(
    model: &CarouselModel,
    state: &crate::CarouselState,
    part: CarouselPart,
) -> bool {
    carousel_render_nodes(model, state)
        .into_iter()
        .find(|node| node.part == part)
        .is_none_or(|node| node.disabled)
}

fn carousel_indicator_label(model: &CarouselModel, state: &crate::CarouselState) -> String {
    carousel_render_nodes(model, state)
        .into_iter()
        .find(|node| node.part == CarouselPart::Indicator)
        .map(|node| node.label)
        .unwrap_or_else(|| "0 of 0".to_owned())
}

const fn carousel_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Chart(#[prop(optional, default = default_chart_model())] model: ChartModel) -> AnyView {
    if let Err(report) = validate_chart_model(&model) {
        let message = format!("Chart validation failed: {report}");
        return view! {
            <div class=CHART_ERROR data-ui-component="chart" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let series_model = model.clone();
    let legend_model = model.clone();
    let tooltip_model = model.clone();
    let axis_model = model.clone();
    let container = chart_render_nodes(&model, &model.state())
        .into_iter()
        .find(|node| node.part == ChartPart::Container)
        .expect("invariant: chart render nodes include container");
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=chart_root_class(density, disabled)
            data-ui-component="chart"
            data-ui-part=ChartPart::Container.label()
            data-ui-density=density.label()
            data-ui-state=chart_state_label(loading, disabled)
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <h3 class=chart_title_class(density)>{container.label}</h3>
            <div class=chart_series_list_class(density) data-ui-part=ChartPart::Series.label()>
                {move || {
                    state.with(|state| {
                        chart_render_nodes(&series_model, state)
                            .into_iter()
                            .filter(|node| node.part == ChartPart::Series)
                            .map(|node| {
                                let value = node.value.clone();
                                view! {
                                    <button
                                        type="button"
                                        class=chart_series_class(node.selected)
                                        data-ui-part=ChartPart::Series.label()
                                        data-ui-value=node.value
                                        data-ui-index=node.index.to_string()
                                        aria-pressed=node.selected.to_string()
                                        disabled=node.disabled
                                        on:click=move |_| {
                                            if !blocked {
                                                let value = value.clone();
                                                set_state.update(|state| {
                                                    let _ = state.apply(ChartIntent::Select(value));
                                                });
                                            }
                                        }
                                    >
                                        <span class=CHART_SERIES_LABEL>
                                            <span>{node.label.clone()}</span>
                                            <span>{format!("{}{}", node.amount, model.unit)}</span>
                                        </span>
                                        <span class=CHART_TRACK aria-hidden="true">
                                            <span
                                                class=chart_bar_class(node.tone)
                                                style=format!("width: {}%;", node.amount)
                                            ></span>
                                        </span>
                                    </button>
                                }
                            })
                            .collect_view()
                    })
                }}
            </div>
            <div class=CHART_LEGEND data-ui-part=ChartPart::Legend.label()>
                {move || {
                    state.with(|state| {
                        chart_render_nodes(&legend_model, state)
                            .into_iter()
                            .filter(|node| node.part == ChartPart::Legend)
                            .map(|node| {
                                view! {
                                    <span
                                        class=chart_legend_class(node.selected)
                                        data-ui-value=node.value
                                    >
                                        {node.label}
                                    </span>
                                }
                            })
                            .collect_view()
                    })
                }}
            </div>
            <p class=CHART_TOOLTIP data-ui-part=ChartPart::Tooltip.label()>
                {move || {
                    state.with(|state| {
                        chart_render_nodes(&tooltip_model, state)
                            .into_iter()
                            .find(|node| node.part == ChartPart::Tooltip)
                            .map(|node| format!("{}: {}", node.label, node.detail))
                            .unwrap_or_else(|| "No series".to_owned())
                    })
                }}
            </p>
            <p class=CHART_AXIS data-ui-part=ChartPart::Axis.label()>
                {move || {
                    state.with(|state| {
                        chart_render_nodes(&axis_model, state)
                            .into_iter()
                            .find(|node| node.part == ChartPart::Axis)
                            .map(|node| format!("{} ({})", node.label, node.detail))
                            .unwrap_or_else(|| "Axis".to_owned())
                    })
                }}
            </p>
        </section>
    }
    .into_any()
}

const fn chart_root_class(density: ChartDensity, disabled: bool) -> &'static str {
    if disabled {
        return CHART_ROOT_DISABLED;
    }
    match density {
        ChartDensity::Standard => CHART_ROOT,
        ChartDensity::Dense => CHART_ROOT_DENSE,
    }
}

const fn chart_title_class(density: ChartDensity) -> &'static str {
    match density {
        ChartDensity::Standard => CHART_TITLE,
        ChartDensity::Dense => CHART_TITLE_DENSE,
    }
}

const fn chart_series_list_class(density: ChartDensity) -> &'static str {
    match density {
        ChartDensity::Standard => CHART_SERIES_LIST,
        ChartDensity::Dense => CHART_SERIES_LIST_DENSE,
    }
}

const fn chart_series_class(selected: bool) -> &'static str {
    if selected {
        CHART_SERIES_SELECTED
    } else {
        CHART_SERIES
    }
}

const fn chart_bar_class(tone: ChartTone) -> &'static str {
    match tone {
        ChartTone::Brand => CHART_BAR_BRAND,
        ChartTone::Info => CHART_BAR_INFO,
        ChartTone::Success => CHART_BAR_SUCCESS,
        ChartTone::Warning => CHART_BAR_WARNING,
        ChartTone::Danger => CHART_BAR_DANGER,
        ChartTone::Muted => CHART_BAR_MUTED,
    }
}

const fn chart_legend_class(selected: bool) -> &'static str {
    if selected {
        CHART_LEGEND_SELECTED
    } else {
        CHART_LEGEND_ITEM
    }
}

const fn chart_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Checkbox(
    #[prop(optional, default = default_checkbox_model())] model: CheckboxModel,
) -> AnyView {
    if let Err(report) = validate_checkbox_model(&model) {
        let message = format!("Checkbox validation failed: {report}");
        return view! {
            <div class=CHECKBOX_ERROR data-ui-component="checkbox" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let blocked = loading || disabled;
    let nodes = checkbox_render_nodes(&model, &model.state());
    let root = nodes
        .iter()
        .find(|node| node.part == CheckboxPart::Root)
        .expect("invariant: checkbox render nodes include root")
        .clone();
    let label = nodes
        .iter()
        .find(|node| node.part == CheckboxPart::Label)
        .expect("invariant: checkbox render nodes include label")
        .clone();
    let description = nodes
        .iter()
        .find(|node| node.part == CheckboxPart::Description)
        .expect("invariant: checkbox render nodes include description")
        .clone();
    let required = root.required;
    let value = root.value.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <div
            class=checkbox_root_class(density, disabled, invalid)
            data-ui-component="checkbox"
            data-ui-part=CheckboxPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    checkbox_state_label(loading, disabled, invalid, state.checked()).to_owned()
                })
            }
            data-ui-checked=move || state.with(|state| state.checked().label().to_owned())
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
        >
            <button
                type="button"
                role="checkbox"
                class=move || {
                    state.with(|state| {
                        checkbox_control_class(state.checked(), blocked, invalid).to_owned()
                    })
                }
                data-ui-part=CheckboxPart::Indicator.label()
                data-ui-value=value
                aria-label=root.label
                aria-checked=move || state.with(|state| state.checked().aria_checked().to_owned())
                disabled=blocked
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(CheckboxIntent::Toggle);
                        });
                    }
                }
            >
                {move || state.with(|state| checkbox_indicator_text(state.checked()).to_owned())}
            </button>
            <div class=CHECKBOX_TEXT>
                <p
                    class=checkbox_label_class(disabled)
                    data-ui-part=CheckboxPart::Label.label()
                >
                    {label.label}
                    {required.then_some(view! { <span class=CHECKBOX_REQUIRED>" *"</span> })}
                </p>
                <p
                    class=checkbox_description_class(disabled, invalid)
                    data-ui-part=CheckboxPart::Description.label()
                >
                    {description.detail}
                </p>
            </div>
        </div>
    }
    .into_any()
}

const fn checkbox_root_class(
    density: CheckboxDensity,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        return CHECKBOX_ROOT_DISABLED;
    }
    if invalid {
        return CHECKBOX_ROOT_INVALID;
    }
    match density {
        CheckboxDensity::Standard => CHECKBOX_ROOT,
        CheckboxDensity::Dense => CHECKBOX_ROOT_DENSE,
    }
}

const fn checkbox_control_class(
    checked: CheckboxChecked,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        return CHECKBOX_CONTROL_DISABLED;
    }
    if invalid {
        return CHECKBOX_CONTROL_INVALID;
    }
    match checked {
        CheckboxChecked::Unchecked => CHECKBOX_CONTROL,
        CheckboxChecked::Checked => CHECKBOX_CONTROL_CHECKED,
        CheckboxChecked::Indeterminate => CHECKBOX_CONTROL_INDETERMINATE,
    }
}

const fn checkbox_label_class(disabled: bool) -> &'static str {
    if disabled {
        CHECKBOX_LABEL_DISABLED
    } else {
        CHECKBOX_LABEL
    }
}

const fn checkbox_description_class(disabled: bool, invalid: bool) -> &'static str {
    if disabled {
        CHECKBOX_DESCRIPTION_DISABLED
    } else if invalid {
        CHECKBOX_DESCRIPTION_INVALID
    } else {
        CHECKBOX_DESCRIPTION
    }
}

const fn checkbox_indicator_text(checked: CheckboxChecked) -> &'static str {
    match checked {
        CheckboxChecked::Unchecked => "",
        CheckboxChecked::Checked => "x",
        CheckboxChecked::Indeterminate => "-",
    }
}

const fn checkbox_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    checked: CheckboxChecked,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else {
        checked.label()
    }
}

#[component]
pub fn Collapsible(
    #[prop(optional, default = default_collapsible_model())] model: CollapsibleModel,
) -> AnyView {
    if let Err(report) = validate_collapsible_model(&model) {
        let message = format!("Collapsible validation failed: {report}");
        return view! {
            <div class=COLLAPSIBLE_ERROR data-ui-component="collapsible" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let trigger_model = model.clone();
    let content_model = model.clone();
    let nodes = collapsible_render_nodes(&model, &model.state());
    let root = nodes
        .iter()
        .find(|node| node.part == CollapsiblePart::Root)
        .expect("invariant: collapsible render nodes include root")
        .clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=collapsible_root_class(density, disabled)
            data-ui-component="collapsible"
            data-ui-part=CollapsiblePart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| collapsible_state_label(loading, disabled, state.is_open()).to_owned())
            }
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <button
                type="button"
                class=move || {
                    state.with(|state| collapsible_trigger_class(density, disabled, state.is_open()).to_owned())
                }
                data-ui-part=CollapsiblePart::Trigger.label()
                data-ui-value=root.value
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(CollapsibleIntent::Toggle);
                        });
                    }
                }
            >
                <span>
                    {move || {
                        state.with(|state| {
                            collapsible_render_nodes(&trigger_model, state)
                                .into_iter()
                                .find(|node| node.part == CollapsiblePart::Trigger)
                                .map(|node| node.label)
                                .unwrap_or_else(|| "Collapsible".to_owned())
                        })
                    }}
                </span>
                <span class=COLLAPSIBLE_INDICATOR aria-hidden="true">
                    {move || state.with(|state| collapsible_indicator_text(state.is_open()).to_owned())}
                </span>
            </button>
            {move || {
                state.with(|state| {
                    collapsible_render_nodes(&content_model, state)
                        .into_iter()
                        .find(|node| node.part == CollapsiblePart::Content)
                        .map(|node| {
                            view! {
                                <div
                                    class=collapsible_content_class(density, state.is_open())
                                    data-ui-part=CollapsiblePart::Content.label()
                                    hidden=!state.is_open()
                                >
                                    {node.detail}
                                </div>
                            }
                        })
                        .unwrap_or_else(|| {
                            view! {
                                <div
                                    class=COLLAPSIBLE_CONTENT_HIDDEN
                                    data-ui-part=CollapsiblePart::Content.label()
                                    hidden=true
                                >
                                    {String::new()}
                                </div>
                            }
                        })
                })
            }}
        </section>
    }
    .into_any()
}

const fn collapsible_root_class(density: CollapsibleDensity, disabled: bool) -> &'static str {
    if disabled {
        return COLLAPSIBLE_ROOT_DISABLED;
    }
    match density {
        CollapsibleDensity::Standard => COLLAPSIBLE_ROOT,
        CollapsibleDensity::Dense => COLLAPSIBLE_ROOT_DENSE,
    }
}

const fn collapsible_trigger_class(
    density: CollapsibleDensity,
    disabled: bool,
    open: bool,
) -> &'static str {
    if disabled {
        return COLLAPSIBLE_TRIGGER_DISABLED;
    }
    if open {
        return COLLAPSIBLE_TRIGGER_OPEN;
    }
    match density {
        CollapsibleDensity::Standard => COLLAPSIBLE_TRIGGER,
        CollapsibleDensity::Dense => COLLAPSIBLE_TRIGGER_DENSE,
    }
}

const fn collapsible_content_class(density: CollapsibleDensity, open: bool) -> &'static str {
    if !open {
        return COLLAPSIBLE_CONTENT_HIDDEN;
    }
    match density {
        CollapsibleDensity::Standard => COLLAPSIBLE_CONTENT,
        CollapsibleDensity::Dense => COLLAPSIBLE_CONTENT_DENSE,
    }
}

const fn collapsible_indicator_text(open: bool) -> &'static str {
    if open { "-" } else { "+" }
}

const fn collapsible_state_label(loading: bool, disabled: bool, open: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if open {
        "open"
    } else {
        "closed"
    }
}

#[component]
pub fn Combobox(
    #[prop(optional, default = default_combobox_model())] model: ComboboxModel,
) -> AnyView {
    if let Err(report) = validate_combobox_model(&model) {
        let message = format!("Combobox validation failed: {report}");
        return view! {
            <div class=COMBOBOX_ERROR data-ui-component="combobox" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let input_nodes = combobox_render_nodes(&model, &model.state());
    let root = input_nodes
        .iter()
        .find(|node| node.part == ComboboxPart::Root)
        .expect("invariant: combobox render nodes include root")
        .clone();
    let input = input_nodes
        .iter()
        .find(|node| node.part == ComboboxPart::Input)
        .expect("invariant: combobox render nodes include input")
        .clone();
    let list_model = model.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=combobox_root_class(density, disabled)
            data-ui-component="combobox"
            data-ui-part=ComboboxPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| combobox_state_label(loading, disabled, state.is_open()).to_owned())
            }
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <input
                type="text"
                role="combobox"
                class=combobox_input_class(density)
                data-ui-part=ComboboxPart::Input.label()
                data-ui-value=root.value
                placeholder=input.label
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                prop:value=move || state.with(|state| state.query().to_owned())
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ComboboxIntent::Open);
                        });
                    }
                }
                on:input=move |event| {
                    if !blocked {
                        let value = event_target_value(&event);
                        set_state.update(|state| {
                            let _ = state.apply(ComboboxIntent::Input(value));
                        });
                    }
                }
            />
            <div
                class=move || state.with(|state| combobox_list_class(state.is_open()).to_owned())
                data-ui-part=ComboboxPart::List.label()
                hidden=move || state.with(|state| !state.is_open())
            >
                {move || {
                    state.with(|state| {
                        let nodes = combobox_render_nodes(&list_model, state);
                        let option_nodes = nodes
                            .iter()
                            .filter(|node| node.part == ComboboxPart::Option)
                            .cloned()
                            .collect::<Vec<_>>();
                        if option_nodes.is_empty() {
                            let empty = nodes
                                .into_iter()
                                .find(|node| node.part == ComboboxPart::Empty)
                                .expect("invariant: combobox render nodes include empty");
                            view! {
                                <p
                                    class=COMBOBOX_EMPTY
                                    data-ui-part=ComboboxPart::Empty.label()
                                >
                                    {empty.label}
                                </p>
                            }
                            .into_any()
                        } else {
                            option_nodes
                                .into_iter()
                                .map(|node| {
                                    let value = node.value.clone();
                                    view! {
                                        <button
                                            type="button"
                                            class=combobox_option_class(node.selected, node.disabled)
                                            data-ui-part=ComboboxPart::Option.label()
                                            data-ui-value=node.value
                                            aria-selected=node.selected.to_string()
                                            disabled=node.disabled
                                            on:click=move |_| {
                                                if !blocked {
                                                    let value = value.clone();
                                                    set_state.update(|state| {
                                                        let _ = state.apply(ComboboxIntent::Select(value));
                                                    });
                                                }
                                            }
                                        >
                                            <span>{node.label}</span>
                                            <span class=COMBOBOX_META>
                                                {if node.selected { "selected" } else { "" }}
                                            </span>
                                        </button>
                                    }
                                })
                                .collect_view()
                                .into_any()
                        }
                    })
                }}
            </div>
        </section>
    }
    .into_any()
}

const fn combobox_root_class(density: ComboboxDensity, disabled: bool) -> &'static str {
    if disabled {
        return COMBOBOX_ROOT_DISABLED;
    }
    match density {
        ComboboxDensity::Standard => COMBOBOX_ROOT,
        ComboboxDensity::Dense => COMBOBOX_ROOT_DENSE,
    }
}

const fn combobox_input_class(density: ComboboxDensity) -> &'static str {
    match density {
        ComboboxDensity::Standard => COMBOBOX_INPUT,
        ComboboxDensity::Dense => COMBOBOX_INPUT_DENSE,
    }
}

const fn combobox_list_class(open: bool) -> &'static str {
    if open {
        COMBOBOX_LIST
    } else {
        COMBOBOX_LIST_HIDDEN
    }
}

const fn combobox_option_class(selected: bool, disabled: bool) -> &'static str {
    if disabled {
        COMBOBOX_OPTION_DISABLED
    } else if selected {
        COMBOBOX_OPTION_SELECTED
    } else {
        COMBOBOX_OPTION
    }
}

const fn combobox_state_label(loading: bool, disabled: bool, open: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if open {
        "open"
    } else {
        "closed"
    }
}

#[component]
pub fn Command(
    #[prop(optional, default = default_command_model())] model: CommandModel,
) -> AnyView {
    if let Err(report) = validate_command_model(&model) {
        let message = format!("Command validation failed: {report}");
        return view! {
            <div class=COMMAND_ERROR data-ui-component="command" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let input_nodes = command_render_nodes(&model, &model.state());
    let root = input_nodes
        .iter()
        .find(|node| node.part == CommandPart::Root)
        .expect("invariant: command render nodes include root")
        .clone();
    let input = input_nodes
        .iter()
        .find(|node| node.part == CommandPart::Input)
        .expect("invariant: command render nodes include input")
        .clone();
    let list_model = model.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=command_root_class(density, disabled)
            data-ui-component="command"
            data-ui-part=CommandPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| command_state_label(loading, disabled, state.is_open()).to_owned())
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <input
                type="text"
                role="searchbox"
                class=command_input_class(density)
                data-ui-part=CommandPart::Input.label()
                placeholder=input.label
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                prop:value=move || state.with(|state| state.query().to_owned())
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(CommandIntent::Open);
                        });
                    }
                }
                on:input=move |event| {
                    if !blocked {
                        let value = event_target_value(&event);
                        set_state.update(|state| {
                            let _ = state.apply(CommandIntent::Input(value));
                        });
                    }
                }
            />
            <div
                role="listbox"
                class=move || state.with(|state| command_list_class(state.is_open()).to_owned())
                data-ui-part=CommandPart::List.label()
                hidden=move || state.with(|state| !state.is_open())
            >
                {move || {
                    state.with(|state| {
                        let nodes = command_render_nodes(&list_model, state);
                        let has_items = nodes
                            .iter()
                            .any(|node| node.part == CommandPart::Item && node.visible);
                        if !has_items {
                            let list = nodes
                                .into_iter()
                                .find(|node| node.part == CommandPart::List)
                                .expect("invariant: command render nodes include list");
                            view! {
                                <p
                                    class=COMMAND_EMPTY
                                    data-ui-part=CommandPart::List.label()
                                >
                                    {list.label}
                                </p>
                            }
                            .into_any()
                        } else {
                            nodes
                                .into_iter()
                                .filter(|node| {
                                    matches!(node.part, CommandPart::Group | CommandPart::Item)
                                })
                                .map(|node| {
                                    match node.part {
                                        CommandPart::Group => view! {
                                            <div
                                                role="group"
                                                class=COMMAND_GROUP
                                                data-ui-part=CommandPart::Group.label()
                                                data-ui-value=node.value
                                            >
                                                <p class=COMMAND_GROUP_LABEL>{node.label}</p>
                                            </div>
                                        }
                                        .into_any(),
                                        CommandPart::Item => {
                                            let value_for_focus = node.value.clone();
                                            let value_for_click = node.value.clone();
                                            let label = node.label.clone();
                                            let detail = node.detail.clone();
                                            let shortcut = node.shortcut.clone();
                                            let selected = node.selected;
                                            let highlighted = node.highlighted;
                                            let disabled_item = node.disabled;
                                            view! {
                                                <button
                                                    type="button"
                                                    role="option"
                                                    class=command_item_class(selected, highlighted, disabled_item)
                                                    data-ui-part=CommandPart::Item.label()
                                                    data-ui-value=node.value
                                                    aria-selected=(selected || highlighted).to_string()
                                                    disabled=disabled_item
                                                    on:focus=move |_| {
                                                        if !blocked {
                                                            let value = value_for_focus.clone();
                                                            set_state.update(|state| {
                                                                let _ = state.apply(CommandIntent::Highlight(value));
                                                            });
                                                        }
                                                    }
                                                    on:click=move |_| {
                                                        if !blocked {
                                                            let value = value_for_click.clone();
                                                            set_state.update(|state| {
                                                                let _ = state.apply(CommandIntent::Select(value));
                                                            });
                                                        }
                                                    }
                                                >
                                                    <span class=COMMAND_ITEM_BODY>
                                                        <span class=COMMAND_ITEM_LABEL>{label}</span>
                                                        {if detail.is_empty() {
                                                            view! { <span></span> }.into_any()
                                                        } else {
                                                            view! { <span class=COMMAND_ITEM_DETAIL>{detail}</span> }.into_any()
                                                        }}
                                                    </span>
                                                    {if shortcut.is_empty() {
                                                        view! { <span></span> }.into_any()
                                                    } else {
                                                        view! {
                                                            <span
                                                                class=COMMAND_SHORTCUT
                                                                data-ui-part=CommandPart::Shortcut.label()
                                                            >
                                                                {shortcut}
                                                            </span>
                                                        }
                                                        .into_any()
                                                    }}
                                                </button>
                                            }
                                            .into_any()
                                        }
                                        CommandPart::Root
                                        | CommandPart::Input
                                        | CommandPart::List
                                        | CommandPart::Shortcut => view! { <span></span> }.into_any(),
                                    }
                                })
                                .collect_view()
                                .into_any()
                        }
                    })
                }}
            </div>
        </section>
    }
    .into_any()
}

const fn command_root_class(density: CommandDensity, disabled: bool) -> &'static str {
    if disabled {
        return COMMAND_ROOT_DISABLED;
    }
    match density {
        CommandDensity::Standard => COMMAND_ROOT,
        CommandDensity::Dense => COMMAND_ROOT_DENSE,
    }
}

const fn command_input_class(density: CommandDensity) -> &'static str {
    match density {
        CommandDensity::Standard => COMMAND_INPUT,
        CommandDensity::Dense => COMMAND_INPUT_DENSE,
    }
}

const fn command_list_class(open: bool) -> &'static str {
    if open {
        COMMAND_LIST
    } else {
        COMMAND_LIST_HIDDEN
    }
}

const fn command_item_class(selected: bool, highlighted: bool, disabled: bool) -> &'static str {
    if disabled {
        COMMAND_ITEM_DISABLED
    } else if selected {
        COMMAND_ITEM_SELECTED
    } else if highlighted {
        COMMAND_ITEM_HIGHLIGHTED
    } else {
        COMMAND_ITEM
    }
}

const fn command_state_label(loading: bool, disabled: bool, open: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if open {
        "open"
    } else {
        "closed"
    }
}

#[component]
pub fn ContextMenu(
    #[prop(optional, default = default_context_menu_model())] model: ContextMenuModel,
) -> AnyView {
    if let Err(report) = validate_context_menu_model(&model) {
        let message = format!("ContextMenu validation failed: {report}");
        return view! {
            <div class=CONTEXT_MENU_ERROR data-ui-component="context-menu" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let input_nodes = context_menu_render_nodes(&model, &model.state());
    let root = input_nodes
        .iter()
        .find(|node| node.part == ContextMenuPart::Root)
        .expect("invariant: context menu render nodes include root")
        .clone();
    let trigger = input_nodes
        .iter()
        .find(|node| node.part == ContextMenuPart::Trigger)
        .expect("invariant: context menu render nodes include trigger")
        .clone();
    let list_model = model.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=context_menu_root_class(density, disabled)
            data-ui-component="context-menu"
            data-ui-part=ContextMenuPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| context_menu_state_label(loading, disabled, state.is_open()).to_owned())
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <button
                type="button"
                class=context_menu_trigger_class(density)
                data-ui-part=ContextMenuPart::Trigger.label()
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ContextMenuIntent::Toggle);
                        });
                    }
                }
                on:contextmenu=move |event| {
                    event.prevent_default();
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(ContextMenuIntent::Open);
                        });
                    }
                }
            >
                {trigger.label}
            </button>
            <div
                role="menu"
                class=move || state.with(|state| context_menu_content_class(state.is_open()).to_owned())
                data-ui-part=ContextMenuPart::Content.label()
                hidden=move || state.with(|state| !state.is_open())
            >
                {move || {
                    state.with(|state| {
                        context_menu_render_nodes(&list_model, state)
                            .into_iter()
                            .filter(|node| {
                                matches!(
                                    node.part,
                                    ContextMenuPart::Item
                                        | ContextMenuPart::Separator
                                        | ContextMenuPart::Submenu
                                ) && node.visible
                            })
                            .map(|node| match node.part {
                                ContextMenuPart::Item => context_menu_item_view(node, blocked, set_state),
                                ContextMenuPart::Separator => view! {
                                    <div
                                        role="separator"
                                        class=CONTEXT_MENU_SEPARATOR
                                        data-ui-part=ContextMenuPart::Separator.label()
                                        data-ui-value=node.value
                                    ></div>
                                }
                                .into_any(),
                                ContextMenuPart::Submenu => context_menu_submenu_view(node, blocked, set_state),
                                ContextMenuPart::Root
                                | ContextMenuPart::Trigger
                                | ContextMenuPart::Content => view! { <span></span> }.into_any(),
                            })
                            .collect_view()
                    })
                }}
            </div>
        </section>
    }
    .into_any()
}

fn context_menu_item_view(
    node: crate::ContextMenuRenderNode,
    blocked: bool,
    set_state: WriteSignal<ContextMenuState>,
) -> AnyView {
    let value_for_focus = node.value.clone();
    let value_for_click = node.value.clone();
    let label = node.label.clone();
    let detail = node.detail.clone();
    let shortcut = node.shortcut.clone();
    let selected = node.selected;
    let active = node.active;
    let disabled = node.disabled;
    let destructive = node.destructive;
    view! {
        <button
            type="button"
            role="menuitem"
            class=context_menu_item_class(selected, active, disabled, destructive)
            data-ui-part=ContextMenuPart::Item.label()
            data-ui-value=node.value
            data-ui-parent=node.parent_value
            aria-selected=(selected || active).to_string()
            disabled=disabled
            on:focus=move |_| {
                if !blocked {
                    let value = value_for_focus.clone();
                    set_state.update(|state| {
                        let _ = state.apply(ContextMenuIntent::Focus(value));
                    });
                }
            }
            on:click=move |_| {
                if !blocked {
                    let value = value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(ContextMenuIntent::Select(value));
                    });
                }
            }
        >
            <span class=CONTEXT_MENU_ITEM_BODY>
                <span class=CONTEXT_MENU_ITEM_LABEL>{label}</span>
                {if detail.is_empty() {
                    view! { <span></span> }.into_any()
                } else {
                    view! { <span class=CONTEXT_MENU_ITEM_DETAIL>{detail}</span> }.into_any()
                }}
            </span>
            {if shortcut.is_empty() {
                view! { <span></span> }.into_any()
            } else {
                view! { <span class=CONTEXT_MENU_SHORTCUT>{shortcut}</span> }.into_any()
            }}
        </button>
    }
    .into_any()
}

fn context_menu_submenu_view(
    node: crate::ContextMenuRenderNode,
    blocked: bool,
    set_state: WriteSignal<ContextMenuState>,
) -> AnyView {
    let value_for_focus = node.value.clone();
    let value_for_click = node.value.clone();
    let disabled = node.disabled;
    let submenu_open = node.submenu_open;
    view! {
        <button
            type="button"
            role="menuitem"
            class=context_menu_submenu_class(submenu_open, disabled)
            data-ui-part=ContextMenuPart::Submenu.label()
            data-ui-value=node.value
            aria-expanded=submenu_open.to_string()
            disabled=disabled
            on:focus=move |_| {
                if !blocked {
                    let value = value_for_focus.clone();
                    set_state.update(|state| {
                        let _ = state.apply(ContextMenuIntent::Focus(value));
                    });
                }
            }
            on:click=move |_| {
                if !blocked {
                    let value = value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(ContextMenuIntent::OpenSubmenu(value));
                    });
                }
            }
        >
            <span>{node.label}</span>
            <span class=CONTEXT_MENU_SUBMENU_MARKER>{if submenu_open { "open" } else { ">" }}</span>
        </button>
    }
    .into_any()
}

const fn context_menu_root_class(density: ContextMenuDensity, disabled: bool) -> &'static str {
    if disabled {
        return CONTEXT_MENU_ROOT_DISABLED;
    }
    match density {
        ContextMenuDensity::Standard => CONTEXT_MENU_ROOT,
        ContextMenuDensity::Dense => CONTEXT_MENU_ROOT_DENSE,
    }
}

const fn context_menu_trigger_class(density: ContextMenuDensity) -> &'static str {
    match density {
        ContextMenuDensity::Standard => CONTEXT_MENU_TRIGGER,
        ContextMenuDensity::Dense => CONTEXT_MENU_TRIGGER_DENSE,
    }
}

const fn context_menu_content_class(open: bool) -> &'static str {
    if open {
        CONTEXT_MENU_CONTENT
    } else {
        CONTEXT_MENU_CONTENT_HIDDEN
    }
}

const fn context_menu_item_class(
    selected: bool,
    active: bool,
    disabled: bool,
    destructive: bool,
) -> &'static str {
    if disabled {
        CONTEXT_MENU_ITEM_DISABLED
    } else if destructive {
        CONTEXT_MENU_ITEM_DANGER
    } else if selected {
        CONTEXT_MENU_ITEM_SELECTED
    } else if active {
        CONTEXT_MENU_ITEM_ACTIVE
    } else {
        CONTEXT_MENU_ITEM
    }
}

const fn context_menu_submenu_class(open: bool, disabled: bool) -> &'static str {
    if disabled {
        CONTEXT_MENU_SUBMENU_DISABLED
    } else if open {
        CONTEXT_MENU_SUBMENU_OPEN
    } else {
        CONTEXT_MENU_SUBMENU
    }
}

const fn context_menu_state_label(loading: bool, disabled: bool, open: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if open {
        "open"
    } else {
        "closed"
    }
}

#[component]
pub fn DataTable(
    #[prop(optional, default = default_data_table_model())] model: DataTableModel,
) -> AnyView {
    if let Err(report) = validate_data_table_model(&model) {
        let message = format!("DataTable validation failed: {report}");
        return view! {
            <div class=DATA_TABLE_ERROR data-ui-component="data-table" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let input_nodes = data_table_render_nodes(&model, &model.state());
    let root = input_nodes
        .iter()
        .find(|node| node.part == DataTablePart::Root)
        .expect("invariant: data table render nodes include root")
        .clone();
    let toolbar = input_nodes
        .iter()
        .find(|node| node.part == DataTablePart::Toolbar)
        .expect("invariant: data table render nodes include toolbar")
        .clone();
    let header_model = model.clone();
    let body_model = model.clone();
    let pagination_label_model = model.clone();
    let next_page_model = model.clone();
    let next_disabled_model = model.clone();
    let column_count = model.columns.len();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=data_table_root_class(density, disabled)
            data-ui-component="data-table"
            data-ui-part=DataTablePart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| data_table_state_label(loading, disabled, state.selected_row().is_some()).to_owned())
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <div class=DATA_TABLE_TOOLBAR data-ui-part=DataTablePart::Toolbar.label()>
                <h3 class=DATA_TABLE_TITLE>{root.label}</h3>
                <input
                    type="search"
                    role="searchbox"
                    class=data_table_filter_class(density)
                    placeholder=toolbar.label
                    disabled=blocked
                    prop:value=move || state.with(|state| state.filter().to_owned())
                    on:input=move |event| {
                        if !blocked {
                            let value = event_target_value(&event);
                            set_state.update(|state| {
                                let max_page = 0;
                                let _ = state.apply(DataTableIntent::Filter(value), max_page);
                            });
                        }
                    }
                />
            </div>
            <div class=DATA_TABLE_FRAME>
                <table class=DATA_TABLE_TABLE role="table">
                    <thead>
                        <tr>
                            {move || {
                                state.with(|state| {
                                    data_table_render_nodes(&header_model, state)
                                        .into_iter()
                                        .filter(|node| node.part == DataTablePart::Header)
                                        .map(|node| data_table_header_view(node, blocked, set_state))
                                        .collect_view()
                                })
                            }}
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            state.with(|state| {
                                let nodes = data_table_render_nodes(&body_model, state);
                                let row_nodes = nodes
                                    .iter()
                                    .filter(|node| node.part == DataTablePart::Row)
                                    .cloned()
                                    .collect::<Vec<_>>();
                                let cell_nodes = nodes
                                    .iter()
                                    .filter(|node| node.part == DataTablePart::Cell)
                                    .cloned()
                                    .collect::<Vec<_>>();
                                if row_nodes.len() == 1 && row_nodes[0].value == "empty" {
                                    let empty = row_nodes[0].clone();
                                    view! {
                                        <tr class=DATA_TABLE_ROW_DISABLED data-ui-part=DataTablePart::Row.label()>
                                            <td class=DATA_TABLE_EMPTY colspan=column_count.to_string()>
                                                {empty.label}
                                            </td>
                                        </tr>
                                    }
                                    .into_any()
                                } else {
                                    row_nodes
                                        .into_iter()
                                        .map(|row| {
                                            let cells = cell_nodes
                                                .iter()
                                                .filter(|cell| cell.row_value == row.value)
                                                .cloned()
                                                .collect::<Vec<_>>();
                                            data_table_row_view(row, cells, blocked, set_state)
                                        })
                                        .collect_view()
                                        .into_any()
                                }
                            })
                        }}
                    </tbody>
                </table>
            </div>
            <footer class=DATA_TABLE_PAGINATION data-ui-part=DataTablePart::Pagination.label()>
                <button
                    type="button"
                    class=DATA_TABLE_PAGE_BUTTON
                    disabled=move || state.with(|state| blocked || state.page_index() == 0)
                    on:click=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(DataTableIntent::PreviousPage, 0);
                            });
                        }
                    }
                >
                    "Previous"
                </button>
                <p class=DATA_TABLE_PAGE_LABEL>
                    {move || {
                        state.with(|state| {
                            data_table_render_nodes(&pagination_label_model, state)
                                .into_iter()
                                .find(|node| node.part == DataTablePart::Pagination)
                                .map(|node| node.label)
                                .unwrap_or_else(|| "Page 1 of 1".to_owned())
                        })
                    }}
                </p>
                <button
                    type="button"
                    class=DATA_TABLE_PAGE_BUTTON
                    disabled=move || {
                        state.with(|state| {
                            blocked
                                || state.page_index()
                                    >= max_data_table_page_index(&next_disabled_model, state)
                        })
                    }
                    on:click=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let max_page = max_data_table_page_index(&next_page_model, state);
                                let _ = state.apply(DataTableIntent::NextPage, max_page);
                            });
                        }
                    }
                >
                    "Next"
                </button>
            </footer>
        </section>
    }
    .into_any()
}

fn data_table_header_view(
    node: crate::DataTableRenderNode,
    blocked: bool,
    set_state: WriteSignal<DataTableState>,
) -> AnyView {
    let value_for_click = node.value.clone();
    let disabled = node.disabled;
    let sort_marker = data_table_sort_marker(node.sort_direction);
    view! {
        <th
            scope="col"
            class=data_table_header_class(node.selected, disabled)
            data-ui-part=DataTablePart::Header.label()
            data-ui-column=node.column_value
            aria-sort=data_table_aria_sort(node.sort_direction)
        >
            <button
                type="button"
                class=DATA_TABLE_HEADER_BUTTON
                disabled=disabled
                on:click=move |_| {
                    if !blocked && !disabled {
                        let value = value_for_click.clone();
                        set_state.update(|state| {
                            let _ = state.apply(DataTableIntent::Sort(value), 0);
                        });
                    }
                }
            >
                <span>{node.label}</span>
                <span class=DATA_TABLE_SORT_MARKER>{sort_marker}</span>
            </button>
        </th>
    }
    .into_any()
}

fn data_table_row_view(
    row: crate::DataTableRenderNode,
    cells: Vec<crate::DataTableRenderNode>,
    blocked: bool,
    set_state: WriteSignal<DataTableState>,
) -> AnyView {
    let row_value = row.value.clone();
    let row_value_for_click = row.value.clone();
    let disabled = row.disabled;
    let density = row.density;
    view! {
        <tr
            class=data_table_row_class(row.selected, disabled)
            data-ui-part=DataTablePart::Row.label()
            data-ui-row=row_value
            aria-selected=row.selected.to_string()
            aria-disabled=disabled.to_string()
            on:click=move |_| {
                if !blocked && !disabled {
                    let value = row_value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(DataTableIntent::SelectRow(value), 0);
                    });
                }
            }
        >
            {cells
                .into_iter()
                .map(|cell| {
                    view! {
                        <td
                            class=data_table_cell_class(density)
                            data-ui-part=DataTablePart::Cell.label()
                            data-ui-row=cell.row_value
                            data-ui-column=cell.column_value
                        >
                            {cell.label}
                        </td>
                    }
                })
                .collect_view()}
        </tr>
    }
    .into_any()
}

const fn data_table_root_class(density: DataTableDensity, disabled: bool) -> &'static str {
    if disabled {
        return DATA_TABLE_ROOT_DISABLED;
    }
    match density {
        DataTableDensity::Standard => DATA_TABLE_ROOT,
        DataTableDensity::Dense => DATA_TABLE_ROOT_DENSE,
    }
}

const fn data_table_filter_class(density: DataTableDensity) -> &'static str {
    match density {
        DataTableDensity::Standard => DATA_TABLE_FILTER,
        DataTableDensity::Dense => DATA_TABLE_FILTER_DENSE,
    }
}

const fn data_table_header_class(selected: bool, disabled: bool) -> &'static str {
    if selected && !disabled {
        DATA_TABLE_HEADER_SORTED
    } else {
        DATA_TABLE_HEADER
    }
}

const fn data_table_row_class(selected: bool, disabled: bool) -> &'static str {
    if disabled {
        DATA_TABLE_ROW_DISABLED
    } else if selected {
        DATA_TABLE_ROW_SELECTED
    } else {
        DATA_TABLE_ROW
    }
}

const fn data_table_cell_class(density: DataTableDensity) -> &'static str {
    match density {
        DataTableDensity::Standard => DATA_TABLE_CELL,
        DataTableDensity::Dense => DATA_TABLE_CELL_DENSE,
    }
}

const fn data_table_sort_marker(direction: Option<crate::DataTableSortDirection>) -> &'static str {
    match direction {
        Some(crate::DataTableSortDirection::Ascending) => "asc",
        Some(crate::DataTableSortDirection::Descending) => "desc",
        None => "-",
    }
}

const fn data_table_aria_sort(direction: Option<crate::DataTableSortDirection>) -> &'static str {
    match direction {
        Some(crate::DataTableSortDirection::Ascending) => "ascending",
        Some(crate::DataTableSortDirection::Descending) => "descending",
        None => "none",
    }
}

const fn data_table_state_label(loading: bool, disabled: bool, selected: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if selected {
        "selected"
    } else {
        "ready"
    }
}

#[component]
pub fn DatePicker(
    #[prop(optional, default = default_date_picker_model())] model: DatePickerModel,
) -> AnyView {
    if let Err(report) = validate_date_picker_model(&model) {
        let message = format!("DatePicker validation failed: {report}");
        return view! {
            <div class=DATE_PICKER_ERROR data-ui-component="date-picker" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let input_nodes = date_picker_render_nodes(&model, &model.state());
    let root = input_nodes
        .iter()
        .find(|node| node.part == DatePickerPart::Root)
        .expect("invariant: date picker render nodes include root")
        .clone();
    let trigger = input_nodes
        .iter()
        .find(|node| node.part == DatePickerPart::Trigger)
        .expect("invariant: date picker render nodes include trigger")
        .clone();
    let value = input_nodes
        .iter()
        .find(|node| node.part == DatePickerPart::Value)
        .expect("invariant: date picker render nodes include value")
        .clone();
    let value_model = model.clone();
    let days_model = model.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=date_picker_root_class(density, disabled)
            data-ui-component="date-picker"
            data-ui-part=DatePickerPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    date_picker_state_label(loading, disabled, state.is_open(), state.selected().is_some()).to_owned()
                })
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <button
                type="button"
                class=move || state.with(|state| date_picker_trigger_class(density, state.is_open(), disabled).to_owned())
                data-ui-part=DatePickerPart::Trigger.label()
                data-ui-value=trigger.value
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(DatePickerIntent::Focus);
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(DatePickerIntent::Blur);
                        });
                    }
                }
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(DatePickerIntent::Toggle);
                        });
                    }
                }
            >
                <span>{trigger.label}</span>
                <span
                    class=move || {
                        state.with(|state| date_picker_value_class(state.selected().is_some()).to_owned())
                    }
                    data-ui-part=DatePickerPart::Value.label()
                    data-ui-value=value.value
                >
                    {move || {
                        state.with(|state| {
                            date_picker_render_nodes(&value_model, state)
                                .into_iter()
                                .find(|node| node.part == DatePickerPart::Value)
                                .map(|node| node.label)
                                .unwrap_or_else(|| "Select date".to_owned())
                        })
                    }}
                </span>
                <span class=DATE_PICKER_MARKER aria-hidden="true">"cal"</span>
            </button>
            <div
                class=move || state.with(|state| date_picker_popover_class(state.is_open()).to_owned())
                data-ui-part=DatePickerPart::Popover.label()
                hidden=move || state.with(|state| !state.is_open())
            >
                <header class=DATE_PICKER_POPOVER_HEADER>
                    <button
                        type="button"
                        class=CALENDAR_NAV
                        aria-label="Previous month"
                        disabled=blocked
                        on:click=move |_| {
                            if !blocked {
                                set_state.update(|state| {
                                    let _ = state.apply(DatePickerIntent::PreviousMonth);
                                });
                            }
                        }
                    >
                        "<"
                    </button>
                    <h3 class=CALENDAR_TITLE>
                        {move || {
                            state.with(|state| {
                                format!("{} {}", month_name(state.visible_month()), state.visible_year())
                            })
                        }}
                    </h3>
                    <button
                        type="button"
                        class=CALENDAR_NAV
                        aria-label="Next month"
                        disabled=blocked
                        on:click=move |_| {
                            if !blocked {
                                set_state.update(|state| {
                                    let _ = state.apply(DatePickerIntent::NextMonth);
                                });
                            }
                        }
                    >
                        ">"
                    </button>
                </header>
                <div class=CALENDAR_GRID data-ui-part=DatePickerPart::Calendar.label() role="grid">
                    {["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
                        .into_iter()
                        .map(|weekday| {
                            view! {
                                <span class=CALENDAR_WEEKDAY role="columnheader">{weekday}</span>
                            }
                        })
                        .collect_view()}
                    {move || {
                        state.with(|state| {
                            date_picker_render_nodes(&days_model, state)
                                .into_iter()
                                .filter(|node| {
                                    node.part == DatePickerPart::Calendar && node.date.is_some()
                                })
                                .map(|node| date_picker_day_view(node, blocked, set_state))
                                .collect_view()
                        })
                    }}
                </div>
                <button
                    type="button"
                    class=DATE_PICKER_CLEAR
                    data-ui-part=DatePickerPart::Value.label()
                    disabled=move || state.with(|state| blocked || state.selected().is_none())
                    on:click=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(DatePickerIntent::Clear);
                            });
                        }
                    }
                >
                    "Clear"
                </button>
            </div>
        </section>
    }
    .into_any()
}

fn date_picker_day_view(
    node: crate::DatePickerRenderNode,
    blocked: bool,
    set_state: WriteSignal<DatePickerState>,
) -> AnyView {
    let date = node
        .date
        .expect("invariant: date picker calendar day nodes include a date");
    let item_disabled = node.disabled || blocked;
    view! {
        <button
            type="button"
            class=calendar_day_class(node.current_month, node.selected, false)
            data-ui-part=DatePickerPart::Calendar.label()
            data-ui-value=node.value
            data-ui-index=node.index.to_string()
            aria-label=node.detail
            aria-pressed=node.selected.to_string()
            disabled=item_disabled
            on:click=move |_| {
                if !item_disabled {
                    set_state.update(|state| {
                        let _ = state.apply(DatePickerIntent::Select(date));
                    });
                }
            }
        >
            {node.label}
        </button>
    }
    .into_any()
}

const fn date_picker_root_class(density: DatePickerDensity, disabled: bool) -> &'static str {
    if disabled {
        return DATE_PICKER_ROOT_DISABLED;
    }
    match density {
        DatePickerDensity::Standard => DATE_PICKER_ROOT,
        DatePickerDensity::Dense => DATE_PICKER_ROOT_DENSE,
    }
}

const fn date_picker_trigger_class(
    density: DatePickerDensity,
    open: bool,
    disabled: bool,
) -> &'static str {
    if open && !disabled {
        DATE_PICKER_TRIGGER_OPEN
    } else {
        match density {
            DatePickerDensity::Standard => DATE_PICKER_TRIGGER,
            DatePickerDensity::Dense => DATE_PICKER_TRIGGER_DENSE,
        }
    }
}

const fn date_picker_value_class(selected: bool) -> &'static str {
    if selected {
        DATE_PICKER_VALUE
    } else {
        DATE_PICKER_VALUE_EMPTY
    }
}

const fn date_picker_popover_class(open: bool) -> &'static str {
    if open {
        DATE_PICKER_POPOVER
    } else {
        DATE_PICKER_POPOVER_HIDDEN
    }
}

const fn date_picker_state_label(
    loading: bool,
    disabled: bool,
    open: bool,
    selected: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if open {
        "open"
    } else if selected {
        "selected"
    } else {
        "empty"
    }
}

#[component]
pub fn Dialog(#[prop(optional, default = default_dialog_model())] model: DialogModel) -> AnyView {
    if let Err(report) = validate_dialog_model(&model) {
        let message = format!("Dialog validation failed: {report}");
        return view! {
            <div class=DIALOG_ERROR data-ui-component="dialog" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let size = model.size;
    let mode = model.mode;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let nodes = dialog_render_nodes(&model, &model.state());
    let root = nodes
        .iter()
        .find(|node| node.part == DialogPart::Root)
        .expect("invariant: dialog render nodes include root")
        .clone();
    let trigger = nodes
        .iter()
        .find(|node| node.part == DialogPart::Trigger)
        .expect("invariant: dialog render nodes include trigger")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == DialogPart::Content)
        .expect("invariant: dialog render nodes include content")
        .clone();
    let title = nodes
        .iter()
        .find(|node| node.part == DialogPart::Title)
        .expect("invariant: dialog render nodes include title")
        .clone();
    let description = nodes
        .iter()
        .find(|node| node.part == DialogPart::Description)
        .expect("invariant: dialog render nodes include description")
        .clone();
    let footer_model = model.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=dialog_root_class(disabled)
            data-ui-component="dialog"
            data-ui-part=DialogPart::Root.label()
            data-ui-size=size.label()
            data-ui-mode=mode.label()
            data-ui-state=move || {
                state.with(|state| dialog_state_label(loading, disabled, state.is_open()).to_owned())
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <button
                type="button"
                class=move || state.with(|state| dialog_trigger_class(state.is_open()).to_owned())
                data-ui-part=DialogPart::Trigger.label()
                data-ui-value=trigger.value
                aria-haspopup="dialog"
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(DialogIntent::Focus);
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(DialogIntent::Blur);
                        });
                    }
                }
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(DialogIntent::Toggle);
                        });
                    }
                }
            >
                {trigger.label}
            </button>
            {move || {
                state.with(|state| {
                    if !state.is_open() {
                        return ().into_any();
                    }
                    let footer_nodes = dialog_render_nodes(&footer_model, state)
                        .into_iter()
                        .filter(|node| node.part == DialogPart::Footer)
                        .collect::<Vec<_>>();
                    view! {
                        <div class=dialog_overlay_class(mode) data-ui-part=DialogPart::Root.label()>
                            <article
                                role="dialog"
                                aria-modal=(mode == DialogMode::Modal).to_string()
                                class=dialog_content_class(size)
                                data-ui-part=DialogPart::Content.label()
                                data-ui-value=content.value.clone()
                            >
                                <header class=DIALOG_HEADER data-ui-part=DialogPart::Header.label()>
                                    <h3 class=dialog_title_class(size) data-ui-part=DialogPart::Title.label()>
                                        {title.label.clone()}
                                    </h3>
                                    <p class=dialog_description_class(size) data-ui-part=DialogPart::Description.label()>
                                        {description.detail.clone()}
                                    </p>
                                </header>
                                <p class=DIALOG_BODY>{content.detail.clone()}</p>
                                <footer class=DIALOG_FOOTER data-ui-part=DialogPart::Footer.label()>
                                    {footer_nodes
                                        .into_iter()
                                        .map(|node| dialog_footer_action_view(node, blocked, set_state))
                                        .collect_view()}
                                </footer>
                            </article>
                        </div>
                    }
                    .into_any()
                })
            }}
        </section>
    }
    .into_any()
}

fn dialog_footer_action_view(
    node: crate::DialogRenderNode,
    blocked: bool,
    set_state: WriteSignal<DialogState>,
) -> AnyView {
    let value_for_click = node.value.clone();
    let close_dialog = node.close_dialog;
    let disabled = node.disabled || blocked || !node.actionable;
    view! {
        <button
            type="button"
            class=dialog_action_class(node.index, node.selected)
            data-ui-part=DialogPart::Footer.label()
            data-ui-value=node.value
            disabled=disabled
            on:click=move |_| {
                if !disabled {
                    let value = value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(DialogIntent::ActivateFooter(value));
                        if close_dialog {
                            let _ = state.apply(DialogIntent::Close);
                        }
                    });
                }
            }
        >
            {node.label}
        </button>
    }
    .into_any()
}

const fn dialog_root_class(disabled: bool) -> &'static str {
    if disabled {
        DIALOG_ROOT_DISABLED
    } else {
        DIALOG_ROOT
    }
}

const fn dialog_overlay_class(mode: DialogMode) -> &'static str {
    match mode {
        DialogMode::Modal => DIALOG_OVERLAY_MODAL,
        DialogMode::NonModal => DIALOG_OVERLAY_NON_MODAL,
    }
}

const fn dialog_content_class(size: DialogSize) -> &'static str {
    match size {
        DialogSize::Default => DIALOG_CONTENT,
        DialogSize::Small => DIALOG_CONTENT_SMALL,
    }
}

const fn dialog_title_class(size: DialogSize) -> &'static str {
    match size {
        DialogSize::Default => DIALOG_TITLE,
        DialogSize::Small => DIALOG_TITLE_SMALL,
    }
}

const fn dialog_description_class(size: DialogSize) -> &'static str {
    match size {
        DialogSize::Default => DIALOG_DESCRIPTION,
        DialogSize::Small => DIALOG_DESCRIPTION_SMALL,
    }
}

const fn dialog_trigger_class(open: bool) -> &'static str {
    if open {
        DIALOG_TRIGGER_OPEN
    } else {
        DIALOG_TRIGGER
    }
}

const fn dialog_action_class(index: usize, selected: bool) -> &'static str {
    if selected {
        DIALOG_ACTION_ACTIVE
    } else if index == 0 {
        DIALOG_ACTION
    } else {
        DIALOG_ACTION_SECONDARY
    }
}

const fn dialog_state_label(loading: bool, disabled: bool, open: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if open {
        "open"
    } else {
        "closed"
    }
}

#[component]
pub fn Direction(
    #[prop(optional, default = default_direction_model())] model: DirectionModel,
) -> AnyView {
    if let Err(report) = validate_direction_model(&model) {
        let message = format!("Direction validation failed: {report}");
        return view! {
            <div class=DIRECTION_ERROR data-ui-component="direction" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let nodes = direction_render_nodes(&model, &model.state());
    let provider = nodes
        .iter()
        .find(|node| node.part == DirectionPart::Provider)
        .expect("invariant: direction render nodes include provider")
        .clone();
    let scope = nodes
        .iter()
        .find(|node| node.part == DirectionPart::Scope)
        .expect("invariant: direction render nodes include scope")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == DirectionPart::AwareContent)
        .expect("invariant: direction render nodes include aware content")
        .clone();
    let scope_direction = model.scope_direction;
    let scope_detail_model = model.clone();
    let content_class_model = model.clone();
    let content_dir_model = model.clone();
    let content_value_model = model.clone();
    let content_badge_model = model.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=direction_root_class(disabled)
            dir=move || state.with(|state| state.direction().label())
            data-ui-component="direction"
            data-ui-part=DirectionPart::Provider.label()
            data-ui-state=move || {
                state.with(|state| {
                    direction_state_label(loading, disabled, state.direction(), state.scope_active()).to_owned()
                })
            }
            data-ui-value=provider.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <header class=DIRECTION_HEADER>
                <p class=DIRECTION_EYEBROW>{provider.label}</p>
                <h3 class=DIRECTION_TITLE>
                    {move || state.with(|state| state.direction().name())}
                </h3>
                <p class=DIRECTION_DETAIL>
                    {move || {
                        state.with(|state| {
                            format!(
                                "Provider dir=\"{}\". Nested scope target is {}.",
                                state.direction().label(),
                                scope_direction.name(),
                            )
                        })
                    }}
                </p>
            </header>
            <div class=DIRECTION_ACTIONS>
                <button
                    type="button"
                    class=move || {
                        state.with(|state| {
                            direction_action_class(state.direction().is_rtl()).to_owned()
                        })
                    }
                    data-ui-part=DirectionPart::Provider.label()
                    data-ui-value=move || state.with(|state| state.direction().label())
                    aria-pressed=move || state.with(|state| state.direction().is_rtl().to_string())
                    disabled=blocked
                    on:focus=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(DirectionIntent::Focus(DirectionPart::Provider));
                            });
                        }
                    }
                    on:blur=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(DirectionIntent::Blur);
                            });
                        }
                    }
                    on:click=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(DirectionIntent::Toggle);
                            });
                        }
                    }
                >
                    {move || {
                        state.with(|state| {
                            format!("Switch to {}", state.direction().next().label())
                        })
                    }}
                </button>
                <button
                    type="button"
                    class=move || {
                        state.with(|state| direction_action_class(state.scope_active()).to_owned())
                    }
                    data-ui-part=DirectionPart::Scope.label()
                    data-ui-value=scope.value.clone()
                    aria-pressed=move || state.with(|state| state.scope_active().to_string())
                    disabled=blocked
                    on:focus=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(DirectionIntent::Focus(DirectionPart::Scope));
                            });
                        }
                    }
                    on:blur=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(DirectionIntent::Blur);
                            });
                        }
                    }
                    on:click=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(DirectionIntent::ToggleScope);
                            });
                        }
                    }
                >
                    {move || {
                        state.with(|state| {
                            if state.scope_active() {
                                "Use provider flow".to_owned()
                            } else {
                                format!("Use {} scope", scope_direction.label())
                            }
                        })
                    }}
                </button>
            </div>
            <article
                class=move || {
                    state.with(|state| {
                        direction_scope_class(state.scope_active(), disabled).to_owned()
                    })
                }
                dir=scope_direction.label()
                data-ui-part=DirectionPart::Scope.label()
                data-ui-value=scope.value
            >
                <span class=DIRECTION_BADGE>
                    {move || {
                        state.with(|state| {
                            if state.scope_active() {
                                format!("scope {}", scope_direction.label())
                            } else {
                                "scope idle".to_owned()
                            }
                        })
                    }}
                </span>
                <p class=DIRECTION_DETAIL>
                    {move || {
                        state.with(|state| {
                            direction_render_nodes(&scope_detail_model, state)
                                .into_iter()
                                .find(|node| node.part == DirectionPart::Scope)
                                .map(|node| node.detail)
                                .unwrap_or_else(|| "Direction scope unavailable".to_owned())
                        })
                    }}
                </p>
                <div
                    class=move || {
                        state.with(|state| {
                            direction_content_class(state.effective_direction(&content_class_model)).to_owned()
                        })
                    }
                    dir=move || state.with(|state| state.effective_direction(&content_dir_model).label())
                    data-ui-part=DirectionPart::AwareContent.label()
                    data-ui-value=move || {
                        state.with(|state| state.effective_direction(&content_value_model).label())
                    }
                >
                    <span class=DIRECTION_BADGE>
                        {move || {
                            state.with(|state| {
                                format!(
                                    "effective {}",
                                    state.effective_direction(&content_badge_model).label()
                                )
                            })
                        }}
                    </span>
                    <p class=DIRECTION_DETAIL>{content.detail}</p>
                </div>
            </article>
        </section>
    }
    .into_any()
}

const fn direction_root_class(disabled: bool) -> &'static str {
    if disabled {
        DIRECTION_ROOT_DISABLED
    } else {
        DIRECTION_ROOT
    }
}

const fn direction_action_class(active: bool) -> &'static str {
    if active {
        DIRECTION_ACTION_ACTIVE
    } else {
        DIRECTION_ACTION
    }
}

const fn direction_scope_class(active: bool, disabled: bool) -> &'static str {
    if disabled {
        DIRECTION_SCOPE_DISABLED
    } else if active {
        DIRECTION_SCOPE_ACTIVE
    } else {
        DIRECTION_SCOPE
    }
}

const fn direction_content_class(direction: DirectionValue) -> &'static str {
    match direction {
        DirectionValue::Ltr => DIRECTION_CONTENT,
        DirectionValue::Rtl => DIRECTION_CONTENT_RTL,
    }
}

const fn direction_state_label(
    loading: bool,
    disabled: bool,
    direction: DirectionValue,
    scope_active: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if scope_active {
        "scoped"
    } else {
        direction.label()
    }
}

#[component]
pub fn Drawer(#[prop(optional, default = default_drawer_model())] model: DrawerModel) -> AnyView {
    if let Err(report) = validate_drawer_model(&model) {
        let message = format!("Drawer validation failed: {report}");
        return view! {
            <div class=DRAWER_ERROR data-ui-component="drawer" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let side = model.side;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let nodes = drawer_render_nodes(&model, &model.state());
    let root = nodes
        .iter()
        .find(|node| node.part == DrawerPart::Root)
        .expect("invariant: drawer render nodes include root")
        .clone();
    let trigger = nodes
        .iter()
        .find(|node| node.part == DrawerPart::Trigger)
        .expect("invariant: drawer render nodes include trigger")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == DrawerPart::Content)
        .expect("invariant: drawer render nodes include content")
        .clone();
    let header = nodes
        .iter()
        .find(|node| node.part == DrawerPart::Header)
        .expect("invariant: drawer render nodes include header")
        .clone();
    let handle = nodes
        .iter()
        .find(|node| node.part == DrawerPart::Handle)
        .expect("invariant: drawer render nodes include handle")
        .clone();
    let footer_model = model.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=drawer_root_class(disabled)
            data-ui-component="drawer"
            data-ui-part=DrawerPart::Root.label()
            data-ui-side=side.label()
            data-ui-state=move || {
                state.with(|state| {
                    drawer_state_label(loading, disabled, state.is_open(), state.is_dragging())
                        .to_owned()
                })
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <button
                type="button"
                class=move || state.with(|state| drawer_trigger_class(state.is_open()).to_owned())
                data-ui-part=DrawerPart::Trigger.label()
                data-ui-value=trigger.value
                aria-haspopup="dialog"
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(DrawerIntent::Focus(DrawerPart::Trigger));
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(DrawerIntent::Blur);
                        });
                    }
                }
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(DrawerIntent::Toggle);
                        });
                    }
                }
            >
                {trigger.label}
            </button>
            {move || {
                state.with(|state| {
                    if !state.is_open() {
                        return ().into_any();
                    }
                    let footer_nodes = drawer_render_nodes(&footer_model, state)
                        .into_iter()
                        .filter(|node| node.part == DrawerPart::Footer)
                        .collect::<Vec<_>>();
                    view! {
                        <div class=drawer_overlay_class(side) data-ui-part=DrawerPart::Root.label()>
                            <article
                                role="dialog"
                                aria-modal="true"
                                class=drawer_content_class(side)
                                data-ui-part=DrawerPart::Content.label()
                                data-ui-value=content.value.clone()
                            >
                                <button
                                    type="button"
                                    class=drawer_handle_class(state.is_dragging())
                                    data-ui-part=DrawerPart::Handle.label()
                                    data-ui-value=handle.value.clone()
                                    aria-label=handle.detail.clone()
                                    disabled=blocked
                                    on:focus=move |_| {
                                        if !blocked {
                                            set_state.update(|state| {
                                                let _ = state.apply(DrawerIntent::Focus(DrawerPart::Handle));
                                            });
                                        }
                                    }
                                    on:blur=move |_| {
                                        if !blocked {
                                            set_state.update(|state| {
                                                let _ = state.apply(DrawerIntent::Blur);
                                            });
                                        }
                                    }
                                    on:click=move |_| {
                                        if !blocked {
                                            set_state.update(|state| {
                                                if state.is_dragging() {
                                                    let _ = state.apply(DrawerIntent::EndDrag);
                                                } else {
                                                    let _ = state.apply(DrawerIntent::StartDrag);
                                                }
                                            });
                                        }
                                    }
                                ></button>
                                <header class=DRAWER_HEADER data-ui-part=DrawerPart::Header.label()>
                                    <h3 class=DRAWER_TITLE>{header.label.clone()}</h3>
                                    <p class=DRAWER_DESCRIPTION>{header.detail.clone()}</p>
                                </header>
                                <p class=DRAWER_BODY>{content.detail.clone()}</p>
                                <footer class=DRAWER_FOOTER data-ui-part=DrawerPart::Footer.label()>
                                    {footer_nodes
                                        .into_iter()
                                        .map(|node| drawer_footer_action_view(node, blocked, set_state))
                                        .collect_view()}
                                </footer>
                            </article>
                        </div>
                    }
                    .into_any()
                })
            }}
        </section>
    }
    .into_any()
}

fn drawer_footer_action_view(
    node: crate::DrawerRenderNode,
    blocked: bool,
    set_state: WriteSignal<DrawerState>,
) -> AnyView {
    let value_for_click = node.value.clone();
    let close_drawer = node.close_drawer;
    let disabled = node.disabled || blocked || !node.actionable;
    view! {
        <button
            type="button"
            class=drawer_action_class(node.index, node.selected)
            data-ui-part=DrawerPart::Footer.label()
            data-ui-value=node.value
            disabled=disabled
            on:click=move |_| {
                if !disabled {
                    let value = value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(DrawerIntent::ActivateFooter(value));
                        if close_drawer {
                            let _ = state.apply(DrawerIntent::Close);
                        }
                    });
                }
            }
        >
            {node.label}
        </button>
    }
    .into_any()
}

const fn drawer_root_class(disabled: bool) -> &'static str {
    if disabled {
        DRAWER_ROOT_DISABLED
    } else {
        DRAWER_ROOT
    }
}

const fn drawer_overlay_class(side: DrawerSide) -> &'static str {
    match side {
        DrawerSide::Top => DRAWER_OVERLAY_TOP,
        DrawerSide::Right => DRAWER_OVERLAY_RIGHT,
        DrawerSide::Bottom => DRAWER_OVERLAY_BOTTOM,
        DrawerSide::Left => DRAWER_OVERLAY_LEFT,
    }
}

const fn drawer_content_class(side: DrawerSide) -> &'static str {
    match side {
        DrawerSide::Top | DrawerSide::Bottom => DRAWER_CONTENT_VERTICAL,
        DrawerSide::Right | DrawerSide::Left => DRAWER_CONTENT_SIDE,
    }
}

const fn drawer_trigger_class(open: bool) -> &'static str {
    if open {
        DRAWER_TRIGGER_OPEN
    } else {
        DRAWER_TRIGGER
    }
}

const fn drawer_handle_class(dragging: bool) -> &'static str {
    if dragging {
        DRAWER_HANDLE_ACTIVE
    } else {
        DRAWER_HANDLE
    }
}

const fn drawer_action_class(index: usize, selected: bool) -> &'static str {
    if selected {
        DRAWER_ACTION_ACTIVE
    } else if index == 0 {
        DRAWER_ACTION
    } else {
        DRAWER_ACTION_SECONDARY
    }
}

const fn drawer_state_label(
    loading: bool,
    disabled: bool,
    open: bool,
    dragging: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if dragging {
        "dragging"
    } else if open {
        "open"
    } else {
        "closed"
    }
}

#[component]
pub fn DropdownMenu(
    #[prop(optional, default = default_dropdown_menu_model())] model: DropdownMenuModel,
) -> AnyView {
    if let Err(report) = validate_dropdown_menu_model(&model) {
        let message = format!("DropdownMenu validation failed: {report}");
        return view! {
            <div class=DROPDOWN_MENU_ERROR data-ui-component="dropdown-menu" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let input_nodes = dropdown_menu_render_nodes(&model, &model.state());
    let root = input_nodes
        .iter()
        .find(|node| node.part == DropdownMenuPart::Root)
        .expect("invariant: dropdown menu render nodes include root")
        .clone();
    let trigger = input_nodes
        .iter()
        .find(|node| node.part == DropdownMenuPart::Trigger)
        .expect("invariant: dropdown menu render nodes include trigger")
        .clone();
    let list_model = model.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=dropdown_menu_root_class(density, disabled)
            data-ui-component="dropdown-menu"
            data-ui-part=DropdownMenuPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| dropdown_menu_state_label(loading, disabled, state.is_open()).to_owned())
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <button
                type="button"
                class=move || state.with(|state| dropdown_menu_trigger_class(density, state.is_open()).to_owned())
                data-ui-part=DropdownMenuPart::Trigger.label()
                aria-haspopup="menu"
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(DropdownMenuIntent::Toggle);
                        });
                    }
                }
            >
                {trigger.label}
            </button>
            <div
                role="menu"
                class=move || {
                    state.with(|state| dropdown_menu_content_class(density, state.is_open()).to_owned())
                }
                data-ui-part=DropdownMenuPart::Content.label()
                hidden=move || state.with(|state| !state.is_open())
            >
                {move || {
                    state.with(|state| {
                        dropdown_menu_render_nodes(&list_model, state)
                            .into_iter()
                            .filter(|node| {
                                matches!(
                                    node.part,
                                    DropdownMenuPart::Item
                                        | DropdownMenuPart::Label
                                        | DropdownMenuPart::Separator
                                ) && node.visible
                            })
                            .map(|node| match node.part {
                                DropdownMenuPart::Item => dropdown_menu_item_view(node, blocked, set_state),
                                DropdownMenuPart::Label => view! {
                                    <p
                                        class=DROPDOWN_MENU_LABEL
                                        data-ui-part=DropdownMenuPart::Label.label()
                                        data-ui-value=node.value
                                    >
                                        {node.label}
                                    </p>
                                }
                                .into_any(),
                                DropdownMenuPart::Separator => view! {
                                    <div
                                        role="separator"
                                        class=DROPDOWN_MENU_SEPARATOR
                                        data-ui-part=DropdownMenuPart::Separator.label()
                                        data-ui-value=node.value
                                    ></div>
                                }
                                .into_any(),
                                DropdownMenuPart::Root
                                | DropdownMenuPart::Trigger
                                | DropdownMenuPart::Content => view! { <span></span> }.into_any(),
                            })
                            .collect_view()
                    })
                }}
            </div>
        </section>
    }
    .into_any()
}

fn dropdown_menu_item_view(
    node: crate::DropdownMenuRenderNode,
    blocked: bool,
    set_state: WriteSignal<DropdownMenuState>,
) -> AnyView {
    let value_for_focus = node.value.clone();
    let value_for_click = node.value.clone();
    let label = node.label.clone();
    let detail = node.detail.clone();
    let shortcut = node.shortcut.clone();
    let selected = node.selected;
    let active = node.active;
    let disabled = node.disabled;
    let destructive = node.destructive;
    let density = node.density;
    view! {
        <button
            type="button"
            role="menuitem"
            class=dropdown_menu_item_class(density, selected, active, disabled, destructive)
            data-ui-part=DropdownMenuPart::Item.label()
            data-ui-value=node.value
            aria-selected=(selected || active).to_string()
            disabled=disabled
            on:focus=move |_| {
                if !blocked {
                    let value = value_for_focus.clone();
                    set_state.update(|state| {
                        let _ = state.apply(DropdownMenuIntent::Focus(value));
                    });
                }
            }
            on:click=move |_| {
                if !blocked {
                    let value = value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(DropdownMenuIntent::Select(value));
                    });
                }
            }
        >
            <span class=DROPDOWN_MENU_ITEM_BODY>
                <span class=DROPDOWN_MENU_ITEM_LABEL>{label}</span>
                {if detail.is_empty() {
                    view! { <span></span> }.into_any()
                } else {
                    view! { <span class=DROPDOWN_MENU_ITEM_DETAIL>{detail}</span> }.into_any()
                }}
            </span>
            {if shortcut.is_empty() {
                view! { <span></span> }.into_any()
            } else {
                view! { <span class=DROPDOWN_MENU_SHORTCUT>{shortcut}</span> }.into_any()
            }}
        </button>
    }
    .into_any()
}

const fn dropdown_menu_root_class(density: DropdownMenuDensity, disabled: bool) -> &'static str {
    if disabled {
        return DROPDOWN_MENU_ROOT_DISABLED;
    }
    match density {
        DropdownMenuDensity::Standard => DROPDOWN_MENU_ROOT,
        DropdownMenuDensity::Dense => DROPDOWN_MENU_ROOT_DENSE,
    }
}

const fn dropdown_menu_trigger_class(density: DropdownMenuDensity, open: bool) -> &'static str {
    if open {
        DROPDOWN_MENU_TRIGGER_OPEN
    } else {
        match density {
            DropdownMenuDensity::Standard => DROPDOWN_MENU_TRIGGER,
            DropdownMenuDensity::Dense => DROPDOWN_MENU_TRIGGER_DENSE,
        }
    }
}

const fn dropdown_menu_content_class(density: DropdownMenuDensity, open: bool) -> &'static str {
    if !open {
        return DROPDOWN_MENU_CONTENT_HIDDEN;
    }
    match density {
        DropdownMenuDensity::Standard => DROPDOWN_MENU_CONTENT,
        DropdownMenuDensity::Dense => DROPDOWN_MENU_CONTENT_DENSE,
    }
}

const fn dropdown_menu_item_class(
    density: DropdownMenuDensity,
    selected: bool,
    active: bool,
    disabled: bool,
    destructive: bool,
) -> &'static str {
    if disabled {
        DROPDOWN_MENU_ITEM_DISABLED
    } else if destructive {
        DROPDOWN_MENU_ITEM_DESTRUCTIVE
    } else if selected {
        DROPDOWN_MENU_ITEM_SELECTED
    } else if active {
        DROPDOWN_MENU_ITEM_ACTIVE
    } else {
        match density {
            DropdownMenuDensity::Standard => DROPDOWN_MENU_ITEM,
            DropdownMenuDensity::Dense => DROPDOWN_MENU_ITEM_DENSE,
        }
    }
}

const fn dropdown_menu_state_label(loading: bool, disabled: bool, open: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if open {
        "open"
    } else {
        "closed"
    }
}

#[component]
pub fn Empty(#[prop(optional, default = default_empty_model())] model: EmptyModel) -> AnyView {
    if let Err(report) = validate_empty_model(&model) {
        let message = format!("Empty validation failed: {report}");
        return view! {
            <div class=EMPTY_ERROR data-ui-component="empty" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = empty_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == EmptyPart::Root)
        .expect("invariant: empty render nodes include root")
        .clone();
    let header = nodes
        .iter()
        .find(|node| node.part == EmptyPart::Header)
        .expect("invariant: empty render nodes include header")
        .clone();
    let title = nodes
        .iter()
        .find(|node| node.part == EmptyPart::Title)
        .expect("invariant: empty render nodes include title")
        .clone();
    let description = nodes
        .iter()
        .find(|node| node.part == EmptyPart::Description)
        .expect("invariant: empty render nodes include description")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == EmptyPart::Content)
        .expect("invariant: empty render nodes include content")
        .clone();
    let action = nodes
        .iter()
        .find(|node| node.part == EmptyPart::Action)
        .expect("invariant: empty render nodes include action")
        .clone();
    let density = root.density;
    let (state, set_state) = signal(state_model);
    let action_disabled = action.disabled || blocked;
    let action_value_for_data = action.value.clone();
    let action_value_for_click = action.value.clone();
    let action_label = if loading {
        "Loading".to_owned()
    } else {
        action.label.clone()
    };
    let action_view = if action.actionable {
        view! {
            <button
                type="button"
                class=move || {
                    state.with(|state| {
                        empty_action_class(
                            density,
                            state.is_active(EmptyPart::Action),
                            action_disabled,
                        )
                    })
                }
                data-ui-part=EmptyPart::Action.label()
                data-ui-value=action_value_for_data
                aria-pressed=move || {
                    state.with(|state| state.is_active(EmptyPart::Action).to_string())
                }
                disabled=action_disabled
                on:focus=move |_| {
                    if !action_disabled {
                        set_state.update(|state| {
                            let _ = state.apply(EmptyIntent::Focus(EmptyPart::Action));
                        });
                    }
                }
                on:blur=move |_| {
                    if !action_disabled {
                        set_state.update(|state| {
                            let _ = state.apply(EmptyIntent::Blur(EmptyPart::Action));
                        });
                    }
                }
                on:click=move |_| {
                    if !action_disabled {
                        let value = action_value_for_click.clone();
                        set_state.update(|state| {
                            let _ = state.apply(EmptyIntent::Activate(value));
                        });
                    }
                }
            >
                {action_label}
            </button>
        }
        .into_any()
    } else {
        view! {
            <button
                type="button"
                class=EMPTY_ACTION_DISABLED
                data-ui-part=EmptyPart::Action.label()
                data-ui-value=action_value_for_data
                aria-disabled="true"
                disabled=true
            >
                {action_label}
            </button>
        }
        .into_any()
    };

    view! {
        <section
            class=empty_root_class(density, loading, disabled)
            data-ui-component="empty"
            data-ui-part=EmptyPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=empty_state_label(loading, disabled)
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <header class=empty_header_class(density) data-ui-part=header.part.label()>
                <h3 class=empty_title_class(density) data-ui-part=title.part.label()>
                    {title.label}
                </h3>
                <p class=empty_description_class(density) data-ui-part=description.part.label()>
                    {description.detail}
                </p>
            </header>
            <div class=empty_content_class(density) data-ui-part=content.part.label()>
                <span class=empty_marker_class(density) aria-hidden="true">
                    {content.label}
                </span>
                <p class=empty_content_text_class(density)>
                    {content.detail}
                </p>
            </div>
            {action_view}
        </section>
    }
    .into_any()
}

const fn empty_root_class(density: EmptyDensity, loading: bool, disabled: bool) -> &'static str {
    if disabled {
        return EMPTY_ROOT_DISABLED;
    }
    if loading {
        return EMPTY_ROOT_LOADING;
    }
    match density {
        EmptyDensity::Standard => EMPTY_ROOT,
        EmptyDensity::Dense => EMPTY_ROOT_DENSE,
    }
}

const fn empty_header_class(density: EmptyDensity) -> &'static str {
    match density {
        EmptyDensity::Standard => EMPTY_HEADER,
        EmptyDensity::Dense => EMPTY_HEADER_DENSE,
    }
}

const fn empty_title_class(density: EmptyDensity) -> &'static str {
    match density {
        EmptyDensity::Standard => EMPTY_TITLE,
        EmptyDensity::Dense => EMPTY_TITLE_DENSE,
    }
}

const fn empty_description_class(density: EmptyDensity) -> &'static str {
    match density {
        EmptyDensity::Standard => EMPTY_DESCRIPTION,
        EmptyDensity::Dense => EMPTY_DESCRIPTION_DENSE,
    }
}

const fn empty_content_class(density: EmptyDensity) -> &'static str {
    match density {
        EmptyDensity::Standard => EMPTY_CONTENT,
        EmptyDensity::Dense => EMPTY_CONTENT_DENSE,
    }
}

const fn empty_marker_class(density: EmptyDensity) -> &'static str {
    match density {
        EmptyDensity::Standard => EMPTY_MARKER,
        EmptyDensity::Dense => EMPTY_MARKER_DENSE,
    }
}

const fn empty_content_text_class(density: EmptyDensity) -> &'static str {
    match density {
        EmptyDensity::Standard => EMPTY_CONTENT_TEXT,
        EmptyDensity::Dense => EMPTY_CONTENT_TEXT_DENSE,
    }
}

const fn empty_action_class(density: EmptyDensity, active: bool, disabled: bool) -> &'static str {
    if disabled {
        EMPTY_ACTION_DISABLED
    } else if active {
        EMPTY_ACTION_ACTIVE
    } else {
        match density {
            EmptyDensity::Standard => EMPTY_ACTION,
            EmptyDensity::Dense => EMPTY_ACTION_DENSE,
        }
    }
}

const fn empty_state_label(loading: bool, disabled: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else {
        "ready"
    }
}

#[component]
pub fn Field(#[prop(optional, default = default_field_model())] model: FieldModel) -> AnyView {
    if let Err(report) = validate_field_model(&model) {
        let message = format!("Field validation failed: {report}");
        return view! {
            <div class=FIELD_ERROR data-ui-component="field" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let input_kind = model.input_kind;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let required = model.required;
    let state_model = model.state();
    let nodes = field_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == FieldPart::Root)
        .expect("invariant: field render nodes include root")
        .clone();
    let label = nodes
        .iter()
        .find(|node| node.part == FieldPart::Label)
        .expect("invariant: field render nodes include label")
        .clone();
    let control = nodes
        .iter()
        .find(|node| node.part == FieldPart::Control)
        .expect("invariant: field render nodes include control")
        .clone();
    let description = nodes
        .iter()
        .find(|node| node.part == FieldPart::Description)
        .expect("invariant: field render nodes include description")
        .clone();
    let error = nodes
        .iter()
        .find(|node| node.part == FieldPart::Error)
        .expect("invariant: field render nodes include error")
        .clone();
    let invalid = root.invalid;
    let (state, set_state) = signal(state_model);
    let placeholder = control.label.clone();
    let label_text = label.label.clone();

    view! {
        <section
            class=field_root_class(density, invalid, loading, disabled)
            data-ui-component="field"
            data-ui-part=FieldPart::Root.label()
            data-ui-density=density.label()
            data-ui-kind=input_kind.label()
            data-ui-state=move || {
                state.with(|state| {
                    field_state_label(loading, disabled, invalid, state.is_focused()).to_owned()
                })
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <label class=field_label_class(density, disabled) data-ui-part=label.part.label()>
                <span>{label_text.clone()}</span>
                {if required {
                    view! { <span class=FIELD_REQUIRED aria-hidden="true">"*"</span> }.into_any()
                } else {
                    ().into_any()
                }}
            </label>
            <input
                type=input_kind.label()
                class=move || {
                    state.with(|state| {
                        field_control_class(density, state.is_focused(), invalid, blocked).to_owned()
                    })
                }
                data-ui-part=control.part.label()
                placeholder=placeholder
                aria-label=label_text
                aria-invalid=invalid.to_string()
                required=required
                disabled=blocked
                prop:value=move || state.with(|state| state.value().to_owned())
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(FieldIntent::Focus);
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(FieldIntent::Blur);
                        });
                    }
                }
                on:input=move |event| {
                    if !blocked {
                        let value = event_target_value(&event);
                        set_state.update(|state| {
                            let _ = state.apply(FieldIntent::Input(value));
                        });
                    }
                }
            />
            <p
                class=field_description_class(density, disabled)
                data-ui-part=description.part.label()
            >
                {description.detail}
            </p>
            <p
                class=field_error_class(error.visible)
                data-ui-part=error.part.label()
                aria-hidden=(!error.visible).to_string()
            >
                {error.detail}
            </p>
        </section>
    }
    .into_any()
}

const fn field_root_class(
    density: FieldDensity,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return FIELD_ROOT_DISABLED;
    }
    if loading {
        return FIELD_ROOT_LOADING;
    }
    if invalid {
        return FIELD_ROOT_INVALID;
    }
    match density {
        FieldDensity::Standard => FIELD_ROOT,
        FieldDensity::Dense => FIELD_ROOT_DENSE,
    }
}

const fn field_label_class(density: FieldDensity, disabled: bool) -> &'static str {
    if disabled {
        return FIELD_LABEL_DISABLED;
    }
    match density {
        FieldDensity::Standard => FIELD_LABEL,
        FieldDensity::Dense => FIELD_LABEL_DENSE,
    }
}

const fn field_control_class(
    density: FieldDensity,
    focused: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    match (density, disabled, invalid, focused) {
        (FieldDensity::Standard, true, _, _) => FIELD_CONTROL_DISABLED,
        (FieldDensity::Dense, true, _, _) => FIELD_CONTROL_DENSE_DISABLED,
        (FieldDensity::Standard, false, true, _) => FIELD_CONTROL_INVALID,
        (FieldDensity::Dense, false, true, _) => FIELD_CONTROL_DENSE_INVALID,
        (FieldDensity::Standard, false, false, true) => FIELD_CONTROL_FOCUSED,
        (FieldDensity::Dense, false, false, true) => FIELD_CONTROL_DENSE_FOCUSED,
        (FieldDensity::Standard, false, false, false) => FIELD_CONTROL,
        (FieldDensity::Dense, false, false, false) => FIELD_CONTROL_DENSE,
    }
}

const fn field_description_class(density: FieldDensity, disabled: bool) -> &'static str {
    if disabled {
        return FIELD_DESCRIPTION_DISABLED;
    }
    match density {
        FieldDensity::Standard => FIELD_DESCRIPTION,
        FieldDensity::Dense => FIELD_DESCRIPTION_DENSE,
    }
}

const fn field_error_class(visible: bool) -> &'static str {
    if visible {
        FIELD_ERROR_TEXT
    } else {
        FIELD_ERROR_HIDDEN
    }
}

const fn field_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    focused: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if focused {
        "focused"
    } else {
        "ready"
    }
}

#[component]
pub fn HoverCard(
    #[prop(optional, default = default_hover_card_model())] model: HoverCardModel,
) -> AnyView {
    if let Err(report) = validate_hover_card_model(&model) {
        let message = format!("HoverCard validation failed: {report}");
        return view! {
            <div class=HOVER_CARD_ERROR data-ui-component="hover-card" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = hover_card_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == HoverCardPart::Root)
        .expect("invariant: hover card render nodes include root")
        .clone();
    let trigger = nodes
        .iter()
        .find(|node| node.part == HoverCardPart::Trigger)
        .expect("invariant: hover card render nodes include trigger")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == HoverCardPart::Content)
        .expect("invariant: hover card render nodes include content")
        .clone();
    let arrow = nodes
        .iter()
        .find(|node| node.part == HoverCardPart::Arrow)
        .expect("invariant: hover card render nodes include arrow")
        .clone();
    let (state, set_state) = signal(state_model);
    let trigger_label = if loading {
        "Loading preview".to_owned()
    } else {
        trigger.label.clone()
    };
    let content_value = content.value.clone();
    let content_label = if loading {
        "Loading".to_owned()
    } else {
        content.label.clone()
    };
    let content_detail = if loading {
        "Preparing hover card content.".to_owned()
    } else {
        content.detail.clone()
    };
    let arrow_value = arrow.value.clone();
    let arrow_label = arrow.label.clone();
    let arrow_detail = arrow.detail.clone();

    view! {
        <section
            class=hover_card_root_class(disabled)
            data-ui-component="hover-card"
            data-ui-part=HoverCardPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| hover_card_state_label(loading, disabled, state.is_open()).to_owned())
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            on:mouseenter=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(HoverCardIntent::Open(HoverCardPart::Trigger));
                    });
                }
            }
            on:mouseleave=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(HoverCardIntent::Close);
                    });
                }
            }
        >
            <button
                type="button"
                class=move || {
                    state.with(|state| {
                        hover_card_trigger_class(density, state.is_open(), blocked).to_owned()
                    })
                }
                data-ui-part=HoverCardPart::Trigger.label()
                data-ui-value=trigger.value
                aria-haspopup="dialog"
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(HoverCardIntent::Focus(HoverCardPart::Trigger));
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(HoverCardIntent::Blur);
                        });
                    }
                }
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(HoverCardIntent::Toggle);
                        });
                    }
                }
            >
                {trigger_label}
            </button>
            <article
                role="tooltip"
                class=move || {
                    state.with(|state| {
                        hover_card_content_class(density, state.is_open(), loading, disabled)
                            .to_owned()
                    })
                }
                data-ui-part=HoverCardPart::Content.label()
                data-ui-value=content_value
                aria-hidden=move || state.with(|state| (!state.is_open()).to_string())
                hidden=move || state.with(|state| !state.is_open())
            >
                <span
                    class=move || state.with(|state| hover_card_arrow_class(state.is_open()).to_owned())
                    data-ui-part=HoverCardPart::Arrow.label()
                    data-ui-value=arrow_value
                    aria-label=arrow_detail
                >
                    {arrow_label}
                </span>
                <p class=HOVER_CARD_META>{content.value}</p>
                <h3 class=hover_card_title_class(density)>{content_label}</h3>
                <p class=hover_card_detail_class(density)>{content_detail}</p>
            </article>
        </section>
    }
    .into_any()
}

const fn hover_card_root_class(disabled: bool) -> &'static str {
    if disabled {
        HOVER_CARD_ROOT_DISABLED
    } else {
        HOVER_CARD_ROOT
    }
}

const fn hover_card_trigger_class(
    density: HoverCardDensity,
    open: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return HOVER_CARD_TRIGGER_DISABLED;
    }
    match (density, open) {
        (HoverCardDensity::Standard, true) => HOVER_CARD_TRIGGER_OPEN,
        (HoverCardDensity::Dense, true) => HOVER_CARD_TRIGGER_DENSE_OPEN,
        (HoverCardDensity::Standard, false) => HOVER_CARD_TRIGGER,
        (HoverCardDensity::Dense, false) => HOVER_CARD_TRIGGER_DENSE,
    }
}

const fn hover_card_content_class(
    density: HoverCardDensity,
    open: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if !open {
        return HOVER_CARD_CONTENT_HIDDEN;
    }
    if disabled {
        return HOVER_CARD_CONTENT_DISABLED;
    }
    if loading {
        return HOVER_CARD_CONTENT_LOADING;
    }
    match density {
        HoverCardDensity::Standard => HOVER_CARD_CONTENT,
        HoverCardDensity::Dense => HOVER_CARD_CONTENT_DENSE,
    }
}

const fn hover_card_title_class(density: HoverCardDensity) -> &'static str {
    match density {
        HoverCardDensity::Standard => HOVER_CARD_TITLE,
        HoverCardDensity::Dense => HOVER_CARD_TITLE_DENSE,
    }
}

const fn hover_card_detail_class(density: HoverCardDensity) -> &'static str {
    match density {
        HoverCardDensity::Standard => HOVER_CARD_DETAIL,
        HoverCardDensity::Dense => HOVER_CARD_DETAIL_DENSE,
    }
}

const fn hover_card_arrow_class(open: bool) -> &'static str {
    if open {
        HOVER_CARD_ARROW
    } else {
        HOVER_CARD_ARROW_HIDDEN
    }
}

const fn hover_card_state_label(loading: bool, disabled: bool, open: bool) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if open {
        "open"
    } else {
        "closed"
    }
}

#[component]
pub fn Input(
    #[prop(optional, default = crate::default_input_model())] model: InputModel,
) -> AnyView {
    if let Err(report) = validate_input_model(&model) {
        let message = format!("Input validation failed: {report}");
        return view! {
            <div class=INPUT_ERROR data-ui-component="input" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let input_kind = model.input_kind;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let required = model.required;
    let error_detail = model
        .error
        .clone()
        .unwrap_or_else(|| "No input error".to_owned());
    let state_model = model.state();
    let nodes = input_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == InputPart::Root)
        .expect("invariant: input render nodes include root")
        .clone();
    let prefix = nodes
        .iter()
        .find(|node| node.part == InputPart::Prefix)
        .expect("invariant: input render nodes include prefix")
        .clone();
    let control = nodes
        .iter()
        .find(|node| node.part == InputPart::Control)
        .expect("invariant: input render nodes include control")
        .clone();
    let suffix = nodes
        .iter()
        .find(|node| node.part == InputPart::Suffix)
        .expect("invariant: input render nodes include suffix")
        .clone();
    let invalid = root.invalid;
    let placeholder = control.label.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=input_root_class(disabled)
            data-ui-component="input"
            data-ui-part=InputPart::Root.label()
            data-ui-density=density.label()
            data-ui-kind=input_kind.label()
            data-ui-state=move || {
                state.with(|state| {
                    input_state_label(loading, disabled, invalid, state.is_focused()).to_owned()
                })
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <div
                class=move || {
                    state.with(|state| {
                        input_row_class(density, state.is_focused(), invalid, loading, disabled)
                            .to_owned()
                    })
                }
            >
                <span
                    class=input_prefix_class(density, prefix.visible)
                    data-ui-part=InputPart::Prefix.label()
                    data-ui-value=prefix.value
                    aria-hidden=(!prefix.visible).to_string()
                >
                    {prefix.label}
                </span>
                <input
                    type=input_kind.label()
                    class=input_control_class(density, blocked)
                    data-ui-part=InputPart::Control.label()
                    placeholder=placeholder
                    aria-label="Input"
                    aria-invalid=invalid.to_string()
                    required=required
                    disabled=blocked
                    prop:value=move || state.with(|state| state.value().to_owned())
                    on:focus=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(InputIntent::Focus);
                            });
                        }
                    }
                    on:blur=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(InputIntent::Blur);
                            });
                        }
                    }
                    on:input=move |event| {
                        if !blocked {
                            let value = event_target_value(&event);
                            set_state.update(|state| {
                                let _ = state.apply(InputIntent::Input(value));
                            });
                        }
                    }
                />
                {input_suffix_view(suffix, density, blocked, set_state)}
            </div>
            <p
                class=input_error_class(invalid)
                data-ui-part=InputPart::Control.label()
                aria-hidden=(!invalid).to_string()
            >
                {error_detail}
            </p>
        </section>
    }
    .into_any()
}

fn input_suffix_view(
    node: crate::InputRenderNode,
    density: InputDensity,
    blocked: bool,
    set_state: WriteSignal<InputState>,
) -> AnyView {
    let disabled = node.disabled || blocked || !node.actionable;
    let value_for_click = node.value.clone();
    let value_for_data = node.value;
    let label = node.label;
    let visible = node.visible;
    let active = node.active;
    view! {
        <button
            type="button"
            class=move || {
                input_suffix_class(density, visible, active, disabled).to_owned()
            }
            data-ui-part=InputPart::Suffix.label()
            data-ui-value=value_for_data
            aria-hidden=(!visible).to_string()
            disabled=disabled
            on:click=move |_| {
                if !disabled {
                    let value = value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(InputIntent::ActivateSuffix(value));
                    });
                }
            }
        >
            {label}
        </button>
    }
    .into_any()
}

const fn input_root_class(disabled: bool) -> &'static str {
    if disabled {
        INPUT_ROOT_DISABLED
    } else {
        INPUT_ROOT
    }
}

const fn input_row_class(
    density: InputDensity,
    focused: bool,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return INPUT_ROW_DISABLED;
    }
    if loading {
        return INPUT_ROW_LOADING;
    }
    match (density, invalid, focused) {
        (InputDensity::Standard, true, _) => INPUT_ROW_INVALID,
        (InputDensity::Dense, true, _) => INPUT_ROW_DENSE_INVALID,
        (InputDensity::Standard, false, true) => INPUT_ROW_FOCUSED,
        (InputDensity::Dense, false, true) => INPUT_ROW_DENSE_FOCUSED,
        (InputDensity::Standard, false, false) => INPUT_ROW,
        (InputDensity::Dense, false, false) => INPUT_ROW_DENSE,
    }
}

const fn input_prefix_class(density: InputDensity, visible: bool) -> &'static str {
    if !visible {
        return INPUT_PREFIX_HIDDEN;
    }
    match density {
        InputDensity::Standard => INPUT_PREFIX,
        InputDensity::Dense => INPUT_PREFIX_DENSE,
    }
}

const fn input_control_class(density: InputDensity, disabled: bool) -> &'static str {
    if disabled {
        return INPUT_CONTROL_DISABLED;
    }
    match density {
        InputDensity::Standard => INPUT_CONTROL,
        InputDensity::Dense => INPUT_CONTROL_DENSE,
    }
}

const fn input_suffix_class(
    density: InputDensity,
    visible: bool,
    active: bool,
    disabled: bool,
) -> &'static str {
    if !visible {
        return INPUT_SUFFIX_HIDDEN;
    }
    if disabled {
        return INPUT_SUFFIX_DISABLED;
    }
    if active {
        return INPUT_SUFFIX_ACTIVE;
    }
    match density {
        InputDensity::Standard => INPUT_SUFFIX,
        InputDensity::Dense => INPUT_SUFFIX_DENSE,
    }
}

const fn input_error_class(visible: bool) -> &'static str {
    if visible {
        INPUT_ERROR_TEXT
    } else {
        INPUT_ERROR_HIDDEN
    }
}

const fn input_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    focused: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if focused {
        "focused"
    } else {
        "ready"
    }
}

#[component]
pub fn InputGroup(
    #[prop(optional, default = default_input_group_model())] model: InputGroupModel,
) -> AnyView {
    if let Err(report) = validate_input_group_model(&model) {
        let message = format!("InputGroup validation failed: {report}");
        return view! {
            <div class=INPUT_ERROR data-ui-component="input-group" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let input_kind = model.input_kind;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let required = model.required;
    let error_detail = model
        .error
        .clone()
        .unwrap_or_else(|| "No input group error".to_owned());
    let state_model = model.state();
    let nodes = input_group_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == InputGroupPart::Root)
        .expect("invariant: input group render nodes include root")
        .clone();
    let addon = nodes
        .iter()
        .find(|node| node.part == InputGroupPart::Addon)
        .expect("invariant: input group render nodes include addon")
        .clone();
    let input = nodes
        .iter()
        .find(|node| node.part == InputGroupPart::Input)
        .expect("invariant: input group render nodes include input")
        .clone();
    let button = nodes
        .iter()
        .find(|node| node.part == InputGroupPart::Button)
        .expect("invariant: input group render nodes include button")
        .clone();
    let invalid = root.invalid;
    let placeholder = input.label.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=input_root_class(disabled)
            data-ui-component="input-group"
            data-ui-part=InputGroupPart::Root.label()
            data-ui-density=density.label()
            data-ui-kind=input_kind.label()
            data-ui-state=move || {
                state.with(|state| {
                    input_state_label(loading, disabled, invalid, state.is_focused()).to_owned()
                })
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <div
                class=move || {
                    state.with(|state| {
                        input_row_class(density, state.is_focused(), invalid, loading, disabled)
                            .to_owned()
                    })
                }
            >
                <span
                    class=input_prefix_class(density, addon.visible)
                    data-ui-part=InputGroupPart::Addon.label()
                    data-ui-value=addon.value
                    aria-hidden=(!addon.visible).to_string()
                >
                    {addon.label}
                </span>
                <input
                    type=input_kind.label()
                    class=input_control_class(density, blocked)
                    data-ui-part=InputGroupPart::Input.label()
                    placeholder=placeholder
                    aria-label="Input group"
                    aria-invalid=invalid.to_string()
                    required=required
                    disabled=blocked
                    prop:value=move || state.with(|state| state.value().to_owned())
                    on:focus=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(InputGroupIntent::Focus);
                            });
                        }
                    }
                    on:blur=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(InputGroupIntent::Blur);
                            });
                        }
                    }
                    on:input=move |event| {
                        if !blocked {
                            let value = event_target_value(&event);
                            set_state.update(|state| {
                                let _ = state.apply(InputGroupIntent::Input(value));
                            });
                        }
                    }
                />
                {input_group_button_view(button, density, blocked, set_state)}
            </div>
            <p
                class=input_error_class(invalid)
                data-ui-part=InputGroupPart::Input.label()
                aria-hidden=(!invalid).to_string()
            >
                {error_detail}
            </p>
        </section>
    }
    .into_any()
}

fn input_group_button_view(
    node: crate::InputGroupRenderNode,
    density: InputDensity,
    blocked: bool,
    set_state: WriteSignal<InputGroupState>,
) -> AnyView {
    let disabled = node.disabled || blocked || !node.actionable;
    let value_for_click = node.value.clone();
    let value_for_data = node.value;
    let label = node.label;
    let visible = node.visible;
    let active = node.active;
    view! {
        <button
            type="button"
            class=move || {
                input_suffix_class(density, visible, active, disabled).to_owned()
            }
            data-ui-part=InputGroupPart::Button.label()
            data-ui-value=value_for_data
            aria-hidden=(!visible).to_string()
            disabled=disabled
            on:click=move |_| {
                if !disabled {
                    let value = value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(InputGroupIntent::ActivateButton(value));
                    });
                }
            }
        >
            {label}
        </button>
    }
    .into_any()
}

#[component]
pub fn InputOtp(
    #[prop(optional, default = default_input_otp_model())] model: InputOtpModel,
) -> AnyView {
    if let Err(report) = validate_input_otp_model(&model) {
        let message = format!("InputOtp validation failed: {report}");
        return view! {
            <div class=INPUT_ERROR data-ui-component="input-otp" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let required = model.required;
    let invalid = model.error.is_some();
    let numeric_only = model.numeric_only;
    let input_mode = if numeric_only { "numeric" } else { "text" };
    let pattern = if numeric_only {
        "[0-9]*"
    } else {
        "[A-Za-z0-9]*"
    };
    let error_detail = model
        .error
        .clone()
        .unwrap_or_else(|| "No input OTP error".to_owned());
    let state_model = model.state();
    let nodes = input_otp_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == InputOtpPart::Root)
        .expect("invariant: input OTP render nodes include root")
        .clone();
    let code_nodes = nodes
        .into_iter()
        .filter(|node| matches!(node.part, InputOtpPart::Slot | InputOtpPart::Separator))
        .collect::<Vec<_>>();
    let (state, set_state) = signal(state_model);
    let view_context = InputOtpViewContext {
        density,
        input_mode,
        pattern,
        required,
        blocked,
        state,
        set_state,
    };

    view! {
        <section
            class=input_root_class(disabled)
            data-ui-component="input-otp"
            data-ui-part=InputOtpPart::Root.label()
            data-ui-density=density.label()
            data-ui-mode=input_mode
            data-ui-state=move || {
                state.with(|state| {
                    input_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.focused_slot().is_some(),
                    )
                    .to_owned()
                })
            }
            data-ui-value=move || state.with(InputOtpState::value)
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <div
                class=INPUT_OTP_GROUP
                data-ui-part=InputOtpPart::Group.label()
                data-ui-value=root.value
                role="group"
                aria-label=root.label
                aria-invalid=invalid.to_string()
            >
                {code_nodes
                    .into_iter()
                    .map(|node| input_otp_code_node_view(node, view_context))
                    .collect_view()}
            </div>
            <p
                class=input_error_class(invalid)
                data-ui-part=InputOtpPart::Root.label()
                aria-hidden=(!invalid).to_string()
            >
                {error_detail}
            </p>
        </section>
    }
    .into_any()
}

#[derive(Clone, Copy)]
struct InputOtpViewContext {
    density: InputDensity,
    input_mode: &'static str,
    pattern: &'static str,
    required: bool,
    blocked: bool,
    state: ReadSignal<InputOtpState>,
    set_state: WriteSignal<InputOtpState>,
}

fn input_otp_code_node_view(
    node: crate::InputOtpRenderNode,
    context: InputOtpViewContext,
) -> AnyView {
    match node.part {
        InputOtpPart::Slot => {
            let index = node
                .index
                .expect("invariant: input OTP slot render node includes index");
            let invalid = node.invalid;
            let loading = node.loading;
            let disabled = node.disabled || context.blocked;
            let placeholder = node.label;
            let detail = node.detail;
            view! {
                <input
                    type="text"
                    inputmode=context.input_mode
                    pattern=context.pattern
                    maxlength="1"
                    class=move || {
                        context.state.with(|state| {
                            input_otp_slot_class(
                                context.density,
                                state.is_slot_focused(index),
                                state.slot_value(index).is_some(),
                                invalid,
                                loading,
                                disabled,
                            )
                            .to_owned()
                        })
                    }
                    data-ui-part=InputOtpPart::Slot.label()
                    data-ui-index=index.to_string()
                    data-ui-value=move || {
                        context.state.with(|state| {
                            state
                                .slot_value(index)
                                .map(|character| character.to_string())
                                .unwrap_or_default()
                        })
                    }
                    placeholder=placeholder
                    aria-label=detail
                    aria-invalid=invalid.to_string()
                    required=context.required
                    disabled=disabled
                    prop:value=move || {
                        context.state.with(|state| {
                            state
                                .slot_value(index)
                                .map(|character| character.to_string())
                                .unwrap_or_default()
                        })
                    }
                    on:focus=move |_| {
                        if !disabled {
                            context.set_state.update(|state| {
                                let _ = state.apply(InputOtpIntent::FocusSlot(index));
                            });
                        }
                    }
                    on:blur=move |_| {
                        if !disabled {
                            context.set_state.update(|state| {
                                let _ = state.apply(InputOtpIntent::Blur);
                            });
                        }
                    }
                    on:input=move |event| {
                        if !disabled {
                            let value = event_target_value(&event);
                            context.set_state.update(|state| {
                                let _ = state.apply(InputOtpIntent::InputSlot { index, value });
                            });
                        }
                    }
                />
            }
            .into_any()
        }
        InputOtpPart::Separator => {
            let visible = node.visible;
            let label = node.label;
            view! {
                <span
                    class=input_otp_separator_class(context.density, visible)
                    data-ui-part=InputOtpPart::Separator.label()
                    aria-hidden="true"
                >
                    {label}
                </span>
            }
            .into_any()
        }
        InputOtpPart::Root | InputOtpPart::Group => {
            unreachable!("invariant: input OTP root/group nodes are not rendered by code helper")
        }
    }
}

const fn input_otp_slot_class(
    density: InputDensity,
    focused: bool,
    filled: bool,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return INPUT_OTP_SLOT_DISABLED;
    }
    if loading {
        return INPUT_OTP_SLOT_LOADING;
    }
    match (density, invalid, focused, filled) {
        (InputDensity::Standard, true, _, _) => INPUT_OTP_SLOT_INVALID,
        (InputDensity::Dense, true, _, _) => INPUT_OTP_SLOT_DENSE_INVALID,
        (InputDensity::Standard, false, true, _) => INPUT_OTP_SLOT_FOCUSED,
        (InputDensity::Dense, false, true, _) => INPUT_OTP_SLOT_DENSE_FOCUSED,
        (InputDensity::Standard, false, false, true) => INPUT_OTP_SLOT_FILLED,
        (InputDensity::Dense, false, false, true) => INPUT_OTP_SLOT_DENSE_FILLED,
        (InputDensity::Standard, false, false, false) => INPUT_OTP_SLOT,
        (InputDensity::Dense, false, false, false) => INPUT_OTP_SLOT_DENSE,
    }
}

const fn input_otp_separator_class(density: InputDensity, visible: bool) -> &'static str {
    if !visible {
        return INPUT_OTP_SEPARATOR_HIDDEN;
    }
    match density {
        InputDensity::Standard => INPUT_OTP_SEPARATOR,
        InputDensity::Dense => INPUT_OTP_SEPARATOR_DENSE,
    }
}

#[component]
pub fn Item(#[prop(optional, default = default_item_model())] model: ItemModel) -> AnyView {
    if let Err(report) = validate_item_model(&model) {
        let message = format!("Item validation failed: {report}");
        return view! {
            <div class=ITEM_ERROR data-ui-component="item" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = item_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == ItemPart::Root)
        .expect("invariant: item render nodes include root")
        .clone();
    let media = nodes
        .iter()
        .find(|node| node.part == ItemPart::Media)
        .expect("invariant: item render nodes include media")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == ItemPart::Content)
        .expect("invariant: item render nodes include content")
        .clone();
    let title = nodes
        .iter()
        .find(|node| node.part == ItemPart::Title)
        .expect("invariant: item render nodes include title")
        .clone();
    let description = nodes
        .iter()
        .find(|node| node.part == ItemPart::Description)
        .expect("invariant: item render nodes include description")
        .clone();
    let actions = nodes
        .into_iter()
        .filter(|node| node.part == ItemPart::Actions)
        .collect::<Vec<_>>();
    let actions_visible = actions.iter().any(|node| node.visible);
    let invalid = root.invalid;
    let root_value = root.value;
    let media_value = media.value;
    let media_label = media.label;
    let media_visible = media.visible;
    let media_disabled = media.disabled;
    let content_value = content.value;
    let title_value = title.value;
    let title_label = title.label;
    let description_value = description.value;
    let description_detail = description.detail;
    let (state, set_state) = signal(state_model);
    let context = ItemViewContext {
        density,
        blocked,
        state,
        set_state,
    };

    view! {
        <article
            role="listitem"
            class=item_root_class(density, invalid, loading, disabled)
            data-ui-component="item"
            data-ui-part=ItemPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    item_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_active(ItemPart::Actions),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root_value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <span
                class=item_media_class(density, media_visible, media_disabled)
                data-ui-part=ItemPart::Media.label()
                data-ui-value=media_value
                aria-hidden=(!media_visible).to_string()
            >
                {media_label}
            </span>
            <div
                class=item_content_class(density)
                data-ui-part=ItemPart::Content.label()
                data-ui-value=content_value
            >
                <h3 class=item_title_class(density, disabled) data-ui-part=ItemPart::Title.label() data-ui-value=title_value>
                    {title_label}
                </h3>
                <p
                    class=item_description_class(density, invalid, disabled)
                    data-ui-part=ItemPart::Description.label()
                    data-ui-value=description_value
                    aria-invalid=invalid.to_string()
                >
                    {description_detail}
                </p>
            </div>
            <div
                class=item_actions_class(actions_visible)
                data-ui-part=ItemPart::Actions.label()
                aria-hidden=(!actions_visible).to_string()
            >
                {actions
                    .into_iter()
                    .filter(|node| node.visible)
                    .map(|node| item_action_view(node, context))
                    .collect_view()}
            </div>
        </article>
    }
    .into_any()
}

#[derive(Clone, Copy)]
struct ItemViewContext {
    density: ItemDensity,
    blocked: bool,
    state: ReadSignal<ItemState>,
    set_state: WriteSignal<ItemState>,
}

fn item_action_view(node: crate::ItemRenderNode, context: ItemViewContext) -> AnyView {
    let disabled = node.disabled || context.blocked || !node.actionable;
    let value_for_class = node.value.clone();
    let value_for_pressed = node.value.clone();
    let value_for_focus = node.value.clone();
    let value_for_click = node.value.clone();
    let value_for_data = node.value;
    let label = node.label;
    view! {
        <button
            type="button"
            class=move || {
                context.state.with(|state| {
                    item_action_class(
                        context.density,
                        state.is_action_active(&value_for_class),
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-part=ItemPart::Actions.label()
            data-ui-index=node.index.map(|index| index.to_string()).unwrap_or_default()
            data-ui-value=value_for_data
            aria-pressed=move || {
                context
                    .state
                    .with(|state| state.is_action_active(&value_for_pressed).to_string())
            }
            disabled=disabled
            on:focus=move |_| {
                if !disabled {
                    let value = value_for_focus.clone();
                    context.set_state.update(|state| {
                        let _ = state.apply(ItemIntent::FocusAction(value));
                    });
                }
            }
            on:blur=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(ItemIntent::Blur);
                    });
                }
            }
            on:click=move |_| {
                if !disabled {
                    let value = value_for_click.clone();
                    context.set_state.update(|state| {
                        let _ = state.apply(ItemIntent::ActivateAction(value));
                    });
                }
            }
        >
            {label}
        </button>
    }
    .into_any()
}

const fn item_root_class(
    density: ItemDensity,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return ITEM_ROOT_DISABLED;
    }
    if loading {
        return ITEM_ROOT_LOADING;
    }
    if invalid {
        return ITEM_ROOT_INVALID;
    }
    match density {
        ItemDensity::Standard => ITEM_ROOT,
        ItemDensity::Dense => ITEM_ROOT_DENSE,
    }
}

const fn item_media_class(density: ItemDensity, visible: bool, disabled: bool) -> &'static str {
    if !visible {
        return ITEM_MEDIA_HIDDEN;
    }
    if disabled {
        return ITEM_MEDIA_DISABLED;
    }
    match density {
        ItemDensity::Standard => ITEM_MEDIA,
        ItemDensity::Dense => ITEM_MEDIA_DENSE,
    }
}

const fn item_content_class(density: ItemDensity) -> &'static str {
    match density {
        ItemDensity::Standard => ITEM_CONTENT,
        ItemDensity::Dense => ITEM_CONTENT_DENSE,
    }
}

const fn item_title_class(density: ItemDensity, disabled: bool) -> &'static str {
    if disabled {
        return ITEM_TITLE_DISABLED;
    }
    match density {
        ItemDensity::Standard => ITEM_TITLE,
        ItemDensity::Dense => ITEM_TITLE_DENSE,
    }
}

const fn item_description_class(
    density: ItemDensity,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return ITEM_DESCRIPTION_DISABLED;
    }
    if invalid {
        return ITEM_DESCRIPTION_INVALID;
    }
    match density {
        ItemDensity::Standard => ITEM_DESCRIPTION,
        ItemDensity::Dense => ITEM_DESCRIPTION_DENSE,
    }
}

const fn item_actions_class(visible: bool) -> &'static str {
    if visible {
        ITEM_ACTIONS
    } else {
        ITEM_ACTIONS_HIDDEN
    }
}

const fn item_action_class(density: ItemDensity, active: bool, disabled: bool) -> &'static str {
    if disabled {
        ITEM_ACTION_DISABLED
    } else if active {
        ITEM_ACTION_ACTIVE
    } else {
        match density {
            ItemDensity::Standard => ITEM_ACTION,
            ItemDensity::Dense => ITEM_ACTION_DENSE,
        }
    }
}

const fn item_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    action_active: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if action_active {
        "active"
    } else {
        "ready"
    }
}

#[component]
pub fn Kbd(#[prop(optional, default = default_kbd_model())] model: KbdModel) -> AnyView {
    if let Err(report) = validate_kbd_model(&model) {
        let message = format!("Kbd validation failed: {report}");
        return view! {
            <div class=KBD_ERROR data-ui-component="kbd" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let separator = model.separator.clone();
    let state_model = model.state();
    let nodes = kbd_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == KbdPart::Root)
        .expect("invariant: kbd render nodes include root")
        .clone();
    let chord = nodes
        .iter()
        .find(|node| node.part == KbdPart::Chord)
        .expect("invariant: kbd render nodes include chord")
        .clone();
    let key_nodes = nodes
        .into_iter()
        .filter(|node| node.part == KbdPart::Key)
        .collect::<Vec<_>>();
    let invalid = root.invalid;
    let root_value = root.value;
    let chord_value = chord.value;
    let chord_label = chord.label;
    let root_aria_label = chord_label.clone();
    let chord_detail = chord.detail;
    let (state, set_state) = signal(state_model);
    let context = KbdViewContext {
        density,
        invalid,
        loading,
        blocked,
        state,
        set_state,
    };

    view! {
        <span
            role="group"
            class=kbd_root_class(density, invalid, loading, disabled)
            data-ui-component="kbd"
            data-ui-part=KbdPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    kbd_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_focused(KbdPart::Chord),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root_value
            aria-label=root_aria_label
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <span
                class=kbd_chord_class(density)
                data-ui-part=KbdPart::Chord.label()
                data-ui-value=chord_value
                aria-label=chord_label
                aria-invalid=invalid.to_string()
                title=chord_detail
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(KbdIntent::FocusChord);
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(KbdIntent::Blur);
                        });
                    }
                }
            >
                {key_nodes
                    .into_iter()
                    .map(|node| kbd_key_view(node, separator.clone(), context))
                    .collect_view()}
            </span>
        </span>
    }
    .into_any()
}

#[derive(Clone, Copy)]
struct KbdViewContext {
    density: KbdDensity,
    invalid: bool,
    loading: bool,
    blocked: bool,
    state: ReadSignal<KbdState>,
    set_state: WriteSignal<KbdState>,
}

fn kbd_key_view(node: crate::KbdRenderNode, separator: String, context: KbdViewContext) -> AnyView {
    let index = node
        .index
        .expect("invariant: kbd key render node includes index");
    let disabled = node.disabled || context.blocked;
    let show_separator = index > 0;
    let value_for_data = node.value;
    let label = node.label;
    let detail = node.detail;
    let tab_index = if disabled { "-1" } else { "0" };
    view! {
        {if show_separator {
            view! {
                <span class=KBD_SEPARATOR aria-hidden="true">
                    {separator.clone()}
                </span>
            }
            .into_any()
        } else {
            ().into_any()
        }}
        <kbd
            class=move || {
                context.state.with(|state| {
                    kbd_key_class(
                        context.density,
                        state.is_key_focused(index),
                        context.invalid,
                        context.loading,
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-part=KbdPart::Key.label()
            data-ui-index=index.to_string()
            data-ui-value=value_for_data
            aria-label=detail
            aria-disabled=disabled.to_string()
            tabindex=tab_index
            on:focus=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(KbdIntent::FocusKey(index));
                    });
                }
            }
            on:blur=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(KbdIntent::Blur);
                    });
                }
            }
            on:mouseenter=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(KbdIntent::FocusKey(index));
                    });
                }
            }
            on:mouseleave=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(KbdIntent::Blur);
                    });
                }
            }
        >
            {label}
        </kbd>
    }
    .into_any()
}

const fn kbd_root_class(
    density: KbdDensity,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return KBD_ROOT_DISABLED;
    }
    if loading {
        return KBD_ROOT_LOADING;
    }
    if invalid {
        return KBD_ROOT_INVALID;
    }
    match density {
        KbdDensity::Standard => KBD_ROOT,
        KbdDensity::Dense => KBD_ROOT_DENSE,
    }
}

const fn kbd_chord_class(density: KbdDensity) -> &'static str {
    match density {
        KbdDensity::Standard => KBD_CHORD,
        KbdDensity::Dense => KBD_CHORD_DENSE,
    }
}

const fn kbd_key_class(
    density: KbdDensity,
    focused: bool,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return KBD_KEY_DISABLED;
    }
    if loading {
        return KBD_KEY_LOADING;
    }
    if invalid {
        return KBD_KEY_INVALID;
    }
    if focused {
        return KBD_KEY_FOCUSED;
    }
    match density {
        KbdDensity::Standard => KBD_KEY,
        KbdDensity::Dense => KBD_KEY_DENSE,
    }
}

const fn kbd_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    focused: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if focused {
        "focused"
    } else {
        "ready"
    }
}

#[component]
pub fn Label(#[prop(optional, default = default_label_model())] model: LabelModel) -> AnyView {
    if let Err(report) = validate_label_model(&model) {
        let message = format!("Label validation failed: {report}");
        return view! {
            <div class=LABEL_ERROR data-ui-component="label" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let requirement = model.requirement;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = label_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == LabelPart::Root)
        .expect("invariant: label render nodes include root")
        .clone();
    let text = nodes
        .iter()
        .find(|node| node.part == LabelPart::Text)
        .expect("invariant: label render nodes include text")
        .clone();
    let requirement_node = nodes
        .iter()
        .find(|node| node.part == LabelPart::Requirement)
        .expect("invariant: label render nodes include requirement")
        .clone();
    let invalid = root.invalid;
    let root_value = root.value;
    let root_detail = root.detail;
    let text_value = text.value;
    let text_label = text.label;
    let control_for = model.control_id.clone();
    let control_for_data = model.control_id.unwrap_or_default();
    let (state, set_state) = signal(state_model);

    view! {
        <label
            class=move || {
                state.with(|state| {
                    label_root_class(
                        density,
                        state.is_active(LabelPart::Root)
                            || state.is_active(LabelPart::Text)
                            || state.is_active(LabelPart::Requirement),
                        invalid,
                        loading,
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-component="label"
            data-ui-part=LabelPart::Root.label()
            data-ui-density=density.label()
            data-ui-requirement=requirement.label()
            data-ui-state=move || {
                state.with(|state| {
                    label_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_active(LabelPart::Root)
                            || state.is_active(LabelPart::Text)
                            || state.is_active(LabelPart::Requirement),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root_value
            data-ui-for=control_for_data
            for=control_for
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
            title=root_detail
            on:focus=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(LabelIntent::Focus(LabelPart::Root));
                    });
                }
            }
            on:blur=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(LabelIntent::Blur);
                    });
                }
            }
            on:mouseenter=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(LabelIntent::Hover(LabelPart::Root));
                    });
                }
            }
            on:mouseleave=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(LabelIntent::Leave);
                    });
                }
            }
        >
            <span
                class=label_text_class(density, disabled)
                data-ui-part=LabelPart::Text.label()
                data-ui-value=text_value
                on:mouseenter=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(LabelIntent::Hover(LabelPart::Text));
                        });
                    }
                }
                on:mouseleave=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(LabelIntent::Leave);
                        });
                    }
                }
            >
                {text_label}
            </span>
            {label_requirement_view(requirement_node, blocked, state, set_state)}
        </label>
    }
    .into_any()
}

fn label_requirement_view(
    node: crate::LabelRenderNode,
    blocked: bool,
    state: ReadSignal<LabelState>,
    set_state: WriteSignal<LabelState>,
) -> AnyView {
    let density = node.density;
    let requirement = node.requirement;
    let visible = node.visible;
    let disabled = node.disabled || blocked;
    let loading = node.loading;
    let invalid = node.invalid;
    let value = node.value;
    let label = node.label;
    let detail = node.detail;
    view! {
        <span
            class=move || {
                state.with(|state| {
                    label_requirement_class(
                        density,
                        requirement,
                        visible,
                        disabled,
                        state.is_active(LabelPart::Requirement),
                    )
                    .to_owned()
                })
            }
            data-ui-part=LabelPart::Requirement.label()
            data-ui-value=value
            data-ui-state=move || {
                state.with(|state| {
                    label_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_active(LabelPart::Requirement),
                    )
                    .to_owned()
                })
            }
            aria-hidden=(!visible).to_string()
            aria-label=label
            title=detail
            on:mouseenter=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(LabelIntent::Hover(LabelPart::Requirement));
                    });
                }
            }
            on:mouseleave=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(LabelIntent::Leave);
                    });
                }
            }
        >
            {label_requirement_copy(requirement)}
        </span>
    }
    .into_any()
}

const fn label_root_class(
    density: LabelDensity,
    active: bool,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return LABEL_ROOT_DISABLED;
    }
    if loading {
        return LABEL_ROOT_LOADING;
    }
    if invalid {
        return LABEL_ROOT_INVALID;
    }
    if active {
        return LABEL_ROOT_ACTIVE;
    }
    match density {
        LabelDensity::Standard => LABEL_ROOT,
        LabelDensity::Dense => LABEL_ROOT_DENSE,
    }
}

const fn label_text_class(density: LabelDensity, disabled: bool) -> &'static str {
    if disabled {
        return LABEL_TEXT_DISABLED;
    }
    match density {
        LabelDensity::Standard => LABEL_TEXT,
        LabelDensity::Dense => LABEL_TEXT_DENSE,
    }
}

const fn label_requirement_class(
    density: LabelDensity,
    requirement: LabelRequirement,
    visible: bool,
    disabled: bool,
    _active: bool,
) -> &'static str {
    if !visible {
        return LABEL_REQUIREMENT_HIDDEN;
    }
    if disabled {
        return LABEL_REQUIREMENT_DISABLED;
    }
    match (density, requirement) {
        (LabelDensity::Standard, LabelRequirement::Required) => LABEL_REQUIREMENT_REQUIRED,
        (LabelDensity::Dense, LabelRequirement::Required) => LABEL_REQUIREMENT_DENSE_REQUIRED,
        (LabelDensity::Standard, LabelRequirement::Optional) => LABEL_REQUIREMENT_OPTIONAL,
        (LabelDensity::Dense, LabelRequirement::Optional) => LABEL_REQUIREMENT_DENSE_OPTIONAL,
        (_, LabelRequirement::None) => LABEL_REQUIREMENT_HIDDEN,
    }
}

const fn label_requirement_copy(requirement: LabelRequirement) -> &'static str {
    match requirement {
        LabelRequirement::None => "",
        LabelRequirement::Optional => "Optional",
        LabelRequirement::Required => "*",
    }
}

const fn label_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    active: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if active {
        "active"
    } else {
        "ready"
    }
}

catalog_component!(Marker, crate::MarkerModel, crate::default_marker_model);
catalog_component!(Menubar, crate::MenubarModel, crate::default_menubar_model);
catalog_component!(Message, crate::MessageModel, crate::default_message_model);
catalog_component!(
    MessageScroller,
    crate::MessageScrollerModel,
    crate::default_message_scroller_model
);
catalog_component!(
    NativeSelect,
    crate::NativeSelectModel,
    crate::default_native_select_model
);
catalog_component!(
    NavigationMenu,
    crate::NavigationMenuModel,
    crate::default_navigation_menu_model
);
catalog_component!(
    Pagination,
    crate::PaginationModel,
    crate::default_pagination_model
);
catalog_component!(Popover, crate::PopoverModel, crate::default_popover_model);
catalog_component!(
    Progress,
    crate::ProgressModel,
    crate::default_progress_model
);
catalog_component!(
    RadioGroup,
    crate::RadioGroupModel,
    crate::default_radio_group_model
);
catalog_component!(
    Resizable,
    crate::ResizableModel,
    crate::default_resizable_model
);
catalog_component!(
    ScrollArea,
    crate::ScrollAreaModel,
    crate::default_scroll_area_model
);
catalog_component!(Select, crate::SelectModel, crate::default_select_model);
catalog_component!(
    Separator,
    crate::SeparatorModel,
    crate::default_separator_model
);
catalog_component!(Sheet, crate::SheetModel, crate::default_sheet_model);
catalog_component!(Sidebar, crate::SidebarModel, crate::default_sidebar_model);
catalog_component!(
    Skeleton,
    crate::SkeletonModel,
    crate::default_skeleton_model
);
catalog_component!(Slider, crate::SliderModel, crate::default_slider_model);
catalog_component!(Sonner, crate::SonnerModel, crate::default_sonner_model);
catalog_component!(Spinner, crate::SpinnerModel, crate::default_spinner_model);
catalog_component!(Switch, crate::SwitchModel, crate::default_switch_model);
catalog_component!(Table, crate::TableModel, crate::default_table_model);
catalog_component!(Tabs, crate::TabsModel, crate::default_tabs_model);
catalog_component!(
    Textarea,
    crate::TextareaModel,
    crate::default_textarea_model
);
catalog_component!(Toast, crate::ToastModel, crate::default_toast_model);
catalog_component!(Toggle, crate::ToggleModel, crate::default_toggle_model);
catalog_component!(
    ToggleGroup,
    crate::ToggleGroupModel,
    crate::default_toggle_group_model
);
catalog_component!(Tooltip, crate::TooltipModel, crate::default_tooltip_model);
catalog_component!(
    Typography,
    crate::TypographyModel,
    crate::default_typography_model
);

#[component]
pub fn ThemeCycleButton(
    #[prop(optional, default = ThemeChoice::Auto)] initial: ThemeChoice,
) -> impl IntoView {
    let (theme, set_theme) = signal(initial);

    Effect::new(move |_| {
        apply_document_theme(theme.get());
    });

    let cycle = move |_| {
        set_theme.update(|theme| *theme = theme.next());
    };
    let label = move || theme.get().label();
    let aria_label = move || format!("Cycle theme. Current theme: {}", label());

    view! {
        <button type="button" class=THEME_BUTTON aria-label=aria_label on:click=cycle>
            <span class=THEME_BUTTON_LABEL>"Theme"</span>
            <span>{label}</span>
        </button>
    }
}

#[component]
pub fn ThemeScope(theme: ThemeId, children: Children) -> impl IntoView {
    view! {
        <div class=THEME_SCOPE data-theme=theme.slug()>
            {children()}
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_document_theme(choice: ThemeChoice) {
    let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    else {
        return;
    };

    if let Some(theme) = choice.data_theme() {
        let _ = root.set_attribute("data-theme", theme);
    } else {
        let _ = root.remove_attribute("data-theme");
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_document_theme(_choice: ThemeChoice) {}
