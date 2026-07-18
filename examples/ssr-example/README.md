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

Hover over the hotline to see a sticky tooltip with the cursor latitude,
longitude, and the hexadecimal route color returned by the Rust lookup.

After hydration, the example exposes its Rust color lookup in the browser
console:

```js
hotlineColorAt(40.2928, -105.6185)        // hex by default
hotlineColorAt(40.2928, -105.6185, "hex")
hotlineColorAt(40.2928, -105.6185, "rgb")
```

Finite geographic coordinates that can be projected are snapped to the
nearest segment in the example hotline. Invalid formats and coordinates throw
a JavaScript error.

The browser loads Leaflet, Leaflet.hotline, and OpenStreetMap tiles from their
public CDNs, so displaying the map requires an internet connection. All Rust,
WebAssembly, Sass, and cargo-leptos build tools come from the repository's Nix
flake; Docker is not used.

For an optimized build, run:

```sh
cargo-leptos build --release
```
