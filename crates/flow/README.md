# permission-flow

`permission-flow` provides a Rust-friendly API for presenting the macOS
permission guidance flow backed by the Swift/AppKit implementation in this
workspace.

## What it does

- Creates a `PermissionFlowController` on the macOS main thread
- Starts guided flows for supported privacy panes
- Infers a reasonable host app bundle with `AppPath::suggested_host_app()`
- Exposes host-app permission status checks through
  `Permission::authorization_state()`

## Important status warning

`Permission::authorization_state()` reports what the current host process or
host app can determine about its own permission state.

It does **not** authoritatively answer whether an arbitrary target `.app`
bundle already has the requested permission.

## Runtime note

Because the final macOS executable links Swift runtime libraries, downstream
binary crates currently need this in their `build.rs`:

```rust
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    }
}
```

## Example

```rust
use permission_flow::{AppPath, Permission, PermissionFlowController, StartFlowOptions};

let controller = PermissionFlowController::new()?;
let app_path = AppPath::suggested_host_app()
    .expect("expected a host app bundle in this launch context");

controller.start_flow(StartFlowOptions::new(Permission::ACCESSIBILITY, app_path))?;
# Ok::<(), Box<dyn std::error::Error>>(())
```
