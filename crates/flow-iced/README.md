# permission-flow-iced

Headless `iced` helpers for [`permission-flow`](../flow).

`PermissionFlowButton` eagerly owns its own `PermissionFlowController`, so each
button can manage its own permission-flow lifecycle without relying on a global
controller.

It also provides a built-in `subscription()` and `update(...)` pair for
host-app status refreshes, so apps do not need to hand-roll timer or
window-focus refresh logic themselves.

## Platform behavior

`permission-flow-iced` is intended for macOS, but it compiles on other
platforms too because the underlying `permission-flow` crate now exposes a
no-op controller there.

Outside macOS, button presses do nothing and status resolves to `Unknown`, but
your `iced` app can still build cleanly in a cross-platform workspace.

## Important status warning

`Permission::authorization_state()` and `PermissionFlowButton::button_state()`
reflect what the current host process or host app can determine about its own
permission state.

They do **not** verify whether the arbitrary `.app` bundle you pass in
`StartFlowOptions` or `PermissionFlowButton::new(...)` already has that
permission.

In practice:

- If the suggested app bundle is the current host app, the status is meaningful.
- If the suggested app bundle is some other app, treat the status as host-app
  information only, not as an authoritative target-app check.

## Example

Run the included example with:

```bash
cargo run -p permission-flow-iced --example button
```

The example infers a host app bundle from the current launch context, starts
the permission flow for that app, and refreshes the displayed status when the
window regains focus.
