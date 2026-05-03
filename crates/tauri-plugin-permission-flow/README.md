# Tauri Plugin permission-flow

Tauri bindings for the `permission-flow` macOS permission UI.

The plugin is designed for macOS, but it compiles in cross-platform Tauri
workspaces too. Outside macOS, controller creation still succeeds, flow
commands become no-ops, and host-app status resolves to `unknown`.

## Install

Add the Rust plugin in your Tauri app's `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-permission-flow = "0.1.39"
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
    "tauri-plugin-permission-flow-api": "0.1.39"
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
import {
  Permission,
  PermissionFlow,
  watchAuthorizationStatus,
} from 'tauri-plugin-permission-flow-api'

const flow = await PermissionFlow.create()

await flow.startFlow({
  permission: Permission.Accessibility,
  appPath: '/Applications/MyApp.app',
})

await flow.stopCurrentFlow()
await flow.close()

const stopWatching = watchAuthorizationStatus(
  Permission.Accessibility,
  (state) => {
    console.log('Current host-app status:', state)
  }
)

stopWatching()
```

## API Shape

The plugin intentionally exposes two separate layers:

- `PermissionFlow`: a handle-backed controller for opening and closing the floating guidance UI
- `authorizationState(...)` / `watchAuthorizationStatus(...)`: host-app status helpers that do not need a controller handle

`startFlow` accepts:

- `permission`: one of `Permission.Accessibility`, `Permission.InputMonitoring`, `Permission.ScreenRecording`, `Permission.AppManagement`, `Permission.Bluetooth`, `Permission.DeveloperTools`, `Permission.FullDiskAccess`, or `Permission.MediaAppleMusic`
- `appPath`: the app bundle path to suggest inside the permission flow
- `useClickSourceFrame`: optional, defaults to `true`

`watchAuthorizationStatus`:

- immediately publishes the current host-app status by default, even if it was already granted before your app started
- then republishes only when the status actually changes
- refreshes on window focus, on visibility changes, and on a light interval by default
- returns a cleanup function you can call when your UI unmounts

## Notes

- This plugin is intended for macOS.
- `PermissionFlow` is a Tauri resource handle. It owns one native controller on Tauri's main thread and marshals calls there internally.
- The JS wrapper now adds best-effort garbage-collection cleanup through `FinalizationRegistry`, but `close()` is still the deterministic and recommended cleanup path.
- `authorizationState(...)` and `watchAuthorizationStatus(...)` are host-app status helpers. On macOS, the public permission-status APIs used by `permission-flow` describe the current host app or host process, not an arbitrary `.app` bundle path.
- In other words, `appPath` controls which app bundle appears in the floating guidance flow, but it should not be confused with an authoritative "does that target app already have permission?" check.
