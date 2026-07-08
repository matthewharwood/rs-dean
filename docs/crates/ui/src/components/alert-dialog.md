# Alert Dialog

A blocking confirmation dialog for destructive or high-friction actions.

## Live Fixtures

The embedded stories surface renders pre-filled fixtures for this component's
variants, states, themed rendering, and validation paths.

<iframe title="Alert Dialog live story fixtures" src="../../../stories/#ui-alert-dialog" loading="lazy" style="width: 100%; min-height: 44rem; border: 1px solid #d0d7de; border-radius: 8px;"></iframe>

Open the [full stories page](../../../stories/#ui-alert-dialog) when a wider canvas is
needed.

## Contract

| Field | Value |
| --- | --- |
| Category | Overlay |
| Framework | Shared Spec |
| State | Ephemeral |
| Render contract | Shared Leptos/Bevy |
| State contract | Renderer ephemeral |
| Layout contract | Overlay surface |

## Variants

- modal
- nonmodal
- anchored
- danger

## States

- closed
- open
- hovered
- focused
- reduced motion

## Anatomy

- AlertDialog
- AlertDialogTrigger
- AlertDialogContent
- AlertDialogHeader
- AlertDialogFooter
- AlertDialogAction
- AlertDialogCancel

## Accessibility

- Trap focus only for modal surfaces.
- Restore focus to the trigger on close.
- Expose title and description for dialog-like content.

## Consumer Implementation

Renderer owns transient open, hover, focus, and animation state; consumer receives semantic events only when useful.

## End User Outcome

Users must explicitly confirm or cancel a high-risk action before it mutates state.
