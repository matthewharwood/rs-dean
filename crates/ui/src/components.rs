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
    LabelPart, LabelRequirement, LabelState, MarkerDensity, MarkerIntent, MarkerModel, MarkerPart,
    MarkerState, MarkerTone, MenubarDensity, MenubarIntent, MenubarModel, MenubarPart,
    MenubarState, MessageDensity, MessageIntent, MessageModel, MessagePart, MessageScrollerDensity,
    MessageScrollerEntry, MessageScrollerIntent, MessageScrollerModel, MessageScrollerPart,
    MessageScrollerState, MessageSide, MessageState, NativeSelectDensity, NativeSelectIntent,
    NativeSelectModel, NativeSelectPart, NativeSelectState, NavigationMenuDensity,
    NavigationMenuIntent, NavigationMenuModel, NavigationMenuPart, NavigationMenuState,
    PaginationDensity, PaginationIntent, PaginationModel, PaginationPart, PaginationState,
    PopoverDensity, PopoverIntent, PopoverModel, PopoverPart, ProgressDensity, ProgressIntent,
    ProgressModel, ProgressPart, RadioGroupDensity, RadioGroupIntent, RadioGroupModel,
    RadioGroupOrientation, RadioGroupPart, ResizableDensity, ResizableIntent, ResizableModel,
    ResizableOrientation, ResizablePart, ScrollAreaAxis, ScrollAreaDensity, ScrollAreaIntent,
    ScrollAreaModel, ScrollAreaOverflow, ScrollAreaPart, SelectDensity, SelectIntent, SelectModel,
    SelectPart, SeparatorDensity, SeparatorIntent, SeparatorModel, SeparatorOrientation,
    SeparatorPart, SheetDensity, SheetIntent, SheetModel, SheetPart, SheetSide, SheetState,
    SidebarDensity, SidebarIntent, SidebarModel, SidebarPart, SkeletonDensity, SkeletonIntent,
    SkeletonModel, SkeletonPart, SkeletonState, SliderDensity, SliderIntent, SliderModel,
    SliderOrientation, SliderPart, SonnerDensity, SonnerIntent, SonnerModel, SonnerPart,
    SonnerPosition, SonnerRenderNode, SonnerState, SonnerTone, SpinnerDensity, SpinnerIntent,
    SpinnerModel, SpinnerPart, SpinnerSize, SpinnerTone, SwitchChecked, SwitchDensity,
    SwitchIntent, SwitchModel, SwitchPart, TableDensity, TableIntent, TableModel, TablePart,
    TableState, TabsDensity, TabsIntent, TabsModel, TabsOrientation, TabsPart, TabsState,
    TextareaDensity, TextareaIntent, TextareaModel, TextareaPart, ThemeChoice, ThemeId, UiBlock,
    UiBlockTone, UiComponentId, UiWidgetIntent, UiWidgetPattern, UiWidgetSlotKind,
    accordion_dom_id, alert_dialog_dom_id, aspect_ratio_render_nodes, attachment_render_nodes,
    avatar_render_nodes, badge_render_nodes, breadcrumb_render_nodes, bubble_render_nodes,
    button_group_render_nodes, button_render_nodes, calendar_render_nodes, card_render_nodes,
    carousel_render_nodes, catalog_component_render_nodes, chart_render_nodes,
    checkbox_render_nodes, collapsible_render_nodes, combobox_render_nodes, command_render_nodes,
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
    default_marker_model, default_menubar_model, default_message_model,
    default_message_scroller_model, default_native_select_model, default_navigation_menu_model,
    default_pagination_model, default_popover_model, default_progress_model,
    default_radio_group_model, default_resizable_model, default_scroll_area_model,
    default_select_model, default_separator_model, default_sheet_model, default_sidebar_model,
    default_skeleton_model, default_slider_model, default_sonner_model, default_spinner_model,
    default_switch_model, default_table_model, default_tabs_model, default_textarea_model,
    dialog_render_nodes, direction_render_nodes, drawer_render_nodes, dropdown_menu_render_nodes,
    empty_render_nodes, field_render_nodes, hover_card_render_nodes, input_group_render_nodes,
    input_otp_render_nodes, input_render_nodes, item_render_nodes, kbd_render_nodes,
    label_render_nodes, marker_render_nodes, max_data_table_page_index, menubar_render_nodes,
    message_render_nodes, message_scroller_render_nodes, month_name, native_select_render_nodes,
    navigation_menu_render_nodes, pagination_render_nodes, popover_render_nodes,
    progress_render_nodes, radio_group_render_nodes, resizable_panel_flex_style,
    resizable_render_nodes, resizable_sizes_label, scroll_area_render_nodes, select_render_nodes,
    selected_select_label, separator_render_nodes, sheet_render_nodes, sidebar_render_nodes,
    skeleton_render_nodes, slider_render_nodes, sonner_render_nodes, spinner_render_nodes,
    switch_render_nodes, table_render_nodes, tabs_dom_id, tabs_render_nodes, textarea_dom_id,
    textarea_render_nodes, validate_accordion_model, validate_alert_dialog_model,
    validate_alert_model, validate_aspect_ratio_model, validate_attachment_model,
    validate_avatar_model, validate_badge_model, validate_breadcrumb_model, validate_bubble_model,
    validate_button_group_model, validate_button_model, validate_calendar_model,
    validate_card_model, validate_carousel_model, validate_chart_model, validate_checkbox_model,
    validate_collapsible_model, validate_combobox_model, validate_command_model,
    validate_context_menu_model, validate_data_table_model, validate_date_picker_model,
    validate_dialog_model, validate_direction_model, validate_drawer_model,
    validate_dropdown_menu_model, validate_empty_model, validate_field_model,
    validate_hover_card_model, validate_input_group_model, validate_input_model,
    validate_input_otp_model, validate_item_model, validate_kbd_model, validate_label_model,
    validate_marker_model, validate_menubar_model, validate_message_model,
    validate_message_scroller_model, validate_native_select_model, validate_navigation_menu_model,
    validate_pagination_model, validate_popover_model, validate_progress_model,
    validate_radio_group_model, validate_resizable_model, validate_scroll_area_model,
    validate_select_model, validate_separator_model, validate_sheet_model, validate_sidebar_model,
    validate_skeleton_model, validate_slider_model, validate_sonner_model, validate_spinner_model,
    validate_switch_model, validate_table_model, validate_tabs_model, validate_textarea_model,
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
const TABLE_ROOT: &str = "grid w-full max-w-2xl gap-2xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const TABLE_ROOT_DENSE: &str = "grid w-full max-w-2xl gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const TABLE_ROOT_INVALID: &str = "grid w-full max-w-2xl gap-2xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const TABLE_ROOT_DISABLED: &str = "grid w-full max-w-2xl gap-2xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const TABLE_FRAME: &str =
    "overflow-hidden rounded-field border border-border-subtle bg-surface-elevated";
const TABLE_TABLE: &str = "w-full border-collapse text-left text-0 text-text-1";
const TABLE_CAPTION: &str = "caption-bottom px-xs py-2xs text-left text-0 leading-0 text-text-2";
const TABLE_CAPTION_INVALID: &str =
    "caption-bottom px-xs py-2xs text-left text-0 leading-0 text-danger";
const TABLE_HEADER: &str = "bg-surface-2";
const TABLE_BODY: &str = "divide-y divide-border-subtle";
const TABLE_HEAD: &str = "border-b border-border-subtle px-xs py-2xs text-left text-00 font-7 uppercase tracking-label text-text-muted";
const TABLE_HEAD_DENSE: &str = "border-b border-border-subtle px-2xs py-3xs text-left text-00 font-7 uppercase tracking-label text-text-muted";
const TABLE_HEAD_NUMERIC: &str = "border-b border-border-subtle px-xs py-2xs text-right text-00 font-7 uppercase tracking-label text-text-muted";
const TABLE_ROW: &str = "transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const TABLE_ROW_SELECTED: &str = "bg-primary-soft transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const TABLE_ROW_DISABLED: &str = "bg-surface-2 text-text-disabled opacity-disabled";
const TABLE_CELL: &str = "px-xs py-2xs text-0 leading-0 text-text-1";
const TABLE_CELL_DENSE: &str = "px-2xs py-3xs text-00 leading-0 text-text-1";
const TABLE_CELL_NUMERIC: &str = "px-xs py-2xs text-right text-0 leading-0 text-text-1";
const TABLE_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const TABS_ROOT: &str = "grid w-full max-w-2xl gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const TABS_ROOT_DENSE: &str = "grid w-full max-w-2xl gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const TABS_ROOT_VERTICAL: &str = "flex w-full max-w-2xl flex-col gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1 md:flex-row";
const TABS_ROOT_DENSE_VERTICAL: &str = "flex w-full max-w-2xl flex-col gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1 md:flex-row";
const TABS_ROOT_INVALID: &str = "grid w-full max-w-2xl gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const TABS_ROOT_DISABLED: &str = "grid w-full max-w-2xl gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const TABS_LIST: &str = "flex flex-wrap items-center gap-2xs rounded-field bg-surface-2 p-2xs";
const TABS_LIST_DENSE: &str =
    "flex flex-wrap items-center gap-2xs rounded-field bg-surface-2 p-3xs";
const TABS_LIST_VERTICAL: &str =
    "flex shrink-0 flex-col items-stretch gap-2xs rounded-field bg-surface-2 p-2xs";
const TABS_LIST_DENSE_VERTICAL: &str =
    "flex shrink-0 flex-col items-stretch gap-2xs rounded-field bg-surface-2 p-3xs";
const TABS_TRIGGER: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-transparent px-xs py-2xs text-0 font-6 text-text-2 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled disabled:opacity-disabled";
const TABS_TRIGGER_DENSE: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-transparent px-2xs py-3xs text-00 font-6 text-text-2 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled disabled:opacity-disabled";
const TABS_TRIGGER_SELECTED: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled disabled:opacity-disabled";
const TABS_TRIGGER_DENSE_SELECTED: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-2xs py-3xs text-00 font-7 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled disabled:opacity-disabled";
const TABS_TRIGGER_FOCUSED: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-focus-ring bg-selected-tint px-xs py-2xs text-0 font-7 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const TABS_TRIGGER_INVALID: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-danger bg-error-soft px-xs py-2xs text-0 font-7 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const TABS_TRIGGER_DISABLED: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 text-text-disabled opacity-disabled";
const TABS_STATUS: &str = "m-0 text-00 font-6 uppercase tracking-label text-text-muted";
const TABS_CONTENT: &str = "grid gap-2xs rounded-field border border-border-subtle bg-surface-elevated p-s text-0 leading-0 text-text-2";
const TABS_CONTENT_DENSE: &str = "grid gap-2xs rounded-field border border-border-subtle bg-surface-elevated p-xs text-00 leading-0 text-text-2";
const TABS_CONTENT_INVALID: &str = "grid gap-2xs rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const TABS_CONTENT_DISABLED: &str = "grid gap-2xs rounded-field border border-border-muted bg-surface-2 p-s text-0 leading-0 text-text-disabled opacity-disabled";
const TABS_CONTENT_COPY: &str = "m-0";
const TABS_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const TEXTAREA_ROOT: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const TEXTAREA_ROOT_DENSE: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const TEXTAREA_ROOT_LOADING: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-subtle bg-surface-2 p-s text-text-1 shadow-1";
const TEXTAREA_ROOT_INVALID: &str = "grid w-full max-w-md gap-2xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const TEXTAREA_ROOT_DISABLED: &str = "grid w-full max-w-md gap-2xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const TEXTAREA_LABEL: &str =
    "flex min-w-0 items-center gap-2xs text-0 font-7 leading-0 text-text-1";
const TEXTAREA_LABEL_DENSE: &str =
    "flex min-w-0 items-center gap-2xs text-00 font-7 leading-0 text-text-1";
const TEXTAREA_LABEL_DISABLED: &str =
    "flex min-w-0 items-center gap-2xs text-0 font-7 leading-0 text-text-disabled";
const TEXTAREA_REQUIRED: &str = "text-danger";
const TEXTAREA_CONTROL: &str = "min-h-xl w-full resize-y rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const TEXTAREA_CONTROL_DENSE: &str = "min-h-l w-full resize-y rounded-field border border-border-strong bg-surface-1 px-2xs py-3xs text-00 leading-0 text-text-1 outline-none transition-colors placeholder:text-text-muted focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const TEXTAREA_CONTROL_FOCUSED: &str = "min-h-xl w-full resize-y rounded-field border border-brand bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 outline-none shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const TEXTAREA_CONTROL_DENSE_FOCUSED: &str = "min-h-l w-full resize-y rounded-field border border-brand bg-surface-1 px-2xs py-3xs text-00 leading-0 text-text-1 outline-none shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const TEXTAREA_CONTROL_INVALID: &str = "min-h-xl w-full resize-y rounded-field border border-danger bg-error-soft px-xs py-2xs text-0 leading-0 text-text-1 outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const TEXTAREA_CONTROL_DENSE_INVALID: &str = "min-h-l w-full resize-y rounded-field border border-danger bg-error-soft px-2xs py-3xs text-00 leading-0 text-text-1 outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const TEXTAREA_CONTROL_DISABLED: &str = "min-h-xl w-full resize-y rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 leading-0 text-text-disabled opacity-disabled";
const TEXTAREA_CONTROL_DENSE_DISABLED: &str = "min-h-l w-full resize-y rounded-field border border-border-muted bg-surface-2 px-2xs py-3xs text-00 leading-0 text-text-disabled opacity-disabled";
const TEXTAREA_META: &str = "flex flex-wrap items-start justify-between gap-2xs";
const TEXTAREA_HINT: &str = "m-0 min-w-0 flex-1 text-00 leading-0 text-text-2";
const TEXTAREA_HINT_INVALID: &str = "m-0 min-w-0 flex-1 text-00 leading-0 text-danger";
const TEXTAREA_HINT_DISABLED: &str = "m-0 min-w-0 flex-1 text-00 leading-0 text-text-disabled";
const TEXTAREA_COUNTER: &str =
    "shrink-0 rounded-pill bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-muted";
const TEXTAREA_COUNTER_LIMIT: &str =
    "shrink-0 rounded-pill bg-warning-soft px-2xs py-3xs text-00 font-7 text-text-1";
const TEXTAREA_COUNTER_DISABLED: &str =
    "shrink-0 rounded-pill bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-disabled";
const TEXTAREA_ERROR: &str =
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
const SHEET_ROOT: &str = "grid w-full gap-2xs text-text-1";
const SHEET_ROOT_DENSE: &str = "grid w-full gap-3xs text-text-1";
const SHEET_ROOT_DISABLED: &str = "grid w-full gap-2xs text-text-disabled";
const SHEET_TRIGGER: &str = "inline-flex min-h-field w-fit items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SHEET_TRIGGER_DENSE: &str = "inline-flex min-h-s w-fit items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SHEET_TRIGGER_OPEN: &str = "inline-flex min-h-field w-fit items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SHEET_TRIGGER_DISABLED: &str = "inline-flex min-h-field w-fit items-center justify-center gap-2xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 text-text-disabled opacity-disabled";
const SHEET_CONTENT: &str = "grid w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-3";
const SHEET_CONTENT_DENSE: &str = "grid w-full max-w-md gap-xs rounded-field border border-border-subtle bg-surface-elevated p-xs text-text-1 shadow-2";
const SHEET_CONTENT_RIGHT: &str = "justify-self-end border-l-brand";
const SHEET_CONTENT_LEFT: &str = "justify-self-start border-r-brand";
const SHEET_CONTENT_TOP: &str = "justify-self-stretch border-b-brand";
const SHEET_CONTENT_BOTTOM: &str = "justify-self-stretch border-t-brand";
const SHEET_HEADER: &str = "grid gap-2xs";
const SHEET_HEADER_DENSE: &str = "grid gap-3xs";
const SHEET_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const SHEET_TITLE_DENSE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const SHEET_DESCRIPTION: &str = "m-0 text-0 leading-0 text-text-2";
const SHEET_DESCRIPTION_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const SHEET_BODY: &str = "m-0 text-0 leading-0 text-text-2";
const SHEET_BODY_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const SHEET_FOOTER: &str = "flex flex-wrap items-center justify-end gap-2xs";
const SHEET_ACTION: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SHEET_ACTION_DENSE: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-2xs py-3xs text-00 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SHEET_ACTION_SECONDARY: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SHEET_ACTION_ACTIVE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-selected-tint px-xs py-2xs text-0 font-7 text-text-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SHEET_ACTION_DISABLED: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 text-text-disabled opacity-disabled";
const SHEET_CLOSE: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SHEET_CLOSE_DISABLED: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 text-text-disabled opacity-disabled";
const SHEET_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const SIDEBAR_ROOT: &str = "flex w-full max-w-md overflow-hidden rounded-box border border-border-subtle bg-surface-1 text-text-1 shadow-1";
const SIDEBAR_ROOT_DENSE: &str = "flex w-full max-w-md overflow-hidden rounded-field border border-border-subtle bg-surface-1 text-text-1 shadow-1";
const SIDEBAR_ROOT_COLLAPSED: &str = "flex w-fit overflow-hidden rounded-box border border-border-subtle bg-surface-1 text-text-1 shadow-1";
const SIDEBAR_ROOT_INVALID: &str = "flex w-full max-w-md overflow-hidden rounded-box border border-danger bg-error-soft text-text-1 shadow-1";
const SIDEBAR_ROOT_DISABLED: &str = "flex w-full max-w-md overflow-hidden rounded-box border border-border-muted bg-surface-2 text-text-disabled opacity-disabled";
const SIDEBAR_RAIL: &str = "flex min-h-xl w-s items-center justify-center border-r border-border-subtle bg-surface-2 text-00 font-7 text-text-muted transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SIDEBAR_RAIL_ACTIVE: &str = "flex min-h-xl w-s items-center justify-center border-r border-brand bg-primary-soft text-00 font-7 text-brand shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SIDEBAR_RAIL_DISABLED: &str = "flex min-h-xl w-s items-center justify-center border-r border-border-muted bg-surface-2 text-00 font-7 text-text-disabled opacity-disabled";
const SIDEBAR_PANEL: &str = "grid min-w-0 gap-xs p-xs";
const SIDEBAR_PANEL_DENSE: &str = "grid min-w-0 gap-2xs p-2xs";
const SIDEBAR_PANEL_COLLAPSED: &str = "hidden";
const SIDEBAR_HEADER: &str = "grid gap-3xs border-b border-border-subtle pb-xs";
const SIDEBAR_HEADER_DENSE: &str = "grid gap-3xs border-b border-border-subtle pb-2xs";
const SIDEBAR_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const SIDEBAR_TITLE_DENSE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const SIDEBAR_DETAIL: &str = "m-0 text-00 leading-0 text-text-muted";
const SIDEBAR_CONTENT: &str = "grid gap-xs";
const SIDEBAR_CONTENT_DENSE: &str = "grid gap-2xs";
const SIDEBAR_GROUP: &str = "grid gap-3xs";
const SIDEBAR_GROUP_DISABLED: &str = "grid gap-3xs opacity-disabled";
const SIDEBAR_GROUP_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const SIDEBAR_MENU: &str = "flex min-h-field w-full items-center justify-between gap-2xs rounded-field border border-transparent bg-surface-1 px-xs py-2xs text-left text-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SIDEBAR_MENU_DENSE: &str = "flex min-h-s w-full items-center justify-between gap-2xs rounded-field border border-transparent bg-surface-1 px-2xs py-3xs text-left text-00 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SIDEBAR_MENU_ACTIVE: &str = "flex min-h-field w-full items-center justify-between gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-left text-0 font-7 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SIDEBAR_MENU_FOCUSED: &str = "flex min-h-field w-full items-center justify-between gap-2xs rounded-field border border-brand bg-surface-1 px-xs py-2xs text-left text-0 font-6 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SIDEBAR_MENU_DISABLED: &str = "flex min-h-field w-full items-center justify-between gap-2xs rounded-field border border-transparent bg-surface-2 px-xs py-2xs text-left text-0 text-text-disabled opacity-disabled";
const SIDEBAR_BADGE: &str = "rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-7 text-text-muted";
const SIDEBAR_FOOTER: &str = "grid gap-3xs border-t border-border-subtle pt-xs";
const SIDEBAR_FOOTER_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const SIDEBAR_FOOTER_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const SIDEBAR_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const SKELETON_ROOT: &str = "grid w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const SKELETON_ROOT_DENSE: &str = "grid w-full max-w-md gap-xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const SKELETON_ROOT_READY: &str = "grid w-full max-w-md gap-s rounded-box border border-border-subtle bg-surface-2 p-s text-text-muted";
const SKELETON_ROOT_INVALID: &str = "grid w-full max-w-md gap-s rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const SKELETON_ROOT_DISABLED: &str = "grid w-full max-w-md gap-s rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const SKELETON_CONTENT: &str = "grid gap-xs";
const SKELETON_CONTENT_DENSE: &str = "grid gap-2xs";
const SKELETON_BLOCK: &str = "h-l rounded-field bg-surface-3 transition-colors";
const SKELETON_BLOCK_DENSE: &str = "h-m rounded-field bg-surface-3 transition-colors";
const SKELETON_BLOCK_ACTIVE: &str =
    "h-l rounded-field border border-brand bg-selected-tint shadow-1";
const SKELETON_BLOCK_DENSE_ACTIVE: &str =
    "h-m rounded-field border border-brand bg-selected-tint shadow-1";
const SKELETON_BLOCK_INVALID: &str = "h-l rounded-field border border-danger bg-error-soft";
const SKELETON_BLOCK_DENSE_INVALID: &str = "h-m rounded-field border border-danger bg-error-soft";
const SKELETON_BLOCK_DISABLED: &str = "h-l rounded-field bg-surface-2 opacity-disabled";
const SKELETON_BLOCK_DENSE_DISABLED: &str = "h-m rounded-field bg-surface-2 opacity-disabled";
const SKELETON_LINE: &str = "h-xs rounded-pill bg-surface-3 transition-colors";
const SKELETON_LINE_DENSE: &str = "h-2xs rounded-pill bg-surface-3 transition-colors";
const SKELETON_LINE_ACTIVE: &str =
    "h-xs rounded-pill border border-brand bg-selected-tint shadow-1";
const SKELETON_LINE_DENSE_ACTIVE: &str =
    "h-2xs rounded-pill border border-brand bg-selected-tint shadow-1";
const SKELETON_LINE_INVALID: &str = "h-xs rounded-pill border border-danger bg-error-soft";
const SKELETON_LINE_DENSE_INVALID: &str = "h-2xs rounded-pill border border-danger bg-error-soft";
const SKELETON_LINE_DISABLED: &str = "h-xs rounded-pill bg-surface-2 opacity-disabled";
const SKELETON_LINE_DENSE_DISABLED: &str = "h-2xs rounded-pill bg-surface-2 opacity-disabled";
const SKELETON_MEDIA: &str = "min-h-xl rounded-field bg-surface-3 transition-colors";
const SKELETON_MEDIA_DENSE: &str = "min-h-l rounded-field bg-surface-3 transition-colors";
const SKELETON_MEDIA_ACTIVE: &str =
    "min-h-xl rounded-field border border-brand bg-selected-tint shadow-1";
const SKELETON_MEDIA_DENSE_ACTIVE: &str =
    "min-h-l rounded-field border border-brand bg-selected-tint shadow-1";
const SKELETON_MEDIA_INVALID: &str = "min-h-xl rounded-field border border-danger bg-error-soft";
const SKELETON_MEDIA_DENSE_INVALID: &str =
    "min-h-l rounded-field border border-danger bg-error-soft";
const SKELETON_MEDIA_DISABLED: &str = "min-h-xl rounded-field bg-surface-2 opacity-disabled";
const SKELETON_MEDIA_DENSE_DISABLED: &str = "min-h-l rounded-field bg-surface-2 opacity-disabled";
const SKELETON_STATUS: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const SKELETON_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-xs text-0 leading-0 text-text-1";
const SLIDER_ROOT: &str = "grid w-full max-w-md gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const SLIDER_ROOT_DENSE: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const SLIDER_ROOT_VERTICAL: &str = "grid w-fit gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const SLIDER_ROOT_INVALID: &str = "grid w-full max-w-md gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const SLIDER_ROOT_DISABLED: &str = "grid w-full max-w-md gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const SLIDER_HEADER: &str = "flex min-w-0 items-center justify-between gap-xs";
const SLIDER_LABEL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const SLIDER_VALUE: &str = "rounded-pill border border-border-subtle bg-surface-2 px-xs py-3xs text-00 font-7 text-text-muted";
const SLIDER_TRACK_WRAP: &str = "relative grid min-h-field items-center";
const SLIDER_TRACK_WRAP_VERTICAL: &str = "relative grid min-h-xl w-l justify-items-center";
const SLIDER_TRACK: &str = "h-xs w-full overflow-hidden rounded-pill bg-surface-3";
const SLIDER_TRACK_DENSE: &str = "h-2xs w-full overflow-hidden rounded-pill bg-surface-3";
const SLIDER_TRACK_INVALID: &str =
    "h-xs w-full overflow-hidden rounded-pill border border-danger bg-error-soft";
const SLIDER_TRACK_DISABLED: &str =
    "h-xs w-full overflow-hidden rounded-pill bg-surface-2 opacity-disabled";
const SLIDER_TRACK_VERTICAL: &str = "h-xl w-xs overflow-hidden rounded-pill bg-surface-3";
const SLIDER_RANGE: &str = "block h-full rounded-pill bg-brand";
const SLIDER_RANGE_DRAGGING: &str = "block h-full rounded-pill bg-accent";
const SLIDER_RANGE_INVALID: &str = "block h-full rounded-pill bg-danger";
const SLIDER_RANGE_DISABLED: &str = "block h-full rounded-pill bg-border-muted";
const SLIDER_INPUT: &str =
    "absolute inset-0 h-full w-full cursor-pointer opacity-0 disabled:cursor-not-allowed";
const SLIDER_THUMB: &str =
    "absolute top-1/2 size-s rounded-pill border border-brand bg-surface-1 shadow-2";
const SLIDER_THUMB_DENSE: &str =
    "absolute top-1/2 size-xs rounded-pill border border-brand bg-surface-1 shadow-1";
const SLIDER_THUMB_FOCUSED: &str =
    "absolute top-1/2 size-s rounded-pill border border-brand bg-primary-soft shadow-2";
const SLIDER_THUMB_DISABLED: &str =
    "absolute top-1/2 size-s rounded-pill border border-border-muted bg-surface-2 opacity-disabled";
const SLIDER_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-xs text-0 leading-0 text-text-1";
const SONNER_PROVIDER: &str = "grid w-full max-w-md gap-xs text-text-1";
const SONNER_PROVIDER_DENSE: &str = "grid w-full max-w-md gap-2xs text-text-1";
const SONNER_PROVIDER_INVALID: &str =
    "grid w-full max-w-md gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1";
const SONNER_PROVIDER_DISABLED: &str = "grid w-full max-w-md gap-xs text-text-disabled";
const SONNER_VIEWPORT_CENTER: &str = "grid justify-items-center gap-xs";
const SONNER_VIEWPORT_END: &str = "grid justify-items-end gap-xs";
const SONNER_VIEWPORT_DENSE_CENTER: &str = "grid justify-items-center gap-2xs";
const SONNER_VIEWPORT_DENSE_END: &str = "grid justify-items-end gap-2xs";
const SONNER_TOAST: &str = "grid gap-xs rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-2";
const SONNER_TOAST_DENSE: &str = "grid gap-2xs rounded-field border border-border-subtle bg-surface-elevated p-xs text-text-1 shadow-1";
const SONNER_TOAST_INFO: &str =
    "grid gap-xs rounded-box border border-info bg-info-soft p-s text-text-1 shadow-2";
const SONNER_TOAST_INFO_DENSE: &str =
    "grid gap-2xs rounded-field border border-info bg-info-soft p-xs text-text-1 shadow-1";
const SONNER_TOAST_SUCCESS: &str =
    "grid gap-xs rounded-box border border-success bg-success-soft p-s text-text-1 shadow-2";
const SONNER_TOAST_SUCCESS_DENSE: &str =
    "grid gap-2xs rounded-field border border-success bg-success-soft p-xs text-text-1 shadow-1";
const SONNER_TOAST_WARNING: &str =
    "grid gap-xs rounded-box border border-warning bg-warning-soft p-s text-text-1 shadow-2";
const SONNER_TOAST_WARNING_DENSE: &str =
    "grid gap-2xs rounded-field border border-warning bg-warning-soft p-xs text-text-1 shadow-1";
const SONNER_TOAST_DESTRUCTIVE: &str =
    "grid gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-2";
const SONNER_TOAST_DESTRUCTIVE_DENSE: &str =
    "grid gap-2xs rounded-field border border-danger bg-error-soft p-xs text-text-1 shadow-1";
const SONNER_TOAST_ACTIVE: &str =
    "grid gap-xs rounded-box border border-brand bg-primary-soft p-s text-text-1 shadow-2";
const SONNER_TOAST_INVALID: &str =
    "grid gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const SONNER_TOAST_DISABLED: &str = "grid gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const SONNER_HEADER: &str = "flex min-w-0 items-start justify-between gap-xs";
const SONNER_COPY: &str = "grid min-w-0 gap-3xs";
const SONNER_TITLE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const SONNER_DESCRIPTION: &str = "m-0 text-0 leading-0 text-text-2";
const SONNER_META: &str = "m-0 text-00 uppercase tracking-label text-text-muted";
const SONNER_ACTION_ROW: &str = "flex flex-wrap items-center gap-2xs";
const SONNER_ACTION: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-3xs text-00 font-6 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SONNER_ACTION_ACTIVE: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-3xs text-00 font-7 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SONNER_ACTION_DISABLED: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-border-muted bg-surface-2 px-xs py-3xs text-00 font-6 text-text-disabled opacity-disabled";
const SONNER_DISMISS: &str = "inline-flex size-m shrink-0 items-center justify-center rounded-field border border-border-strong bg-surface-2 text-00 font-7 text-text-muted transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SONNER_DISMISS_ACTIVE: &str = "inline-flex size-m shrink-0 items-center justify-center rounded-field border border-brand bg-primary-soft text-00 font-7 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SONNER_DISMISS_DISABLED: &str = "inline-flex size-m shrink-0 items-center justify-center rounded-field border border-border-muted bg-surface-2 text-00 font-7 text-text-disabled opacity-disabled";
const SONNER_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-xs text-0 leading-0 text-text-1";
const SPINNER_ROOT: &str = "inline-flex w-fit items-center gap-2xs rounded-field border border-border-subtle bg-surface-1 px-xs py-2xs text-text-1 shadow-1";
const SPINNER_ROOT_DENSE: &str = "inline-flex w-fit items-center gap-2xs rounded-field border border-border-subtle bg-surface-1 px-2xs py-3xs text-text-1 shadow-1";
const SPINNER_ROOT_INVALID: &str = "inline-flex w-fit items-center gap-2xs rounded-field border border-danger bg-error-soft px-xs py-2xs text-text-1 shadow-1";
const SPINNER_ROOT_DISABLED: &str = "inline-flex w-fit items-center gap-2xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-text-disabled opacity-disabled";
const SPINNER_TRACK_SMALL: &str = "relative inline-grid size-s shrink-0 place-items-center rounded-pill border border-border-muted";
const SPINNER_TRACK_MEDIUM: &str = "relative inline-grid size-m shrink-0 place-items-center rounded-pill border border-border-muted";
const SPINNER_TRACK_LARGE: &str = "relative inline-grid size-l shrink-0 place-items-center rounded-pill border border-border-muted";
const SPINNER_TRACK_INVALID: &str =
    "relative inline-grid size-m shrink-0 place-items-center rounded-pill border border-danger";
const SPINNER_TRACK_DISABLED: &str = "relative inline-grid size-m shrink-0 place-items-center rounded-pill border border-border-muted opacity-disabled";
const SPINNER_INDICATOR_BRAND: &str = "absolute inset-0 rounded-pill border border-transparent border-t-brand animate-spin motion-reduce:animate-none";
const SPINNER_INDICATOR_DEFAULT: &str = "absolute inset-0 rounded-pill border border-transparent border-t-accent animate-spin motion-reduce:animate-none";
const SPINNER_INDICATOR_INFO: &str = "absolute inset-0 rounded-pill border border-transparent border-t-info animate-spin motion-reduce:animate-none";
const SPINNER_INDICATOR_SUCCESS: &str = "absolute inset-0 rounded-pill border border-transparent border-t-success animate-spin motion-reduce:animate-none";
const SPINNER_INDICATOR_WARNING: &str = "absolute inset-0 rounded-pill border border-transparent border-t-warning animate-spin motion-reduce:animate-none";
const SPINNER_INDICATOR_DESTRUCTIVE: &str = "absolute inset-0 rounded-pill border border-transparent border-t-danger animate-spin motion-reduce:animate-none";
const SPINNER_INDICATOR_PAUSED: &str =
    "absolute inset-0 rounded-pill border border-transparent border-t-accent";
const SPINNER_INDICATOR_INVALID: &str =
    "absolute inset-0 rounded-pill border border-transparent border-t-danger";
const SPINNER_INDICATOR_DISABLED: &str =
    "absolute inset-0 rounded-pill border border-transparent border-t-border-muted";
const SPINNER_LABEL: &str = "m-0 text-0 font-6 leading-0 text-text-1";
const SPINNER_LABEL_DENSE: &str = "m-0 text-00 font-6 leading-0 text-text-2";
const SPINNER_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-xs text-0 leading-0 text-text-1";
const SWITCH_ROOT: &str = "flex w-full max-w-md items-center justify-between gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const SWITCH_ROOT_DENSE: &str = "flex w-full max-w-md items-center justify-between gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const SWITCH_ROOT_INVALID: &str = "flex w-full max-w-md items-center justify-between gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const SWITCH_ROOT_DISABLED: &str = "flex w-full max-w-md items-center justify-between gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const SWITCH_COPY: &str = "grid min-w-0 gap-3xs";
const SWITCH_LABEL_ROW: &str = "flex min-w-0 flex-wrap items-center gap-2xs";
const SWITCH_LABEL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const SWITCH_LABEL_DISABLED: &str = "m-0 text-0 font-7 leading-0 text-text-disabled";
const SWITCH_DETAIL: &str = "m-0 text-0 leading-0 text-text-2";
const SWITCH_DETAIL_INVALID: &str = "m-0 text-0 leading-0 text-danger";
const SWITCH_DETAIL_DISABLED: &str = "m-0 text-0 leading-0 text-text-disabled";
const SWITCH_REQUIRED: &str = "text-danger";
const SWITCH_STATUS: &str = "rounded-pill border border-border-subtle bg-surface-2 px-2xs py-3xs text-00 font-7 uppercase tracking-label text-text-muted";
const SWITCH_STATUS_ON: &str = "rounded-pill border border-brand bg-primary-soft px-2xs py-3xs text-00 font-7 uppercase tracking-label text-text-1";
const SWITCH_STATUS_INVALID: &str = "rounded-pill border border-danger bg-error-soft px-2xs py-3xs text-00 font-7 uppercase tracking-label text-text-1";
const SWITCH_STATUS_DISABLED: &str = "rounded-pill border border-border-muted bg-surface-2 px-2xs py-3xs text-00 font-7 uppercase tracking-label text-text-disabled";
const SWITCH_TRACK: &str = "inline-flex min-h-m w-xl shrink-0 items-center justify-start rounded-pill border border-border-strong bg-surface-3 p-3xs shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SWITCH_TRACK_DENSE: &str = "inline-flex min-h-s w-l shrink-0 items-center justify-start rounded-pill border border-border-strong bg-surface-3 p-3xs shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SWITCH_TRACK_ON: &str = "inline-flex min-h-m w-xl shrink-0 items-center justify-end rounded-pill border border-brand bg-brand p-3xs shadow-2 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SWITCH_TRACK_DENSE_ON: &str = "inline-flex min-h-s w-l shrink-0 items-center justify-end rounded-pill border border-brand bg-brand p-3xs shadow-2 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SWITCH_TRACK_ACTIVE: &str = "inline-flex min-h-m w-xl shrink-0 items-center justify-start rounded-pill border border-brand bg-primary-soft p-3xs shadow-2 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SWITCH_TRACK_ACTIVE_ON: &str = "inline-flex min-h-m w-xl shrink-0 items-center justify-end rounded-pill border border-brand bg-primary-soft p-3xs shadow-2 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SWITCH_TRACK_DENSE_ACTIVE: &str = "inline-flex min-h-s w-l shrink-0 items-center justify-start rounded-pill border border-brand bg-primary-soft p-3xs shadow-2 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SWITCH_TRACK_DENSE_ACTIVE_ON: &str = "inline-flex min-h-s w-l shrink-0 items-center justify-end rounded-pill border border-brand bg-primary-soft p-3xs shadow-2 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SWITCH_TRACK_INVALID: &str = "inline-flex min-h-m w-xl shrink-0 items-center justify-start rounded-pill border border-danger bg-error-soft p-3xs shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SWITCH_TRACK_DISABLED: &str = "inline-flex min-h-m w-xl shrink-0 items-center justify-start rounded-pill border border-border-muted bg-surface-2 p-3xs opacity-disabled";
const SWITCH_THUMB: &str =
    "size-s rounded-pill border border-border-subtle bg-surface-1 shadow-2 transition-colors";
const SWITCH_THUMB_DENSE: &str =
    "size-xs rounded-pill border border-border-subtle bg-surface-1 shadow-1 transition-colors";
const SWITCH_THUMB_ON: &str =
    "size-s rounded-pill border border-surface-1 bg-surface-1 shadow-2 transition-colors";
const SWITCH_THUMB_ACTIVE: &str =
    "size-s rounded-pill border border-brand bg-surface-1 shadow-2 transition-colors";
const SWITCH_THUMB_DENSE_ACTIVE: &str =
    "size-xs rounded-pill border border-brand bg-surface-1 shadow-1 transition-colors";
const SWITCH_THUMB_INVALID: &str = "size-s rounded-pill border border-danger bg-surface-1 shadow-1";
const SWITCH_THUMB_DISABLED: &str =
    "size-s rounded-pill border border-border-muted bg-surface-2 shadow-1";
const SWITCH_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-xs text-0 leading-0 text-text-1";
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
const NATIVE_SELECT_ROOT: &str = "grid w-full max-w-md gap-2xs text-text-1";
const NATIVE_SELECT_ROOT_DISABLED: &str = "grid w-full max-w-md gap-2xs text-text-disabled";
const NATIVE_SELECT_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const NATIVE_SELECT_LABEL_DISABLED: &str =
    "m-0 text-00 font-7 uppercase tracking-label text-text-disabled";
const NATIVE_SELECT_TRIGGER: &str = "min-h-field w-full rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 shadow-1 outline-none transition-colors focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled disabled:opacity-disabled";
const NATIVE_SELECT_TRIGGER_DENSE: &str = "min-h-s w-full rounded-field border border-border-strong bg-surface-1 px-2xs py-3xs text-00 leading-0 text-text-1 shadow-1 outline-none transition-colors focus-visible:border-brand focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled disabled:opacity-disabled";
const NATIVE_SELECT_TRIGGER_FOCUSED: &str = "min-h-field w-full rounded-field border border-brand bg-surface-1 px-xs py-2xs text-0 leading-0 text-text-1 shadow-2 outline-none transition-colors";
const NATIVE_SELECT_TRIGGER_DENSE_FOCUSED: &str = "min-h-s w-full rounded-field border border-brand bg-surface-1 px-2xs py-3xs text-00 leading-0 text-text-1 shadow-2 outline-none transition-colors";
const NATIVE_SELECT_TRIGGER_INVALID: &str = "min-h-field w-full rounded-field border border-danger bg-error-soft px-xs py-2xs text-0 leading-0 text-text-1 shadow-1 outline-none transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NATIVE_SELECT_TRIGGER_DENSE_INVALID: &str = "min-h-s w-full rounded-field border border-danger bg-error-soft px-2xs py-3xs text-00 leading-0 text-text-1 shadow-1 outline-none transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NATIVE_SELECT_TRIGGER_LOADING: &str = "min-h-field w-full rounded-field border border-info bg-info-soft px-xs py-2xs text-0 leading-0 text-text-1 shadow-1 outline-none";
const NATIVE_SELECT_TRIGGER_DISABLED: &str = "min-h-field w-full rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 leading-0 text-text-disabled opacity-disabled outline-none";
const NATIVE_SELECT_VALUE: &str = "m-0 text-00 leading-0 text-text-2";
const NATIVE_SELECT_VALUE_PLACEHOLDER: &str = "m-0 text-00 leading-0 text-text-muted";
const NATIVE_SELECT_VALUE_INVALID: &str = "m-0 text-00 font-6 leading-0 text-danger";
const NAVIGATION_MENU_ROOT: &str = "grid w-full max-w-3xl gap-2xs text-text-1";
const NAVIGATION_MENU_ROOT_DISABLED: &str = "grid w-full max-w-3xl gap-2xs text-text-disabled";
const NAVIGATION_MENU_LIST: &str = "flex flex-wrap items-start gap-2xs rounded-box border border-border-subtle bg-surface-1 p-2xs shadow-1";
const NAVIGATION_MENU_LIST_DENSE: &str = "flex flex-wrap items-start gap-3xs rounded-box border border-border-subtle bg-surface-1 p-3xs shadow-1";
const NAVIGATION_MENU_LIST_LOADING: &str =
    "flex flex-wrap items-start gap-2xs rounded-box border border-info bg-info-soft p-2xs shadow-1";
const NAVIGATION_MENU_ITEM: &str = "relative grid gap-2xs";
const NAVIGATION_MENU_TRIGGER: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-border-subtle bg-surface-1 px-xs py-2xs text-0 font-6 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled disabled:opacity-disabled";
const NAVIGATION_MENU_TRIGGER_DENSE: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-subtle bg-surface-1 px-2xs py-3xs text-00 font-6 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled disabled:opacity-disabled";
const NAVIGATION_MENU_TRIGGER_OPEN: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 leading-0 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_TRIGGER_DENSE_OPEN: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-brand bg-primary-soft px-2xs py-3xs text-00 font-7 leading-0 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_TRIGGER_INVALID: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-danger bg-error-soft px-xs py-2xs text-0 font-7 leading-0 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_TRIGGER_DISABLED: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 leading-0 text-text-disabled opacity-disabled";
const NAVIGATION_MENU_LINK: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-border-subtle bg-surface-1 px-xs py-2xs text-0 font-6 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_LINK_DENSE: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-subtle bg-surface-1 px-2xs py-3xs text-00 font-6 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_LINK_SELECTED: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-brand bg-selected-tint px-xs py-2xs text-0 font-7 leading-0 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_LINK_DENSE_SELECTED: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-brand bg-selected-tint px-2xs py-3xs text-00 font-7 leading-0 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_LINK_INVALID: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-danger bg-error-soft px-xs py-2xs text-0 font-7 leading-0 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_LINK_DISABLED: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 leading-0 text-text-disabled opacity-disabled";
const NAVIGATION_MENU_CONTENT: &str = "grid min-w-xl gap-2xs rounded-box border border-border-subtle bg-surface-elevated p-s shadow-2";
const NAVIGATION_MENU_CONTENT_DENSE: &str = "grid min-w-l gap-3xs rounded-box border border-border-subtle bg-surface-elevated p-xs shadow-1";
const NAVIGATION_MENU_CONTENT_INVALID: &str =
    "grid min-w-xl gap-2xs rounded-box border border-danger bg-error-soft p-s shadow-1";
const NAVIGATION_MENU_CONTENT_LOADING: &str =
    "grid min-w-xl gap-2xs rounded-box border border-info bg-info-soft p-s shadow-1";
const NAVIGATION_MENU_CONTENT_DISABLED: &str = "grid min-w-xl gap-2xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const NAVIGATION_MENU_CONTENT_HIDDEN: &str = "hidden";
const NAVIGATION_MENU_PANEL_LINK: &str = "grid gap-3xs rounded-field border border-border-subtle bg-surface-1 p-xs text-left text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_PANEL_LINK_DENSE: &str = "grid gap-3xs rounded-field border border-border-subtle bg-surface-1 p-2xs text-left text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_PANEL_LINK_SELECTED: &str = "grid gap-3xs rounded-field border border-brand bg-primary-soft p-xs text-left text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const NAVIGATION_MENU_PANEL_LINK_DISABLED: &str = "grid gap-3xs rounded-field border border-border-muted bg-surface-2 p-xs text-left text-text-disabled opacity-disabled";
const NAVIGATION_MENU_PANEL_TITLE: &str = "text-0 font-7 leading-0 text-text-1";
const NAVIGATION_MENU_PANEL_DETAIL: &str = "text-00 leading-0 text-text-2";
const NAVIGATION_MENU_ERROR: &str = "m-0 text-00 font-6 leading-0 text-danger";
const NAVIGATION_MENU_ERROR_HIDDEN: &str = "hidden";
const PAGINATION_ROOT: &str = "grid w-full max-w-2xl gap-2xs text-text-1";
const PAGINATION_ROOT_DISABLED: &str = "grid w-full max-w-2xl gap-2xs text-text-disabled";
const PAGINATION_CONTENT: &str = "flex flex-wrap items-center gap-2xs rounded-box border border-border-subtle bg-surface-1 p-2xs shadow-1";
const PAGINATION_CONTENT_DENSE: &str = "flex flex-wrap items-center gap-3xs rounded-box border border-border-subtle bg-surface-1 p-3xs shadow-1";
const PAGINATION_CONTENT_LOADING: &str = "flex flex-wrap items-center gap-2xs rounded-box border border-info bg-info-soft p-2xs shadow-1";
const PAGINATION_ITEM: &str = "contents";
const PAGINATION_CONTROL: &str = "inline-flex min-h-field min-w-field items-center justify-center rounded-field border border-border-subtle bg-surface-1 px-xs py-2xs text-0 font-6 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled disabled:opacity-disabled";
const PAGINATION_CONTROL_DENSE: &str = "inline-flex min-h-s min-w-s items-center justify-center rounded-field border border-border-subtle bg-surface-1 px-2xs py-3xs text-00 font-6 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:text-text-disabled disabled:opacity-disabled";
const PAGINATION_CONTROL_CURRENT: &str = "inline-flex min-h-field min-w-field items-center justify-center rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 leading-0 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const PAGINATION_CONTROL_DENSE_CURRENT: &str = "inline-flex min-h-s min-w-s items-center justify-center rounded-field border border-brand bg-primary-soft px-2xs py-3xs text-00 font-7 leading-0 text-text-1 shadow-1 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const PAGINATION_CONTROL_INVALID: &str = "inline-flex min-h-field min-w-field items-center justify-center rounded-field border border-danger bg-error-soft px-xs py-2xs text-0 font-7 leading-0 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const PAGINATION_CONTROL_DISABLED: &str = "inline-flex min-h-field min-w-field items-center justify-center rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 leading-0 text-text-disabled opacity-disabled";
const PAGINATION_ERROR: &str = "m-0 text-00 font-6 leading-0 text-danger";
const PAGINATION_ERROR_HIDDEN: &str = "hidden";
const POPOVER_ROOT: &str = "relative grid w-full max-w-md gap-2xs text-text-1";
const POPOVER_ROOT_DISABLED: &str = "relative grid w-full max-w-md gap-2xs text-text-disabled";
const POPOVER_TRIGGER: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const POPOVER_TRIGGER_DENSE: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const POPOVER_TRIGGER_OPEN: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-0 font-7 text-text-1 shadow-2 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const POPOVER_TRIGGER_DENSE_OPEN: &str = "inline-flex min-h-s items-center justify-center gap-2xs rounded-field border border-brand bg-primary-soft px-2xs py-3xs text-00 font-7 text-text-1 shadow-2 transition-colors hover:bg-selected-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const POPOVER_TRIGGER_INVALID: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-danger bg-error-soft px-xs py-2xs text-0 font-7 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const POPOVER_TRIGGER_DISABLED: &str = "inline-flex min-h-field items-center justify-center gap-2xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 text-text-disabled opacity-disabled";
const POPOVER_CONTENT: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-xs rounded-box border border-border-subtle bg-surface-elevated p-s text-text-1 shadow-3";
const POPOVER_CONTENT_DENSE: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-2xs rounded-field border border-border-subtle bg-surface-elevated p-xs text-text-1 shadow-2";
const POPOVER_CONTENT_LOADING: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-xs rounded-box border border-info bg-info-soft p-s text-text-1 shadow-2";
const POPOVER_CONTENT_INVALID: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-2";
const POPOVER_CONTENT_DISABLED: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled shadow-1";
const POPOVER_CONTENT_HIDDEN: &str = "hidden";
const POPOVER_META: &str = "m-0 text-00 font-7 uppercase tracking-label text-brand";
const POPOVER_TITLE: &str = "m-0 text-1 font-7 leading-2 text-text-1";
const POPOVER_TITLE_DENSE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const POPOVER_DETAIL: &str = "m-0 text-0 leading-0 text-text-2";
const POPOVER_DETAIL_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const POPOVER_ARROW: &str = "grid size-s place-items-center rounded-field border border-border-subtle bg-surface-elevated text-00 font-7 text-text-muted shadow-1";
const POPOVER_ARROW_HIDDEN: &str = "hidden";
const POPOVER_ERROR: &str = "m-0 text-00 font-6 leading-0 text-danger";
const POPOVER_ERROR_HIDDEN: &str = "hidden";
const PROGRESS_ROOT: &str = "grid w-full max-w-md gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const PROGRESS_ROOT_DENSE: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const PROGRESS_ROOT_INVALID: &str = "grid w-full max-w-md gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const PROGRESS_ROOT_DISABLED: &str = "grid w-full max-w-md gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const PROGRESS_TRACK: &str = "h-s w-full overflow-hidden rounded-pill bg-surface-3";
const PROGRESS_TRACK_DENSE: &str = "h-xs w-full overflow-hidden rounded-pill bg-surface-3";
const PROGRESS_TRACK_LOADING: &str = "h-s w-full overflow-hidden rounded-pill bg-info-soft";
const PROGRESS_TRACK_INVALID: &str = "h-s w-full overflow-hidden rounded-pill bg-error-soft";
const PROGRESS_TRACK_DISABLED: &str = "h-s w-full overflow-hidden rounded-pill bg-border-muted";
const PROGRESS_INDICATOR: &str = "block h-full rounded-pill bg-brand transition-all";
const PROGRESS_INDICATOR_HIGHLIGHTED: &str =
    "block h-full rounded-pill bg-accent shadow-1 transition-all";
const PROGRESS_INDICATOR_LOADING: &str = "block h-full rounded-pill bg-info transition-all";
const PROGRESS_INDICATOR_INVALID: &str = "block h-full rounded-pill bg-danger transition-all";
const PROGRESS_INDICATOR_DISABLED: &str = "block h-full rounded-pill bg-surface-3";
const PROGRESS_LABEL_ROW: &str = "flex flex-wrap items-start justify-between gap-2xs";
const PROGRESS_LABEL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const PROGRESS_LABEL_DENSE: &str = "m-0 text-00 font-7 leading-0 text-text-1";
const PROGRESS_VALUE: &str = "m-0 text-0 font-6 leading-0 text-brand";
const PROGRESS_VALUE_DENSE: &str = "m-0 text-00 font-6 leading-0 text-brand";
const PROGRESS_DETAIL: &str = "m-0 text-0 leading-0 text-text-2";
const PROGRESS_DETAIL_DENSE: &str = "m-0 text-00 leading-0 text-text-2";
const PROGRESS_DETAIL_INVALID: &str = "m-0 text-00 font-6 leading-0 text-danger";
const PROGRESS_DETAIL_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const RADIO_GROUP_ROOT: &str = "grid w-full max-w-md gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const RADIO_GROUP_ROOT_DENSE: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const RADIO_GROUP_ROOT_INVALID: &str = "grid w-full max-w-md gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const RADIO_GROUP_ROOT_DISABLED: &str = "grid w-full max-w-md gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const RADIO_GROUP_TITLE_ROW: &str = "flex flex-wrap items-center justify-between gap-2xs";
const RADIO_GROUP_TITLE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const RADIO_GROUP_TITLE_DENSE: &str = "m-0 text-00 font-7 leading-0 text-text-1";
const RADIO_GROUP_STATUS: &str = "m-0 text-00 font-6 leading-0 text-text-muted";
const RADIO_GROUP_LIST: &str = "grid gap-2xs";
const RADIO_GROUP_LIST_DENSE: &str = "grid gap-3xs";
const RADIO_GROUP_LIST_HORIZONTAL: &str = "flex flex-wrap gap-2xs";
const RADIO_GROUP_ITEM: &str = "flex min-w-0 items-start gap-xs rounded-field border border-border-subtle bg-surface-1 p-xs text-left transition-colors hover:bg-hover-tint";
const RADIO_GROUP_ITEM_DENSE: &str = "flex min-w-0 items-start gap-2xs rounded-field border border-border-subtle bg-surface-1 p-2xs text-left transition-colors hover:bg-hover-tint";
const RADIO_GROUP_ITEM_SELECTED: &str = "flex min-w-0 items-start gap-xs rounded-field border border-brand bg-primary-soft p-xs text-left shadow-1 transition-colors hover:bg-selected-tint";
const RADIO_GROUP_ITEM_FOCUSED: &str = "flex min-w-0 items-start gap-xs rounded-field border border-brand bg-surface-1 p-xs text-left shadow-1";
const RADIO_GROUP_ITEM_DISABLED: &str = "flex min-w-0 items-start gap-xs rounded-field border border-border-muted bg-surface-2 p-xs text-left text-text-disabled opacity-disabled";
const RADIO_GROUP_CONTROL: &str = "grid size-s shrink-0 place-items-center rounded-pill border border-border-strong bg-surface-1 text-00 font-7 text-text-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const RADIO_GROUP_CONTROL_SELECTED: &str = "grid size-s shrink-0 place-items-center rounded-pill border border-brand bg-brand text-00 font-7 text-text-on-brand shadow-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const RADIO_GROUP_CONTROL_INVALID: &str = "grid size-s shrink-0 place-items-center rounded-pill border border-danger bg-error-soft text-00 font-7 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const RADIO_GROUP_CONTROL_DISABLED: &str = "grid size-s shrink-0 place-items-center rounded-pill border border-border-muted bg-surface-3 text-00 font-6 text-text-disabled opacity-disabled";
const RADIO_GROUP_DOT: &str = "size-2xs rounded-pill bg-text-on-brand";
const RADIO_GROUP_DOT_EMPTY: &str = "hidden";
const RADIO_GROUP_TEXT: &str = "grid min-w-0 gap-3xs";
const RADIO_GROUP_LABEL: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const RADIO_GROUP_LABEL_DENSE: &str = "m-0 text-00 font-7 leading-0 text-text-1";
const RADIO_GROUP_LABEL_DISABLED: &str = "m-0 text-0 font-7 leading-0 text-text-disabled";
const RADIO_GROUP_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const RADIO_GROUP_DETAIL_INVALID: &str = "m-0 text-00 font-6 leading-0 text-danger";
const RADIO_GROUP_DETAIL_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const RESIZABLE_ROOT: &str = "grid w-full max-w-2xl gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const RESIZABLE_ROOT_DENSE: &str = "grid w-full max-w-2xl gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const RESIZABLE_ROOT_INVALID: &str = "grid w-full max-w-2xl gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const RESIZABLE_ROOT_DISABLED: &str = "grid w-full max-w-2xl gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const RESIZABLE_HEADER: &str = "flex flex-wrap items-center justify-between gap-2xs";
const RESIZABLE_TITLE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const RESIZABLE_TITLE_DENSE: &str = "m-0 text-00 font-7 leading-0 text-text-1";
const RESIZABLE_STATUS: &str = "m-0 text-00 font-6 leading-0 text-text-muted";
const RESIZABLE_STATUS_INVALID: &str = "m-0 text-00 font-6 leading-0 text-danger";
const RESIZABLE_STATUS_DISABLED: &str = "m-0 text-00 font-6 leading-0 text-text-disabled";
const RESIZABLE_GROUP_HORIZONTAL: &str = "flex min-h-2xl w-full overflow-hidden rounded-field border border-border-subtle bg-surface-2 text-text-1";
const RESIZABLE_GROUP_VERTICAL: &str = "flex min-h-4xl w-full flex-col overflow-hidden rounded-field border border-border-subtle bg-surface-2 text-text-1";
const RESIZABLE_GROUP_DENSE_HORIZONTAL: &str = "flex min-h-xl w-full overflow-hidden rounded-field border border-border-subtle bg-surface-2 text-text-1";
const RESIZABLE_GROUP_DENSE_VERTICAL: &str = "flex min-h-3xl w-full flex-col overflow-hidden rounded-field border border-border-subtle bg-surface-2 text-text-1";
const RESIZABLE_PANEL: &str = "relative grid min-w-0 content-start gap-2xs border-border-subtle bg-surface-1 p-s text-text-1 outline-none transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const RESIZABLE_PANEL_DENSE: &str = "relative grid min-w-0 content-start gap-3xs border-border-subtle bg-surface-1 p-xs text-text-1 outline-none transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const RESIZABLE_PANEL_ACTIVE: &str = "relative grid min-w-0 content-start gap-2xs border border-brand bg-primary-soft p-s text-text-1 shadow-1 outline-none transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const RESIZABLE_PANEL_INVALID: &str = "relative grid min-w-0 content-start gap-2xs border border-danger bg-error-soft p-s text-text-1 outline-none";
const RESIZABLE_PANEL_DISABLED: &str = "relative grid min-w-0 content-start gap-2xs border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled outline-none";
const RESIZABLE_PANEL_TITLE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const RESIZABLE_PANEL_TITLE_DENSE: &str = "m-0 text-00 font-7 leading-0 text-text-1";
const RESIZABLE_PANEL_TITLE_DISABLED: &str = "m-0 text-0 font-7 leading-0 text-text-disabled";
const RESIZABLE_PANEL_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const RESIZABLE_PANEL_DETAIL_INVALID: &str = "m-0 text-00 font-6 leading-0 text-danger";
const RESIZABLE_PANEL_DETAIL_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const RESIZABLE_PANEL_META: &str = "m-0 text-00 font-6 leading-0 text-brand";
const RESIZABLE_PANEL_META_DISABLED: &str = "m-0 text-00 font-6 leading-0 text-text-disabled";
const RESIZABLE_HANDLE: &str =
    "mt-xs grid gap-3xs rounded-field border border-border-subtle bg-surface-2 p-2xs text-text-2";
const RESIZABLE_HANDLE_DENSE: &str =
    "mt-2xs grid gap-3xs rounded-field border border-border-subtle bg-surface-2 p-3xs text-text-2";
const RESIZABLE_HANDLE_ACTIVE: &str = "mt-xs grid gap-3xs rounded-field border border-brand bg-primary-soft p-2xs text-text-1 shadow-1";
const RESIZABLE_HANDLE_INVALID: &str =
    "mt-xs grid gap-3xs rounded-field border border-danger bg-error-soft p-2xs text-danger";
const RESIZABLE_HANDLE_DISABLED: &str = "mt-xs grid gap-3xs rounded-field border border-border-muted bg-surface-3 p-2xs text-text-disabled opacity-disabled";
const RESIZABLE_HANDLE_LABEL: &str = "m-0 text-00 font-6 leading-0 text-text-muted";
const RESIZABLE_HANDLE_LABEL_ACTIVE: &str = "m-0 text-00 font-7 leading-0 text-brand";
const RESIZABLE_HANDLE_LABEL_DISABLED: &str = "m-0 text-00 font-6 leading-0 text-text-disabled";
const RESIZABLE_RANGE: &str = "w-full accent-brand";
const SCROLL_AREA_ROOT: &str = "grid w-full max-w-md gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const SCROLL_AREA_ROOT_DENSE: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const SCROLL_AREA_ROOT_INVALID: &str = "grid w-full max-w-md gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const SCROLL_AREA_ROOT_DISABLED: &str = "grid w-full max-w-md gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const SCROLL_AREA_HEADER: &str = "flex flex-wrap items-center justify-between gap-2xs";
const SCROLL_AREA_TITLE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const SCROLL_AREA_TITLE_DENSE: &str = "m-0 text-00 font-7 leading-0 text-text-1";
const SCROLL_AREA_STATUS: &str = "m-0 text-00 font-6 leading-0 text-text-muted";
const SCROLL_AREA_STATUS_INVALID: &str = "m-0 text-00 font-6 leading-0 text-danger";
const SCROLL_AREA_STATUS_DISABLED: &str = "m-0 text-00 font-6 leading-0 text-text-disabled";
const SCROLL_AREA_FRAME: &str =
    "grid gap-2xs rounded-field border border-border-subtle bg-surface-2 p-2xs";
const SCROLL_AREA_FRAME_DENSE: &str =
    "grid gap-3xs rounded-field border border-border-subtle bg-surface-2 p-3xs";
const SCROLL_AREA_VIEWPORT: &str = "max-h-2xl overflow-auto rounded-field bg-surface-1 p-xs outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SCROLL_AREA_VIEWPORT_DENSE: &str = "max-h-xl overflow-auto rounded-field bg-surface-1 p-2xs outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SCROLL_AREA_VIEWPORT_FOCUSED: &str = "max-h-2xl overflow-auto rounded-field border border-brand bg-surface-1 p-xs shadow-1 outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SCROLL_AREA_VIEWPORT_INVALID: &str =
    "max-h-2xl overflow-auto rounded-field border border-danger bg-error-soft p-xs outline-none";
const SCROLL_AREA_VIEWPORT_DISABLED: &str = "max-h-2xl overflow-auto rounded-field border border-border-muted bg-surface-2 p-xs text-text-disabled opacity-disabled outline-none";
const SCROLL_AREA_CONTENT: &str = "grid min-w-max gap-2xs";
const SCROLL_AREA_CONTENT_DENSE: &str = "grid min-w-max gap-3xs";
const SCROLL_AREA_ITEM: &str = "grid min-w-0 gap-3xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 transition-colors hover:bg-hover-tint";
const SCROLL_AREA_ITEM_DENSE: &str = "grid min-w-0 gap-3xs rounded-field border border-border-subtle bg-surface-1 p-2xs text-text-1 transition-colors hover:bg-hover-tint";
const SCROLL_AREA_ITEM_ACTIVE: &str = "grid min-w-0 gap-3xs rounded-field border border-brand bg-primary-soft p-xs text-text-1 shadow-1";
const SCROLL_AREA_ITEM_INVALID: &str =
    "grid min-w-0 gap-3xs rounded-field border border-danger bg-error-soft p-xs text-text-1";
const SCROLL_AREA_ITEM_DISABLED: &str = "grid min-w-0 gap-3xs rounded-field border border-border-muted bg-surface-2 p-xs text-text-disabled opacity-disabled";
const SCROLL_AREA_ITEM_TITLE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const SCROLL_AREA_ITEM_TITLE_DENSE: &str = "m-0 text-00 font-7 leading-0 text-text-1";
const SCROLL_AREA_ITEM_TITLE_DISABLED: &str = "m-0 text-0 font-7 leading-0 text-text-disabled";
const SCROLL_AREA_ITEM_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const SCROLL_AREA_ITEM_DETAIL_INVALID: &str = "m-0 text-00 font-6 leading-0 text-danger";
const SCROLL_AREA_ITEM_DETAIL_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const SCROLL_AREA_BAR_ROW: &str = "flex flex-wrap items-center gap-2xs";
const SCROLL_AREA_BAR: &str = "rounded-pill bg-border-subtle";
const SCROLL_AREA_BAR_ACTIVE: &str = "rounded-pill bg-brand shadow-1";
const SCROLL_AREA_BAR_INVALID: &str = "rounded-pill bg-danger";
const SCROLL_AREA_BAR_DISABLED: &str = "rounded-pill bg-border-muted opacity-disabled";
const SCROLL_AREA_BAR_VERTICAL: &str = "h-m w-2xs";
const SCROLL_AREA_BAR_HORIZONTAL: &str = "h-2xs w-m";
const SCROLL_AREA_CORNER: &str = "size-2xs rounded-field bg-border-subtle";
const SCROLL_AREA_CORNER_ACTIVE: &str = "size-2xs rounded-field bg-brand";
const SCROLL_AREA_CORNER_HIDDEN: &str = "hidden";
const SELECT_ROOT: &str = "relative grid w-full max-w-md gap-2xs text-text-1";
const SELECT_ROOT_DISABLED: &str = "relative grid w-full max-w-md gap-2xs text-text-disabled";
const SELECT_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const SELECT_LABEL_DISABLED: &str =
    "m-0 text-00 font-7 uppercase tracking-label text-text-disabled";
const SELECT_TRIGGER: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-strong bg-surface-1 px-xs py-2xs text-left text-0 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SELECT_TRIGGER_DENSE: &str = "flex min-h-s w-full items-center justify-between gap-2xs rounded-field border border-border-strong bg-surface-1 px-2xs py-3xs text-left text-00 font-6 text-text-1 shadow-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const SELECT_TRIGGER_OPEN: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-brand bg-primary-soft px-xs py-2xs text-left text-0 font-7 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SELECT_TRIGGER_INVALID: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-danger bg-error-soft px-xs py-2xs text-left text-0 font-7 text-text-1 shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SELECT_TRIGGER_DISABLED: &str = "flex min-h-field w-full items-center justify-between gap-xs rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-left text-0 font-6 text-text-disabled opacity-disabled";
const SELECT_VALUE: &str = "truncate";
const SELECT_CHEVRON: &str = "text-text-muted";
const SELECT_CONTENT: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-2xs rounded-box border border-border-subtle bg-surface-elevated p-2xs text-text-1 shadow-3";
const SELECT_CONTENT_DENSE: &str = "absolute left-0 top-full z-10 mt-2xs grid w-full gap-3xs rounded-field border border-border-subtle bg-surface-elevated p-3xs text-text-1 shadow-2";
const SELECT_CONTENT_HIDDEN: &str = "hidden";
const SELECT_GROUP: &str =
    "grid gap-3xs border-t border-border-subtle pt-2xs first:border-t-0 first:pt-0";
const SELECT_GROUP_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const SELECT_GROUP_LABEL_DISABLED: &str =
    "m-0 text-00 font-7 uppercase tracking-label text-text-disabled";
const SELECT_ITEM: &str = "grid gap-3xs rounded-field border border-border-subtle bg-surface-1 p-xs text-left transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SELECT_ITEM_DENSE: &str = "grid gap-3xs rounded-field border border-border-subtle bg-surface-1 p-2xs text-left transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SELECT_ITEM_SELECTED: &str = "grid gap-3xs rounded-field border border-brand bg-primary-soft p-xs text-left shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SELECT_ITEM_FOCUSED: &str = "grid gap-3xs rounded-field border border-brand bg-surface-1 p-xs text-left shadow-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SELECT_ITEM_DISABLED: &str = "grid gap-3xs rounded-field border border-border-muted bg-surface-2 p-xs text-left text-text-disabled opacity-disabled";
const SELECT_ITEM_TITLE: &str = "m-0 text-0 font-7 leading-0 text-text-1";
const SELECT_ITEM_TITLE_DENSE: &str = "m-0 text-00 font-7 leading-0 text-text-1";
const SELECT_ITEM_TITLE_DISABLED: &str = "m-0 text-0 font-7 leading-0 text-text-disabled";
const SELECT_ITEM_DETAIL: &str = "m-0 text-00 leading-0 text-text-2";
const SELECT_ITEM_DETAIL_DISABLED: &str = "m-0 text-00 leading-0 text-text-disabled";
const SELECT_ERROR: &str = "m-0 text-00 font-6 leading-0 text-danger";
const SEPARATOR_ROOT: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1 outline-none transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SEPARATOR_ROOT_DENSE: &str = "grid w-full max-w-md gap-3xs rounded-field border border-border-subtle bg-surface-1 p-2xs text-text-1 shadow-1 outline-none transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SEPARATOR_ROOT_VERTICAL: &str = "flex min-h-xl w-fit items-center gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1 outline-none transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SEPARATOR_ROOT_ACTIVE: &str = "grid w-full max-w-md gap-2xs rounded-field border border-brand bg-primary-soft p-xs text-text-1 shadow-1 outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SEPARATOR_ROOT_VERTICAL_ACTIVE: &str = "flex min-h-xl w-fit items-center gap-2xs rounded-field border border-brand bg-primary-soft p-xs text-text-1 shadow-1 outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SEPARATOR_ROOT_INVALID: &str = "grid w-full max-w-md gap-2xs rounded-field border border-danger bg-error-soft p-xs text-text-1 shadow-1 outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const SEPARATOR_ROOT_DISABLED: &str = "grid w-full max-w-md gap-2xs rounded-field border border-border-muted bg-surface-2 p-xs text-text-disabled opacity-disabled";
const SEPARATOR_LINE: &str = "h-3xs w-full rounded-pill bg-border-subtle";
const SEPARATOR_LINE_DENSE: &str = "h-2xs w-full rounded-pill bg-border-subtle";
const SEPARATOR_LINE_VERTICAL: &str = "h-xl w-2xs rounded-pill bg-border-subtle";
const SEPARATOR_LINE_ACTIVE: &str = "h-3xs w-full rounded-pill bg-brand shadow-1";
const SEPARATOR_LINE_VERTICAL_ACTIVE: &str = "h-xl w-2xs rounded-pill bg-brand shadow-1";
const SEPARATOR_LINE_INVALID: &str = "h-3xs w-full rounded-pill bg-danger";
const SEPARATOR_LINE_DISABLED: &str = "h-3xs w-full rounded-pill bg-border-muted opacity-disabled";
const SEPARATOR_LABEL: &str = "m-0 text-00 font-7 uppercase tracking-label text-text-muted";
const SEPARATOR_LABEL_DENSE: &str = "m-0 text-00 font-6 uppercase tracking-label text-text-muted";
const SEPARATOR_LABEL_INVALID: &str = "m-0 text-00 font-7 uppercase tracking-label text-danger";
const SEPARATOR_LABEL_DISABLED: &str =
    "m-0 text-00 font-7 uppercase tracking-label text-text-disabled";
const SEPARATOR_LABEL_HIDDEN: &str = "hidden";
const SEPARATOR_ERROR: &str = "m-0 text-00 font-6 leading-0 text-danger";
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
const MARKER_ROOT: &str = "inline-flex max-w-full items-center gap-2xs rounded-pill border border-border-subtle bg-surface-1 px-2xs py-3xs text-text-1 shadow-1 transition-colors";
const MARKER_ROOT_DENSE: &str = "inline-flex max-w-full items-center gap-3xs rounded-pill border border-border-subtle bg-surface-1 px-3xs py-3xs text-text-1 shadow-1 transition-colors";
const MARKER_ROOT_ACTIVE: &str = "inline-flex max-w-full items-center gap-2xs rounded-pill border border-brand bg-selected-tint px-2xs py-3xs text-text-1 shadow-1 transition-colors";
const MARKER_ROOT_INVALID: &str = "inline-flex max-w-full items-center gap-2xs rounded-pill border border-danger bg-error-soft px-2xs py-3xs text-danger shadow-1 transition-colors";
const MARKER_ROOT_LOADING: &str = "inline-flex max-w-full items-center gap-2xs rounded-pill border border-info bg-info-soft px-2xs py-3xs text-info shadow-1 transition-colors";
const MARKER_ROOT_DISABLED: &str = "inline-flex max-w-full items-center gap-2xs rounded-pill border border-border-muted bg-surface-2 px-2xs py-3xs text-text-disabled opacity-disabled";
const MARKER_DOT: &str = "size-2xs shrink-0 rounded-pill bg-brand";
const MARKER_DOT_DENSE: &str = "size-3xs shrink-0 rounded-pill bg-brand";
const MARKER_DOT_NEUTRAL: &str = "size-2xs shrink-0 rounded-pill bg-text-muted";
const MARKER_DOT_INFO: &str = "size-2xs shrink-0 rounded-pill bg-info";
const MARKER_DOT_SUCCESS: &str = "size-2xs shrink-0 rounded-pill bg-success";
const MARKER_DOT_WARNING: &str = "size-2xs shrink-0 rounded-pill bg-warning";
const MARKER_DOT_DANGER: &str = "size-2xs shrink-0 rounded-pill bg-danger";
const MARKER_DOT_INVALID: &str = "size-2xs shrink-0 rounded-pill bg-danger";
const MARKER_DOT_LOADING: &str = "size-2xs shrink-0 rounded-pill bg-info";
const MARKER_DOT_DISABLED: &str = "size-2xs shrink-0 rounded-pill bg-border-muted";
const MARKER_LABEL: &str = "m-0 truncate text-00 font-7 leading-0 text-inherit";
const MARKER_LABEL_DENSE: &str = "m-0 truncate text-00 font-6 leading-0 text-inherit";
const MARKER_LABEL_DISABLED: &str = "m-0 truncate text-00 font-6 leading-0 text-text-disabled";
const MARKER_ANCHOR: &str = "inline-flex min-h-s items-center rounded-field px-2xs py-3xs text-00 font-7 leading-0 text-brand transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const MARKER_ANCHOR_ACTIVE: &str = "inline-flex min-h-s items-center rounded-field bg-selected-tint px-2xs py-3xs text-00 font-7 leading-0 text-brand transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring";
const MARKER_ANCHOR_DISABLED: &str = "inline-flex min-h-s items-center rounded-field px-2xs py-3xs text-00 font-6 leading-0 text-text-disabled opacity-disabled";
const MARKER_ANCHOR_HIDDEN: &str = "hidden";
const MARKER_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const MENUBAR_ROOT: &str = "grid max-w-xl gap-2xs rounded-box border border-border-subtle bg-surface-1 p-2xs text-text-1 shadow-1";
const MENUBAR_ROOT_DENSE: &str = "grid max-w-xl gap-3xs rounded-field border border-border-subtle bg-surface-1 p-3xs text-text-1 shadow-1";
const MENUBAR_ROOT_INVALID: &str = "grid max-w-xl gap-2xs rounded-box border border-danger bg-error-soft p-2xs text-text-1 shadow-1";
const MENUBAR_ROOT_LOADING: &str =
    "grid max-w-xl gap-2xs rounded-box border border-info bg-info-soft p-2xs text-text-1 shadow-1";
const MENUBAR_ROOT_DISABLED: &str = "grid max-w-xl gap-2xs rounded-box border border-border-muted bg-surface-2 p-2xs text-text-disabled opacity-disabled";
const MENUBAR_ROW: &str = "flex flex-wrap items-center gap-2xs";
const MENUBAR_MENU: &str = "relative grid gap-2xs";
const MENUBAR_MENU_DENSE: &str = "relative grid gap-3xs";
const MENUBAR_TRIGGER: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-border-strong bg-surface-2 px-xs py-2xs text-0 font-7 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MENUBAR_TRIGGER_DENSE: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-7 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MENUBAR_TRIGGER_OPEN: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-brand bg-selected-tint px-xs py-2xs text-0 font-7 leading-0 text-text-1 shadow-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MENUBAR_TRIGGER_DISABLED: &str = "inline-flex min-h-field items-center justify-center rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 leading-0 text-text-disabled opacity-disabled";
const MENUBAR_CONTENT: &str = "grid min-w-xl gap-2xs rounded-box border border-border-subtle bg-surface-elevated p-2xs shadow-2";
const MENUBAR_CONTENT_DENSE: &str = "grid min-w-l gap-3xs rounded-field border border-border-subtle bg-surface-elevated p-3xs shadow-1";
const MENUBAR_CONTENT_HIDDEN: &str = "hidden";
const MENUBAR_ITEM: &str = "flex min-h-field w-full items-center justify-between gap-s rounded-field px-xs py-2xs text-left text-0 font-6 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MENUBAR_ITEM_DENSE: &str = "flex min-h-s w-full items-center justify-between gap-xs rounded-field px-2xs py-3xs text-left text-00 font-6 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MENUBAR_ITEM_ACTIVE: &str = "flex min-h-field w-full items-center justify-between gap-s rounded-field bg-selected-tint px-xs py-2xs text-left text-0 font-7 leading-0 text-text-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MENUBAR_ITEM_DISABLED: &str = "flex min-h-field w-full items-center justify-between gap-s rounded-field px-xs py-2xs text-left text-0 font-6 leading-0 text-text-disabled opacity-disabled";
const MENUBAR_SHORTCUT: &str = "ml-s text-00 font-6 leading-0 text-text-muted";
const MENUBAR_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const MESSAGE_ROOT_INCOMING: &str = "grid w-full max-w-2xl gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1 transition-colors";
const MESSAGE_ROOT_OUTGOING: &str = "ml-auto grid w-full max-w-2xl gap-xs rounded-box border border-brand bg-primary-soft p-s text-text-1 shadow-1 transition-colors";
const MESSAGE_ROOT_SYSTEM: &str = "mx-auto grid w-full max-w-xl gap-xs rounded-box border border-border-subtle bg-surface-2 p-s text-text-1 transition-colors";
const MESSAGE_ROOT_DENSE_INCOMING: &str = "grid w-full max-w-2xl gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1 transition-colors";
const MESSAGE_ROOT_DENSE_OUTGOING: &str = "ml-auto grid w-full max-w-2xl gap-2xs rounded-field border border-brand bg-primary-soft p-xs text-text-1 shadow-1 transition-colors";
const MESSAGE_ROOT_DENSE_SYSTEM: &str = "mx-auto grid w-full max-w-xl gap-2xs rounded-field border border-border-subtle bg-surface-2 p-xs text-text-1 transition-colors";
const MESSAGE_ROOT_ACTIVE: &str = "grid w-full max-w-2xl gap-xs rounded-box border border-brand bg-selected-tint p-s text-text-1 shadow-1 transition-colors";
const MESSAGE_ROOT_INVALID: &str = "grid w-full max-w-2xl gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1 transition-colors";
const MESSAGE_ROOT_LOADING: &str =
    "grid w-full max-w-2xl gap-xs rounded-box border border-info bg-info-soft p-s text-text-1";
const MESSAGE_ROOT_DISABLED: &str = "grid w-full max-w-2xl gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const MESSAGE_HEADER: &str = "flex min-w-0 flex-wrap items-start justify-between gap-xs";
const MESSAGE_HEADER_DENSE: &str = "flex min-w-0 flex-wrap items-start justify-between gap-2xs";
const MESSAGE_SENDER: &str = "m-0 min-w-0 truncate text-0 font-7 leading-0 text-text-1";
const MESSAGE_SENDER_DISABLED: &str =
    "m-0 min-w-0 truncate text-0 font-6 leading-0 text-text-disabled";
const MESSAGE_META: &str = "m-0 shrink-0 text-00 font-6 leading-0 text-text-muted";
const MESSAGE_META_DISABLED: &str = "m-0 shrink-0 text-00 font-6 leading-0 text-text-disabled";
const MESSAGE_CONTENT: &str = "m-0 whitespace-pre-wrap text-0 leading-0 text-text-1";
const MESSAGE_CONTENT_DENSE: &str = "m-0 whitespace-pre-wrap text-00 leading-0 text-text-1";
const MESSAGE_CONTENT_DISABLED: &str =
    "m-0 whitespace-pre-wrap text-0 leading-0 text-text-disabled";
const MESSAGE_FOOTER: &str = "flex min-w-0 flex-wrap items-center justify-between gap-xs border-t border-border-subtle pt-xs";
const MESSAGE_FOOTER_DENSE: &str = "flex min-w-0 flex-wrap items-center justify-between gap-2xs border-t border-border-subtle pt-2xs";
const MESSAGE_STATUS: &str = "m-0 text-00 font-6 leading-0 text-text-muted";
const MESSAGE_STATUS_INVALID: &str = "m-0 text-00 font-7 leading-0 text-danger";
const MESSAGE_STATUS_DISABLED: &str = "m-0 text-00 font-6 leading-0 text-text-disabled";
const MESSAGE_ACTIONS: &str = "flex flex-wrap items-center gap-2xs";
const MESSAGE_ACTIONS_HIDDEN: &str = "hidden";
const MESSAGE_ACTION: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-strong bg-surface-2 px-2xs py-3xs text-00 font-7 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MESSAGE_ACTION_DENSE: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-strong bg-surface-2 px-3xs py-3xs text-00 font-7 leading-0 text-text-1 transition-colors hover:bg-hover-tint focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MESSAGE_ACTION_ACTIVE: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-brand bg-selected-tint px-2xs py-3xs text-00 font-7 leading-0 text-text-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MESSAGE_ACTION_DISABLED: &str = "inline-flex min-h-s items-center justify-center rounded-field border border-border-muted bg-surface-2 px-2xs py-3xs text-00 font-6 leading-0 text-text-disabled opacity-disabled";
const MESSAGE_ERROR: &str =
    "rounded-field border border-danger bg-error-soft p-s text-0 leading-0 text-text-1";
const MESSAGE_SCROLLER_ROOT: &str = "grid max-w-3xl gap-xs rounded-box border border-border-subtle bg-surface-1 p-s text-text-1 shadow-1";
const MESSAGE_SCROLLER_ROOT_DENSE: &str = "grid max-w-3xl gap-2xs rounded-field border border-border-subtle bg-surface-1 p-xs text-text-1 shadow-1";
const MESSAGE_SCROLLER_ROOT_INVALID: &str =
    "grid max-w-3xl gap-xs rounded-box border border-danger bg-error-soft p-s text-text-1 shadow-1";
const MESSAGE_SCROLLER_ROOT_LOADING: &str =
    "grid max-w-3xl gap-xs rounded-box border border-info bg-info-soft p-s text-text-1";
const MESSAGE_SCROLLER_ROOT_DISABLED: &str = "grid max-w-3xl gap-xs rounded-box border border-border-muted bg-surface-2 p-s text-text-disabled opacity-disabled";
const MESSAGE_SCROLLER_VIEWPORT: &str = "max-h-4xl overflow-y-auto rounded-field border border-border-subtle bg-surface-2 p-xs scroll-smooth";
const MESSAGE_SCROLLER_VIEWPORT_DENSE: &str =
    "max-h-3xl overflow-y-auto rounded-field border border-border-subtle bg-surface-2 p-2xs";
const MESSAGE_SCROLLER_LIST: &str = "grid gap-xs";
const MESSAGE_SCROLLER_LIST_DENSE: &str = "grid gap-2xs";
const MESSAGE_SCROLLER_EMPTY: &str =
    "rounded-field border border-border-subtle bg-surface-1 p-s text-0 leading-0 text-text-muted";
const MESSAGE_SCROLLER_ANCHOR: &str = "h-selector rounded-pill bg-border-subtle";
const MESSAGE_SCROLLER_ANCHOR_ACTIVE: &str = "h-selector rounded-pill bg-brand";
const MESSAGE_SCROLLER_JUMP: &str = "justify-self-end inline-flex min-h-field items-center justify-center rounded-field border border-brand bg-brand px-xs py-2xs text-0 font-7 leading-0 text-text-on-brand shadow-1 transition-colors hover:bg-selected-tint hover:text-text-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MESSAGE_SCROLLER_JUMP_DENSE: &str = "justify-self-end inline-flex min-h-s items-center justify-center rounded-field border border-brand bg-brand px-2xs py-3xs text-00 font-7 leading-0 text-text-on-brand shadow-1 transition-colors hover:bg-selected-tint hover:text-text-1 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MESSAGE_SCROLLER_JUMP_ACTIVE: &str = "justify-self-end inline-flex min-h-field items-center justify-center rounded-field border border-brand bg-selected-tint px-xs py-2xs text-0 font-7 leading-0 text-text-1 shadow-1 transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring disabled:opacity-disabled";
const MESSAGE_SCROLLER_JUMP_DISABLED: &str = "justify-self-end inline-flex min-h-field items-center justify-center rounded-field border border-border-muted bg-surface-2 px-xs py-2xs text-0 font-6 leading-0 text-text-disabled opacity-disabled";
const MESSAGE_SCROLLER_JUMP_HIDDEN: &str = "hidden";
const MESSAGE_SCROLLER_ERROR: &str =
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
pub fn Table(#[prop(optional, default = default_table_model())] model: TableModel) -> AnyView {
    if let Err(report) = validate_table_model(&model) {
        let message = format!("Table validation failed: {report}");
        return view! {
            <div class=TABLE_ERROR data-ui-component="table" data-ui-state="invalid" role="alert">
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
    let input_nodes = table_render_nodes(&model, &model.state());
    let root = input_nodes
        .iter()
        .find(|node| node.part == TablePart::Root)
        .expect("invariant: table render nodes include root")
        .clone();
    let caption = input_nodes
        .iter()
        .find(|node| node.part == TablePart::Caption)
        .expect("invariant: table render nodes include caption")
        .clone();
    let header_model = model.clone();
    let body_model = model.clone();
    let caption_detail = caption.detail.clone();
    let caption_error = caption.detail.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=table_root_class(density, disabled, invalid)
            data-ui-component="table"
            data-ui-part=TablePart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| table_state_label(loading, disabled, invalid, state.selected_row().is_some()).to_owned())
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
        >
            <div class=TABLE_FRAME>
                <table class=TABLE_TABLE role="table">
                    <caption
                        class=table_caption_class(invalid)
                        data-ui-part=TablePart::Caption.label()
                    >
                        {caption_detail}
                    </caption>
                    <thead class=TABLE_HEADER data-ui-part=TablePart::Header.label()>
                        <tr>
                            {move || {
                                state.with(|state| {
                                    table_render_nodes(&header_model, state)
                                        .into_iter()
                                        .filter(|node| node.part == TablePart::Head)
                                        .map(table_head_view)
                                        .collect_view()
                                })
                            }}
                        </tr>
                    </thead>
                    <tbody class=TABLE_BODY data-ui-part=TablePart::Body.label()>
                        {move || {
                            state.with(|state| {
                                let nodes = table_render_nodes(&body_model, state);
                                let row_nodes = nodes
                                    .iter()
                                    .filter(|node| node.part == TablePart::Row)
                                    .cloned()
                                    .collect::<Vec<_>>();
                                let cell_nodes = nodes
                                    .iter()
                                    .filter(|node| node.part == TablePart::Cell)
                                    .cloned()
                                    .collect::<Vec<_>>();
                                row_nodes
                                    .into_iter()
                                    .map(|row| {
                                        let cells = cell_nodes
                                            .iter()
                                            .filter(|cell| cell.row_value == row.value)
                                            .cloned()
                                            .collect::<Vec<_>>();
                                        table_row_view(row, cells, blocked, set_state)
                                    })
                                    .collect_view()
                            })
                        }}
                    </tbody>
                </table>
            </div>
            {invalid.then_some(view! { <p class=TABLE_ERROR>{caption_error}</p> })}
        </section>
    }
    .into_any()
}

fn table_head_view(node: crate::TableRenderNode) -> AnyView {
    view! {
        <th
            scope="col"
            class=table_head_class(node.density, node.numeric)
            data-ui-part=TablePart::Head.label()
            data-ui-column=node.column_value
        >
            {node.label}
        </th>
    }
    .into_any()
}

fn table_row_view(
    row: crate::TableRenderNode,
    cells: Vec<crate::TableRenderNode>,
    blocked: bool,
    set_state: WriteSignal<TableState>,
) -> AnyView {
    let row_value = row.value.clone();
    let row_value_for_click = row.value.clone();
    let disabled = row.disabled;
    let density = row.density;
    view! {
        <tr
            class=table_row_class(row.selected, disabled)
            data-ui-part=TablePart::Row.label()
            data-ui-row=row_value
            aria-selected=row.selected.to_string()
            aria-disabled=disabled.to_string()
            tabindex=if disabled { "-1" } else { "0" }
            on:focus=move |_| {
                if !blocked && !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(TableIntent::Focus(TablePart::Row));
                    });
                }
            }
            on:blur=move |_| {
                if !blocked && !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(TableIntent::Blur);
                    });
                }
            }
            on:click=move |_| {
                if !blocked && !disabled {
                    let value = row_value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(TableIntent::SelectRow(value));
                    });
                }
            }
        >
            {cells
                .into_iter()
                .map(|cell| {
                    view! {
                        <td
                            class=table_cell_class(density, cell.numeric)
                            data-ui-part=TablePart::Cell.label()
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

const fn table_root_class(density: TableDensity, disabled: bool, invalid: bool) -> &'static str {
    if disabled {
        return TABLE_ROOT_DISABLED;
    }
    if invalid {
        return TABLE_ROOT_INVALID;
    }
    match density {
        TableDensity::Standard => TABLE_ROOT,
        TableDensity::Dense => TABLE_ROOT_DENSE,
    }
}

const fn table_caption_class(invalid: bool) -> &'static str {
    if invalid {
        TABLE_CAPTION_INVALID
    } else {
        TABLE_CAPTION
    }
}

const fn table_head_class(density: TableDensity, numeric: bool) -> &'static str {
    if numeric {
        return TABLE_HEAD_NUMERIC;
    }
    match density {
        TableDensity::Standard => TABLE_HEAD,
        TableDensity::Dense => TABLE_HEAD_DENSE,
    }
}

const fn table_row_class(selected: bool, disabled: bool) -> &'static str {
    if disabled {
        TABLE_ROW_DISABLED
    } else if selected {
        TABLE_ROW_SELECTED
    } else {
        TABLE_ROW
    }
}

const fn table_cell_class(density: TableDensity, numeric: bool) -> &'static str {
    if numeric {
        return TABLE_CELL_NUMERIC;
    }
    match density {
        TableDensity::Standard => TABLE_CELL,
        TableDensity::Dense => TABLE_CELL_DENSE,
    }
}

const fn table_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    selected: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if selected {
        "selected"
    } else {
        "ready"
    }
}

#[component]
pub fn Tabs(#[prop(optional, default = default_tabs_model())] model: TabsModel) -> AnyView {
    if let Err(report) = validate_tabs_model(&model) {
        let message = format!("Tabs validation failed: {report}");
        return view! {
            <div class=TABS_ERROR data-ui-component="tabs" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let orientation = model.orientation;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let blocked = loading || disabled;
    let input_nodes = tabs_render_nodes(&model, &model.state());
    let root = input_nodes
        .iter()
        .find(|node| node.part == TabsPart::Root)
        .expect("invariant: tabs render nodes include root")
        .clone();
    let status_model = model.clone();
    let trigger_model = model.clone();
    let content_model = model.clone();
    let error_detail = model.error.clone().unwrap_or_default();
    let root_label = root.label.clone();
    let root_value = root.value.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=tabs_root_class(density, orientation, disabled, invalid)
            data-ui-component="tabs"
            data-ui-part=TabsPart::Root.label()
            data-ui-density=density.label()
            data-ui-orientation=orientation.label()
            data-ui-state=move || {
                state.with(|state| tabs_state_label(loading, disabled, invalid, state.selected_value()).to_owned())
            }
            data-ui-value=root_value
            aria-label=root_label
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
        >
            <p class=TABS_STATUS>
                {move || {
                    state.with(|state| {
                        tabs_render_nodes(&status_model, state)
                            .into_iter()
                            .find(|node| node.part == TabsPart::Root)
                            .map(|node| node.detail)
                            .unwrap_or_else(|| "Selected tab: none".to_owned())
                    })
                }}
            </p>
            <div
                class=tabs_list_class(density, orientation)
                role="tablist"
                data-ui-part=TabsPart::List.label()
                aria-orientation=orientation.label()
            >
                {move || {
                    state.with(|state| {
                        tabs_render_nodes(&trigger_model, state)
                            .into_iter()
                            .filter(|node| node.part == TabsPart::Trigger)
                            .map(|node| tabs_trigger_view(node, blocked, set_state))
                            .collect_view()
                    })
                }}
            </div>
            <div class=TABS_CONTENT_COPY>
                {move || {
                    state.with(|state| {
                        tabs_render_nodes(&content_model, state)
                            .into_iter()
                            .filter(|node| node.part == TabsPart::Content)
                            .map(tabs_content_view)
                            .collect_view()
                    })
                }}
            </div>
            {invalid.then_some(view! { <p class=TABS_ERROR>{error_detail}</p> })}
        </section>
    }
    .into_any()
}

fn tabs_trigger_view(
    node: crate::TabsRenderNode,
    blocked: bool,
    set_state: WriteSignal<TabsState>,
) -> AnyView {
    let value = node.value.clone();
    let value_for_focus = node.value.clone();
    let value_for_click = node.value.clone();
    let disabled = blocked || node.disabled;
    let trigger_id = tabs_dom_id("tabs-trigger", value.as_str());
    let panel_id = tabs_dom_id("tabs-panel", value.as_str());
    let tab_index = if node.selected && !disabled {
        "0"
    } else {
        "-1"
    };

    view! {
        <button
            id=trigger_id
            type="button"
            role="tab"
            class=tabs_trigger_class(node.density, node.selected, node.focused, disabled, node.invalid)
            data-ui-part=TabsPart::Trigger.label()
            data-ui-value=value
            aria-controls=panel_id
            aria-selected=node.selected.to_string()
            aria-disabled=disabled.to_string()
            tabindex=tab_index
            disabled=disabled
            on:focus=move |_| {
                if !blocked && !disabled {
                    let value = value_for_focus.clone();
                    set_state.update(|state| {
                        let _ = state.apply(TabsIntent::Focus(value));
                    });
                }
            }
            on:blur=move |_| {
                if !blocked && !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(TabsIntent::Blur);
                    });
                }
            }
            on:click=move |_| {
                if !blocked && !disabled {
                    let value = value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(TabsIntent::Select(value));
                    });
                }
            }
        >
            {node.label}
        </button>
    }
    .into_any()
}

fn tabs_content_view(node: crate::TabsRenderNode) -> AnyView {
    let value = node.value.clone();
    let trigger_id = tabs_dom_id("tabs-trigger", value.as_str());
    let panel_id = tabs_dom_id("tabs-panel", value.as_str());
    let tab_index = if node.visible && !node.disabled {
        "0"
    } else {
        "-1"
    };

    view! {
        <section
            id=panel_id
            role="tabpanel"
            class=tabs_content_class(node.density, node.disabled, node.invalid)
            data-ui-part=TabsPart::Content.label()
            data-ui-value=value
            aria-labelledby=trigger_id
            tabindex=tab_index
            hidden=!node.visible
        >
            <p class=TABS_CONTENT_COPY>{node.detail}</p>
        </section>
    }
    .into_any()
}

const fn tabs_root_class(
    density: TabsDensity,
    orientation: TabsOrientation,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        return TABS_ROOT_DISABLED;
    }
    if invalid {
        return TABS_ROOT_INVALID;
    }
    match (density, orientation) {
        (TabsDensity::Standard, TabsOrientation::Horizontal) => TABS_ROOT,
        (TabsDensity::Dense, TabsOrientation::Horizontal) => TABS_ROOT_DENSE,
        (TabsDensity::Standard, TabsOrientation::Vertical) => TABS_ROOT_VERTICAL,
        (TabsDensity::Dense, TabsOrientation::Vertical) => TABS_ROOT_DENSE_VERTICAL,
    }
}

const fn tabs_list_class(density: TabsDensity, orientation: TabsOrientation) -> &'static str {
    match (density, orientation) {
        (TabsDensity::Standard, TabsOrientation::Horizontal) => TABS_LIST,
        (TabsDensity::Dense, TabsOrientation::Horizontal) => TABS_LIST_DENSE,
        (TabsDensity::Standard, TabsOrientation::Vertical) => TABS_LIST_VERTICAL,
        (TabsDensity::Dense, TabsOrientation::Vertical) => TABS_LIST_DENSE_VERTICAL,
    }
}

const fn tabs_trigger_class(
    density: TabsDensity,
    selected: bool,
    focused: bool,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        return TABS_TRIGGER_DISABLED;
    }
    if invalid {
        return TABS_TRIGGER_INVALID;
    }
    if focused {
        return TABS_TRIGGER_FOCUSED;
    }
    match (density, selected) {
        (TabsDensity::Standard, true) => TABS_TRIGGER_SELECTED,
        (TabsDensity::Dense, true) => TABS_TRIGGER_DENSE_SELECTED,
        (TabsDensity::Standard, false) => TABS_TRIGGER,
        (TabsDensity::Dense, false) => TABS_TRIGGER_DENSE,
    }
}

const fn tabs_content_class(density: TabsDensity, disabled: bool, invalid: bool) -> &'static str {
    if disabled {
        return TABS_CONTENT_DISABLED;
    }
    if invalid {
        return TABS_CONTENT_INVALID;
    }
    match density {
        TabsDensity::Standard => TABS_CONTENT,
        TabsDensity::Dense => TABS_CONTENT_DENSE,
    }
}

fn tabs_state_label(loading: bool, disabled: bool, invalid: bool, selected_value: &str) -> String {
    if disabled {
        "disabled".to_owned()
    } else if loading {
        "loading".to_owned()
    } else if invalid {
        "invalid".to_owned()
    } else {
        format!("selected-{selected_value}")
    }
}

#[component]
pub fn Textarea(
    #[prop(optional, default = default_textarea_model())] model: TextareaModel,
) -> AnyView {
    if let Err(report) = validate_textarea_model(&model) {
        let message = format!("Textarea validation failed: {report}");
        return view! {
            <div class=TEXTAREA_ERROR data-ui-component="textarea" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let required = model.required;
    let rows = model.rows;
    let max_length = model.max_length;
    let blocked = loading || disabled;
    let input_nodes = textarea_render_nodes(&model, &model.state());
    let root = input_nodes
        .iter()
        .find(|node| node.part == TextareaPart::Root)
        .expect("invariant: textarea render nodes include root")
        .clone();
    let control = input_nodes
        .iter()
        .find(|node| node.part == TextareaPart::Control)
        .expect("invariant: textarea render nodes include control")
        .clone();
    let hint = input_nodes
        .iter()
        .find(|node| node.part == TextareaPart::Hint)
        .expect("invariant: textarea render nodes include hint")
        .clone();
    let counter_model = model.clone();
    let control_id = textarea_dom_id("textarea-control", root.value.as_str());
    let hint_id = textarea_dom_id("textarea-hint", root.value.as_str());
    let counter_id = textarea_dom_id("textarea-counter", root.value.as_str());
    let described_by = format!("{hint_id} {counter_id}");
    let placeholder = if loading {
        "Loading draft".to_owned()
    } else {
        control.label.clone()
    };
    let label_text = root.label.clone();
    let root_detail = root.detail.clone();
    let hint_detail = hint.detail.clone();
    let max_length_attr = max_length.map(|value| value.to_string());
    let (state, set_state) = signal(model.state());

    view! {
        <section
            class=textarea_root_class(density, loading, disabled, invalid)
            data-ui-component="textarea"
            data-ui-part=TextareaPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| textarea_state_label(loading, disabled, invalid, state.is_focused()).to_owned())
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
        >
            <label
                class=textarea_label_class(density, disabled)
                for=control_id.clone()
            >
                <span>{label_text.clone()}</span>
                {required.then_some(view! { <span class=TEXTAREA_REQUIRED aria-hidden="true">"*"</span> })}
            </label>
            <textarea
                id=control_id
                class=move || {
                    state.with(|state| {
                        textarea_control_class(density, state.is_focused(), invalid, blocked).to_owned()
                    })
                }
                data-ui-part=TextareaPart::Control.label()
                placeholder=placeholder
                aria-label=label_text
                aria-describedby=described_by
                aria-invalid=invalid.to_string()
                required=required
                disabled=blocked
                rows=rows.to_string()
                maxlength=max_length_attr
                prop:value=move || state.with(|state| state.value().to_owned())
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(TextareaIntent::Focus);
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(TextareaIntent::Blur);
                        });
                    }
                }
                on:input=move |event| {
                    if !blocked {
                        let value = event_target_value(&event);
                        set_state.update(|state| {
                            let _ = state.apply(TextareaIntent::Input(value));
                        });
                    }
                }
            ></textarea>
            <div class=TEXTAREA_META>
                <p
                    id=hint_id
                    class=textarea_hint_class(invalid, disabled)
                    data-ui-part=TextareaPart::Hint.label()
                >
                    {if invalid { hint_detail } else { root_detail }}
                </p>
                <p
                    id=counter_id
                    class=move || {
                        state.with(|state| {
                            textarea_counter_class(state.current_length(), max_length, invalid, disabled).to_owned()
                        })
                    }
                    data-ui-part=TextareaPart::Counter.label()
                    aria-live="polite"
                >
                    {move || {
                        state.with(|state| {
                            textarea_render_nodes(&counter_model, state)
                                .into_iter()
                                .find(|node| node.part == TextareaPart::Counter)
                                .map(|node| node.label)
                                .unwrap_or_else(|| "0 chars".to_owned())
                        })
                    }}
                </p>
            </div>
        </section>
    }
    .into_any()
}

const fn textarea_root_class(
    density: TextareaDensity,
    loading: bool,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        return TEXTAREA_ROOT_DISABLED;
    }
    if invalid {
        return TEXTAREA_ROOT_INVALID;
    }
    if loading {
        return TEXTAREA_ROOT_LOADING;
    }
    match density {
        TextareaDensity::Standard => TEXTAREA_ROOT,
        TextareaDensity::Dense => TEXTAREA_ROOT_DENSE,
    }
}

const fn textarea_label_class(density: TextareaDensity, disabled: bool) -> &'static str {
    if disabled {
        return TEXTAREA_LABEL_DISABLED;
    }
    match density {
        TextareaDensity::Standard => TEXTAREA_LABEL,
        TextareaDensity::Dense => TEXTAREA_LABEL_DENSE,
    }
}

const fn textarea_control_class(
    density: TextareaDensity,
    focused: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    match (density, disabled, invalid, focused) {
        (TextareaDensity::Standard, true, _, _) => TEXTAREA_CONTROL_DISABLED,
        (TextareaDensity::Dense, true, _, _) => TEXTAREA_CONTROL_DENSE_DISABLED,
        (TextareaDensity::Standard, false, true, _) => TEXTAREA_CONTROL_INVALID,
        (TextareaDensity::Dense, false, true, _) => TEXTAREA_CONTROL_DENSE_INVALID,
        (TextareaDensity::Standard, false, false, true) => TEXTAREA_CONTROL_FOCUSED,
        (TextareaDensity::Dense, false, false, true) => TEXTAREA_CONTROL_DENSE_FOCUSED,
        (TextareaDensity::Standard, false, false, false) => TEXTAREA_CONTROL,
        (TextareaDensity::Dense, false, false, false) => TEXTAREA_CONTROL_DENSE,
    }
}

const fn textarea_hint_class(invalid: bool, disabled: bool) -> &'static str {
    if disabled {
        TEXTAREA_HINT_DISABLED
    } else if invalid {
        TEXTAREA_HINT_INVALID
    } else {
        TEXTAREA_HINT
    }
}

fn textarea_counter_class(
    current_length: usize,
    max_length: Option<u16>,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return TEXTAREA_COUNTER_DISABLED;
    }
    if invalid || max_length.is_some_and(|max_length| current_length >= usize::from(max_length)) {
        TEXTAREA_COUNTER_LIMIT
    } else {
        TEXTAREA_COUNTER
    }
}

const fn textarea_state_label(
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
pub fn NativeSelect(
    #[prop(optional, default = default_native_select_model())] model: NativeSelectModel,
) -> AnyView {
    if let Err(report) = validate_native_select_model(&model) {
        let message = format!("NativeSelect validation failed: {report}");
        return view! {
            <div class=INPUT_ERROR data-ui-component="native-select" data-ui-state="invalid" role="alert">
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
    let state_model = model.state();
    let nodes = native_select_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == NativeSelectPart::Root)
        .expect("invariant: native select render nodes include root")
        .clone();
    let trigger = nodes
        .iter()
        .find(|node| node.part == NativeSelectPart::Trigger)
        .expect("invariant: native select render nodes include trigger")
        .clone();
    let value = nodes
        .iter()
        .find(|node| node.part == NativeSelectPart::Value)
        .expect("invariant: native select render nodes include value")
        .clone();
    let options = nodes
        .iter()
        .filter(|node| node.part == NativeSelectPart::Option)
        .cloned()
        .collect::<Vec<_>>();
    let invalid = root.invalid;
    let label = root.label.clone();
    let placeholder = model.placeholder.clone();
    let value_detail = value.detail.clone();
    let option_lookup = model.options.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=native_select_root_class(disabled)
            data-ui-component="native-select"
            data-ui-part=NativeSelectPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    native_select_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_focused(),
                        state.selected_value().is_some(),
                    )
                    .to_owned()
                })
            }
            data-ui-value=move || {
                state.with(|state| state.selected_value().unwrap_or("").to_owned())
            }
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <label class=native_select_label_class(disabled)>
                {label}
            </label>
            <select
                class=move || {
                    state.with(|state| {
                        native_select_trigger_class(
                            density,
                            state.is_focused(),
                            invalid,
                            loading,
                            disabled,
                        )
                        .to_owned()
                    })
                }
                data-ui-part=NativeSelectPart::Trigger.label()
                data-ui-value=trigger.value
                aria-label=trigger.label
                aria-invalid=invalid.to_string()
                required=required
                disabled=blocked
                prop:value=move || {
                    state.with(|state| state.selected_value().unwrap_or("").to_owned())
                }
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(NativeSelectIntent::Focus);
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(NativeSelectIntent::Blur);
                        });
                    }
                }
                on:change=move |event| {
                    if !blocked {
                        let value = event_target_value(&event);
                        set_state.update(|state| {
                            let _ = state.apply(NativeSelectIntent::Select(value));
                        });
                    }
                }
            >
                <option
                    value=""
                    selected=move || state.with(|state| state.selected_value().is_none())
                    disabled=required
                >
                    {placeholder.clone()}
                </option>
                {options
                    .into_iter()
                    .map(|option| native_select_option_view(option, state))
                    .collect_view()}
            </select>
            <p
                class=move || {
                    state.with(|state| {
                        native_select_value_class(invalid, state.selected_value().is_some())
                            .to_owned()
                    })
                }
                data-ui-part=NativeSelectPart::Value.label()
                aria-live="polite"
            >
                {move || {
                    state.with(|state| {
                        native_select_value_label(
                            &option_lookup,
                            state.selected_value(),
                            &placeholder,
                            invalid,
                            &value_detail,
                        )
                    })
                }}
            </p>
        </section>
    }
    .into_any()
}

fn native_select_option_view(
    node: crate::NativeSelectRenderNode,
    state: ReadSignal<NativeSelectState>,
) -> AnyView {
    let value_for_selected = node.value.clone();
    let value_for_attr = node.value;
    let label = node.label;
    let detail = node.detail;
    let index = node.index.to_string();
    let disabled = node.disabled;
    view! {
        <option
            value=value_for_attr
            selected=move || state.with(|state| state.is_selected(&value_for_selected))
            disabled=disabled
            data-ui-part=NativeSelectPart::Option.label()
            data-ui-index=index
            data-ui-detail=detail
        >
            {label}
        </option>
    }
    .into_any()
}

fn native_select_value_label(
    options: &[crate::NativeSelectOption],
    selected: Option<&str>,
    placeholder: &str,
    invalid: bool,
    error_detail: &str,
) -> String {
    if invalid {
        return error_detail.to_owned();
    }
    selected
        .and_then(|selected| {
            options
                .iter()
                .find(|option| option.value == selected)
                .map(|option| option.label.clone())
        })
        .unwrap_or_else(|| placeholder.to_owned())
}

const fn native_select_root_class(disabled: bool) -> &'static str {
    if disabled {
        NATIVE_SELECT_ROOT_DISABLED
    } else {
        NATIVE_SELECT_ROOT
    }
}

const fn native_select_label_class(disabled: bool) -> &'static str {
    if disabled {
        NATIVE_SELECT_LABEL_DISABLED
    } else {
        NATIVE_SELECT_LABEL
    }
}

const fn native_select_trigger_class(
    density: NativeSelectDensity,
    focused: bool,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return NATIVE_SELECT_TRIGGER_DISABLED;
    }
    if loading {
        return NATIVE_SELECT_TRIGGER_LOADING;
    }
    match (density, invalid, focused) {
        (NativeSelectDensity::Standard, true, _) => NATIVE_SELECT_TRIGGER_INVALID,
        (NativeSelectDensity::Dense, true, _) => NATIVE_SELECT_TRIGGER_DENSE_INVALID,
        (NativeSelectDensity::Standard, false, true) => NATIVE_SELECT_TRIGGER_FOCUSED,
        (NativeSelectDensity::Dense, false, true) => NATIVE_SELECT_TRIGGER_DENSE_FOCUSED,
        (NativeSelectDensity::Standard, false, false) => NATIVE_SELECT_TRIGGER,
        (NativeSelectDensity::Dense, false, false) => NATIVE_SELECT_TRIGGER_DENSE,
    }
}

const fn native_select_value_class(invalid: bool, selected: bool) -> &'static str {
    if invalid {
        NATIVE_SELECT_VALUE_INVALID
    } else if selected {
        NATIVE_SELECT_VALUE
    } else {
        NATIVE_SELECT_VALUE_PLACEHOLDER
    }
}

const fn native_select_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    focused: bool,
    selected: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if focused {
        "focused"
    } else if selected {
        "selected"
    } else {
        "empty"
    }
}

#[component]
pub fn NavigationMenu(
    #[prop(optional, default = default_navigation_menu_model())] model: NavigationMenuModel,
) -> AnyView {
    if let Err(report) = validate_navigation_menu_model(&model) {
        let message = format!("NavigationMenu validation failed: {report}");
        return view! {
            <div class=INPUT_ERROR data-ui-component="navigation-menu" data-ui-state="invalid" role="alert">
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
    let nodes = navigation_menu_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == NavigationMenuPart::Root)
        .expect("invariant: navigation menu render nodes include root")
        .clone();
    let list = nodes
        .iter()
        .find(|node| node.part == NavigationMenuPart::List)
        .expect("invariant: navigation menu render nodes include list")
        .clone();
    let item_nodes = nodes
        .iter()
        .filter(|node| node.part == NavigationMenuPart::Item)
        .cloned()
        .collect::<Vec<_>>();
    let invalid = root.invalid;
    let error_detail = root.detail.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <nav
            class=navigation_menu_root_class(disabled)
            data-ui-component="navigation-menu"
            data-ui-part=NavigationMenuPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    navigation_menu_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.open_item().is_some(),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root.value
            aria-label=root.label
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            on:mouseleave=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(NavigationMenuIntent::Close);
                    });
                }
            }
        >
            <ul
                class=navigation_menu_list_class(density, loading)
                data-ui-part=NavigationMenuPart::List.label()
                data-ui-value=list.value
                role="menubar"
            >
                {item_nodes
                    .into_iter()
                    .map(|item| {
                        navigation_menu_item_view(
                            item,
                            nodes.clone(),
                            density,
                            blocked,
                            state,
                            set_state,
                        )
                    })
                    .collect_view()}
            </ul>
            <p
                class=navigation_menu_error_class(invalid)
                role="alert"
                aria-hidden=(!invalid).to_string()
            >
                {error_detail}
            </p>
        </nav>
    }
    .into_any()
}

fn navigation_menu_item_view(
    item: crate::NavigationMenuRenderNode,
    nodes: Vec<crate::NavigationMenuRenderNode>,
    density: NavigationMenuDensity,
    blocked: bool,
    state: ReadSignal<NavigationMenuState>,
    set_state: WriteSignal<NavigationMenuState>,
) -> AnyView {
    let item_value = item.value.clone();
    let item_data_value = item.value.clone();
    let item_disabled = item.disabled;
    let fallback_item = item.clone();
    let child_nodes = nodes
        .into_iter()
        .filter(|node| node.item_index == item.item_index && node.part != NavigationMenuPart::Item)
        .collect::<Vec<_>>();
    let trigger = child_nodes
        .iter()
        .find(|node| node.part == NavigationMenuPart::Trigger)
        .cloned();
    let content = child_nodes
        .iter()
        .find(|node| node.part == NavigationMenuPart::Content)
        .cloned();
    let links = child_nodes
        .into_iter()
        .filter(|node| node.part == NavigationMenuPart::Link)
        .collect::<Vec<_>>();

    view! {
        <li
            class=NAVIGATION_MENU_ITEM
            data-ui-part=NavigationMenuPart::Item.label()
            data-ui-value=item_data_value
            aria-disabled=(item_disabled || blocked).to_string()
        >
            {if let Some(trigger) = trigger {
                navigation_menu_panel_view(
                    trigger,
                    content,
                    links,
                    density,
                    blocked,
                    state,
                    set_state,
                )
            } else {
                let link = links
                    .into_iter()
                    .find(|link| link.value == item_value)
                    .unwrap_or(fallback_item);
                navigation_menu_direct_link_view(link, density, blocked, state, set_state)
            }}
        </li>
    }
    .into_any()
}

fn navigation_menu_panel_view(
    trigger: crate::NavigationMenuRenderNode,
    content: Option<crate::NavigationMenuRenderNode>,
    links: Vec<crate::NavigationMenuRenderNode>,
    density: NavigationMenuDensity,
    blocked: bool,
    state: ReadSignal<NavigationMenuState>,
    set_state: WriteSignal<NavigationMenuState>,
) -> AnyView {
    let disabled = trigger.disabled || blocked;
    let value_for_click = trigger.value.clone();
    let value_for_focus = trigger.value.clone();
    let value_for_expanded = trigger.value.clone();
    let value_for_mouse = trigger.value.clone();
    let trigger_value = trigger.value.clone();
    let trigger_label = trigger.label.clone();
    let content_node = content.unwrap_or_else(|| trigger.clone());
    let content_value_for_class = content_node.value.clone();
    let content_value_for_hidden = content_node.value.clone();
    let content_detail = content_node.detail.clone();

    view! {
        <button
            type="button"
            class=move || {
                state.with(|state| {
                    navigation_menu_trigger_class(
                        density,
                        state.is_open(&trigger_value),
                        trigger.invalid,
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-part=NavigationMenuPart::Trigger.label()
            data-ui-value=trigger.value
            aria-haspopup="menu"
            aria-expanded=move || state.with(|state| state.is_open(&value_for_expanded).to_string())
            disabled=disabled
            on:focus=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(NavigationMenuIntent::Focus(value_for_focus.clone()));
                    });
                }
            }
            on:mouseenter=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(NavigationMenuIntent::Open(value_for_mouse.clone()));
                    });
                }
            }
            on:click=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(NavigationMenuIntent::Toggle(value_for_click.clone()));
                    });
                }
            }
        >
            {trigger_label}
        </button>
        <div
            role="menu"
            class=move || {
                state.with(|state| {
                    navigation_menu_content_class(
                        density,
                        state.is_open(&content_value_for_class),
                        content_node.invalid,
                        content_node.loading,
                        content_node.disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-part=NavigationMenuPart::Content.label()
            data-ui-value=content_node.value
            aria-label=content_detail
            hidden=move || state.with(|state| !state.is_open(&content_value_for_hidden))
        >
            {links
                .into_iter()
                .map(|link| {
                    navigation_menu_panel_link_view(link, density, blocked, state, set_state)
                })
                .collect_view()}
        </div>
    }
    .into_any()
}

fn navigation_menu_direct_link_view(
    node: crate::NavigationMenuRenderNode,
    density: NavigationMenuDensity,
    blocked: bool,
    state: ReadSignal<NavigationMenuState>,
    set_state: WriteSignal<NavigationMenuState>,
) -> AnyView {
    let disabled = node.disabled || blocked;
    let actionable = node.actionable;
    let value_for_click = node.value.clone();
    let value_for_current = node.value.clone();
    let value_for_focus = node.value.clone();
    let value_for_class = node.value.clone();
    let href = node.href.clone();
    let label = node.label.clone();
    view! {
        <a
            class=move || {
                state.with(|state| {
                    navigation_menu_link_class(
                        density,
                        state.is_selected(&value_for_class),
                        node.invalid,
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-part=NavigationMenuPart::Link.label()
            data-ui-value=node.value
            href=href
            aria-disabled=disabled.to_string()
            aria-current=move || {
                state.with(|state| {
                    if state.is_selected(&value_for_current) { "page" } else { "false" }.to_owned()
                })
            }
            tabindex=if disabled { "-1" } else { "0" }
            on:focus=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(NavigationMenuIntent::Focus(value_for_focus.clone()));
                    });
                }
            }
            on:click=move |event| {
                if disabled || !actionable {
                    event.prevent_default();
                } else {
                    set_state.update(|state| {
                        let _ = state.apply(NavigationMenuIntent::Navigate(value_for_click.clone()));
                    });
                }
            }
        >
            {label}
        </a>
    }
    .into_any()
}

fn navigation_menu_panel_link_view(
    node: crate::NavigationMenuRenderNode,
    density: NavigationMenuDensity,
    blocked: bool,
    state: ReadSignal<NavigationMenuState>,
    set_state: WriteSignal<NavigationMenuState>,
) -> AnyView {
    let disabled = node.disabled || blocked;
    let actionable = node.actionable;
    let visible = node.visible;
    let value_for_click = node.value.clone();
    let value_for_current = node.value.clone();
    let value_for_focus = node.value.clone();
    let value_for_class = node.value.clone();
    let href = node.href.clone();
    let label = node.label.clone();
    let detail = node.detail.clone();
    view! {
        <a
            class=move || {
                state.with(|state| {
                    navigation_menu_panel_link_class(
                        density,
                        visible,
                        state.is_selected(&value_for_class),
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-part=NavigationMenuPart::Link.label()
            data-ui-value=node.value
            data-ui-parent=node.item_value
            href=href
            aria-disabled=disabled.to_string()
            aria-hidden=(!visible).to_string()
            aria-current=move || {
                state.with(|state| {
                    if state.is_selected(&value_for_current) { "page" } else { "false" }.to_owned()
                })
            }
            tabindex=if disabled { "-1" } else { "0" }
            on:focus=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(NavigationMenuIntent::Focus(value_for_focus.clone()));
                    });
                }
            }
            on:click=move |event| {
                if disabled || !actionable {
                    event.prevent_default();
                } else {
                    set_state.update(|state| {
                        let _ = state.apply(NavigationMenuIntent::Navigate(value_for_click.clone()));
                    });
                }
            }
        >
            <span class=NAVIGATION_MENU_PANEL_TITLE>{label}</span>
            <span class=NAVIGATION_MENU_PANEL_DETAIL>{detail}</span>
        </a>
    }
    .into_any()
}

const fn navigation_menu_root_class(disabled: bool) -> &'static str {
    if disabled {
        NAVIGATION_MENU_ROOT_DISABLED
    } else {
        NAVIGATION_MENU_ROOT
    }
}

const fn navigation_menu_list_class(density: NavigationMenuDensity, loading: bool) -> &'static str {
    if loading {
        return NAVIGATION_MENU_LIST_LOADING;
    }
    match density {
        NavigationMenuDensity::Standard => NAVIGATION_MENU_LIST,
        NavigationMenuDensity::Dense => NAVIGATION_MENU_LIST_DENSE,
    }
}

const fn navigation_menu_trigger_class(
    density: NavigationMenuDensity,
    open: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return NAVIGATION_MENU_TRIGGER_DISABLED;
    }
    if invalid {
        return NAVIGATION_MENU_TRIGGER_INVALID;
    }
    match (density, open) {
        (NavigationMenuDensity::Standard, true) => NAVIGATION_MENU_TRIGGER_OPEN,
        (NavigationMenuDensity::Dense, true) => NAVIGATION_MENU_TRIGGER_DENSE_OPEN,
        (NavigationMenuDensity::Standard, false) => NAVIGATION_MENU_TRIGGER,
        (NavigationMenuDensity::Dense, false) => NAVIGATION_MENU_TRIGGER_DENSE,
    }
}

const fn navigation_menu_link_class(
    density: NavigationMenuDensity,
    selected: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return NAVIGATION_MENU_LINK_DISABLED;
    }
    if invalid {
        return NAVIGATION_MENU_LINK_INVALID;
    }
    match (density, selected) {
        (NavigationMenuDensity::Standard, true) => NAVIGATION_MENU_LINK_SELECTED,
        (NavigationMenuDensity::Dense, true) => NAVIGATION_MENU_LINK_DENSE_SELECTED,
        (NavigationMenuDensity::Standard, false) => NAVIGATION_MENU_LINK,
        (NavigationMenuDensity::Dense, false) => NAVIGATION_MENU_LINK_DENSE,
    }
}

const fn navigation_menu_content_class(
    density: NavigationMenuDensity,
    open: bool,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if !open {
        return NAVIGATION_MENU_CONTENT_HIDDEN;
    }
    if disabled {
        return NAVIGATION_MENU_CONTENT_DISABLED;
    }
    if loading {
        return NAVIGATION_MENU_CONTENT_LOADING;
    }
    if invalid {
        return NAVIGATION_MENU_CONTENT_INVALID;
    }
    match density {
        NavigationMenuDensity::Standard => NAVIGATION_MENU_CONTENT,
        NavigationMenuDensity::Dense => NAVIGATION_MENU_CONTENT_DENSE,
    }
}

const fn navigation_menu_panel_link_class(
    density: NavigationMenuDensity,
    visible: bool,
    selected: bool,
    disabled: bool,
) -> &'static str {
    if !visible {
        return NAVIGATION_MENU_CONTENT_HIDDEN;
    }
    if disabled {
        return NAVIGATION_MENU_PANEL_LINK_DISABLED;
    }
    match (density, selected) {
        (NavigationMenuDensity::Standard, true) => NAVIGATION_MENU_PANEL_LINK_SELECTED,
        (NavigationMenuDensity::Dense, true) => NAVIGATION_MENU_PANEL_LINK_SELECTED,
        (NavigationMenuDensity::Standard, false) => NAVIGATION_MENU_PANEL_LINK,
        (NavigationMenuDensity::Dense, false) => NAVIGATION_MENU_PANEL_LINK_DENSE,
    }
}

const fn navigation_menu_error_class(visible: bool) -> &'static str {
    if visible {
        NAVIGATION_MENU_ERROR
    } else {
        NAVIGATION_MENU_ERROR_HIDDEN
    }
}

const fn navigation_menu_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    open: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if open {
        "open"
    } else {
        "closed"
    }
}

#[component]
pub fn Pagination(
    #[prop(optional, default = default_pagination_model())] model: PaginationModel,
) -> AnyView {
    if let Err(report) = validate_pagination_model(&model) {
        let message = format!("Pagination validation failed: {report}");
        return view! {
            <div class=INPUT_ERROR data-ui-component="pagination" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let page_count = model.page_count;
    let state_model = model.state();
    let nodes = pagination_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == PaginationPart::Root)
        .expect("invariant: pagination render nodes include root")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == PaginationPart::Content)
        .expect("invariant: pagination render nodes include content")
        .clone();
    let invalid = root.invalid;
    let error_detail = root.detail.clone();
    let render_model = model.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <nav
            class=pagination_root_class(disabled)
            data-ui-component="pagination"
            data-ui-part=PaginationPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    pagination_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.current_page(),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root.value
            aria-label=root.label
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <ul
                class=pagination_content_class(density, loading)
                data-ui-part=PaginationPart::Content.label()
                data-ui-value=content.value
            >
                {move || {
                    state.with(|state| {
                        pagination_render_nodes(&render_model, state)
                            .into_iter()
                            .filter(|node| {
                                matches!(
                                    node.part,
                                    PaginationPart::Previous
                                        | PaginationPart::Link
                                        | PaginationPart::Next
                                )
                            })
                            .map(|node| {
                                pagination_control_view(
                                    node,
                                    page_count,
                                    density,
                                    blocked,
                                    state,
                                    set_state,
                                )
                            })
                            .collect_view()
                    })
                }}
            </ul>
            <p
                class=pagination_error_class(invalid)
                role="alert"
                aria-hidden=(!invalid).to_string()
            >
                {error_detail}
            </p>
        </nav>
    }
    .into_any()
}

fn pagination_control_view(
    node: crate::PaginationRenderNode,
    page_count: u16,
    density: PaginationDensity,
    blocked: bool,
    state: &PaginationState,
    set_state: WriteSignal<PaginationState>,
) -> AnyView {
    let page = node.page;
    let disabled = pagination_control_is_disabled(node.part, page, page_count, blocked, state);
    let value_for_focus = page;
    let value_for_click = page;
    let current = node.current;
    let invalid = node.invalid;
    let part = node.part;
    let part_label = node.part.label();
    let item_value = node.value.clone();
    let button_value = node.value.clone();
    let label = node.label.clone();
    let detail = node.detail.clone();
    view! {
        <li
            class=PAGINATION_ITEM
            data-ui-part=PaginationPart::Item.label()
            data-ui-value=item_value
        >
            <button
                type="button"
                class=pagination_control_class(density, current, invalid, disabled)
                data-ui-part=part_label
                data-ui-value=button_value
                data-ui-page=page.to_string()
                aria-label=detail
                aria-current=if current { "page" } else { "false" }
                disabled=disabled
                on:focus=move |_| {
                    if !disabled {
                        set_state.update(|state| {
                            let _ = state.apply(PaginationIntent::Focus(value_for_focus), page_count);
                        });
                    }
                }
                on:click=move |_| {
                    if !disabled {
                        let intent = match part {
                            PaginationPart::Previous => PaginationIntent::Previous,
                            PaginationPart::Next => PaginationIntent::Next,
                            PaginationPart::Link => PaginationIntent::GoTo(value_for_click),
                            _ => PaginationIntent::Focus(value_for_click),
                        };
                        set_state.update(|state| {
                            let _ = state.apply(intent, page_count);
                        });
                    }
                }
            >
                {label}
            </button>
        </li>
    }
    .into_any()
}

const fn pagination_control_is_disabled(
    part: PaginationPart,
    page: u16,
    page_count: u16,
    blocked: bool,
    state: &PaginationState,
) -> bool {
    if blocked {
        return true;
    }
    match part {
        PaginationPart::Previous => state.current_page() <= 1,
        PaginationPart::Next => state.current_page() >= page_count,
        PaginationPart::Link => state.current_page() == page,
        PaginationPart::Root | PaginationPart::Content | PaginationPart::Item => true,
    }
}

const fn pagination_root_class(disabled: bool) -> &'static str {
    if disabled {
        PAGINATION_ROOT_DISABLED
    } else {
        PAGINATION_ROOT
    }
}

const fn pagination_content_class(density: PaginationDensity, loading: bool) -> &'static str {
    if loading {
        return PAGINATION_CONTENT_LOADING;
    }
    match density {
        PaginationDensity::Standard => PAGINATION_CONTENT,
        PaginationDensity::Dense => PAGINATION_CONTENT_DENSE,
    }
}

const fn pagination_control_class(
    density: PaginationDensity,
    current: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled && !current {
        return PAGINATION_CONTROL_DISABLED;
    }
    if invalid {
        return PAGINATION_CONTROL_INVALID;
    }
    match (density, current) {
        (PaginationDensity::Standard, true) => PAGINATION_CONTROL_CURRENT,
        (PaginationDensity::Dense, true) => PAGINATION_CONTROL_DENSE_CURRENT,
        (PaginationDensity::Standard, false) => PAGINATION_CONTROL,
        (PaginationDensity::Dense, false) => PAGINATION_CONTROL_DENSE,
    }
}

const fn pagination_error_class(visible: bool) -> &'static str {
    if visible {
        PAGINATION_ERROR
    } else {
        PAGINATION_ERROR_HIDDEN
    }
}

fn pagination_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    current_page: u16,
) -> String {
    if disabled {
        "disabled".to_owned()
    } else if loading {
        "loading".to_owned()
    } else if invalid {
        "invalid".to_owned()
    } else {
        format!("page-{current_page}")
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

#[component]
pub fn Marker(#[prop(optional, default = default_marker_model())] model: MarkerModel) -> AnyView {
    if let Err(report) = validate_marker_model(&model) {
        let message = format!("Marker validation failed: {report}");
        return view! {
            <div class=MARKER_ERROR data-ui-component="marker" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let tone = model.tone;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = marker_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == MarkerPart::Root)
        .expect("invariant: marker render nodes include root")
        .clone();
    let dot = nodes
        .iter()
        .find(|node| node.part == MarkerPart::Dot)
        .expect("invariant: marker render nodes include dot")
        .clone();
    let label = nodes
        .iter()
        .find(|node| node.part == MarkerPart::Label)
        .expect("invariant: marker render nodes include label")
        .clone();
    let anchor = nodes
        .iter()
        .find(|node| node.part == MarkerPart::Anchor)
        .expect("invariant: marker render nodes include anchor")
        .clone();
    let invalid = root.invalid;
    let root_value = root.value;
    let root_detail = root.detail;
    let dot_value = dot.value;
    let dot_label = dot.label;
    let label_value = label.value;
    let label_text = label.label;
    let (state, set_state) = signal(state_model);

    view! {
        <span
            role="group"
            class=move || {
                state.with(|state| {
                    marker_root_class(
                        density,
                        marker_any_active(state),
                        invalid,
                        loading,
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-component="marker"
            data-ui-part=MarkerPart::Root.label()
            data-ui-density=density.label()
            data-ui-tone=tone.label()
            data-ui-state=move || {
                state.with(|state| {
                    marker_state_label(
                        loading,
                        disabled,
                        invalid,
                        marker_any_active(state),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root_value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
            title=root_detail
            on:mouseenter=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(MarkerIntent::Hover(MarkerPart::Root));
                    });
                }
            }
            on:mouseleave=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(MarkerIntent::Leave);
                    });
                }
            }
        >
            <span
                role="img"
                class=marker_dot_class(density, tone, invalid, loading, disabled)
                data-ui-part=MarkerPart::Dot.label()
                data-ui-value=dot_value
                aria-label=dot_label
                on:mouseenter=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(MarkerIntent::Hover(MarkerPart::Dot));
                        });
                    }
                }
                on:mouseleave=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(MarkerIntent::Leave);
                        });
                    }
                }
            ></span>
            <span
                class=marker_label_class(density, disabled)
                data-ui-part=MarkerPart::Label.label()
                data-ui-value=label_value
                on:mouseenter=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(MarkerIntent::Hover(MarkerPart::Label));
                        });
                    }
                }
                on:mouseleave=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(MarkerIntent::Leave);
                        });
                    }
                }
            >
                {label_text}
            </span>
            {marker_anchor_view(anchor, blocked, state, set_state)}
        </span>
    }
    .into_any()
}

fn marker_anchor_view(
    node: crate::MarkerRenderNode,
    blocked: bool,
    state: ReadSignal<MarkerState>,
    set_state: WriteSignal<MarkerState>,
) -> AnyView {
    let visible = node.visible;
    let actionable = node.actionable;
    let disabled = node.disabled || blocked;
    let loading = node.loading;
    let invalid = node.invalid;
    let href = node.value;
    let label = node.label;
    let detail = node.detail;
    let href_for_click = href.clone();
    if !visible {
        return view! {
            <span
                class=MARKER_ANCHOR_HIDDEN
                data-ui-part=MarkerPart::Anchor.label()
                data-ui-value=href
                aria-hidden="true"
            >
                {label}
            </span>
        }
        .into_any();
    }

    view! {
        <a
            class=move || {
                state.with(|state| {
                    marker_anchor_class(state.is_active(MarkerPart::Anchor), disabled).to_owned()
                })
            }
            data-ui-part=MarkerPart::Anchor.label()
            data-ui-value=href.clone()
            data-ui-state=move || {
                state.with(|state| {
                    marker_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_active(MarkerPart::Anchor),
                    )
                    .to_owned()
                })
            }
            href=href
            aria-disabled=disabled.to_string()
            tabindex=if disabled { "-1" } else { "0" }
            title=detail
            on:focus=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(MarkerIntent::Focus(MarkerPart::Anchor));
                    });
                }
            }
            on:blur=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(MarkerIntent::Blur);
                    });
                }
            }
            on:click=move |event| {
                if disabled || !actionable {
                    event.prevent_default();
                } else {
                    set_state.update(|state| {
                        let _ = state.apply(MarkerIntent::Navigate(href_for_click.clone()));
                    });
                }
            }
        >
            {label}
        </a>
    }
    .into_any()
}

const fn marker_root_class(
    density: MarkerDensity,
    active: bool,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return MARKER_ROOT_DISABLED;
    }
    if loading {
        return MARKER_ROOT_LOADING;
    }
    if invalid {
        return MARKER_ROOT_INVALID;
    }
    if active {
        return MARKER_ROOT_ACTIVE;
    }
    match density {
        MarkerDensity::Standard => MARKER_ROOT,
        MarkerDensity::Dense => MARKER_ROOT_DENSE,
    }
}

const fn marker_dot_class(
    density: MarkerDensity,
    tone: MarkerTone,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return MARKER_DOT_DISABLED;
    }
    if loading {
        return MARKER_DOT_LOADING;
    }
    if invalid {
        return MARKER_DOT_INVALID;
    }
    match (density, tone) {
        (MarkerDensity::Dense, MarkerTone::Brand) => MARKER_DOT_DENSE,
        (_, MarkerTone::Neutral) => MARKER_DOT_NEUTRAL,
        (_, MarkerTone::Brand) => MARKER_DOT,
        (_, MarkerTone::Info) => MARKER_DOT_INFO,
        (_, MarkerTone::Success) => MARKER_DOT_SUCCESS,
        (_, MarkerTone::Warning) => MARKER_DOT_WARNING,
        (_, MarkerTone::Danger) => MARKER_DOT_DANGER,
    }
}

const fn marker_label_class(density: MarkerDensity, disabled: bool) -> &'static str {
    if disabled {
        return MARKER_LABEL_DISABLED;
    }
    match density {
        MarkerDensity::Standard => MARKER_LABEL,
        MarkerDensity::Dense => MARKER_LABEL_DENSE,
    }
}

const fn marker_anchor_class(active: bool, disabled: bool) -> &'static str {
    if disabled {
        MARKER_ANCHOR_DISABLED
    } else if active {
        MARKER_ANCHOR_ACTIVE
    } else {
        MARKER_ANCHOR
    }
}

const fn marker_any_active(state: &MarkerState) -> bool {
    state.is_active(MarkerPart::Root)
        || state.is_active(MarkerPart::Dot)
        || state.is_active(MarkerPart::Label)
        || state.is_active(MarkerPart::Anchor)
}

const fn marker_state_label(
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

#[component]
pub fn Menubar(
    #[prop(optional, default = default_menubar_model())] model: MenubarModel,
) -> AnyView {
    if let Err(report) = validate_menubar_model(&model) {
        let message = format!("Menubar validation failed: {report}");
        return view! {
            <div class=MENUBAR_ERROR data-ui-component="menubar" data-ui-state="invalid" role="alert">
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
    let nodes = menubar_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == MenubarPart::Root)
        .expect("invariant: menubar render nodes include root")
        .clone();
    let menu_nodes = nodes
        .iter()
        .filter(|node| node.part == MenubarPart::Menu)
        .cloned()
        .collect::<Vec<_>>();
    let trigger_nodes = nodes
        .iter()
        .filter(|node| node.part == MenubarPart::Trigger)
        .cloned()
        .collect::<Vec<_>>();
    let content_nodes = nodes
        .iter()
        .filter(|node| node.part == MenubarPart::Content)
        .cloned()
        .collect::<Vec<_>>();
    let item_nodes = nodes
        .into_iter()
        .filter(|node| node.part == MenubarPart::Item)
        .collect::<Vec<_>>();
    let invalid = root.invalid;
    let root_value = root.value;
    let root_detail = root.detail;
    let (state, set_state) = signal(state_model);
    let context = MenubarViewContext {
        density,
        blocked,
        state,
        set_state,
    };

    view! {
        <nav
            role="menubar"
            class=menubar_root_class(density, invalid, loading, disabled)
            data-ui-component="menubar"
            data-ui-part=MenubarPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    menubar_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.open_menu().is_some(),
                        state.focused_value().is_some(),
                        state.selected_value().is_some(),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root_value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
            title=root_detail
        >
            <div class=MENUBAR_ROW>
                {menu_nodes
                    .into_iter()
                    .map(|menu| {
                        let menu_value = menu.value.clone();
                        let trigger = trigger_nodes
                            .iter()
                            .find(|node| node.menu_value == menu_value)
                            .expect("invariant: menubar menu includes trigger")
                            .clone();
                        let content = content_nodes
                            .iter()
                            .find(|node| node.menu_value == menu_value)
                            .expect("invariant: menubar menu includes content")
                            .clone();
                        let items = item_nodes
                            .iter()
                            .filter(|node| node.menu_value == menu_value)
                            .cloned()
                            .collect::<Vec<_>>();
                        menubar_menu_view(menu, trigger, content, items, context)
                    })
                    .collect_view()}
            </div>
        </nav>
    }
    .into_any()
}

#[derive(Clone, Copy)]
struct MenubarViewContext {
    density: MenubarDensity,
    blocked: bool,
    state: ReadSignal<MenubarState>,
    set_state: WriteSignal<MenubarState>,
}

fn menubar_menu_view(
    menu: crate::MenubarRenderNode,
    trigger: crate::MenubarRenderNode,
    content: crate::MenubarRenderNode,
    items: Vec<crate::MenubarRenderNode>,
    context: MenubarViewContext,
) -> AnyView {
    let menu_value = menu.value;
    let menu_label = menu.label;
    let menu_detail = menu.detail;
    view! {
        <div
            class=menubar_menu_class(context.density)
            data-ui-part=MenubarPart::Menu.label()
            data-ui-index=menu.menu_index.to_string()
            data-ui-value=menu_value
            aria-label=menu_label
            title=menu_detail
        >
            {menubar_trigger_view(trigger, context)}
            {menubar_content_view(content, items, context)}
        </div>
    }
    .into_any()
}

fn menubar_trigger_view(node: crate::MenubarRenderNode, context: MenubarViewContext) -> AnyView {
    let disabled = node.disabled || context.blocked;
    let value = node.value;
    let value_for_class = value.clone();
    let value_for_expanded = value.clone();
    let value_for_data = value.clone();
    let value_for_click = value.clone();
    let value_for_enter = value.clone();
    let label = node.label;
    let detail = node.detail;
    view! {
        <button
            type="button"
            role="menuitem"
            class=move || {
                context.state.with(|state| {
                    menubar_trigger_class(
                        context.density,
                        state.is_open(value_for_class.as_str()),
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-part=MenubarPart::Trigger.label()
            data-ui-index=node.menu_index.to_string()
            data-ui-value=value_for_data
            aria-haspopup="menu"
            aria-expanded=move || context.state.with(|state| state.is_open(value_for_expanded.as_str()).to_string())
            aria-disabled=disabled.to_string()
            disabled=disabled
            title=detail
            on:click=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(MenubarIntent::Toggle(value_for_click.clone()));
                    });
                }
            }
            on:mouseenter=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(MenubarIntent::Open(value_for_enter.clone()));
                    });
                }
            }
        >
            {label}
        </button>
    }
    .into_any()
}

fn menubar_content_view(
    node: crate::MenubarRenderNode,
    items: Vec<crate::MenubarRenderNode>,
    context: MenubarViewContext,
) -> AnyView {
    let menu_value = node.value;
    let menu_value_for_class = menu_value.clone();
    let menu_value_for_data = menu_value.clone();
    let menu_value_for_hidden = menu_value.clone();
    let label = node.label;
    let detail = node.detail;
    view! {
        <div
            role="menu"
            class=move || {
                context.state.with(|state| {
                    menubar_content_class(context.density, state.is_open(menu_value_for_class.as_str()))
                        .to_owned()
                })
            }
            data-ui-part=MenubarPart::Content.label()
            data-ui-index=node.menu_index.to_string()
            data-ui-value=menu_value_for_data
            aria-label=label
            aria-hidden=move || {
                context.state.with(|state| (!state.is_open(menu_value_for_hidden.as_str())).to_string())
            }
            title=detail
        >
            {items
                .into_iter()
                .map(|item| menubar_item_view(item, context))
                .collect_view()}
        </div>
    }
    .into_any()
}

fn menubar_item_view(node: crate::MenubarRenderNode, context: MenubarViewContext) -> AnyView {
    let disabled = node.disabled || context.blocked;
    let value = node.value;
    let value_for_class = value.clone();
    let value_for_selected = value.clone();
    let value_for_data = value.clone();
    let value_for_focus = value.clone();
    let value_for_enter = value.clone();
    let value_for_click = value.clone();
    let label = node.label;
    let shortcut = node.shortcut;
    let menu_value = node.menu_value;
    view! {
        <button
            type="button"
            role="menuitem"
            class=move || {
                context.state.with(|state| {
                    menubar_item_class(
                        context.density,
                        state.is_focused(value_for_class.as_str()) || state.is_selected(value_for_class.as_str()),
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-part=MenubarPart::Item.label()
            data-ui-index=node.item_index.to_string()
            data-ui-menu=menu_value
            data-ui-value=value_for_data
            aria-selected=move || context.state.with(|state| state.is_selected(value_for_selected.as_str()).to_string())
            disabled=disabled
            on:focus=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(MenubarIntent::FocusItem(value_for_focus.clone()));
                    });
                }
            }
            on:mouseenter=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(MenubarIntent::FocusItem(value_for_enter.clone()));
                    });
                }
            }
            on:click=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(MenubarIntent::Activate(value_for_click.clone()));
                    });
                }
            }
        >
            <span>{label}</span>
            {if shortcut.is_empty() {
                ().into_any()
            } else {
                view! { <span class=MENUBAR_SHORTCUT>{shortcut}</span> }.into_any()
            }}
        </button>
    }
    .into_any()
}

const fn menubar_root_class(
    density: MenubarDensity,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return MENUBAR_ROOT_DISABLED;
    }
    if loading {
        return MENUBAR_ROOT_LOADING;
    }
    if invalid {
        return MENUBAR_ROOT_INVALID;
    }
    match density {
        MenubarDensity::Standard => MENUBAR_ROOT,
        MenubarDensity::Dense => MENUBAR_ROOT_DENSE,
    }
}

const fn menubar_menu_class(density: MenubarDensity) -> &'static str {
    match density {
        MenubarDensity::Standard => MENUBAR_MENU,
        MenubarDensity::Dense => MENUBAR_MENU_DENSE,
    }
}

const fn menubar_trigger_class(
    density: MenubarDensity,
    open: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return MENUBAR_TRIGGER_DISABLED;
    }
    if open {
        return MENUBAR_TRIGGER_OPEN;
    }
    match density {
        MenubarDensity::Standard => MENUBAR_TRIGGER,
        MenubarDensity::Dense => MENUBAR_TRIGGER_DENSE,
    }
}

const fn menubar_content_class(density: MenubarDensity, open: bool) -> &'static str {
    if !open {
        return MENUBAR_CONTENT_HIDDEN;
    }
    match density {
        MenubarDensity::Standard => MENUBAR_CONTENT,
        MenubarDensity::Dense => MENUBAR_CONTENT_DENSE,
    }
}

const fn menubar_item_class(density: MenubarDensity, active: bool, disabled: bool) -> &'static str {
    if disabled {
        return MENUBAR_ITEM_DISABLED;
    }
    if active {
        return MENUBAR_ITEM_ACTIVE;
    }
    match density {
        MenubarDensity::Standard => MENUBAR_ITEM,
        MenubarDensity::Dense => MENUBAR_ITEM_DENSE,
    }
}

const fn menubar_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    open: bool,
    focused: bool,
    selected: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if selected {
        "selected"
    } else if focused {
        "focused"
    } else if open {
        "open"
    } else {
        "ready"
    }
}

#[component]
pub fn Message(
    #[prop(optional, default = default_message_model())] model: MessageModel,
) -> AnyView {
    if let Err(report) = validate_message_model(&model) {
        let message = format!("Message validation failed: {report}");
        return view! {
            <div class=MESSAGE_ERROR data-ui-component="message" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let initial_state = model.state();
    let nodes = message_render_nodes(&model, &initial_state);
    let root = nodes
        .iter()
        .find(|node| node.part == MessagePart::Root)
        .expect("invariant: message render nodes include root")
        .clone();
    let header = nodes
        .iter()
        .find(|node| node.part == MessagePart::Header)
        .expect("invariant: message render nodes include header")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == MessagePart::Content)
        .expect("invariant: message render nodes include content")
        .clone();
    let footer = nodes
        .iter()
        .find(|node| node.part == MessagePart::Footer)
        .expect("invariant: message render nodes include footer")
        .clone();
    let action_nodes = nodes
        .into_iter()
        .filter(|node| node.part == MessagePart::Actions)
        .collect::<Vec<_>>();
    let actions_visible = action_nodes.iter().any(|node| node.visible);
    let density = root.density;
    let side = root.side;
    let invalid = root.invalid;
    let loading = root.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let root_value = root.value;
    let root_label = root.label;
    let root_detail = root.detail;
    let header_label = header.label;
    let header_meta = if loading {
        "Pending".to_owned()
    } else {
        header.value
    };
    let content_text = if loading {
        "Loading message".to_owned()
    } else {
        content.value
    };
    let footer_text = if invalid {
        footer.detail
    } else if loading {
        "Sending".to_owned()
    } else {
        footer.value
    };
    let (state, set_state) = signal(initial_state);
    let context = MessageViewContext {
        density,
        blocked,
        state,
        set_state,
    };

    view! {
        <article
            class=move || {
                state.with(|state| {
                    message_root_class(
                        density,
                        side,
                        state.is_active_part(MessagePart::Root) || state.active_action().is_some(),
                        invalid,
                        loading,
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-component="message"
            data-ui-part=MessagePart::Root.label()
            data-ui-density=density.label()
            data-ui-side=side.label()
            data-ui-state=move || {
                state.with(|state| {
                    message_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_active_part(MessagePart::Root),
                        state.active_action().is_some(),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root_value
            aria-label=root_label
            aria-busy=loading.to_string()
            aria-disabled=blocked.to_string()
            aria-invalid=invalid.to_string()
            title=root_detail
            on:mouseenter=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(MessageIntent::Hover(MessagePart::Root));
                    });
                }
            }
            on:mouseleave=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(MessageIntent::Leave);
                    });
                }
            }
        >
            <header
                class=message_header_class(density)
                data-ui-part=MessagePart::Header.label()
            >
                <p class=message_sender_class(disabled)>{header_label}</p>
                <p class=message_meta_class(disabled)>{header_meta}</p>
            </header>
            <p
                class=message_content_class(density, disabled)
                data-ui-part=MessagePart::Content.label()
            >
                {content_text}
            </p>
            <footer
                class=message_footer_class(density)
                data-ui-part=MessagePart::Footer.label()
            >
                <p class=message_status_class(invalid, disabled)>{footer_text}</p>
                <div
                    class=message_actions_class(actions_visible)
                    data-ui-part=MessagePart::Actions.label()
                    aria-hidden=(!actions_visible).to_string()
                >
                    {action_nodes
                        .into_iter()
                        .filter(|node| node.visible)
                        .map(|node| message_action_view(node, context))
                        .collect_view()}
                </div>
            </footer>
        </article>
    }
    .into_any()
}

#[derive(Clone, Copy)]
struct MessageViewContext {
    density: MessageDensity,
    blocked: bool,
    state: ReadSignal<MessageState>,
    set_state: WriteSignal<MessageState>,
}

fn message_action_view(node: crate::MessageRenderNode, context: MessageViewContext) -> AnyView {
    let disabled = node.disabled || context.blocked;
    let value = node.value;
    let value_for_class = value.clone();
    let value_for_data = value.clone();
    let value_for_click = value.clone();
    let label = node.label;
    view! {
        <button
            type="button"
            class=move || {
                context.state.with(|state| {
                    message_action_class(
                        context.density,
                        state.is_active_action(value_for_class.as_str()),
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-part=MessagePart::Actions.label()
            data-ui-index=node.index.to_string()
            data-ui-action=value_for_data
            data-ui-intent="activate"
            disabled=disabled
            on:focus=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(MessageIntent::Focus(MessagePart::Actions));
                    });
                }
            }
            on:blur=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(MessageIntent::Blur);
                    });
                }
            }
            on:mouseenter=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(MessageIntent::Hover(MessagePart::Actions));
                    });
                }
            }
            on:click=move |_| {
                if !disabled {
                    context.set_state.update(|state| {
                        let _ = state.apply(MessageIntent::Activate(value_for_click.clone()));
                    });
                }
            }
        >
            {label}
        </button>
    }
    .into_any()
}

const fn message_root_class(
    density: MessageDensity,
    side: MessageSide,
    active: bool,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return MESSAGE_ROOT_DISABLED;
    }
    if loading {
        return MESSAGE_ROOT_LOADING;
    }
    if invalid {
        return MESSAGE_ROOT_INVALID;
    }
    if active {
        return MESSAGE_ROOT_ACTIVE;
    }
    match (density, side) {
        (MessageDensity::Standard, MessageSide::Incoming) => MESSAGE_ROOT_INCOMING,
        (MessageDensity::Standard, MessageSide::Outgoing) => MESSAGE_ROOT_OUTGOING,
        (MessageDensity::Standard, MessageSide::System) => MESSAGE_ROOT_SYSTEM,
        (MessageDensity::Dense, MessageSide::Incoming) => MESSAGE_ROOT_DENSE_INCOMING,
        (MessageDensity::Dense, MessageSide::Outgoing) => MESSAGE_ROOT_DENSE_OUTGOING,
        (MessageDensity::Dense, MessageSide::System) => MESSAGE_ROOT_DENSE_SYSTEM,
    }
}

const fn message_header_class(density: MessageDensity) -> &'static str {
    match density {
        MessageDensity::Standard => MESSAGE_HEADER,
        MessageDensity::Dense => MESSAGE_HEADER_DENSE,
    }
}

const fn message_sender_class(disabled: bool) -> &'static str {
    if disabled {
        MESSAGE_SENDER_DISABLED
    } else {
        MESSAGE_SENDER
    }
}

const fn message_meta_class(disabled: bool) -> &'static str {
    if disabled {
        MESSAGE_META_DISABLED
    } else {
        MESSAGE_META
    }
}

const fn message_content_class(density: MessageDensity, disabled: bool) -> &'static str {
    if disabled {
        return MESSAGE_CONTENT_DISABLED;
    }
    match density {
        MessageDensity::Standard => MESSAGE_CONTENT,
        MessageDensity::Dense => MESSAGE_CONTENT_DENSE,
    }
}

const fn message_footer_class(density: MessageDensity) -> &'static str {
    match density {
        MessageDensity::Standard => MESSAGE_FOOTER,
        MessageDensity::Dense => MESSAGE_FOOTER_DENSE,
    }
}

const fn message_status_class(invalid: bool, disabled: bool) -> &'static str {
    if disabled {
        MESSAGE_STATUS_DISABLED
    } else if invalid {
        MESSAGE_STATUS_INVALID
    } else {
        MESSAGE_STATUS
    }
}

const fn message_actions_class(visible: bool) -> &'static str {
    if visible {
        MESSAGE_ACTIONS
    } else {
        MESSAGE_ACTIONS_HIDDEN
    }
}

const fn message_action_class(
    density: MessageDensity,
    active: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return MESSAGE_ACTION_DISABLED;
    }
    if active {
        return MESSAGE_ACTION_ACTIVE;
    }
    match density {
        MessageDensity::Standard => MESSAGE_ACTION,
        MessageDensity::Dense => MESSAGE_ACTION_DENSE,
    }
}

const fn message_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    active: bool,
    action_active: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if action_active {
        "action-active"
    } else if active {
        "active"
    } else {
        "ready"
    }
}

#[component]
pub fn MessageScroller(
    #[prop(optional, default = default_message_scroller_model())] model: MessageScrollerModel,
) -> AnyView {
    if let Err(report) = validate_message_scroller_model(&model) {
        let message = format!("MessageScroller validation failed: {report}");
        return view! {
            <div class=MESSAGE_SCROLLER_ERROR data-ui-component="message-scroller" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let initial_state = model.state();
    let nodes = message_scroller_render_nodes(&model, &initial_state);
    let root = nodes
        .iter()
        .find(|node| node.part == MessageScrollerPart::Root)
        .expect("invariant: message scroller render nodes include root")
        .clone();
    let viewport = nodes
        .iter()
        .find(|node| node.part == MessageScrollerPart::Viewport)
        .expect("invariant: message scroller render nodes include viewport")
        .clone();
    let list = nodes
        .iter()
        .find(|node| node.part == MessageScrollerPart::List)
        .expect("invariant: message scroller render nodes include list")
        .clone();
    let anchor = nodes
        .iter()
        .find(|node| node.part == MessageScrollerPart::Anchor)
        .expect("invariant: message scroller render nodes include anchor")
        .clone();
    let jump = nodes
        .into_iter()
        .find(|node| node.part == MessageScrollerPart::JumpButton)
        .expect("invariant: message scroller render nodes include jump button");
    let density = root.density;
    let invalid = root.invalid;
    let loading = root.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let entries = model.messages;
    let root_value = root.value;
    let root_label = root.label;
    let root_detail = root.detail;
    let viewport_detail = viewport.detail;
    let list_label = if loading {
        "Loading messages".to_owned()
    } else {
        list.label
    };
    let list_count = list.message_count;
    let anchor_label = anchor.label;
    let jump_label = jump.label;
    let (state, set_state) = signal(initial_state);
    let context = MessageScrollerViewContext {
        density,
        blocked,
        state,
        set_state,
    };

    view! {
        <section
            class=move || {
                state.with(|state| {
                    message_scroller_root_class(
                        density,
                        state.is_active_part(MessageScrollerPart::Root) || state.jump_active(),
                        invalid,
                        loading,
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-component="message-scroller"
            data-ui-part=MessageScrollerPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    message_scroller_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.at_latest(),
                        state.jump_active(),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root_value
            aria-label=root_label
            aria-busy=loading.to_string()
            aria-disabled=blocked.to_string()
            aria-invalid=invalid.to_string()
            title=root_detail
            on:mouseenter=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(MessageScrollerIntent::Hover(MessageScrollerPart::Root));
                    });
                }
            }
            on:mouseleave=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(MessageScrollerIntent::Leave);
                    });
                }
            }
        >
            <div
                class=message_scroller_viewport_class(density)
                data-ui-part=MessageScrollerPart::Viewport.label()
                title=viewport_detail
                on:scroll=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(MessageScrollerIntent::SetAtLatest(false));
                        });
                    }
                }
            >
                <div
                    class=message_scroller_list_class(density)
                    data-ui-part=MessageScrollerPart::List.label()
                    data-ui-count=list_count.to_string()
                    aria-label=list_label.clone()
                >
                    {if entries.is_empty() {
                        view! {
                            <p class=MESSAGE_SCROLLER_EMPTY>{list_label.clone()}</p>
                        }
                        .into_any()
                    } else {
                        entries
                            .into_iter()
                            .map(message_scroller_entry_view)
                            .collect_view()
                            .into_any()
                    }}
                    <span
                        class=move || {
                            state.with(|state| message_scroller_anchor_class(state.at_latest()).to_owned())
                        }
                        data-ui-part=MessageScrollerPart::Anchor.label()
                        aria-label=anchor_label
                    ></span>
                </div>
            </div>
            {message_scroller_jump_view(jump_label, context)}
        </section>
    }
    .into_any()
}

#[derive(Clone, Copy)]
struct MessageScrollerViewContext {
    density: MessageScrollerDensity,
    blocked: bool,
    state: ReadSignal<MessageScrollerState>,
    set_state: WriteSignal<MessageScrollerState>,
}

fn message_scroller_entry_view(entry: MessageScrollerEntry) -> AnyView {
    view! {
        <div data-ui-entry=entry.value>
            <Message model=entry.message />
        </div>
    }
    .into_any()
}

fn message_scroller_jump_view(label: String, context: MessageScrollerViewContext) -> AnyView {
    view! {
        <button
            type="button"
            class=move || {
                context.state.with(|state| {
                    message_scroller_jump_class(
                        context.density,
                        state.jump_active(),
                        !state.at_latest(),
                        context.blocked || state.at_latest(),
                    )
                    .to_owned()
                })
            }
            data-ui-part=MessageScrollerPart::JumpButton.label()
            data-ui-intent="jump-to-latest"
            aria-hidden=move || context.state.with(|state| state.at_latest().to_string())
            disabled=move || context.state.with(|state| context.blocked || state.at_latest())
            on:focus=move |_| {
                if !context.blocked {
                    context.set_state.update(|state| {
                        let _ = state.apply(MessageScrollerIntent::Focus(MessageScrollerPart::JumpButton));
                    });
                }
            }
            on:blur=move |_| {
                if !context.blocked {
                    context.set_state.update(|state| {
                        let _ = state.apply(MessageScrollerIntent::Blur);
                    });
                }
            }
            on:mouseenter=move |_| {
                if !context.blocked {
                    context.set_state.update(|state| {
                        let _ = state.apply(MessageScrollerIntent::Hover(MessageScrollerPart::JumpButton));
                    });
                }
            }
            on:click=move |_| {
                if !context.blocked {
                    context.set_state.update(|state| {
                        let _ = state.apply(MessageScrollerIntent::JumpToLatest);
                    });
                }
            }
        >
            {label}
        </button>
    }
    .into_any()
}

const fn message_scroller_root_class(
    density: MessageScrollerDensity,
    active: bool,
    invalid: bool,
    loading: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return MESSAGE_SCROLLER_ROOT_DISABLED;
    }
    if loading {
        return MESSAGE_SCROLLER_ROOT_LOADING;
    }
    if invalid {
        return MESSAGE_SCROLLER_ROOT_INVALID;
    }
    if active {
        return MESSAGE_SCROLLER_ROOT;
    }
    match density {
        MessageScrollerDensity::Standard => MESSAGE_SCROLLER_ROOT,
        MessageScrollerDensity::Dense => MESSAGE_SCROLLER_ROOT_DENSE,
    }
}

const fn message_scroller_viewport_class(density: MessageScrollerDensity) -> &'static str {
    match density {
        MessageScrollerDensity::Standard => MESSAGE_SCROLLER_VIEWPORT,
        MessageScrollerDensity::Dense => MESSAGE_SCROLLER_VIEWPORT_DENSE,
    }
}

const fn message_scroller_list_class(density: MessageScrollerDensity) -> &'static str {
    match density {
        MessageScrollerDensity::Standard => MESSAGE_SCROLLER_LIST,
        MessageScrollerDensity::Dense => MESSAGE_SCROLLER_LIST_DENSE,
    }
}

const fn message_scroller_anchor_class(at_latest: bool) -> &'static str {
    if at_latest {
        MESSAGE_SCROLLER_ANCHOR_ACTIVE
    } else {
        MESSAGE_SCROLLER_ANCHOR
    }
}

const fn message_scroller_jump_class(
    density: MessageScrollerDensity,
    active: bool,
    visible: bool,
    disabled: bool,
) -> &'static str {
    if !visible {
        return MESSAGE_SCROLLER_JUMP_HIDDEN;
    }
    if disabled {
        return MESSAGE_SCROLLER_JUMP_DISABLED;
    }
    if active {
        return MESSAGE_SCROLLER_JUMP_ACTIVE;
    }
    match density {
        MessageScrollerDensity::Standard => MESSAGE_SCROLLER_JUMP,
        MessageScrollerDensity::Dense => MESSAGE_SCROLLER_JUMP_DENSE,
    }
}

const fn message_scroller_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    at_latest: bool,
    jump_active: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if jump_active {
        "jumped"
    } else if at_latest {
        "latest"
    } else {
        "history"
    }
}

#[component]
pub fn Popover(
    #[prop(optional, default = default_popover_model())] model: PopoverModel,
) -> AnyView {
    if let Err(report) = validate_popover_model(&model) {
        let message = format!("Popover validation failed: {report}");
        return view! {
            <div class=POPOVER_CONTENT_INVALID data-ui-component="popover" data-ui-state="invalid" role="alert">
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
    let error_message = model.error.clone().unwrap_or_default();
    let state_model = model.state();
    let nodes = popover_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == PopoverPart::Root)
        .expect("invariant: popover render nodes include root")
        .clone();
    let trigger = nodes
        .iter()
        .find(|node| node.part == PopoverPart::Trigger)
        .expect("invariant: popover render nodes include trigger")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == PopoverPart::Content)
        .expect("invariant: popover render nodes include content")
        .clone();
    let arrow = nodes
        .iter()
        .find(|node| node.part == PopoverPart::Arrow)
        .expect("invariant: popover render nodes include arrow")
        .clone();
    let (state, set_state) = signal(state_model);
    let trigger_label = if loading {
        "Loading controls".to_owned()
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
        "Preparing popover content.".to_owned()
    } else {
        content.detail.clone()
    };
    let arrow_value = arrow.value.clone();
    let arrow_label = arrow.label.clone();
    let arrow_detail = arrow.detail.clone();

    view! {
        <section
            class=popover_root_class(disabled)
            data-ui-component="popover"
            data-ui-part=PopoverPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    popover_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_open(),
                        state.is_active(PopoverPart::Trigger),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
        >
            <button
                type="button"
                class=move || {
                    state.with(|state| {
                        popover_trigger_class(density, state.is_open(), invalid, blocked)
                            .to_owned()
                    })
                }
                data-ui-part=PopoverPart::Trigger.label()
                data-ui-value=trigger.value
                aria-haspopup="dialog"
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(PopoverIntent::Focus(PopoverPart::Trigger));
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(PopoverIntent::ClearFocus);
                        });
                    }
                }
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(PopoverIntent::Toggle);
                        });
                    }
                }
            >
                {trigger_label}
            </button>
            <article
                role="dialog"
                tabindex="-1"
                class=move || {
                    state.with(|state| {
                        popover_content_class(
                            density,
                            state.is_open(),
                            loading,
                            disabled,
                            invalid,
                        )
                        .to_owned()
                    })
                }
                data-ui-part=PopoverPart::Content.label()
                data-ui-value=content_value
                aria-hidden=move || state.with(|state| (!state.is_open()).to_string())
                aria-invalid=invalid.to_string()
                hidden=move || state.with(|state| !state.is_open())
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(PopoverIntent::Focus(PopoverPart::Content));
                        });
                    }
                }
            >
                <span
                    class=move || state.with(|state| popover_arrow_class(state.is_open()).to_owned())
                    data-ui-part=PopoverPart::Arrow.label()
                    data-ui-value=arrow_value
                    aria-label=arrow_detail
                >
                    {arrow_label}
                </span>
                <p class=POPOVER_META>{content.value}</p>
                <h3 class=popover_title_class(density)>{content_label}</h3>
                <p class=popover_detail_class(density)>{content_detail}</p>
                <p class=popover_error_class(invalid)>{error_message}</p>
            </article>
        </section>
    }
    .into_any()
}

const fn popover_root_class(disabled: bool) -> &'static str {
    if disabled {
        POPOVER_ROOT_DISABLED
    } else {
        POPOVER_ROOT
    }
}

const fn popover_trigger_class(
    density: PopoverDensity,
    open: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return POPOVER_TRIGGER_DISABLED;
    }
    if invalid {
        return POPOVER_TRIGGER_INVALID;
    }
    match (density, open) {
        (PopoverDensity::Standard, true) => POPOVER_TRIGGER_OPEN,
        (PopoverDensity::Dense, true) => POPOVER_TRIGGER_DENSE_OPEN,
        (PopoverDensity::Standard, false) => POPOVER_TRIGGER,
        (PopoverDensity::Dense, false) => POPOVER_TRIGGER_DENSE,
    }
}

const fn popover_content_class(
    density: PopoverDensity,
    open: bool,
    loading: bool,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if !open {
        return POPOVER_CONTENT_HIDDEN;
    }
    if disabled {
        return POPOVER_CONTENT_DISABLED;
    }
    if loading {
        return POPOVER_CONTENT_LOADING;
    }
    if invalid {
        return POPOVER_CONTENT_INVALID;
    }
    match density {
        PopoverDensity::Standard => POPOVER_CONTENT,
        PopoverDensity::Dense => POPOVER_CONTENT_DENSE,
    }
}

const fn popover_title_class(density: PopoverDensity) -> &'static str {
    match density {
        PopoverDensity::Standard => POPOVER_TITLE,
        PopoverDensity::Dense => POPOVER_TITLE_DENSE,
    }
}

const fn popover_detail_class(density: PopoverDensity) -> &'static str {
    match density {
        PopoverDensity::Standard => POPOVER_DETAIL,
        PopoverDensity::Dense => POPOVER_DETAIL_DENSE,
    }
}

const fn popover_arrow_class(open: bool) -> &'static str {
    if open {
        POPOVER_ARROW
    } else {
        POPOVER_ARROW_HIDDEN
    }
}

const fn popover_error_class(visible: bool) -> &'static str {
    if visible {
        POPOVER_ERROR
    } else {
        POPOVER_ERROR_HIDDEN
    }
}

const fn popover_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    open: bool,
    trigger_active: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if open {
        "open"
    } else if trigger_active {
        "focused"
    } else {
        "closed"
    }
}
#[component]
pub fn Progress(
    #[prop(optional, default = default_progress_model())] model: ProgressModel,
) -> AnyView {
    if let Err(report) = validate_progress_model(&model) {
        let message = format!("Progress validation failed: {report}");
        return view! {
            <div class=PROGRESS_ROOT_INVALID data-ui-component="progress" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let state_model = model.state();
    let nodes = progress_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == ProgressPart::Root)
        .expect("invariant: progress render nodes include root")
        .clone();
    let track = nodes
        .iter()
        .find(|node| node.part == ProgressPart::Track)
        .expect("invariant: progress render nodes include track")
        .clone();
    let indicator = nodes
        .iter()
        .find(|node| node.part == ProgressPart::Indicator)
        .expect("invariant: progress render nodes include indicator")
        .clone();
    let label = nodes
        .iter()
        .find(|node| node.part == ProgressPart::Label)
        .expect("invariant: progress render nodes include label")
        .clone();
    let (state, set_state) = signal(state_model);
    let percent = indicator.percent;
    let determinate = indicator.determinate;
    let value_label = indicator.value.clone();
    let label_text = if loading {
        "Loading".to_owned()
    } else {
        label.label.clone()
    };
    let detail = label.detail.clone();
    let aria_value_now = if determinate {
        percent.to_string()
    } else {
        String::new()
    };

    view! {
        <section
            class=progress_root_class(density, invalid, disabled)
            data-ui-component="progress"
            data-ui-part=ProgressPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    progress_state_label(
                        loading,
                        disabled,
                        invalid,
                        determinate,
                        state.is_highlighted(),
                    )
                    .to_owned()
                })
            }
            data-ui-value=root.value
            aria-disabled=disabled.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
            on:mouseenter=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(ProgressIntent::Hover);
                    });
                }
            }
            on:mouseleave=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(ProgressIntent::Leave);
                    });
                }
            }
        >
            <div
                role="progressbar"
                tabindex="0"
                class=progress_track_class(density, loading, invalid, disabled)
                data-ui-part=ProgressPart::Track.label()
                data-ui-value=track.value
                aria-label=label_text.clone()
                aria-valuemin="0"
                aria-valuemax="100"
                aria-valuenow=aria_value_now
                aria-valuetext=value_label.clone()
                on:focus=move |_| {
                    if !disabled {
                        set_state.update(|state| {
                            let _ = state.apply(ProgressIntent::Focus);
                        });
                    }
                }
                on:blur=move |_| {
                    if !disabled {
                        set_state.update(|state| {
                            let _ = state.apply(ProgressIntent::Blur);
                        });
                    }
                }
            >
                <span
                    class=move || {
                        state.with(|state| {
                            progress_indicator_class(
                                loading,
                                invalid,
                                disabled,
                                state.is_highlighted(),
                            )
                            .to_owned()
                        })
                    }
                    style=progress_indicator_style(percent, determinate)
                    data-ui-part=ProgressPart::Indicator.label()
                    data-ui-value=indicator.value
                ></span>
            </div>
            <div class=PROGRESS_LABEL_ROW data-ui-part=ProgressPart::Label.label() data-ui-value=label.value>
                <p class=progress_label_class(density)>{label_text}</p>
                <p class=progress_value_class(density)>{value_label}</p>
            </div>
            <p class=progress_detail_class(density, invalid, disabled)>{detail}</p>
        </section>
    }
    .into_any()
}

const fn progress_root_class(
    density: ProgressDensity,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return PROGRESS_ROOT_DISABLED;
    }
    if invalid {
        return PROGRESS_ROOT_INVALID;
    }
    match density {
        ProgressDensity::Standard => PROGRESS_ROOT,
        ProgressDensity::Dense => PROGRESS_ROOT_DENSE,
    }
}

const fn progress_track_class(
    density: ProgressDensity,
    loading: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return PROGRESS_TRACK_DISABLED;
    }
    if invalid {
        return PROGRESS_TRACK_INVALID;
    }
    if loading {
        return PROGRESS_TRACK_LOADING;
    }
    match density {
        ProgressDensity::Standard => PROGRESS_TRACK,
        ProgressDensity::Dense => PROGRESS_TRACK_DENSE,
    }
}

const fn progress_indicator_class(
    loading: bool,
    invalid: bool,
    disabled: bool,
    highlighted: bool,
) -> &'static str {
    if disabled {
        return PROGRESS_INDICATOR_DISABLED;
    }
    if invalid {
        return PROGRESS_INDICATOR_INVALID;
    }
    if loading {
        return PROGRESS_INDICATOR_LOADING;
    }
    if highlighted {
        return PROGRESS_INDICATOR_HIGHLIGHTED;
    }
    PROGRESS_INDICATOR
}

fn progress_indicator_style(percent: u8, determinate: bool) -> String {
    if determinate {
        format!("inline-size: {percent}%;")
    } else {
        "inline-size: 100%;".to_owned()
    }
}

const fn progress_label_class(density: ProgressDensity) -> &'static str {
    match density {
        ProgressDensity::Standard => PROGRESS_LABEL,
        ProgressDensity::Dense => PROGRESS_LABEL_DENSE,
    }
}

const fn progress_value_class(density: ProgressDensity) -> &'static str {
    match density {
        ProgressDensity::Standard => PROGRESS_VALUE,
        ProgressDensity::Dense => PROGRESS_VALUE_DENSE,
    }
}

const fn progress_detail_class(
    density: ProgressDensity,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return PROGRESS_DETAIL_DISABLED;
    }
    if invalid {
        return PROGRESS_DETAIL_INVALID;
    }
    match density {
        ProgressDensity::Standard => PROGRESS_DETAIL,
        ProgressDensity::Dense => PROGRESS_DETAIL_DENSE,
    }
}

const fn progress_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    determinate: bool,
    highlighted: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if highlighted {
        "highlighted"
    } else if determinate {
        "determinate"
    } else {
        "indeterminate"
    }
}
#[component]
pub fn RadioGroup(
    #[prop(optional, default = default_radio_group_model())] model: RadioGroupModel,
) -> AnyView {
    if let Err(report) = validate_radio_group_model(&model) {
        let message = format!("RadioGroup validation failed: {report}");
        return view! {
            <div class=RADIO_GROUP_ROOT_INVALID data-ui-component="radio-group" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let orientation = model.orientation;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let blocked = loading || disabled;
    let required = model.required;
    let state_model = model.state();
    let nodes = radio_group_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == RadioGroupPart::Root)
        .expect("invariant: radio group render nodes include root")
        .clone();
    let root_value = root.value.clone();
    let root_label = root.label.clone();
    let root_aria_label = root.label.clone();
    let root_detail = root.detail.clone();
    let options = model.options.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=radio_group_root_class(density, invalid, disabled)
            data-ui-component="radio-group"
            data-ui-part=RadioGroupPart::Root.label()
            data-ui-density=density.label()
            data-ui-orientation=orientation.label()
            data-ui-state=move || {
                state.with(|state| {
                    radio_group_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.selected_value(),
                        state.focused_value(),
                    )
                })
            }
            data-ui-value=root_value
            role="radiogroup"
            aria-label=root_aria_label
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
            aria-required=required.to_string()
        >
            <div class=RADIO_GROUP_TITLE_ROW>
                <p class=radio_group_title_class(density)>
                    {root_label}
                    {required.then_some(view! { <span class=CHECKBOX_REQUIRED>" *"</span> })}
                </p>
                <p class=RADIO_GROUP_STATUS>{root_detail}</p>
            </div>
            <div class=radio_group_list_class(density, orientation)>
                {options
                    .into_iter()
                    .enumerate()
                    .map(|(index, option)| {
                        let value = option.value.clone();
                        let value_for_item = value.clone();
                        let value_for_item_data = value.clone();
                        let value_for_control = value.clone();
                        let value_for_control_data = value.clone();
                        let value_for_checked = value.clone();
                        let value_for_focus = value.clone();
                        let value_for_select = value.clone();
                        let value_for_dot = value.clone();
                        let option_disabled = blocked || option.disabled;
                        view! {
                            <div
                                class=move || {
                                    state.with(|state| {
                                        radio_group_item_class(
                                            density,
                                            state.is_selected(&value_for_item),
                                            state.is_focused(&value_for_item),
                                            option_disabled,
                                        )
                                        .to_owned()
                                    })
                                }
                                data-ui-part=RadioGroupPart::Item.label()
                                data-ui-value=value_for_item_data
                                data-ui-index=index.to_string()
                            >
                                <button
                                    type="button"
                                    role="radio"
                                    class=move || {
                                        state.with(|state| {
                                            radio_group_control_class(
                                                state.is_selected(&value_for_control),
                                                option_disabled,
                                                invalid,
                                            )
                                            .to_owned()
                                        })
                                    }
                                    data-ui-part=RadioGroupPart::Indicator.label()
                                    data-ui-value=value_for_control_data
                                    aria-label=option.label.clone()
                                    aria-checked=move || {
                                        state.with(|state| state.is_selected(&value_for_checked).to_string())
                                    }
                                    disabled=option_disabled
                                    on:focus=move |_| {
                                        if !option_disabled {
                                            let value = value_for_focus.clone();
                                            set_state.update(|state| {
                                                let _ = state.apply(RadioGroupIntent::Focus(value));
                                            });
                                        }
                                    }
                                    on:blur=move |_| {
                                        if !option_disabled {
                                            set_state.update(|state| {
                                                let _ = state.apply(RadioGroupIntent::Blur);
                                            });
                                        }
                                    }
                                    on:click=move |_| {
                                        if !option_disabled {
                                            let value = value_for_select.clone();
                                            set_state.update(|state| {
                                                let _ = state.apply(RadioGroupIntent::Select(value));
                                            });
                                        }
                                    }
                                >
                                    <span
                                        class=move || {
                                            state.with(|state| {
                                                radio_group_dot_class(state.is_selected(&value_for_dot)).to_owned()
                                            })
                                        }
                                        data-ui-part=RadioGroupPart::Indicator.label()
                                    ></span>
                                </button>
                                <div class=RADIO_GROUP_TEXT data-ui-part=RadioGroupPart::Label.label()>
                                    <p class=radio_group_label_class(density, option_disabled)>{option.label}</p>
                                    <p class=radio_group_detail_class(invalid, option_disabled)>{option.detail}</p>
                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
    .into_any()
}

const fn radio_group_root_class(
    density: RadioGroupDensity,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return RADIO_GROUP_ROOT_DISABLED;
    }
    if invalid {
        return RADIO_GROUP_ROOT_INVALID;
    }
    match density {
        RadioGroupDensity::Standard => RADIO_GROUP_ROOT,
        RadioGroupDensity::Dense => RADIO_GROUP_ROOT_DENSE,
    }
}

const fn radio_group_title_class(density: RadioGroupDensity) -> &'static str {
    match density {
        RadioGroupDensity::Standard => RADIO_GROUP_TITLE,
        RadioGroupDensity::Dense => RADIO_GROUP_TITLE_DENSE,
    }
}

const fn radio_group_list_class(
    density: RadioGroupDensity,
    orientation: RadioGroupOrientation,
) -> &'static str {
    match (density, orientation) {
        (_, RadioGroupOrientation::Horizontal) => RADIO_GROUP_LIST_HORIZONTAL,
        (RadioGroupDensity::Standard, RadioGroupOrientation::Vertical) => RADIO_GROUP_LIST,
        (RadioGroupDensity::Dense, RadioGroupOrientation::Vertical) => RADIO_GROUP_LIST_DENSE,
    }
}

const fn radio_group_item_class(
    density: RadioGroupDensity,
    selected: bool,
    focused: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return RADIO_GROUP_ITEM_DISABLED;
    }
    if selected {
        return RADIO_GROUP_ITEM_SELECTED;
    }
    if focused {
        return RADIO_GROUP_ITEM_FOCUSED;
    }
    match density {
        RadioGroupDensity::Standard => RADIO_GROUP_ITEM,
        RadioGroupDensity::Dense => RADIO_GROUP_ITEM_DENSE,
    }
}

const fn radio_group_control_class(selected: bool, disabled: bool, invalid: bool) -> &'static str {
    if disabled {
        return RADIO_GROUP_CONTROL_DISABLED;
    }
    if invalid {
        return RADIO_GROUP_CONTROL_INVALID;
    }
    if selected {
        RADIO_GROUP_CONTROL_SELECTED
    } else {
        RADIO_GROUP_CONTROL
    }
}

const fn radio_group_dot_class(selected: bool) -> &'static str {
    if selected {
        RADIO_GROUP_DOT
    } else {
        RADIO_GROUP_DOT_EMPTY
    }
}

const fn radio_group_label_class(density: RadioGroupDensity, disabled: bool) -> &'static str {
    if disabled {
        return RADIO_GROUP_LABEL_DISABLED;
    }
    match density {
        RadioGroupDensity::Standard => RADIO_GROUP_LABEL,
        RadioGroupDensity::Dense => RADIO_GROUP_LABEL_DENSE,
    }
}

const fn radio_group_detail_class(invalid: bool, disabled: bool) -> &'static str {
    if disabled {
        RADIO_GROUP_DETAIL_DISABLED
    } else if invalid {
        RADIO_GROUP_DETAIL_INVALID
    } else {
        RADIO_GROUP_DETAIL
    }
}

fn radio_group_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    selected_value: Option<&str>,
    focused_value: Option<&str>,
) -> String {
    if disabled {
        "disabled".to_owned()
    } else if loading {
        "loading".to_owned()
    } else if invalid {
        "invalid".to_owned()
    } else if let Some(value) = focused_value {
        format!("focused-{value}")
    } else if let Some(value) = selected_value {
        format!("selected-{value}")
    } else {
        "unselected".to_owned()
    }
}

#[component]
pub fn Resizable(
    #[prop(optional, default = default_resizable_model())] model: ResizableModel,
) -> AnyView {
    if let Err(report) = validate_resizable_model(&model) {
        let message = format!("Resizable validation failed: {report}");
        return view! {
            <div class=RESIZABLE_ROOT_INVALID data-ui-component="resizable" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let orientation = model.orientation;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = resizable_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == ResizablePart::PanelGroup)
        .expect("invariant: resizable render nodes include panel group")
        .clone();
    let root_value = root.value.clone();
    let root_label = root.label.clone();
    let root_aria_label = root.label.clone();
    let root_detail = root.detail.clone();
    let panels = model.panels.clone();
    let panel_count = panels.len();
    let handle_labels = panels
        .windows(2)
        .map(|pair| format!("Resize {} and {}", pair[0].title, pair[1].title))
        .collect::<Vec<_>>();
    let handle_disableds = panels
        .windows(2)
        .map(|pair| blocked || pair[0].disabled || pair[1].disabled)
        .collect::<Vec<_>>();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=resizable_root_class(density, invalid, disabled)
            data-ui-component="resizable"
            data-ui-part=ResizablePart::PanelGroup.label()
            data-ui-density=density.label()
            data-ui-orientation=orientation.label()
            data-ui-state=move || {
                state.with(|state| {
                    resizable_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.active_panel(),
                        state.resizing_handle(),
                    )
                })
            }
            data-ui-value=root_value
            aria-label=root_aria_label
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
        >
            <div class=RESIZABLE_HEADER>
                <p class=resizable_title_class(density)>{root_label}</p>
                <p class=resizable_status_class(invalid, disabled)>
                    {move || {
                        state.with(|state| {
                            if invalid {
                                root_detail.clone()
                            } else {
                                format!("{} ({})", root_detail, resizable_sizes_label(state.panel_sizes()))
                            }
                        })
                    }}
                </p>
            </div>
            <div
                class=resizable_group_class(density, orientation)
                data-ui-part=ResizablePart::PanelGroup.label()
                role="group"
                aria-orientation=orientation.label()
            >
                {panels
                    .into_iter()
                    .enumerate()
                    .map(|(index, panel)| {
                        let panel_title = panel.title.clone();
                        let panel_detail = panel.detail.clone();
                        let panel_value_for_class = panel.value.clone();
                        let panel_value_for_focus = panel.value.clone();
                        let panel_value_for_data = panel.value.clone();
                        let panel_disabled = blocked || panel.disabled;
                        let panel_tab_index = if panel_disabled { "-1" } else { "0" };
                        let initial_size = panel.size_percent;
                        let has_handle = index + 1 < panel_count;
                        let handle_label = handle_labels.get(index).cloned().unwrap_or_default();
                        let handle_disabled = handle_disableds.get(index).copied().unwrap_or(true);
                        let handle_value = format!("handle-{index}");
                        let handle_value_for_input = handle_value.clone();
                        let handle_label_for_display = handle_label.clone();
                        let handle_label_for_input = handle_label.clone();
                        let handle_view = has_handle.then_some(view! {
                            <label
                                class=move || {
                                    state.with(|state| {
                                        resizable_handle_class(
                                            density,
                                            state.is_resizing_handle(index),
                                            handle_disabled,
                                            invalid,
                                        )
                                        .to_owned()
                                    })
                                }
                                data-ui-part=ResizablePart::Handle.label()
                                data-ui-value=handle_value
                            >
                                <span
                                    class=move || {
                                        state.with(|state| {
                                            resizable_handle_label_class(
                                                state.is_resizing_handle(index),
                                                handle_disabled,
                                            )
                                            .to_owned()
                                        })
                                    }
                                >
                                    {move || {
                                        state.with(|state| {
                                            format!(
                                                "{} ({}%)",
                                                handle_label_for_display,
                                                state.panel_size(index).unwrap_or(initial_size),
                                            )
                                        })
                                    }}
                                </span>
                                <input
                                    type="range"
                                    class=RESIZABLE_RANGE
                                    data-ui-part=ResizablePart::Handle.label()
                                    data-ui-value=handle_value_for_input
                                    min=move || {
                                        state.with(|state| {
                                            state
                                                .handle_range(index)
                                                .map(|(min, _)| min)
                                                .unwrap_or(panel.min_percent)
                                                .to_string()
                                        })
                                    }
                                    max=move || {
                                        state.with(|state| {
                                            state
                                                .handle_range(index)
                                                .map(|(_, max)| max)
                                                .unwrap_or(panel.max_percent)
                                                .to_string()
                                        })
                                    }
                                    prop:value=move || {
                                        state.with(|state| {
                                            state.panel_size(index).unwrap_or(initial_size).to_string()
                                        })
                                    }
                                    aria-label=handle_label_for_input
                                    aria-valuetext=move || {
                                        state.with(|state| resizable_sizes_label(state.panel_sizes()))
                                    }
                                    disabled=handle_disabled
                                    on:focus=move |_| {
                                        if !handle_disabled {
                                            set_state.update(|state| {
                                                let _ = state.apply(ResizableIntent::StartResize(index));
                                            });
                                        }
                                    }
                                    on:input=move |event| {
                                        if !handle_disabled
                                            && let Ok(value) = event_target_value(&event).parse::<u8>()
                                        {
                                            set_state.update(|state| {
                                                let _ = state.apply(ResizableIntent::Resize {
                                                    handle_index: index,
                                                    leading_percent: value,
                                                });
                                            });
                                        }
                                    }
                                    on:change=move |_| {
                                        if !handle_disabled {
                                            set_state.update(|state| {
                                                let _ = state.apply(ResizableIntent::EndResize);
                                            });
                                        }
                                    }
                                    on:blur=move |_| {
                                        if !handle_disabled {
                                            set_state.update(|state| {
                                                let _ = state.apply(ResizableIntent::EndResize);
                                            });
                                        }
                                    }
                                />
                            </label>
                        });
                        view! {
                            <article
                                class=move || {
                                    state.with(|state| {
                                        resizable_panel_class(
                                            density,
                                            state.is_active_panel(&panel_value_for_class),
                                            panel_disabled,
                                            invalid,
                                        )
                                        .to_owned()
                                    })
                                }
                                style=move || {
                                    state.with(|state| {
                                        resizable_panel_flex_style(
                                            state.panel_size(index).unwrap_or(initial_size),
                                        )
                                    })
                                }
                                data-ui-part=ResizablePart::Panel.label()
                                data-ui-value=panel_value_for_data
                                data-ui-index=index.to_string()
                                tabindex=panel_tab_index
                                aria-disabled=panel_disabled.to_string()
                                on:focus=move |_| {
                                    if !panel_disabled {
                                        let value = panel_value_for_focus.clone();
                                        set_state.update(|state| {
                                            let _ = state.apply(ResizableIntent::FocusPanel(value));
                                        });
                                    }
                                }
                                on:blur=move |_| {
                                    if !panel_disabled {
                                        set_state.update(|state| {
                                            let _ = state.apply(ResizableIntent::BlurPanel);
                                        });
                                    }
                                }
                            >
                                <p class=resizable_panel_title_class(density, panel_disabled)>
                                    {panel_title}
                                </p>
                                <p class=resizable_panel_detail_class(invalid, panel_disabled)>
                                    {panel_detail}
                                </p>
                                <p class=resizable_panel_meta_class(panel_disabled)>
                                    {move || {
                                        state.with(|state| {
                                            format!(
                                                "{} of {}",
                                                state.panel_size(index).unwrap_or(initial_size),
                                                100,
                                            )
                                        })
                                    }}
                                </p>
                                {handle_view}
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
    .into_any()
}

const fn resizable_root_class(
    density: ResizableDensity,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return RESIZABLE_ROOT_DISABLED;
    }
    if invalid {
        return RESIZABLE_ROOT_INVALID;
    }
    match density {
        ResizableDensity::Standard => RESIZABLE_ROOT,
        ResizableDensity::Dense => RESIZABLE_ROOT_DENSE,
    }
}

const fn resizable_title_class(density: ResizableDensity) -> &'static str {
    match density {
        ResizableDensity::Standard => RESIZABLE_TITLE,
        ResizableDensity::Dense => RESIZABLE_TITLE_DENSE,
    }
}

const fn resizable_status_class(invalid: bool, disabled: bool) -> &'static str {
    if disabled {
        RESIZABLE_STATUS_DISABLED
    } else if invalid {
        RESIZABLE_STATUS_INVALID
    } else {
        RESIZABLE_STATUS
    }
}

const fn resizable_group_class(
    density: ResizableDensity,
    orientation: ResizableOrientation,
) -> &'static str {
    match (density, orientation) {
        (ResizableDensity::Standard, ResizableOrientation::Horizontal) => {
            RESIZABLE_GROUP_HORIZONTAL
        }
        (ResizableDensity::Standard, ResizableOrientation::Vertical) => RESIZABLE_GROUP_VERTICAL,
        (ResizableDensity::Dense, ResizableOrientation::Horizontal) => {
            RESIZABLE_GROUP_DENSE_HORIZONTAL
        }
        (ResizableDensity::Dense, ResizableOrientation::Vertical) => RESIZABLE_GROUP_DENSE_VERTICAL,
    }
}

const fn resizable_panel_class(
    density: ResizableDensity,
    active: bool,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        return RESIZABLE_PANEL_DISABLED;
    }
    if invalid {
        return RESIZABLE_PANEL_INVALID;
    }
    if active {
        return RESIZABLE_PANEL_ACTIVE;
    }
    match density {
        ResizableDensity::Standard => RESIZABLE_PANEL,
        ResizableDensity::Dense => RESIZABLE_PANEL_DENSE,
    }
}

const fn resizable_panel_title_class(density: ResizableDensity, disabled: bool) -> &'static str {
    if disabled {
        return RESIZABLE_PANEL_TITLE_DISABLED;
    }
    match density {
        ResizableDensity::Standard => RESIZABLE_PANEL_TITLE,
        ResizableDensity::Dense => RESIZABLE_PANEL_TITLE_DENSE,
    }
}

const fn resizable_panel_detail_class(invalid: bool, disabled: bool) -> &'static str {
    if disabled {
        RESIZABLE_PANEL_DETAIL_DISABLED
    } else if invalid {
        RESIZABLE_PANEL_DETAIL_INVALID
    } else {
        RESIZABLE_PANEL_DETAIL
    }
}

const fn resizable_panel_meta_class(disabled: bool) -> &'static str {
    if disabled {
        RESIZABLE_PANEL_META_DISABLED
    } else {
        RESIZABLE_PANEL_META
    }
}

const fn resizable_handle_class(
    density: ResizableDensity,
    resizing: bool,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        return RESIZABLE_HANDLE_DISABLED;
    }
    if invalid {
        return RESIZABLE_HANDLE_INVALID;
    }
    if resizing {
        return RESIZABLE_HANDLE_ACTIVE;
    }
    match density {
        ResizableDensity::Standard => RESIZABLE_HANDLE,
        ResizableDensity::Dense => RESIZABLE_HANDLE_DENSE,
    }
}

const fn resizable_handle_label_class(resizing: bool, disabled: bool) -> &'static str {
    if disabled {
        RESIZABLE_HANDLE_LABEL_DISABLED
    } else if resizing {
        RESIZABLE_HANDLE_LABEL_ACTIVE
    } else {
        RESIZABLE_HANDLE_LABEL
    }
}

fn resizable_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    active_panel: Option<&str>,
    resizing_handle: Option<usize>,
) -> String {
    if disabled {
        "disabled".to_owned()
    } else if loading {
        "loading".to_owned()
    } else if invalid {
        "invalid".to_owned()
    } else if let Some(handle) = resizing_handle {
        format!("resizing-{handle}")
    } else if let Some(panel) = active_panel {
        format!("focused-{panel}")
    } else {
        "idle".to_owned()
    }
}

#[component]
pub fn ScrollArea(
    #[prop(optional, default = default_scroll_area_model())] model: ScrollAreaModel,
) -> AnyView {
    if let Err(report) = validate_scroll_area_model(&model) {
        let message = format!("ScrollArea validation failed: {report}");
        return view! {
            <div class=SCROLL_AREA_ROOT_INVALID data-ui-component="scroll-area" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let overflow = model.overflow;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = scroll_area_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == ScrollAreaPart::Root)
        .expect("invariant: scroll area render nodes include root")
        .clone();
    let root_value = root.value.clone();
    let root_label = root.label.clone();
    let root_aria_label = root.label.clone();
    let root_detail = root.detail.clone();
    let items = model.items.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=scroll_area_root_class(density, invalid, disabled)
            data-ui-component="scroll-area"
            data-ui-part=ScrollAreaPart::Root.label()
            data-ui-density=density.label()
            data-ui-overflow=overflow.label()
            data-ui-state=move || {
                state.with(|state| {
                    scroll_area_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.active_item(),
                        state.hovered_axis(),
                        state.is_viewport_focused(),
                    )
                })
            }
            data-ui-value=root_value
            aria-label=root_aria_label
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
        >
            <div class=SCROLL_AREA_HEADER>
                <p class=scroll_area_title_class(density)>{root_label}</p>
                <p class=scroll_area_status_class(invalid, disabled)>{root_detail}</p>
            </div>
            <div class=scroll_area_frame_class(density) data-ui-part=ScrollAreaPart::Viewport.label()>
                <div
                    class=move || {
                        state.with(|state| {
                            scroll_area_viewport_class(
                                density,
                                state.is_viewport_focused(),
                                invalid,
                                blocked,
                            )
                            .to_owned()
                        })
                    }
                    data-ui-part=ScrollAreaPart::Viewport.label()
                    data-ui-value=move || {
                        state.with(|state| state.active_item().unwrap_or("viewport").to_owned())
                    }
                    tabindex=if blocked { "-1" } else { "0" }
                    role="region"
                    aria-label="Scrollable content"
                    on:focus=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(ScrollAreaIntent::FocusViewport);
                            });
                        }
                    }
                    on:blur=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(ScrollAreaIntent::BlurViewport);
                            });
                        }
                    }
                >
                    <div class=scroll_area_content_class(density) data-ui-part=ScrollAreaPart::Content.label()>
                        {items
                            .into_iter()
                            .enumerate()
                            .map(|(index, item)| {
                                let item_value_for_class = item.value.clone();
                                let item_value_for_data = item.value.clone();
                                let item_value_for_click = item.value.clone();
                                let item_disabled = blocked || item.disabled;
                                view! {
                                    <article
                                        class=move || {
                                            state.with(|state| {
                                                scroll_area_item_class(
                                                    density,
                                                    state.is_active_item(&item_value_for_class),
                                                    item_disabled,
                                                    invalid,
                                                )
                                                .to_owned()
                                            })
                                        }
                                        data-ui-part=ScrollAreaPart::Content.label()
                                        data-ui-value=item_value_for_data
                                        data-ui-index=index.to_string()
                                        aria-disabled=item_disabled.to_string()
                                        on:click=move |_| {
                                            if !item_disabled {
                                                let value = item_value_for_click.clone();
                                                set_state.update(|state| {
                                                    let _ = state.apply(ScrollAreaIntent::ScrollTo(value));
                                                });
                                            }
                                        }
                                    >
                                        <p class=scroll_area_item_title_class(density, item_disabled)>
                                            {item.title}
                                        </p>
                                        <p class=scroll_area_item_detail_class(invalid, item_disabled)>
                                            {item.detail}
                                        </p>
                                    </article>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
                <div class=SCROLL_AREA_BAR_ROW aria-hidden="true">
                    {overflow.has_vertical().then_some(view! {
                        <span
                            class=move || {
                                state.with(|state| {
                                    scroll_area_bar_class(
                                        ScrollAreaAxis::Vertical,
                                        state.is_hovering_axis(ScrollAreaAxis::Vertical),
                                        invalid,
                                        blocked,
                                    )
                                    .to_owned()
                                })
                            }
                            data-ui-part=ScrollAreaPart::Bar.label()
                            data-ui-axis=ScrollAreaAxis::Vertical.label()
                            on:mouseenter=move |_| {
                                if !blocked {
                                    set_state.update(|state| {
                                        let _ = state.apply(ScrollAreaIntent::HoverBar(ScrollAreaAxis::Vertical));
                                    });
                                }
                            }
                            on:mouseleave=move |_| {
                                if !blocked {
                                    set_state.update(|state| {
                                        let _ = state.apply(ScrollAreaIntent::LeaveBar);
                                    });
                                }
                            }
                        ></span>
                    })}
                    {overflow.has_horizontal().then_some(view! {
                        <span
                            class=move || {
                                state.with(|state| {
                                    scroll_area_bar_class(
                                        ScrollAreaAxis::Horizontal,
                                        state.is_hovering_axis(ScrollAreaAxis::Horizontal),
                                        invalid,
                                        blocked,
                                    )
                                    .to_owned()
                                })
                            }
                            data-ui-part=ScrollAreaPart::Bar.label()
                            data-ui-axis=ScrollAreaAxis::Horizontal.label()
                            on:mouseenter=move |_| {
                                if !blocked {
                                    set_state.update(|state| {
                                        let _ = state.apply(ScrollAreaIntent::HoverBar(ScrollAreaAxis::Horizontal));
                                    });
                                }
                            }
                            on:mouseleave=move |_| {
                                if !blocked {
                                    set_state.update(|state| {
                                        let _ = state.apply(ScrollAreaIntent::LeaveBar);
                                    });
                                }
                            }
                        ></span>
                    })}
                    <span
                        class=move || {
                            state.with(|state| {
                                scroll_area_corner_class(
                                    overflow,
                                    state.hovered_axis().is_some(),
                                )
                                .to_owned()
                            })
                        }
                        data-ui-part=ScrollAreaPart::Corner.label()
                    ></span>
                </div>
            </div>
        </section>
    }
    .into_any()
}

const fn scroll_area_root_class(
    density: ScrollAreaDensity,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SCROLL_AREA_ROOT_DISABLED;
    }
    if invalid {
        return SCROLL_AREA_ROOT_INVALID;
    }
    match density {
        ScrollAreaDensity::Standard => SCROLL_AREA_ROOT,
        ScrollAreaDensity::Dense => SCROLL_AREA_ROOT_DENSE,
    }
}

const fn scroll_area_title_class(density: ScrollAreaDensity) -> &'static str {
    match density {
        ScrollAreaDensity::Standard => SCROLL_AREA_TITLE,
        ScrollAreaDensity::Dense => SCROLL_AREA_TITLE_DENSE,
    }
}

const fn scroll_area_status_class(invalid: bool, disabled: bool) -> &'static str {
    if disabled {
        SCROLL_AREA_STATUS_DISABLED
    } else if invalid {
        SCROLL_AREA_STATUS_INVALID
    } else {
        SCROLL_AREA_STATUS
    }
}

const fn scroll_area_frame_class(density: ScrollAreaDensity) -> &'static str {
    match density {
        ScrollAreaDensity::Standard => SCROLL_AREA_FRAME,
        ScrollAreaDensity::Dense => SCROLL_AREA_FRAME_DENSE,
    }
}

const fn scroll_area_viewport_class(
    density: ScrollAreaDensity,
    focused: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SCROLL_AREA_VIEWPORT_DISABLED;
    }
    if invalid {
        return SCROLL_AREA_VIEWPORT_INVALID;
    }
    if focused {
        return SCROLL_AREA_VIEWPORT_FOCUSED;
    }
    match density {
        ScrollAreaDensity::Standard => SCROLL_AREA_VIEWPORT,
        ScrollAreaDensity::Dense => SCROLL_AREA_VIEWPORT_DENSE,
    }
}

const fn scroll_area_content_class(density: ScrollAreaDensity) -> &'static str {
    match density {
        ScrollAreaDensity::Standard => SCROLL_AREA_CONTENT,
        ScrollAreaDensity::Dense => SCROLL_AREA_CONTENT_DENSE,
    }
}

const fn scroll_area_item_class(
    density: ScrollAreaDensity,
    active: bool,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        return SCROLL_AREA_ITEM_DISABLED;
    }
    if invalid {
        return SCROLL_AREA_ITEM_INVALID;
    }
    if active {
        return SCROLL_AREA_ITEM_ACTIVE;
    }
    match density {
        ScrollAreaDensity::Standard => SCROLL_AREA_ITEM,
        ScrollAreaDensity::Dense => SCROLL_AREA_ITEM_DENSE,
    }
}

const fn scroll_area_item_title_class(density: ScrollAreaDensity, disabled: bool) -> &'static str {
    if disabled {
        return SCROLL_AREA_ITEM_TITLE_DISABLED;
    }
    match density {
        ScrollAreaDensity::Standard => SCROLL_AREA_ITEM_TITLE,
        ScrollAreaDensity::Dense => SCROLL_AREA_ITEM_TITLE_DENSE,
    }
}

const fn scroll_area_item_detail_class(invalid: bool, disabled: bool) -> &'static str {
    if disabled {
        SCROLL_AREA_ITEM_DETAIL_DISABLED
    } else if invalid {
        SCROLL_AREA_ITEM_DETAIL_INVALID
    } else {
        SCROLL_AREA_ITEM_DETAIL
    }
}

fn scroll_area_bar_class(
    axis: ScrollAreaAxis,
    active: bool,
    invalid: bool,
    disabled: bool,
) -> String {
    let tone = if disabled {
        SCROLL_AREA_BAR_DISABLED
    } else if invalid {
        SCROLL_AREA_BAR_INVALID
    } else if active {
        SCROLL_AREA_BAR_ACTIVE
    } else {
        SCROLL_AREA_BAR
    };
    let axis_class = match axis {
        ScrollAreaAxis::Vertical => SCROLL_AREA_BAR_VERTICAL,
        ScrollAreaAxis::Horizontal => SCROLL_AREA_BAR_HORIZONTAL,
    };
    format!("{tone} {axis_class}")
}

const fn scroll_area_corner_class(overflow: ScrollAreaOverflow, active: bool) -> &'static str {
    match (overflow, active) {
        (ScrollAreaOverflow::Both, true) => SCROLL_AREA_CORNER_ACTIVE,
        (ScrollAreaOverflow::Both, false) => SCROLL_AREA_CORNER,
        _ => SCROLL_AREA_CORNER_HIDDEN,
    }
}

fn scroll_area_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    active_item: Option<&str>,
    hovered_axis: Option<ScrollAreaAxis>,
    viewport_focused: bool,
) -> String {
    if disabled {
        "disabled".to_owned()
    } else if loading {
        "loading".to_owned()
    } else if invalid {
        "invalid".to_owned()
    } else if let Some(axis) = hovered_axis {
        format!("bar-{}", axis.label())
    } else if let Some(item) = active_item {
        format!("active-{item}")
    } else if viewport_focused {
        "focused".to_owned()
    } else {
        "idle".to_owned()
    }
}

#[component]
pub fn Select(#[prop(optional, default = default_select_model())] model: SelectModel) -> AnyView {
    if let Err(report) = validate_select_model(&model) {
        let message = format!("Select validation failed: {report}");
        return view! {
            <div class=SELECT_TRIGGER_INVALID data-ui-component="select" data-ui-state="invalid" role="alert">
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
    let required = model.required;
    let state_model = model.state();
    let nodes = select_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == SelectPart::Root)
        .expect("invariant: select render nodes include root")
        .clone();
    let root_value = root.value.clone();
    let root_label = root.label.clone();
    let root_content_label = root.detail.clone();
    let root_error = root.detail.clone();
    let groups = model.groups.clone();
    let label_model = model.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=select_root_class(disabled)
            data-ui-component="select"
            data-ui-part=SelectPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    select_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_open(),
                        state.focused_value(),
                        state.selected_value(),
                    )
                })
            }
            data-ui-value=root_value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
        >
            <p class=select_label_class(disabled)>
                {root_label}
                {required.then_some(view! { <span class=CHECKBOX_REQUIRED>" *"</span> })}
            </p>
            <button
                type="button"
                class=move || {
                    state.with(|state| {
                        select_trigger_class(density, state.is_open(), invalid, blocked).to_owned()
                    })
                }
                data-ui-part=SelectPart::Trigger.label()
                data-ui-value=move || {
                    state.with(|state| state.selected_value().unwrap_or("none").to_owned())
                }
                aria-haspopup="listbox"
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(SelectIntent::Toggle);
                        });
                    }
                }
            >
                <span class=SELECT_VALUE data-ui-part=SelectPart::Value.label()>
                    {move || state.with(|state| selected_select_label(&label_model, state))}
                </span>
                <span class=SELECT_CHEVRON aria-hidden="true">"v"</span>
            </button>
            <div
                class=move || {
                    state.with(|state| select_content_class(density, state.is_open()).to_owned())
                }
                data-ui-part=SelectPart::Content.label()
                hidden=move || state.with(|state| !state.is_open())
                role="listbox"
                aria-label=root_content_label
            >
                {groups
                    .into_iter()
                    .enumerate()
                    .map(|(group_index, group)| {
                        let group_value = group.value.clone();
                        let group_label = group.label.clone();
                        let group_options = group.options.clone();
                        let group_disabled = blocked || group.disabled;
                        view! {
                            <div
                                class=SELECT_GROUP
                                data-ui-part=SelectPart::Group.label()
                                data-ui-value=group_value
                                data-ui-index=group_index.to_string()
                            >
                                <p class=select_group_label_class(group_disabled)>{group_label}</p>
                                {group_options
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, option)| {
                                        let option_value_for_class = option.value.clone();
                                        let option_value_for_data = option.value.clone();
                                        let option_value_for_selected = option.value.clone();
                                        let option_value_for_focus = option.value.clone();
                                        let option_value_for_select = option.value.clone();
                                        let option_disabled = group_disabled || option.disabled;
                                        view! {
                                            <button
                                                type="button"
                                                role="option"
                                                class=move || {
                                                    state.with(|state| {
                                                        select_item_class(
                                                            density,
                                                            state.is_selected(&option_value_for_class),
                                                            state.is_focused(&option_value_for_class),
                                                            option_disabled,
                                                        )
                                                        .to_owned()
                                                    })
                                                }
                                                data-ui-part=SelectPart::Item.label()
                                                data-ui-value=option_value_for_data
                                                data-ui-index=index.to_string()
                                                aria-selected=move || {
                                                    state.with(|state| state.is_selected(&option_value_for_selected).to_string())
                                                }
                                                disabled=option_disabled
                                                on:focus=move |_| {
                                                    if !option_disabled {
                                                        let value = option_value_for_focus.clone();
                                                        set_state.update(|state| {
                                                            let _ = state.apply(SelectIntent::Focus(value));
                                                        });
                                                    }
                                                }
                                                on:click=move |_| {
                                                    if !option_disabled {
                                                        let value = option_value_for_select.clone();
                                                        set_state.update(|state| {
                                                            let _ = state.apply(SelectIntent::Select(value));
                                                        });
                                                    }
                                                }
                                            >
                                                <span class=select_item_title_class(density, option_disabled)>
                                                    {option.label}
                                                </span>
                                                <span class=select_item_detail_class(option_disabled)>
                                                    {option.detail}
                                                </span>
                                            </button>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
            {invalid.then_some(view! { <p class=SELECT_ERROR>{root_error}</p> })}
        </section>
    }
    .into_any()
}

const fn select_root_class(disabled: bool) -> &'static str {
    if disabled {
        SELECT_ROOT_DISABLED
    } else {
        SELECT_ROOT
    }
}

const fn select_label_class(disabled: bool) -> &'static str {
    if disabled {
        SELECT_LABEL_DISABLED
    } else {
        SELECT_LABEL
    }
}

const fn select_trigger_class(
    density: SelectDensity,
    open: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SELECT_TRIGGER_DISABLED;
    }
    if invalid {
        return SELECT_TRIGGER_INVALID;
    }
    if open {
        return SELECT_TRIGGER_OPEN;
    }
    match density {
        SelectDensity::Standard => SELECT_TRIGGER,
        SelectDensity::Dense => SELECT_TRIGGER_DENSE,
    }
}

const fn select_content_class(density: SelectDensity, open: bool) -> &'static str {
    if !open {
        return SELECT_CONTENT_HIDDEN;
    }
    match density {
        SelectDensity::Standard => SELECT_CONTENT,
        SelectDensity::Dense => SELECT_CONTENT_DENSE,
    }
}

const fn select_group_label_class(disabled: bool) -> &'static str {
    if disabled {
        SELECT_GROUP_LABEL_DISABLED
    } else {
        SELECT_GROUP_LABEL
    }
}

const fn select_item_class(
    density: SelectDensity,
    selected: bool,
    focused: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SELECT_ITEM_DISABLED;
    }
    if selected {
        return SELECT_ITEM_SELECTED;
    }
    if focused {
        return SELECT_ITEM_FOCUSED;
    }
    match density {
        SelectDensity::Standard => SELECT_ITEM,
        SelectDensity::Dense => SELECT_ITEM_DENSE,
    }
}

const fn select_item_title_class(density: SelectDensity, disabled: bool) -> &'static str {
    if disabled {
        return SELECT_ITEM_TITLE_DISABLED;
    }
    match density {
        SelectDensity::Standard => SELECT_ITEM_TITLE,
        SelectDensity::Dense => SELECT_ITEM_TITLE_DENSE,
    }
}

const fn select_item_detail_class(disabled: bool) -> &'static str {
    if disabled {
        SELECT_ITEM_DETAIL_DISABLED
    } else {
        SELECT_ITEM_DETAIL
    }
}

fn select_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    open: bool,
    focused_value: Option<&str>,
    selected_value: Option<&str>,
) -> String {
    if disabled {
        "disabled".to_owned()
    } else if loading {
        "loading".to_owned()
    } else if invalid {
        "invalid".to_owned()
    } else if open {
        focused_value
            .map(|value| format!("open-{value}"))
            .unwrap_or_else(|| "open".to_owned())
    } else if let Some(value) = selected_value {
        format!("selected-{value}")
    } else {
        "empty".to_owned()
    }
}

#[component]
pub fn Separator(
    #[prop(optional, default = default_separator_model())] model: SeparatorModel,
) -> AnyView {
    if let Err(report) = validate_separator_model(&model) {
        let message = format!("Separator validation failed: {report}");
        return view! {
            <div class=SEPARATOR_ROOT_INVALID data-ui-component="separator" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let orientation = model.orientation;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let decorative = model.decorative;
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = separator_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == SeparatorPart::Root)
        .expect("invariant: separator render nodes include root")
        .clone();
    let line = nodes
        .iter()
        .find(|node| node.part == SeparatorPart::Line)
        .expect("invariant: separator render nodes include line")
        .clone();
    let label = nodes
        .iter()
        .find(|node| node.part == SeparatorPart::Label)
        .expect("invariant: separator render nodes include label")
        .clone();
    let root_value = root.value.clone();
    let root_label = root.label.clone();
    let root_description = root.detail.clone();
    let root_error = root.detail.clone();
    let line_value = line.value.clone();
    let label_value = label.value.clone();
    let label_text = label.label.clone();
    let label_visible = label.visible;
    let (state, set_state) = signal(state_model);

    view! {
        <div
            class=move || {
                state.with(|state| {
                    separator_root_class(
                        density,
                        orientation,
                        state.is_active(),
                        invalid,
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-component="separator"
            data-ui-part=SeparatorPart::Root.label()
            data-ui-density=density.label()
            data-ui-orientation=orientation.label()
            data-ui-state=move || {
                state.with(|state| {
                    separator_state_label(loading, disabled, invalid, state.is_active()).to_owned()
                })
            }
            data-ui-value=root_value
            role=if decorative { "presentation" } else { "separator" }
            aria-label=root_label
            aria-description=root_description
            aria-orientation=orientation.label()
            aria-hidden=decorative.to_string()
            aria-busy=loading.to_string()
            aria-disabled=blocked.to_string()
            tabindex=if blocked || decorative { "-1" } else { "0" }
            on:focus=move |_| {
                if !blocked && !decorative {
                    set_state.update(|state| {
                        let _ = state.apply(SeparatorIntent::Focus);
                    });
                }
            }
            on:blur=move |_| {
                if !blocked && !decorative {
                    set_state.update(|state| {
                        let _ = state.apply(SeparatorIntent::Blur);
                    });
                }
            }
            on:mouseenter=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(SeparatorIntent::Hover);
                    });
                }
            }
            on:mouseleave=move |_| {
                if !blocked {
                    set_state.update(|state| {
                        let _ = state.apply(SeparatorIntent::Leave);
                    });
                }
            }
        >
            <span
                class=move || {
                    state.with(|state| {
                        separator_line_class(
                            density,
                            orientation,
                            state.is_active(),
                            invalid,
                            blocked,
                        )
                        .to_owned()
                    })
                }
                data-ui-part=SeparatorPart::Line.label()
                data-ui-value=line_value
                aria-hidden="true"
            ></span>
            <span
                class=separator_label_class(density, label_visible, invalid, blocked)
                data-ui-part=SeparatorPart::Label.label()
                data-ui-value=label_value
                hidden=!label_visible
            >
                {label_text}
            </span>
            {invalid.then_some(view! { <p class=SEPARATOR_ERROR>{root_error}</p> })}
        </div>
    }
    .into_any()
}

const fn separator_root_class(
    density: SeparatorDensity,
    orientation: SeparatorOrientation,
    active: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SEPARATOR_ROOT_DISABLED;
    }
    if invalid {
        return SEPARATOR_ROOT_INVALID;
    }
    match (orientation, active, density) {
        (SeparatorOrientation::Vertical, true, _) => SEPARATOR_ROOT_VERTICAL_ACTIVE,
        (SeparatorOrientation::Vertical, false, _) => SEPARATOR_ROOT_VERTICAL,
        (SeparatorOrientation::Horizontal, true, _) => SEPARATOR_ROOT_ACTIVE,
        (SeparatorOrientation::Horizontal, false, SeparatorDensity::Dense) => SEPARATOR_ROOT_DENSE,
        (SeparatorOrientation::Horizontal, false, SeparatorDensity::Standard) => SEPARATOR_ROOT,
    }
}

const fn separator_line_class(
    density: SeparatorDensity,
    orientation: SeparatorOrientation,
    active: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SEPARATOR_LINE_DISABLED;
    }
    if invalid {
        return SEPARATOR_LINE_INVALID;
    }
    match (orientation, active, density) {
        (SeparatorOrientation::Vertical, true, _) => SEPARATOR_LINE_VERTICAL_ACTIVE,
        (SeparatorOrientation::Vertical, false, _) => SEPARATOR_LINE_VERTICAL,
        (SeparatorOrientation::Horizontal, true, _) => SEPARATOR_LINE_ACTIVE,
        (SeparatorOrientation::Horizontal, false, SeparatorDensity::Dense) => SEPARATOR_LINE_DENSE,
        (SeparatorOrientation::Horizontal, false, SeparatorDensity::Standard) => SEPARATOR_LINE,
    }
}

const fn separator_label_class(
    density: SeparatorDensity,
    visible: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if !visible {
        return SEPARATOR_LABEL_HIDDEN;
    }
    if disabled {
        return SEPARATOR_LABEL_DISABLED;
    }
    if invalid {
        return SEPARATOR_LABEL_INVALID;
    }
    match density {
        SeparatorDensity::Standard => SEPARATOR_LABEL,
        SeparatorDensity::Dense => SEPARATOR_LABEL_DENSE,
    }
}

const fn separator_state_label(
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
        "idle"
    }
}

#[component]
pub fn Sheet(#[prop(optional, default = default_sheet_model())] model: SheetModel) -> AnyView {
    if let Err(report) = validate_sheet_model(&model) {
        let message = format!("Sheet validation failed: {report}");
        return view! {
            <div class=SHEET_ERROR data-ui-component="sheet" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let side = model.side;
    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = sheet_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == SheetPart::Root)
        .expect("invariant: sheet render nodes include root")
        .clone();
    let trigger = nodes
        .iter()
        .find(|node| node.part == SheetPart::Trigger)
        .expect("invariant: sheet render nodes include trigger")
        .clone();
    let content = nodes
        .iter()
        .find(|node| node.part == SheetPart::Content)
        .expect("invariant: sheet render nodes include content")
        .clone();
    let header = nodes
        .iter()
        .find(|node| node.part == SheetPart::Header)
        .expect("invariant: sheet render nodes include header")
        .clone();
    let close = nodes
        .iter()
        .find(|node| node.part == SheetPart::Close)
        .expect("invariant: sheet render nodes include close")
        .clone();
    let footer_model = model.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=sheet_root_class(density, disabled)
            data-ui-component="sheet"
            data-ui-part=SheetPart::Root.label()
            data-ui-side=side.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    sheet_state_label(loading, disabled, state.is_open()).to_owned()
                })
            }
            data-ui-value=root.value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <button
                type="button"
                class=move || {
                    state.with(|state| {
                        sheet_trigger_class(density, state.is_open(), blocked).to_owned()
                    })
                }
                data-ui-part=SheetPart::Trigger.label()
                data-ui-value=trigger.value
                aria-haspopup="dialog"
                aria-expanded=move || state.with(|state| state.is_open().to_string())
                disabled=blocked
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(SheetIntent::Focus(SheetPart::Trigger));
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(SheetIntent::Blur);
                        });
                    }
                }
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(SheetIntent::Toggle);
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
                    let footer_nodes = sheet_render_nodes(&footer_model, state)
                        .into_iter()
                        .filter(|node| node.part == SheetPart::Footer)
                        .collect::<Vec<_>>();
                    view! {
                        <article
                            role="dialog"
                            aria-modal="true"
                            class=sheet_content_class(side, density)
                            data-ui-part=SheetPart::Content.label()
                            data-ui-value=content.value.clone()
                            data-ui-side=side.label()
                        >
                            <header
                                class=sheet_header_class(density)
                                data-ui-part=SheetPart::Header.label()
                            >
                                <h3 class=sheet_title_class(density)>{header.label.clone()}</h3>
                                <p class=sheet_description_class(density)>{header.detail.clone()}</p>
                            </header>
                            <p class=sheet_body_class(density)>{content.detail.clone()}</p>
                            <footer class=SHEET_FOOTER data-ui-part=SheetPart::Footer.label()>
                                {footer_nodes
                                    .into_iter()
                                    .map(|node| sheet_footer_action_view(node, density, blocked, set_state))
                                    .collect_view()}
                                <button
                                    type="button"
                                    class=sheet_close_class(blocked)
                                    data-ui-part=SheetPart::Close.label()
                                    data-ui-value=close.value.clone()
                                    disabled=blocked
                                    on:focus=move |_| {
                                        if !blocked {
                                            set_state.update(|state| {
                                                let _ = state.apply(SheetIntent::Focus(SheetPart::Close));
                                            });
                                        }
                                    }
                                    on:blur=move |_| {
                                        if !blocked {
                                            set_state.update(|state| {
                                                let _ = state.apply(SheetIntent::Blur);
                                            });
                                        }
                                    }
                                    on:click=move |_| {
                                        if !blocked {
                                            set_state.update(|state| {
                                                let _ = state.apply(SheetIntent::Close);
                                            });
                                        }
                                    }
                                >
                                    {close.label.clone()}
                                </button>
                            </footer>
                        </article>
                    }
                    .into_any()
                })
            }}
        </section>
    }
    .into_any()
}

fn sheet_footer_action_view(
    node: crate::SheetRenderNode,
    density: SheetDensity,
    blocked: bool,
    set_state: WriteSignal<SheetState>,
) -> AnyView {
    let value_for_click = node.value.clone();
    let close_sheet = node.close_sheet;
    let disabled = node.disabled || blocked || !node.actionable;
    view! {
        <button
            type="button"
            class=sheet_action_class(density, node.index, node.selected, disabled)
            data-ui-part=SheetPart::Footer.label()
            data-ui-value=node.value
            disabled=disabled
            on:click=move |_| {
                if !disabled {
                    let value = value_for_click.clone();
                    set_state.update(|state| {
                        let _ = state.apply(SheetIntent::ActivateFooter(value));
                        if close_sheet {
                            let _ = state.apply(SheetIntent::Close);
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

const fn sheet_root_class(density: SheetDensity, disabled: bool) -> &'static str {
    if disabled {
        return SHEET_ROOT_DISABLED;
    }
    match density {
        SheetDensity::Standard => SHEET_ROOT,
        SheetDensity::Dense => SHEET_ROOT_DENSE,
    }
}

const fn sheet_trigger_class(density: SheetDensity, open: bool, disabled: bool) -> &'static str {
    if disabled {
        return SHEET_TRIGGER_DISABLED;
    }
    if open {
        return SHEET_TRIGGER_OPEN;
    }
    match density {
        SheetDensity::Standard => SHEET_TRIGGER,
        SheetDensity::Dense => SHEET_TRIGGER_DENSE,
    }
}

fn sheet_content_class(side: SheetSide, density: SheetDensity) -> String {
    let base = match density {
        SheetDensity::Standard => SHEET_CONTENT,
        SheetDensity::Dense => SHEET_CONTENT_DENSE,
    };
    let side_class = match side {
        SheetSide::Top => SHEET_CONTENT_TOP,
        SheetSide::Right => SHEET_CONTENT_RIGHT,
        SheetSide::Bottom => SHEET_CONTENT_BOTTOM,
        SheetSide::Left => SHEET_CONTENT_LEFT,
    };
    format!("{base} {side_class}")
}

const fn sheet_header_class(density: SheetDensity) -> &'static str {
    match density {
        SheetDensity::Standard => SHEET_HEADER,
        SheetDensity::Dense => SHEET_HEADER_DENSE,
    }
}

const fn sheet_title_class(density: SheetDensity) -> &'static str {
    match density {
        SheetDensity::Standard => SHEET_TITLE,
        SheetDensity::Dense => SHEET_TITLE_DENSE,
    }
}

const fn sheet_description_class(density: SheetDensity) -> &'static str {
    match density {
        SheetDensity::Standard => SHEET_DESCRIPTION,
        SheetDensity::Dense => SHEET_DESCRIPTION_DENSE,
    }
}

const fn sheet_body_class(density: SheetDensity) -> &'static str {
    match density {
        SheetDensity::Standard => SHEET_BODY,
        SheetDensity::Dense => SHEET_BODY_DENSE,
    }
}

const fn sheet_action_class(
    density: SheetDensity,
    index: usize,
    selected: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SHEET_ACTION_DISABLED;
    }
    if selected {
        return SHEET_ACTION_ACTIVE;
    }
    match (density, index) {
        (SheetDensity::Dense, 0) => SHEET_ACTION_DENSE,
        (_, 0) => SHEET_ACTION,
        _ => SHEET_ACTION_SECONDARY,
    }
}

const fn sheet_close_class(disabled: bool) -> &'static str {
    if disabled {
        SHEET_CLOSE_DISABLED
    } else {
        SHEET_CLOSE
    }
}

const fn sheet_state_label(loading: bool, disabled: bool, open: bool) -> &'static str {
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
pub fn Sidebar(
    #[prop(optional, default = default_sidebar_model())] model: SidebarModel,
) -> AnyView {
    if let Err(report) = validate_sidebar_model(&model) {
        let message = format!("Sidebar validation failed: {report}");
        return view! {
            <div class=SIDEBAR_ERROR data-ui-component="sidebar" data-ui-state="invalid" role="alert">
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
    let state_model = model.state();
    let nodes = sidebar_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == SidebarPart::Root)
        .expect("invariant: sidebar render nodes include root")
        .clone();
    let header = nodes
        .iter()
        .find(|node| node.part == SidebarPart::Header)
        .expect("invariant: sidebar render nodes include header")
        .clone();
    let footer = nodes
        .iter()
        .find(|node| node.part == SidebarPart::Footer)
        .expect("invariant: sidebar render nodes include footer")
        .clone();
    let rail = nodes
        .iter()
        .find(|node| node.part == SidebarPart::Rail)
        .expect("invariant: sidebar render nodes include rail")
        .clone();
    let groups = model.groups.clone();
    let root_value = root.value.clone();
    let root_label = root.label.clone();
    let root_error = root.detail.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <aside
            class=move || {
                state.with(|state| {
                    sidebar_root_class(
                        density,
                        state.is_collapsed(),
                        invalid,
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-component="sidebar"
            data-ui-part=SidebarPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    sidebar_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_collapsed(),
                        state.active_value(),
                        state.focused_value(),
                    )
                })
            }
            data-ui-value=root_value
            aria-label=root_label
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <button
                type="button"
                class=move || {
                    state.with(|state| sidebar_rail_class(state.is_collapsed(), blocked).to_owned())
                }
                data-ui-part=SidebarPart::Rail.label()
                data-ui-value=rail.value
                aria-label=rail.label
                aria-pressed=move || state.with(|state| state.is_collapsed().to_string())
                disabled=blocked
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(SidebarIntent::ToggleCollapse);
                        });
                    }
                }
            >
                "||"
            </button>
            <div
                class=move || {
                    state.with(|state| sidebar_panel_class(density, state.is_collapsed()).to_owned())
                }
                data-ui-part=SidebarPart::Content.label()
                hidden=move || state.with(|state| state.is_collapsed())
            >
                <header class=sidebar_header_class(density) data-ui-part=SidebarPart::Header.label()>
                    <h3 class=sidebar_title_class(density)>{header.label}</h3>
                    <p class=SIDEBAR_DETAIL>{header.detail}</p>
                </header>
                <nav class=sidebar_content_class(density) aria-label=root.label>
                    {groups
                        .into_iter()
                        .enumerate()
                        .map(|(group_index, group)| {
                            let group_disabled = blocked || group.disabled;
                            let items = group.items.clone();
                            view! {
                                <section
                                    class=sidebar_group_class(group_disabled)
                                    data-ui-part=SidebarPart::Group.label()
                                    data-ui-value=group.value
                                    data-ui-index=group_index.to_string()
                                >
                                    <p class=SIDEBAR_GROUP_LABEL>{group.label}</p>
                                    <div class=sidebar_content_class(density)>
                                        {items
                                            .into_iter()
                                            .enumerate()
                                            .map(|(item_index, item)| {
                                                let item_value_for_class = item.value.clone();
                                                let item_value_for_current = item.value.clone();
                                                let item_value_for_focus = item.value.clone();
                                                let item_value_for_click = item.value.clone();
                                                let item_disabled = group_disabled || item.disabled;
                                                let badge = item.badge.clone();
                                                view! {
                                                    <button
                                                        type="button"
                                                        class=move || {
                                                            state.with(|state| {
                                                                sidebar_menu_class(
                                                                    density,
                                                                    state.is_active(&item_value_for_class),
                                                                    state.is_focused(&item_value_for_class),
                                                                    item_disabled,
                                                                )
                                                                .to_owned()
                                                            })
                                                        }
                                                        data-ui-part=SidebarPart::Menu.label()
                                                        data-ui-value=item.value
                                                        data-ui-index=item_index.to_string()
                                                        aria-current=move || {
                                                            state.with(|state| {
                                                                if state.is_active(&item_value_for_current) {
                                                                    "page"
                                                                } else {
                                                                    "false"
                                                                }
                                                            })
                                                        }
                                                        disabled=item_disabled
                                                        on:focus=move |_| {
                                                            if !item_disabled {
                                                                let value = item_value_for_focus.clone();
                                                                set_state.update(|state| {
                                                                    let _ = state.apply(SidebarIntent::Focus(value));
                                                                });
                                                            }
                                                        }
                                                        on:click=move |_| {
                                                            if !item_disabled {
                                                                let value = item_value_for_click.clone();
                                                                set_state.update(|state| {
                                                                    let _ = state.apply(SidebarIntent::Activate(value));
                                                                });
                                                            }
                                                        }
                                                    >
                                                        <span>{item.label}</span>
                                                        {badge
                                                            .map(|badge| view! { <span class=SIDEBAR_BADGE>{badge}</span> })}
                                                    </button>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </section>
                            }
                        })
                        .collect_view()}
                </nav>
                <footer class=SIDEBAR_FOOTER data-ui-part=SidebarPart::Footer.label()>
                    <p class=SIDEBAR_FOOTER_LABEL>{footer.label}</p>
                    <p class=SIDEBAR_FOOTER_DETAIL>{footer.detail}</p>
                </footer>
                {invalid.then_some(view! { <p class=SIDEBAR_ERROR>{root_error}</p> })}
            </div>
        </aside>
    }
    .into_any()
}

const fn sidebar_root_class(
    density: SidebarDensity,
    collapsed: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SIDEBAR_ROOT_DISABLED;
    }
    if invalid {
        return SIDEBAR_ROOT_INVALID;
    }
    if collapsed {
        return SIDEBAR_ROOT_COLLAPSED;
    }
    match density {
        SidebarDensity::Standard => SIDEBAR_ROOT,
        SidebarDensity::Dense => SIDEBAR_ROOT_DENSE,
    }
}

const fn sidebar_rail_class(collapsed: bool, disabled: bool) -> &'static str {
    if disabled {
        SIDEBAR_RAIL_DISABLED
    } else if collapsed {
        SIDEBAR_RAIL_ACTIVE
    } else {
        SIDEBAR_RAIL
    }
}

const fn sidebar_panel_class(density: SidebarDensity, collapsed: bool) -> &'static str {
    if collapsed {
        return SIDEBAR_PANEL_COLLAPSED;
    }
    match density {
        SidebarDensity::Standard => SIDEBAR_PANEL,
        SidebarDensity::Dense => SIDEBAR_PANEL_DENSE,
    }
}

const fn sidebar_header_class(density: SidebarDensity) -> &'static str {
    match density {
        SidebarDensity::Standard => SIDEBAR_HEADER,
        SidebarDensity::Dense => SIDEBAR_HEADER_DENSE,
    }
}

const fn sidebar_title_class(density: SidebarDensity) -> &'static str {
    match density {
        SidebarDensity::Standard => SIDEBAR_TITLE,
        SidebarDensity::Dense => SIDEBAR_TITLE_DENSE,
    }
}

const fn sidebar_content_class(density: SidebarDensity) -> &'static str {
    match density {
        SidebarDensity::Standard => SIDEBAR_CONTENT,
        SidebarDensity::Dense => SIDEBAR_CONTENT_DENSE,
    }
}

const fn sidebar_group_class(disabled: bool) -> &'static str {
    if disabled {
        SIDEBAR_GROUP_DISABLED
    } else {
        SIDEBAR_GROUP
    }
}

const fn sidebar_menu_class(
    density: SidebarDensity,
    active: bool,
    focused: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SIDEBAR_MENU_DISABLED;
    }
    if active {
        return SIDEBAR_MENU_ACTIVE;
    }
    if focused {
        return SIDEBAR_MENU_FOCUSED;
    }
    match density {
        SidebarDensity::Standard => SIDEBAR_MENU,
        SidebarDensity::Dense => SIDEBAR_MENU_DENSE,
    }
}

fn sidebar_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    collapsed: bool,
    active_value: Option<&str>,
    focused_value: Option<&str>,
) -> String {
    if disabled {
        "disabled".to_owned()
    } else if loading {
        "loading".to_owned()
    } else if invalid {
        "invalid".to_owned()
    } else if collapsed {
        "collapsed".to_owned()
    } else if let Some(value) = active_value {
        format!("active-{value}")
    } else if let Some(value) = focused_value {
        format!("focused-{value}")
    } else {
        "expanded".to_owned()
    }
}

#[component]
pub fn Skeleton(
    #[prop(optional, default = default_skeleton_model())] model: SkeletonModel,
) -> AnyView {
    if let Err(report) = validate_skeleton_model(&model) {
        let message = format!("Skeleton validation failed: {report}");
        return view! {
            <div class=SKELETON_ERROR data-ui-component="skeleton" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let animated = model.animated;
    let state_model = model.state();
    let nodes = skeleton_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == SkeletonPart::Root)
        .expect("invariant: skeleton render nodes include root")
        .clone();
    let block = nodes
        .iter()
        .find(|node| node.part == SkeletonPart::Block)
        .expect("invariant: skeleton render nodes include block")
        .clone();
    let text = nodes
        .iter()
        .find(|node| node.part == SkeletonPart::Text)
        .expect("invariant: skeleton render nodes include text")
        .clone();
    let media = nodes
        .iter()
        .find(|node| node.part == SkeletonPart::Media)
        .expect("invariant: skeleton render nodes include media")
        .clone();
    let root_value = root.value.clone();
    let root_label = root.label.clone();
    let root_detail = root.detail.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=skeleton_root_class(density, loading, invalid, disabled)
            data-ui-component="skeleton"
            data-ui-part=SkeletonPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    skeleton_state_label(
                        loading,
                        disabled,
                        invalid,
                        animated,
                        state.animation_paused(),
                    )
                })
            }
            data-ui-value=root_value
            aria-label=root_label
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
            on:click=move |_| {
                if loading && animated && !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SkeletonIntent::ToggleAnimation);
                    });
                }
            }
        >
            <p class=SKELETON_STATUS>{root_detail}</p>
            <div class=skeleton_content_class(density)>
                {skeleton_placeholder_view(block, density, state, set_state)}
                {skeleton_text_view(text, density, state, set_state)}
                {skeleton_placeholder_view(media, density, state, set_state)}
            </div>
            {invalid.then_some(view! { <p class=SKELETON_ERROR>{root.detail}</p> })}
        </section>
    }
    .into_any()
}

fn skeleton_placeholder_view(
    node: crate::SkeletonRenderNode,
    density: SkeletonDensity,
    state: ReadSignal<SkeletonState>,
    set_state: WriteSignal<SkeletonState>,
) -> AnyView {
    let part = node.part;
    let hidden = !node.visible;
    let invalid = node.invalid;
    let disabled = node.disabled;
    let label = node.label.clone();
    view! {
        <span
            class=move || {
                state.with(|state| {
                    skeleton_placeholder_class(
                        part,
                        density,
                        state.is_active(part),
                        invalid,
                        disabled,
                    )
                    .to_owned()
                })
            }
            data-ui-part=part.label()
            data-ui-value=node.value
            aria-hidden=hidden.to_string()
            hidden=hidden
            on:mouseenter=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SkeletonIntent::Focus(part));
                    });
                }
            }
            on:mouseleave=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SkeletonIntent::Blur(part));
                    });
                }
            }
        >
            <span class="sr-only">{label}</span>
        </span>
    }
    .into_any()
}

fn skeleton_text_view(
    node: crate::SkeletonRenderNode,
    density: SkeletonDensity,
    state: ReadSignal<SkeletonState>,
    set_state: WriteSignal<SkeletonState>,
) -> AnyView {
    let part = node.part;
    let hidden = !node.visible;
    let invalid = node.invalid;
    let disabled = node.disabled;
    let label = node.label.clone();
    let text_lines = node.text_lines;
    view! {
        <div
            class=SKELETON_CONTENT_DENSE
            data-ui-part=part.label()
            data-ui-value=node.value
            aria-hidden=hidden.to_string()
            hidden=hidden
            on:mouseenter=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SkeletonIntent::Focus(part));
                    });
                }
            }
            on:mouseleave=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SkeletonIntent::Blur(part));
                    });
                }
            }
        >
            <span class="sr-only">{label}</span>
            {(0..text_lines)
                .map(|index| {
                    view! {
                        <span
                            class=move || {
                                state.with(|state| {
                                    skeleton_line_class(
                                        density,
                                        state.is_active(part),
                                        invalid,
                                        disabled,
                                    )
                                    .to_owned()
                                })
                            }
                            data-ui-line=index.to_string()
                        ></span>
                    }
                })
                .collect_view()}
        </div>
    }
    .into_any()
}

const fn skeleton_root_class(
    density: SkeletonDensity,
    loading: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SKELETON_ROOT_DISABLED;
    }
    if invalid {
        return SKELETON_ROOT_INVALID;
    }
    if !loading {
        return SKELETON_ROOT_READY;
    }
    match density {
        SkeletonDensity::Standard => SKELETON_ROOT,
        SkeletonDensity::Dense => SKELETON_ROOT_DENSE,
    }
}

const fn skeleton_content_class(density: SkeletonDensity) -> &'static str {
    match density {
        SkeletonDensity::Standard => SKELETON_CONTENT,
        SkeletonDensity::Dense => SKELETON_CONTENT_DENSE,
    }
}

const fn skeleton_placeholder_class(
    part: SkeletonPart,
    density: SkeletonDensity,
    active: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    match part {
        SkeletonPart::Block => skeleton_block_class(density, active, invalid, disabled),
        SkeletonPart::Media => skeleton_media_class(density, active, invalid, disabled),
        SkeletonPart::Root | SkeletonPart::Text => {
            skeleton_line_class(density, active, invalid, disabled)
        }
    }
}

const fn skeleton_block_class(
    density: SkeletonDensity,
    active: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    match (density, active, invalid, disabled) {
        (_, _, _, true) => match density {
            SkeletonDensity::Standard => SKELETON_BLOCK_DISABLED,
            SkeletonDensity::Dense => SKELETON_BLOCK_DENSE_DISABLED,
        },
        (_, _, true, _) => match density {
            SkeletonDensity::Standard => SKELETON_BLOCK_INVALID,
            SkeletonDensity::Dense => SKELETON_BLOCK_DENSE_INVALID,
        },
        (_, true, _, _) => match density {
            SkeletonDensity::Standard => SKELETON_BLOCK_ACTIVE,
            SkeletonDensity::Dense => SKELETON_BLOCK_DENSE_ACTIVE,
        },
        (SkeletonDensity::Standard, _, _, _) => SKELETON_BLOCK,
        (SkeletonDensity::Dense, _, _, _) => SKELETON_BLOCK_DENSE,
    }
}

const fn skeleton_line_class(
    density: SkeletonDensity,
    active: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    match (density, active, invalid, disabled) {
        (_, _, _, true) => match density {
            SkeletonDensity::Standard => SKELETON_LINE_DISABLED,
            SkeletonDensity::Dense => SKELETON_LINE_DENSE_DISABLED,
        },
        (_, _, true, _) => match density {
            SkeletonDensity::Standard => SKELETON_LINE_INVALID,
            SkeletonDensity::Dense => SKELETON_LINE_DENSE_INVALID,
        },
        (_, true, _, _) => match density {
            SkeletonDensity::Standard => SKELETON_LINE_ACTIVE,
            SkeletonDensity::Dense => SKELETON_LINE_DENSE_ACTIVE,
        },
        (SkeletonDensity::Standard, _, _, _) => SKELETON_LINE,
        (SkeletonDensity::Dense, _, _, _) => SKELETON_LINE_DENSE,
    }
}

const fn skeleton_media_class(
    density: SkeletonDensity,
    active: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    match (density, active, invalid, disabled) {
        (_, _, _, true) => match density {
            SkeletonDensity::Standard => SKELETON_MEDIA_DISABLED,
            SkeletonDensity::Dense => SKELETON_MEDIA_DENSE_DISABLED,
        },
        (_, _, true, _) => match density {
            SkeletonDensity::Standard => SKELETON_MEDIA_INVALID,
            SkeletonDensity::Dense => SKELETON_MEDIA_DENSE_INVALID,
        },
        (_, true, _, _) => match density {
            SkeletonDensity::Standard => SKELETON_MEDIA_ACTIVE,
            SkeletonDensity::Dense => SKELETON_MEDIA_DENSE_ACTIVE,
        },
        (SkeletonDensity::Standard, _, _, _) => SKELETON_MEDIA,
        (SkeletonDensity::Dense, _, _, _) => SKELETON_MEDIA_DENSE,
    }
}

const fn skeleton_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    animated: bool,
    animation_paused: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if invalid {
        "invalid"
    } else if !loading {
        "ready"
    } else if animation_paused {
        "paused"
    } else if animated {
        "loading"
    } else {
        "static"
    }
}

#[component]
pub fn Slider(#[prop(optional, default = default_slider_model())] model: SliderModel) -> AnyView {
    if let Err(report) = validate_slider_model(&model) {
        let message = format!("Slider validation failed: {report}");
        return view! {
            <div class=SLIDER_ERROR data-ui-component="slider" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let orientation = model.orientation;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let blocked = loading || disabled;
    let state_model = model.state();
    let nodes = slider_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == SliderPart::Root)
        .expect("invariant: slider render nodes include root")
        .clone();
    let track = nodes
        .iter()
        .find(|node| node.part == SliderPart::Track)
        .expect("invariant: slider render nodes include track")
        .clone();
    let range = nodes
        .iter()
        .find(|node| node.part == SliderPart::Range)
        .expect("invariant: slider render nodes include range")
        .clone();
    let thumb = nodes
        .iter()
        .find(|node| node.part == SliderPart::Thumb)
        .expect("invariant: slider render nodes include thumb")
        .clone();
    let value = nodes
        .iter()
        .find(|node| node.part == SliderPart::Value)
        .expect("invariant: slider render nodes include value")
        .clone();
    let root_value = root.value.clone();
    let root_label = root.label.clone();
    let root_detail = root.detail.clone();
    let min = model.min;
    let max = model.max;
    let step = model.step;
    let unit = model.unit.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=slider_root_class(density, orientation, invalid, disabled)
            data-ui-component="slider"
            data-ui-part=SliderPart::Root.label()
            data-ui-density=density.label()
            data-ui-orientation=orientation.label()
            data-ui-state=move || {
                state.with(|state| {
                    slider_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_focused(),
                        state.is_dragging(),
                    )
                })
            }
            data-ui-value=root_value
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
        >
            <header class=SLIDER_HEADER>
                <p class=SLIDER_LABEL>{root_label}</p>
                <output
                    class=SLIDER_VALUE
                    data-ui-part=SliderPart::Value.label()
                    data-ui-value=value.value
                >
                    {move || state.with(|state| crate::slider_value_label(state.value(), &unit))}
                </output>
            </header>
            <div
                class=slider_track_wrap_class(orientation)
                data-ui-part=SliderPart::Track.label()
                data-ui-value=track.value
                aria-label=track.label
            >
                <span class=slider_track_class(density, orientation, invalid, blocked) aria-hidden="true">
                    <span
                        class=move || {
                            state.with(|state| {
                                slider_range_class(state.is_dragging(), invalid, blocked).to_owned()
                            })
                        }
                        style=move || state.with(|state| slider_range_style(orientation, state.percent()))
                        data-ui-part=SliderPart::Range.label()
                        data-ui-value=range.value.clone()
                    ></span>
                </span>
                <span
                    class=move || {
                        state.with(|state| {
                            slider_thumb_class(density, state.is_focused(), blocked).to_owned()
                        })
                    }
                    style=move || state.with(|state| slider_thumb_style(orientation, state.percent()))
                    data-ui-part=SliderPart::Thumb.label()
                    data-ui-value=thumb.value.clone()
                    aria-hidden="true"
                ></span>
                <input
                    type="range"
                    class=SLIDER_INPUT
                    min=min.to_string()
                    max=max.to_string()
                    step=step.to_string()
                    prop:value=move || state.with(|state| state.value().to_string())
                    aria-label=thumb.label
                    aria-orientation=orientation.label()
                    disabled=blocked
                    on:focus=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(SliderIntent::Focus);
                            });
                        }
                    }
                    on:blur=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(SliderIntent::Blur);
                                let _ = state.apply(SliderIntent::StopDrag);
                            });
                        }
                    }
                    on:mousedown=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(SliderIntent::StartDrag);
                            });
                        }
                    }
                    on:mouseup=move |_| {
                        if !blocked {
                            set_state.update(|state| {
                                let _ = state.apply(SliderIntent::StopDrag);
                            });
                        }
                    }
                    on:input=move |event| {
                        if !blocked && let Ok(value) = event_target_value(&event).parse::<i32>() {
                            set_state.update(|state| {
                                let _ = state.apply(SliderIntent::SetValue(value));
                            });
                        }
                    }
                />
            </div>
            <p class=BLOCK_DETAIL>{root_detail}</p>
            {invalid.then_some(view! { <p class=SLIDER_ERROR>{root.detail}</p> })}
        </section>
    }
    .into_any()
}

const fn slider_root_class(
    density: SliderDensity,
    orientation: SliderOrientation,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SLIDER_ROOT_DISABLED;
    }
    if invalid {
        return SLIDER_ROOT_INVALID;
    }
    match (density, orientation) {
        (_, SliderOrientation::Vertical) => SLIDER_ROOT_VERTICAL,
        (SliderDensity::Standard, SliderOrientation::Horizontal) => SLIDER_ROOT,
        (SliderDensity::Dense, SliderOrientation::Horizontal) => SLIDER_ROOT_DENSE,
    }
}

const fn slider_track_wrap_class(orientation: SliderOrientation) -> &'static str {
    match orientation {
        SliderOrientation::Horizontal => SLIDER_TRACK_WRAP,
        SliderOrientation::Vertical => SLIDER_TRACK_WRAP_VERTICAL,
    }
}

const fn slider_track_class(
    density: SliderDensity,
    orientation: SliderOrientation,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SLIDER_TRACK_DISABLED;
    }
    if invalid {
        return SLIDER_TRACK_INVALID;
    }
    match (density, orientation) {
        (_, SliderOrientation::Vertical) => SLIDER_TRACK_VERTICAL,
        (SliderDensity::Standard, SliderOrientation::Horizontal) => SLIDER_TRACK,
        (SliderDensity::Dense, SliderOrientation::Horizontal) => SLIDER_TRACK_DENSE,
    }
}

const fn slider_range_class(dragging: bool, invalid: bool, disabled: bool) -> &'static str {
    if disabled {
        SLIDER_RANGE_DISABLED
    } else if invalid {
        SLIDER_RANGE_INVALID
    } else if dragging {
        SLIDER_RANGE_DRAGGING
    } else {
        SLIDER_RANGE
    }
}

const fn slider_thumb_class(density: SliderDensity, focused: bool, disabled: bool) -> &'static str {
    if disabled {
        return SLIDER_THUMB_DISABLED;
    }
    if focused {
        return SLIDER_THUMB_FOCUSED;
    }
    match density {
        SliderDensity::Standard => SLIDER_THUMB,
        SliderDensity::Dense => SLIDER_THUMB_DENSE,
    }
}

fn slider_range_style(orientation: SliderOrientation, percent: u8) -> String {
    match orientation {
        SliderOrientation::Horizontal => format!("width: {percent}%;"),
        SliderOrientation::Vertical => {
            format!("height: {percent}%; width: 100%; margin-top: auto;")
        }
    }
}

fn slider_thumb_style(orientation: SliderOrientation, percent: u8) -> String {
    match orientation {
        SliderOrientation::Horizontal => {
            format!("left: {percent}%; transform: translate(-50%, -50%);")
        }
        SliderOrientation::Vertical => {
            let top = 100u8.saturating_sub(percent);
            format!("left: 50%; top: {top}%; transform: translate(-50%, -50%);")
        }
    }
}

const fn slider_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    focused: bool,
    dragging: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if loading {
        "loading"
    } else if invalid {
        "invalid"
    } else if dragging {
        "dragging"
    } else if focused {
        "focused"
    } else {
        "ready"
    }
}

#[component]
pub fn Sonner(#[prop(optional, default = default_sonner_model())] model: SonnerModel) -> AnyView {
    if let Err(report) = validate_sonner_model(&model) {
        let message = format!("Sonner validation failed: {report}");
        return view! {
            <div class=SONNER_ERROR data-ui-component="sonner" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let position = model.position;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let state_model = model.state();
    let nodes = sonner_render_nodes(&model, &state_model);
    let provider = nodes
        .iter()
        .find(|node| node.part == SonnerPart::Provider)
        .expect("invariant: sonner render nodes include provider")
        .clone();
    let viewport = nodes
        .iter()
        .find(|node| node.part == SonnerPart::Viewport)
        .expect("invariant: sonner render nodes include viewport")
        .clone();
    let toast_nodes: Vec<SonnerRenderNode> = nodes
        .iter()
        .filter(|node| node.part == SonnerPart::Toast)
        .cloned()
        .collect();
    let action_nodes: Vec<SonnerRenderNode> = nodes
        .iter()
        .filter(|node| node.part == SonnerPart::Action)
        .cloned()
        .collect();
    let dismiss_nodes: Vec<SonnerRenderNode> = nodes
        .iter()
        .filter(|node| node.part == SonnerPart::Dismiss)
        .cloned()
        .collect();
    let provider_value = provider.value.clone();
    let provider_label = provider.label.clone();
    let provider_detail = provider.detail.clone();
    let viewport_value = viewport.value.clone();
    let viewport_label = viewport.label.clone();
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=sonner_provider_class(density, invalid, disabled)
            data-ui-component="sonner"
            data-ui-part=SonnerPart::Provider.label()
            data-ui-density=density.label()
            data-ui-position=position.label()
            data-ui-state=move || {
                state.with(|state| {
                    sonner_provider_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_paused(),
                        state.active_value(),
                    )
                })
            }
            data-ui-value=provider_value
            aria-label=provider_label
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
            on:mouseenter=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SonnerIntent::Pause);
                    });
                }
            }
            on:mouseleave=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SonnerIntent::Resume);
                    });
                }
            }
        >
            <p class=SONNER_META>{provider_detail}</p>
            <div
                class=sonner_viewport_class(density, position)
                data-ui-part=SonnerPart::Viewport.label()
                data-ui-value=viewport_value
                aria-label=viewport_label
                aria-live="polite"
                aria-relevant="additions removals"
            >
                {toast_nodes
                    .into_iter()
                    .map(|toast| {
                        let action = action_nodes
                            .iter()
                            .find(|node| node.index == toast.index)
                            .cloned();
                        let dismiss = dismiss_nodes
                            .iter()
                            .find(|node| node.index == toast.index)
                            .cloned()
                            .expect("invariant: sonner toast render nodes include dismiss");
                        sonner_toast_view(toast, action, dismiss, state, set_state)
                    })
                    .collect_view()}
            </div>
            {invalid.then_some(view! { <p class=SONNER_ERROR>{provider.detail}</p> })}
        </section>
    }
    .into_any()
}

fn sonner_toast_view(
    toast: SonnerRenderNode,
    action: Option<SonnerRenderNode>,
    dismiss: SonnerRenderNode,
    state: ReadSignal<SonnerState>,
    set_state: WriteSignal<SonnerState>,
) -> AnyView {
    let toast_value = toast.toast_value.clone();
    let active_value = toast_value.clone();
    let hidden_value = toast_value.clone();
    let state_value = toast_value.clone();
    let focus_value = toast_value.clone();
    let blur_disabled = toast.disabled;
    let toast_disabled = toast.disabled;
    let toast_invalid = toast.invalid;
    let toast_tone = toast.tone;
    let toast_density = toast.density;
    let toast_label = toast.label.clone();
    let toast_detail = toast.detail.clone();
    let toast_data_value = toast.value.clone();
    let toast_position = toast.position;
    let action_view = action.map(|action| {
        let action_label = action.label.clone();
        let action_value = action.value.clone();
        let action_toast_value = action.toast_value.clone();
        let action_state_value = action.toast_value.clone();
        let action_disabled = action.disabled;
        let actioned = action.actioned;
        view! {
            <button
                type="button"
                class=move || {
                    state.with(|state| {
                        sonner_action_class(
                            actioned || state.is_actioned(&action_state_value),
                            action_disabled,
                        )
                    })
                }
                data-ui-part=SonnerPart::Action.label()
                data-ui-value=action_value
                disabled=action_disabled
                on:click=move |_| {
                    if !action_disabled {
                        set_state.update(|state| {
                            let _ = state.apply(SonnerIntent::Activate(action_toast_value.clone()));
                        });
                    }
                }
            >
                {action_label}
            </button>
        }
    });
    let dismiss_label = dismiss.label.clone();
    let dismiss_value = dismiss.value.clone();
    let dismiss_toast_value = dismiss.toast_value.clone();
    let dismiss_state_value = dismiss.toast_value.clone();
    let dismiss_disabled = dismiss.disabled;

    view! {
        <article
            class=move || {
                state.with(|state| {
                    sonner_toast_class(
                        toast_density,
                        toast_tone,
                        state.is_active(&active_value),
                        toast_invalid,
                        toast_disabled,
                    )
                })
            }
            data-ui-part=SonnerPart::Toast.label()
            data-ui-tone=toast_tone.label()
            data-ui-position=toast_position.label()
            data-ui-value=toast_data_value
            data-ui-state=move || {
                state.with(|state| {
                    sonner_toast_state_label(
                        toast_disabled,
                        toast_invalid,
                        state.is_dismissed(&state_value),
                        state.is_actioned(&state_value),
                        state.is_active(&state_value),
                    )
                })
            }
            role="status"
            aria-atomic="true"
            aria-disabled=toast_disabled.to_string()
            hidden=move || state.with(|state| state.is_dismissed(&hidden_value))
            on:mouseenter=move |_| {
                if !toast_disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SonnerIntent::Focus(focus_value.clone()));
                    });
                }
            }
            on:mouseleave=move |_| {
                if !blur_disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SonnerIntent::Blur);
                    });
                }
            }
        >
            <div class=SONNER_HEADER>
                <div class=SONNER_COPY>
                    <h3 class=SONNER_TITLE>{toast_label}</h3>
                    <p class=SONNER_DESCRIPTION>{toast_detail}</p>
                </div>
                <button
                    type="button"
                    class=move || {
                        state.with(|state| {
                            sonner_dismiss_class(
                                state.is_active(&dismiss_state_value),
                                dismiss_disabled,
                            )
                        })
                    }
                    data-ui-part=SonnerPart::Dismiss.label()
                    data-ui-value=dismiss_value
                    aria-label=dismiss_label
                    disabled=dismiss_disabled
                    on:click=move |_| {
                        if !dismiss_disabled {
                            set_state.update(|state| {
                                let _ = state.apply(SonnerIntent::Dismiss(
                                    dismiss_toast_value.clone(),
                                ));
                            });
                        }
                    }
                >
                    "x"
                </button>
            </div>
            {action_view.map(|view| view! { <div class=SONNER_ACTION_ROW>{view}</div> })}
        </article>
    }
    .into_any()
}

const fn sonner_provider_class(
    density: SonnerDensity,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SONNER_PROVIDER_DISABLED;
    }
    if invalid {
        return SONNER_PROVIDER_INVALID;
    }
    match density {
        SonnerDensity::Standard => SONNER_PROVIDER,
        SonnerDensity::Dense => SONNER_PROVIDER_DENSE,
    }
}

const fn sonner_viewport_class(density: SonnerDensity, position: SonnerPosition) -> &'static str {
    match (density, position) {
        (SonnerDensity::Dense, SonnerPosition::BottomCenter) => SONNER_VIEWPORT_DENSE_CENTER,
        (SonnerDensity::Dense, SonnerPosition::TopRight | SonnerPosition::BottomRight) => {
            SONNER_VIEWPORT_DENSE_END
        }
        (SonnerDensity::Standard, SonnerPosition::BottomCenter) => SONNER_VIEWPORT_CENTER,
        (SonnerDensity::Standard, SonnerPosition::TopRight | SonnerPosition::BottomRight) => {
            SONNER_VIEWPORT_END
        }
    }
}

const fn sonner_toast_class(
    density: SonnerDensity,
    tone: SonnerTone,
    active: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SONNER_TOAST_DISABLED;
    }
    if invalid {
        return SONNER_TOAST_INVALID;
    }
    if active {
        return SONNER_TOAST_ACTIVE;
    }
    match (density, tone) {
        (SonnerDensity::Dense, SonnerTone::Info) => SONNER_TOAST_INFO_DENSE,
        (SonnerDensity::Dense, SonnerTone::Success) => SONNER_TOAST_SUCCESS_DENSE,
        (SonnerDensity::Dense, SonnerTone::Warning) => SONNER_TOAST_WARNING_DENSE,
        (SonnerDensity::Dense, SonnerTone::Destructive) => SONNER_TOAST_DESTRUCTIVE_DENSE,
        (SonnerDensity::Dense, SonnerTone::Default) => SONNER_TOAST_DENSE,
        (SonnerDensity::Standard, SonnerTone::Info) => SONNER_TOAST_INFO,
        (SonnerDensity::Standard, SonnerTone::Success) => SONNER_TOAST_SUCCESS,
        (SonnerDensity::Standard, SonnerTone::Warning) => SONNER_TOAST_WARNING,
        (SonnerDensity::Standard, SonnerTone::Destructive) => SONNER_TOAST_DESTRUCTIVE,
        (SonnerDensity::Standard, SonnerTone::Default) => SONNER_TOAST,
    }
}

const fn sonner_action_class(active: bool, disabled: bool) -> &'static str {
    if disabled {
        SONNER_ACTION_DISABLED
    } else if active {
        SONNER_ACTION_ACTIVE
    } else {
        SONNER_ACTION
    }
}

const fn sonner_dismiss_class(active: bool, disabled: bool) -> &'static str {
    if disabled {
        SONNER_DISMISS_DISABLED
    } else if active {
        SONNER_DISMISS_ACTIVE
    } else {
        SONNER_DISMISS
    }
}

fn sonner_provider_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    paused: bool,
    active_value: Option<&str>,
) -> String {
    if disabled {
        "disabled".to_owned()
    } else if loading {
        "loading".to_owned()
    } else if invalid {
        "invalid".to_owned()
    } else if paused {
        "paused".to_owned()
    } else if let Some(value) = active_value {
        format!("active-{value}")
    } else {
        "ready".to_owned()
    }
}

const fn sonner_toast_state_label(
    disabled: bool,
    invalid: bool,
    dismissed: bool,
    actioned: bool,
    active: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if invalid {
        "invalid"
    } else if dismissed {
        "dismissed"
    } else if actioned {
        "actioned"
    } else if active {
        "active"
    } else {
        "visible"
    }
}

#[component]
pub fn Spinner(
    #[prop(optional, default = default_spinner_model())] model: SpinnerModel,
) -> AnyView {
    if let Err(report) = validate_spinner_model(&model) {
        let message = format!("Spinner validation failed: {report}");
        return view! {
            <div class=SPINNER_ERROR data-ui-component="spinner" data-ui-state="invalid" role="alert">
                {message}
            </div>
        }
        .into_any();
    }

    let density = model.density;
    let size = model.size;
    let tone = model.tone;
    let loading = model.loading;
    let disabled = model.disabled;
    let invalid = model.error.is_some();
    let state_model = model.state();
    let nodes = spinner_render_nodes(&model, &state_model);
    let root = nodes
        .iter()
        .find(|node| node.part == SpinnerPart::Root)
        .expect("invariant: spinner render nodes include root")
        .clone();
    let track = nodes
        .iter()
        .find(|node| node.part == SpinnerPart::Track)
        .expect("invariant: spinner render nodes include track")
        .clone();
    let indicator = nodes
        .iter()
        .find(|node| node.part == SpinnerPart::Indicator)
        .expect("invariant: spinner render nodes include indicator")
        .clone();
    let label = nodes
        .iter()
        .find(|node| node.part == SpinnerPart::Label)
        .expect("invariant: spinner render nodes include label")
        .clone();
    let root_value = root.value.clone();
    let root_label = root.label.clone();
    let root_detail = root.detail.clone();
    let track_value = track.value.clone();
    let indicator_value = indicator.value.clone();
    let label_value = label.value.clone();
    let label_visible = label.visible;
    let (state, set_state) = signal(state_model);

    view! {
        <section
            class=spinner_root_class(density, invalid, disabled)
            data-ui-component="spinner"
            data-ui-part=SpinnerPart::Root.label()
            data-ui-density=density.label()
            data-ui-size=size.label()
            data-ui-tone=tone.label()
            data-ui-state=move || {
                state.with(|state| {
                    spinner_state_label(
                        loading,
                        disabled,
                        invalid,
                        state.is_paused(),
                        state.is_active(SpinnerPart::Root),
                    )
                })
            }
            data-ui-value=root_value
            aria-label=root_label
            aria-busy=loading.to_string()
            aria-disabled=disabled.to_string()
            tabindex=if disabled { "-1" } else { "0" }
            on:focus=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SpinnerIntent::Focus(SpinnerPart::Root));
                    });
                }
            }
            on:blur=move |_| {
                if !disabled {
                    set_state.update(|state| {
                        let _ = state.apply(SpinnerIntent::Blur);
                    });
                }
            }
            on:mouseenter=move |_| {
                if !disabled && loading {
                    set_state.update(|state| {
                        let _ = state.apply(SpinnerIntent::Pause);
                    });
                }
            }
            on:mouseleave=move |_| {
                if !disabled && loading {
                    set_state.update(|state| {
                        let _ = state.apply(SpinnerIntent::Resume);
                    });
                }
            }
            on:click=move |_| {
                if !disabled && loading {
                    set_state.update(|state| {
                        let _ = state.apply(SpinnerIntent::ToggleMotion);
                    });
                }
            }
        >
            <span
                class=spinner_track_class(size, invalid, disabled)
                data-ui-part=SpinnerPart::Track.label()
                data-ui-value=track_value
                aria-hidden=(!loading).to_string()
                hidden=!loading
            >
                <span
                    class=move || {
                        state.with(|state| {
                            spinner_indicator_class(
                                tone,
                                state.is_paused(),
                                invalid,
                                disabled,
                            )
                        })
                    }
                    style=spinner_indicator_style(indicator.speed_ms)
                    data-ui-part=SpinnerPart::Indicator.label()
                    data-ui-value=indicator_value
                    aria-hidden="true"
                ></span>
            </span>
            <span
                class=spinner_label_class(density)
                data-ui-part=SpinnerPart::Label.label()
                data-ui-value=label_value
                hidden=!label_visible
            >
                {label.label}
            </span>
            <span class="sr-only">{root_detail}</span>
            {invalid.then_some(view! { <p class=SPINNER_ERROR>{root.detail}</p> })}
        </section>
    }
    .into_any()
}

const fn spinner_root_class(
    density: SpinnerDensity,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SPINNER_ROOT_DISABLED;
    }
    if invalid {
        return SPINNER_ROOT_INVALID;
    }
    match density {
        SpinnerDensity::Standard => SPINNER_ROOT,
        SpinnerDensity::Dense => SPINNER_ROOT_DENSE,
    }
}

const fn spinner_track_class(size: SpinnerSize, invalid: bool, disabled: bool) -> &'static str {
    if disabled {
        return SPINNER_TRACK_DISABLED;
    }
    if invalid {
        return SPINNER_TRACK_INVALID;
    }
    match size {
        SpinnerSize::Small => SPINNER_TRACK_SMALL,
        SpinnerSize::Medium => SPINNER_TRACK_MEDIUM,
        SpinnerSize::Large => SPINNER_TRACK_LARGE,
    }
}

const fn spinner_indicator_class(
    tone: SpinnerTone,
    paused: bool,
    invalid: bool,
    disabled: bool,
) -> &'static str {
    if disabled {
        return SPINNER_INDICATOR_DISABLED;
    }
    if invalid {
        return SPINNER_INDICATOR_INVALID;
    }
    if paused {
        return SPINNER_INDICATOR_PAUSED;
    }
    match tone {
        SpinnerTone::Default => SPINNER_INDICATOR_DEFAULT,
        SpinnerTone::Brand => SPINNER_INDICATOR_BRAND,
        SpinnerTone::Info => SPINNER_INDICATOR_INFO,
        SpinnerTone::Success => SPINNER_INDICATOR_SUCCESS,
        SpinnerTone::Warning => SPINNER_INDICATOR_WARNING,
        SpinnerTone::Destructive => SPINNER_INDICATOR_DESTRUCTIVE,
    }
}

const fn spinner_label_class(density: SpinnerDensity) -> &'static str {
    match density {
        SpinnerDensity::Standard => SPINNER_LABEL,
        SpinnerDensity::Dense => SPINNER_LABEL_DENSE,
    }
}

fn spinner_indicator_style(speed_ms: u16) -> String {
    format!("animation-duration: {speed_ms}ms;")
}

const fn spinner_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    paused: bool,
    active: bool,
) -> &'static str {
    if disabled {
        "disabled"
    } else if invalid {
        "invalid"
    } else if !loading {
        "ready"
    } else if paused {
        "paused"
    } else if active {
        "active"
    } else {
        "loading"
    }
}

#[component]
pub fn Switch(#[prop(optional, default = default_switch_model())] model: SwitchModel) -> AnyView {
    if let Err(report) = validate_switch_model(&model) {
        let message = format!("Switch validation failed: {report}");
        return view! {
            <div class=SWITCH_ERROR data-ui-component="switch" data-ui-state="invalid" role="alert">
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
    let nodes = switch_render_nodes(&model, &model.state());
    let root = nodes
        .iter()
        .find(|node| node.part == SwitchPart::Root)
        .expect("invariant: switch render nodes include root")
        .clone();
    let track = nodes
        .iter()
        .find(|node| node.part == SwitchPart::Track)
        .expect("invariant: switch render nodes include track")
        .clone();
    let thumb = nodes
        .iter()
        .find(|node| node.part == SwitchPart::Thumb)
        .expect("invariant: switch render nodes include thumb")
        .clone();
    let label = nodes
        .iter()
        .find(|node| node.part == SwitchPart::Label)
        .expect("invariant: switch render nodes include label")
        .clone();
    let required = root.required;
    let root_value = root.value.clone();
    let track_value = track.value.clone();
    let thumb_value = thumb.value.clone();
    let label_value = label.value.clone();
    let root_label = root.label.clone();
    let label_copy = label.label.clone();
    let label_detail = label.detail.clone();
    let error_detail = label.detail.clone();
    let on_status_for_data = model.on_label.clone();
    let off_status_for_data = model.off_label.clone();
    let on_status_for_text = model.on_label.clone();
    let off_status_for_text = model.off_label.clone();
    let (state, set_state) = signal(model.state());

    view! {
        <div
            class=switch_root_class(density, disabled, invalid)
            data-ui-component="switch"
            data-ui-part=SwitchPart::Root.label()
            data-ui-density=density.label()
            data-ui-state=move || {
                state.with(|state| {
                    switch_state_label(loading, disabled, invalid, state.checked()).to_owned()
                })
            }
            data-ui-checked=move || state.with(|state| state.checked().label().to_owned())
            data-ui-value=root_value
            data-ui-status=move || {
                state.with(|state| {
                    switch_status_copy(
                        state.checked(),
                        on_status_for_data.as_str(),
                        off_status_for_data.as_str(),
                    )
                })
            }
            aria-disabled=blocked.to_string()
            aria-busy=loading.to_string()
            aria-invalid=invalid.to_string()
        >
            <div class=SWITCH_COPY>
                <div class=SWITCH_LABEL_ROW data-ui-part=SwitchPart::Label.label() data-ui-value=label_value>
                    <p class=switch_label_class(disabled)>
                        {label_copy}
                        {required.then_some(view! { <span class=SWITCH_REQUIRED>" *"</span> })}
                    </p>
                    <span
                        class=move || {
                            state.with(|state| {
                                switch_status_class(state.checked(), disabled, invalid).to_owned()
                            })
                        }
                    >
                        {move || {
                            state.with(|state| {
                                switch_status_copy(
                                    state.checked(),
                                    on_status_for_text.as_str(),
                                    off_status_for_text.as_str(),
                                )
                            })
                        }}
                    </span>
                </div>
                <p class=switch_detail_class(disabled, invalid)>{label_detail}</p>
                {invalid.then_some(view! { <p class=SWITCH_ERROR>{error_detail}</p> })}
            </div>
            <button
                type="button"
                role="switch"
                class=move || {
                    state.with(|state| {
                        switch_track_class(
                            density,
                            state.checked(),
                            state.is_active(SwitchPart::Track),
                            blocked,
                            invalid,
                        )
                        .to_owned()
                    })
                }
                data-ui-part=SwitchPart::Track.label()
                data-ui-value=track_value
                aria-label=root_label
                aria-checked=move || state.with(|state| state.checked().aria_checked().to_owned())
                disabled=blocked
                on:focus=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(SwitchIntent::Focus(SwitchPart::Track));
                        });
                    }
                }
                on:blur=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(SwitchIntent::Blur);
                        });
                    }
                }
                on:click=move |_| {
                    if !blocked {
                        set_state.update(|state| {
                            let _ = state.apply(SwitchIntent::Toggle);
                        });
                    }
                }
            >
                <span
                    class=move || {
                        state.with(|state| {
                            switch_thumb_class(
                                density,
                                state.checked(),
                                state.is_active(SwitchPart::Track),
                                blocked,
                                invalid,
                            )
                            .to_owned()
                        })
                    }
                    data-ui-part=SwitchPart::Thumb.label()
                    data-ui-value=thumb_value
                    aria-hidden="true"
                ></span>
            </button>
        </div>
    }
    .into_any()
}

const fn switch_root_class(density: SwitchDensity, disabled: bool, invalid: bool) -> &'static str {
    if disabled {
        return SWITCH_ROOT_DISABLED;
    }
    if invalid {
        return SWITCH_ROOT_INVALID;
    }
    match density {
        SwitchDensity::Standard => SWITCH_ROOT,
        SwitchDensity::Dense => SWITCH_ROOT_DENSE,
    }
}

const fn switch_track_class(
    density: SwitchDensity,
    checked: SwitchChecked,
    active: bool,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        return SWITCH_TRACK_DISABLED;
    }
    if invalid {
        return SWITCH_TRACK_INVALID;
    }
    if active {
        return match (density, checked) {
            (SwitchDensity::Dense, SwitchChecked::On) => SWITCH_TRACK_DENSE_ACTIVE_ON,
            (SwitchDensity::Dense, SwitchChecked::Off) => SWITCH_TRACK_DENSE_ACTIVE,
            (SwitchDensity::Standard, SwitchChecked::On) => SWITCH_TRACK_ACTIVE_ON,
            (SwitchDensity::Standard, SwitchChecked::Off) => SWITCH_TRACK_ACTIVE,
        };
    }
    match (density, checked) {
        (SwitchDensity::Dense, SwitchChecked::On) => SWITCH_TRACK_DENSE_ON,
        (SwitchDensity::Dense, SwitchChecked::Off) => SWITCH_TRACK_DENSE,
        (SwitchDensity::Standard, SwitchChecked::On) => SWITCH_TRACK_ON,
        (SwitchDensity::Standard, SwitchChecked::Off) => SWITCH_TRACK,
    }
}

const fn switch_thumb_class(
    density: SwitchDensity,
    checked: SwitchChecked,
    active: bool,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        return SWITCH_THUMB_DISABLED;
    }
    if invalid {
        return SWITCH_THUMB_INVALID;
    }
    if active {
        return match density {
            SwitchDensity::Standard => SWITCH_THUMB_ACTIVE,
            SwitchDensity::Dense => SWITCH_THUMB_DENSE_ACTIVE,
        };
    }
    match (density, checked) {
        (SwitchDensity::Dense, SwitchChecked::Off | SwitchChecked::On) => SWITCH_THUMB_DENSE,
        (SwitchDensity::Standard, SwitchChecked::On) => SWITCH_THUMB_ON,
        (SwitchDensity::Standard, SwitchChecked::Off) => SWITCH_THUMB,
    }
}

const fn switch_label_class(disabled: bool) -> &'static str {
    if disabled {
        SWITCH_LABEL_DISABLED
    } else {
        SWITCH_LABEL
    }
}

const fn switch_detail_class(disabled: bool, invalid: bool) -> &'static str {
    if disabled {
        SWITCH_DETAIL_DISABLED
    } else if invalid {
        SWITCH_DETAIL_INVALID
    } else {
        SWITCH_DETAIL
    }
}

const fn switch_status_class(
    checked: SwitchChecked,
    disabled: bool,
    invalid: bool,
) -> &'static str {
    if disabled {
        SWITCH_STATUS_DISABLED
    } else if invalid {
        SWITCH_STATUS_INVALID
    } else if checked.is_on() {
        SWITCH_STATUS_ON
    } else {
        SWITCH_STATUS
    }
}

fn switch_status_copy(checked: SwitchChecked, on_label: &str, off_label: &str) -> String {
    match checked {
        SwitchChecked::Off => off_label.to_owned(),
        SwitchChecked::On => on_label.to_owned(),
    }
}

const fn switch_state_label(
    loading: bool,
    disabled: bool,
    invalid: bool,
    checked: SwitchChecked,
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
