# permission-flow

`permission-flow` is a macOS-first workspace for guiding users through system
permission onboarding from Rust apps.

The workspace currently contains:

- `permission-flow`: the core Rust crate backed by the Swift/AppKit flow
- `permission-flow-iced`: `iced`-friendly helpers on top of the core crate
- `tauri-plugin-permission-flow`: a Tauri plugin and guest-side JS bindings

## Publishing notes

The core crate is intended to be published first.

Because the final macOS executable links against Swift runtime libraries,
downstream binary crates currently need this in their `build.rs`:

```rust
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
    }
}
```

## Development

The CI pipeline checks:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --lib`
- `cargo check -p permission-flow-iced --examples`
- `cargo publish -p permission-flow --dry-run`
