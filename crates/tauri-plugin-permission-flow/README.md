# Tauri Plugin permission-flow

[![CI](https://github.com/veecore/permission-flow/actions/workflows/ci.yml/badge.svg)](https://github.com/veecore/permission-flow/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/tauri-plugin-permission-flow.svg)](https://crates.io/crates/tauri-plugin-permission-flow)
[![docs.rs](https://img.shields.io/docsrs/tauri-plugin-permission-flow)](https://docs.rs/tauri-plugin-permission-flow)
[![npm](https://img.shields.io/npm/v/%40veecore%2Ftauri-plugin-permission-flow-api)](https://www.npmjs.com/package/@veecore/tauri-plugin-permission-flow-api)

Tauri bindings for the `permission-flow` macOS permission UI.

The plugin is designed for macOS, but it compiles in cross-platform Tauri
workspaces too. Outside macOS, controller creation still succeeds, flow
commands become no-ops, and host-app status resolves to `unknown`.

![permission-flow demo](https://raw.githubusercontent.com/veecore/permission-flow/main/assets/demo-readme.gif)

## What you get

The Tauri package gives you two layers:

- `PermissionFlow`: a handle-backed controller for opening the floating
  permission guidance UI
- `authorizationState(...)` and `watchAuthorizationStatus(...)`: host-app
  status helpers for reflecting permission state in your frontend

Use the controller when you want to start the flow. Use the status helpers when
you want your UI to react to the current host app's permission state.

## Install

Add the Rust plugin in your Tauri app's `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-permission-flow = "0.1.40"
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
    "@veecore/tauri-plugin-permission-flow-api": "0.1.40"
  }
}
```

Because the final macOS executable links the Swift runtime, add this to your
app's `src-tauri/build.rs`:

```rust
fn main() {
    tauri_build::build();

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    }
}
```

## Quick start

```ts
import {
  Permission,
  PermissionFlow,
  suggestedHostAppPath,
  watchAuthorizationStatus,
} from '@veecore/tauri-plugin-permission-flow-api'

const flow = await PermissionFlow.create()
const appPath = await suggestedHostAppPath()

if (appPath) {
  await flow.startFlow({
    permission: Permission.Accessibility,
    appPath,
  })
}

const stopWatching = watchAuthorizationStatus(
  Permission.Accessibility,
  (state) => {
    console.log('Current host-app status:', state)
  }
)

stopWatching()
await flow.close()
```

## API shape

`startFlow` accepts:

- `permission`: one of `Permission.Accessibility`, `Permission.InputMonitoring`, `Permission.ScreenRecording`, `Permission.AppManagement`, `Permission.Bluetooth`, `Permission.DeveloperTools`, `Permission.FullDiskAccess`, or `Permission.MediaAppleMusic`
- `appPath`: the app bundle path to suggest inside the permission flow
- `useClickSourceFrame`: optional, defaults to `true`

`suggestedHostAppPath()`:

- returns a best-effort `.app` bundle path for the current launch context
- is a good default for examples, dev builds, IDE-integrated terminals, and
  bundled Tauri apps
- returns `null` when there is no meaningful host app bundle to infer

`watchAuthorizationStatus`:

- immediately publishes the current host-app status by default, even if it was already granted before your app started
- then republishes only when the status actually changes
- refreshes on window focus, on visibility changes, and on a light interval by default
- returns a cleanup function you can call when your UI unmounts

## Example app

The repo includes a tiny demo app under
`crates/tauri-plugin-permission-flow/examples/tauri-app`.

It intentionally stays minimal:

- one permission picker
- one button
- button state driven by the host-app authorization watcher

## Bundling the resource bundle

The SwiftPM target underlying this plugin ships a localized resource bundle
(`PermissionFlow_PermissionFlow.bundle`). On Tauri hosts, that bundle has to be
copied into your `.app`'s `Contents/Resources/` directory, otherwise the
onboarding panel will fail to localize strings and may crash with
`fatalError("could not load resource bundle...")` the first time a user opens
it.

This plugin's `build.rs` publishes the bundle's on-disk path via Cargo's
`links`-key metadata, so consuming apps don't have to scan hashed Cargo
`OUT_DIR`s to find it. The mechanism follows the official Cargo Book pattern
("The links Manifest Key" + the `libz-sys` example).

### Step 1 — read the bundle path from your app's `build.rs`

```rust
fn main() {
    // ... your other build logic ...

    #[cfg(target_os = "macos")]
    {
        // Cargo synthesizes this from `cargo:bundle-dir=...` emitted by this
        // plugin's build script. The `DEP_<LINKS-NAME-UPPER>_<KEY-UPPER>`
        // convention is documented in the Cargo Book; the LINKS name comes
        // from `links = "tauri-plugin-permission-flow"` in our Cargo.toml.
        println!(
            "cargo:rerun-if-env-changed=DEP_TAURI_PLUGIN_PERMISSION_FLOW_BUNDLE_DIR"
        );
        let bundle_src = std::env::var("DEP_TAURI_PLUGIN_PERMISSION_FLOW_BUNDLE_DIR")
            .expect(
                "tauri-plugin-permission-flow did not publish bundle-dir; \
                 ensure you're on a recent enough plugin revision.",
            );

        // Stage the bundle next to your Cargo manifest so Tauri's resource
        // glob (see step 2) can pick it up.
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let bundle_dst = std::path::PathBuf::from(&manifest_dir)
            .join("PermissionFlow_PermissionFlow.bundle");
        let _ = std::fs::remove_dir_all(&bundle_dst);
        // Use any recursive copy helper you like; the bundle is a directory
        // with .lproj subdirectories.
        copy_dir_recursive(
            std::path::Path::new(&bundle_src),
            &bundle_dst,
        )
        .expect("failed to stage permission-flow resource bundle");
    }
}
```

### Step 2 — list the bundle in `tauri.conf.json`

```jsonc
{
  "bundle": {
    "resources": [
      "PermissionFlow_PermissionFlow.bundle"
    ]
  }
}
```

Tauri copies it into `Contents/Resources/PermissionFlow_PermissionFlow.bundle/`
at package time. The Swift code in `permission-flow` uses a layered
`Bundle.permissionFlow` resolver that finds the bundle at that path, falls
back gracefully if it isn't there, and never `fatalError`s on first access.

### Why this indirection?

In principle the consuming app could read
`DEP_PERMISSION_FLOW_BUNDLE_BUNDLE_DIR` directly from the underlying
`permission-flow` crate, but Cargo's `DEP_*` metadata only propagates to
**direct** dependents. Most Tauri apps depend on this plugin, not on the
lower-level `permission-flow` crate, so they never see that env var. This
plugin's `build.rs` re-emits the path under its own `links` key to bridge the
hop.

## Notes

- This plugin is intended for macOS.
- `PermissionFlow` is a Tauri resource handle. It owns one native controller on Tauri's main thread and marshals calls there internally.
- The JS wrapper now adds best-effort garbage-collection cleanup through `FinalizationRegistry`, but `close()` is still the deterministic and recommended cleanup path.
- `authorizationState(...)` and `watchAuthorizationStatus(...)` are host-app status helpers. On macOS, the public permission-status APIs used by `permission-flow` describe the current host app or host process, not an arbitrary `.app` bundle path.
- In other words, `appPath` controls which app bundle appears in the floating guidance flow, but it should not be confused with an authoritative "does that target app already have permission?" check.

## Maintainer Note

The guest package is published from GitHub Actions through npm trusted
publishing. The repository workflow lives at
`.github/workflows/publish-npm.yml`.
