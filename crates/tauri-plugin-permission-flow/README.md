# Tauri Plugin permission-flow

Tauri bindings for the `permission-flow` macOS permission UI.

## Install

Add the Rust plugin in your Tauri app's `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-permission-flow = "0.1.38"
```

Register it in your Tauri builder:

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_permission_flow::init())
```

Add the JavaScript package:

```json
{
  "dependencies": {
    "tauri-plugin-permission-flow-api": "0.1.38"
  }
}
```

Because the final macOS executable links the Swift runtime, add this to your app's `src-tauri/build.rs`:

```rust
fn main() {
    tauri_build::build();

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    }
}
```

## Use

```ts
import { Permission, startFlow, stopCurrentFlow } from 'tauri-plugin-permission-flow-api'

await startFlow({
  permission: Permission.Accessibility,
  appPath: '/Applications/MyApp.app',
})

await stopCurrentFlow()
```

`startFlow` accepts:

- `permission`: one of `Permission.Accessibility`, `Permission.InputMonitoring`, `Permission.ScreenRecording`, `Permission.AppManagement`, `Permission.Bluetooth`, `Permission.DeveloperTools`, `Permission.FullDiskAccess`, or `Permission.MediaAppleMusic`
- `appPath`: the app bundle path to suggest inside the permission flow
- `useClickSourceFrame`: optional, defaults to `true`

## Notes

- This plugin is intended for macOS.
- The plugin keeps its controller on Tauri's main thread and marshals commands there internally.
- This plugin currently exposes the flow launcher, not a target-app permission-status API. On macOS, the public permission-status APIs used by `permission-flow` describe the current host app or host process, not an arbitrary `.app` bundle path.
- In other words, `appPath` controls which app bundle appears in the floating guidance flow, but it should not be confused with an authoritative "does that target app already have permission?" check.
