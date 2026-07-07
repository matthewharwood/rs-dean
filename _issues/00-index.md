# shadcn Component Implementation Backlog

Source inventory: https://ui.shadcn.com/docs/components

Each task tracks the dual Leptos/Bevy component contract for rs-dean. The
current sweep implements all 64 entries through shared recipes, literal widget
constructors, named Leptos exports, story coverage, Bevy primitive derivation,
and the shared `garde`-validated widget render-node contract.

- [Implementation Sweep Log](sweep-log.md)
- [Accordion](01-accordion.md) - Disclosure, Ephemeral.
- [Alert](02-alert.md) - Feedback, Stateless.
- [Alert Dialog](03-alert-dialog.md) - Overlay, Ephemeral.
- [Aspect Ratio](04-aspect-ratio.md) - Layout, Stateless.
- [Attachment](05-attachment.md) - Messaging, ConsumerDurable.
- [Avatar](06-avatar.md) - Display, Stateless.
- [Badge](07-badge.md) - Display, Stateless.
- [Breadcrumb](08-breadcrumb.md) - Navigation, Ephemeral.
- [Bubble](09-bubble.md) - Messaging, Ephemeral.
- [Button](10-button.md) - Action, Ephemeral.
- [Button Group](11-button-group.md) - Action, Ephemeral.
- [Calendar](12-calendar.md) - Form, ConsumerDurable.
- [Card](13-card.md) - Display, Stateless.
- [Carousel](14-carousel.md) - Display, Ephemeral.
- [Chart](15-chart.md) - Data, ConsumerDurable.
- [Checkbox](16-checkbox.md) - Form, ConsumerDurable.
- [Collapsible](17-collapsible.md) - Disclosure, Ephemeral.
- [Combobox](18-combobox.md) - Form, ConsumerDurable.
- [Command](19-command.md) - Navigation, Ephemeral.
- [Context Menu](20-context-menu.md) - Overlay, Ephemeral.
- [Data Table](21-data-table.md) - Data, ConsumerDurable.
- [Date Picker](22-date-picker.md) - Form, ConsumerDurable.
- [Dialog](23-dialog.md) - Overlay, Ephemeral.
- [Direction](24-direction.md) - Utility, Ephemeral.
- [Drawer](25-drawer.md) - Overlay, Ephemeral.
- [Dropdown Menu](26-dropdown-menu.md) - Overlay, Ephemeral.
- [Empty](27-empty.md) - Feedback, Stateless.
- [Field](28-field.md) - Form, ConsumerDurable.
- [Hover Card](29-hover-card.md) - Overlay, Ephemeral.
- [Input](30-input.md) - Form, ConsumerDurable.
- [Input Group](31-input-group.md) - Form, ConsumerDurable.
- [Input OTP](32-input-otp.md) - Form, ConsumerDurable.
- [Item](33-item.md) - Display, Stateless.
- [Kbd](34-kbd.md) - Display, Stateless.
- [Label](35-label.md) - Form, Stateless.
- [Marker](36-marker.md) - Messaging, Ephemeral.
- [Menubar](37-menubar.md) - Navigation, Ephemeral.
- [Message](38-message.md) - Messaging, ConsumerDurable.
- [Message Scroller](39-message-scroller.md) - Messaging, Ephemeral.
- [Native Select](40-native-select.md) - Form, ConsumerDurable.
- [Navigation Menu](41-navigation-menu.md) - Navigation, Ephemeral.
- [Pagination](42-pagination.md) - Navigation, ConsumerDurable.
- [Popover](43-popover.md) - Overlay, Ephemeral.
- [Progress](44-progress.md) - Feedback, ConsumerDurable.
- [Radio Group](45-radio-group.md) - Form, ConsumerDurable.
- [Resizable](46-resizable.md) - Layout, ConsumerDurable.
- [Scroll Area](47-scroll-area.md) - Layout, Ephemeral.
- [Select](48-select.md) - Form, ConsumerDurable.
- [Separator](49-separator.md) - Layout, Stateless.
- [Sheet](50-sheet.md) - Overlay, Ephemeral.
- [Sidebar](51-sidebar.md) - Navigation, ConsumerDurable.
- [Skeleton](52-skeleton.md) - Feedback, Ephemeral.
- [Slider](53-slider.md) - Form, ConsumerDurable.
- [Sonner](54-sonner.md) - Feedback, Ephemeral.
- [Spinner](55-spinner.md) - Feedback, Ephemeral.
- [Switch](56-switch.md) - Form, ConsumerDurable.
- [Table](57-table.md) - Data, ConsumerDurable.
- [Tabs](58-tabs.md) - Navigation, ConsumerDurable.
- [Textarea](59-textarea.md) - Form, ConsumerDurable.
- [Toast](60-toast.md) - Feedback, Ephemeral.
- [Toggle](61-toggle.md) - Action, ConsumerDurable.
- [Toggle Group](62-toggle-group.md) - Action, ConsumerDurable.
- [Tooltip](63-tooltip.md) - Overlay, Ephemeral.
- [Typography](64-typography.md) - Typography, Stateless.
