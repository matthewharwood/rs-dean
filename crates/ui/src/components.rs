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
    ComboboxPart, ComponentImplementation, ThemeChoice, ThemeId, UiBlock, UiBlockTone,
    UiComponentId, UiWidgetIntent, UiWidgetPattern, UiWidgetSlotKind, accordion_dom_id,
    alert_dialog_dom_id, aspect_ratio_render_nodes, attachment_render_nodes, avatar_render_nodes,
    badge_render_nodes, breadcrumb_render_nodes, bubble_render_nodes, button_group_render_nodes,
    button_render_nodes, calendar_render_nodes, card_render_nodes, carousel_render_nodes,
    catalog_component_render_nodes, chart_render_nodes, checkbox_render_nodes,
    collapsible_render_nodes, combobox_render_nodes, component_implementation, component_spec,
    default_accordion_items, default_alert_dialog_model, default_alert_model,
    default_aspect_ratio_model, default_attachment_model, default_avatar_model,
    default_badge_model, default_breadcrumb_model, default_bubble_model,
    default_button_group_model, default_button_model, default_calendar_model, default_card_model,
    default_carousel_model, default_chart_model, default_checkbox_model, default_collapsible_model,
    default_combobox_model, month_name, validate_accordion_model, validate_alert_dialog_model,
    validate_alert_model, validate_aspect_ratio_model, validate_attachment_model,
    validate_avatar_model, validate_badge_model, validate_breadcrumb_model, validate_bubble_model,
    validate_button_group_model, validate_button_model, validate_calendar_model,
    validate_card_model, validate_carousel_model, validate_chart_model, validate_checkbox_model,
    validate_collapsible_model, validate_combobox_model,
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

catalog_component!(Command, crate::CommandModel, crate::default_command_model);
catalog_component!(
    ContextMenu,
    crate::ContextMenuModel,
    crate::default_context_menu_model
);
catalog_component!(
    DataTable,
    crate::DataTableModel,
    crate::default_data_table_model
);
catalog_component!(
    DatePicker,
    crate::DatePickerModel,
    crate::default_date_picker_model
);
catalog_component!(Dialog, crate::DialogModel, crate::default_dialog_model);
catalog_component!(
    Direction,
    crate::DirectionModel,
    crate::default_direction_model
);
catalog_component!(Drawer, crate::DrawerModel, crate::default_drawer_model);
catalog_component!(
    DropdownMenu,
    crate::DropdownMenuModel,
    crate::default_dropdown_menu_model
);
catalog_component!(Empty, crate::EmptyModel, crate::default_empty_model);
catalog_component!(Field, crate::FieldModel, crate::default_field_model);
catalog_component!(
    HoverCard,
    crate::HoverCardModel,
    crate::default_hover_card_model
);
catalog_component!(Input, crate::InputModel, crate::default_input_model);
catalog_component!(
    InputGroup,
    crate::InputGroupModel,
    crate::default_input_group_model
);
catalog_component!(
    InputOtp,
    crate::InputOtpModel,
    crate::default_input_otp_model
);
catalog_component!(Item, crate::ItemModel, crate::default_item_model);
catalog_component!(Kbd, crate::KbdModel, crate::default_kbd_model);
catalog_component!(Label, crate::LabelModel, crate::default_label_model);
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
