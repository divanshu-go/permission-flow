# permission-flow

[![CI](https://github.com/veecore/permission-flow/actions/workflows/ci.yml/badge.svg)](https://github.com/veecore/permission-flow/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/permission-flow.svg)](https://crates.io/crates/permission-flow)
[![docs.rs](https://img.shields.io/docsrs/permission-flow)](https://docs.rs/permission-flow)

`permission-flow` is a macOS-first workspace for guiding users through system
permission onboarding from Rust apps.

The workspace currently contains:

- `permission-flow`: the core Rust crate backed by the Swift/AppKit flow
- `permission-flow-iced`: `iced`-friendly helpers on top of the core crate
- `tauri-plugin-permission-flow`: a Tauri plugin and guest-side JS bindings

## Install

Add the core crate:

```toml
[dependencies]
permission-flow = "0.1.39"
```

## Publishing notes

Because the final macOS executable links against Swift runtime libraries,
downstream binary crates currently need this in their `build.rs`:

```rust
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    }
}
```

## Acknowledgements

This workspace builds on top of the excellent
[`PermissionFlow`](https://github.com/jaywcjlove/PermissionFlow) Swift package
and uses [`swift-rs`](https://github.com/Brendonovich/swift-rs) for the Rust to
Swift bridge.

## Development

The CI pipeline checks:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --lib`
- `cargo check -p permission-flow-iced --examples`
- `cargo publish -p permission-flow --dry-run`
