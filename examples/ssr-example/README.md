# SSR example

This is the runnable Leptos/Axum example restored from the repository's `main`
branch and updated for Leptos 0.8 and leptos-leaflet 0.10.

From the repository root:

```sh
nix develop
cd examples/ssr-example
cargo-leptos watch
```

Invoking `cargo-leptos` directly ensures the executable from the Nix shell wins
even when Cargo's home directory contains an older installed subcommand.

Then open <http://127.0.0.1:3000>. The development server rebuilds and reloads
the server-rendered app and its hydrated browser bundle as files change.

The browser loads Leaflet, Leaflet.hotline, and OpenStreetMap tiles from their
public CDNs, so displaying the map requires an internet connection. All Rust,
WebAssembly, Sass, and cargo-leptos build tools come from the repository's Nix
flake; Docker is not used.

For an optimized build, run:

```sh
cargo-leptos build --release
```
