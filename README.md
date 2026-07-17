# leptos-leaflet-hotline

`leptos-leaflet-hotline` provides a Leptos component for drawing Leaflet polylines whose color changes along the path. It targets Leptos 0.8 and leptos-leaflet 0.10.

The third value in every position is the value visualized by the gradient:

```text
(latitude, longitude, value)
```

## Browser prerequisites

Load these browser assets before the application's WebAssembly starts:

1. Leaflet CSS
2. Leaflet JavaScript
3. Leaflet.hotline JavaScript

For example, when the assets are served locally:

```html
<link rel="stylesheet" href="/leaflet/leaflet.css">
<script src="/leaflet/leaflet.js"></script>
<script src="/leaflet-hotline/leaflet.hotline.js"></script>
```

Leaflet.hotline does not require a separate stylesheet, but the map container must have an explicit height. The plugin uses a canvas renderer and expects Leaflet's global `L` object to be available. It can also be installed with `npm install leaflet leaflet-hotline` and initialized through a JavaScript bundler.

## Cargo setup

Use Leptos 0.8 and leptos-leaflet 0.10, disable their default features, and select one rendering mode through this crate:

```toml
[dependencies]
leptos = { version = "0.8.20", default-features = false }
leptos-leaflet = { version = "0.10.2", default-features = false }
leptos-leaflet-hotline = { git = "https://github.com/ekrall1/leptos-leaflet-hotline", default-features = false, features = ["csr"] }
```

Choose exactly one mode for each build target:

- `csr` for a client-rendered browser application
- `hydrate` for the browser half of a server-rendered application
- `ssr` for the server half

These features are forwarded to both Leptos and leptos-leaflet. Browser builds also require the `wasm32-unknown-unknown` Rust target.

## Runnable SSR example

The restored [Axum example](examples/ssr-example) uses the same three-crate
`app`/`frontend`/`server` layout that was previously on `main`, updated for
Leptos 0.8 hydration and leptos-leaflet 0.10.

The repository flake supplies Rust, the WebAssembly target, cargo-leptos, Sass,
wasm-bindgen, and wasm-opt. Docker is not required:

```sh
nix develop
cd examples/ssr-example
cargo-leptos watch
```

Open <http://127.0.0.1:3000>. See the [example README](examples/ssr-example/README.md)
for release-build and runtime details.

## Usage

```rust
use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use leptos_leaflet_hotline::{HotPolyline, HotlinePalette, HotlinePositionVec};

#[component]
pub fn HotlineMap() -> impl IntoView {
    let positions = HotlinePositionVec::new(&[
        (40.2928, -105.6180, 0.00),
        (40.2928, -105.6190, 0.35),
        (40.2928, -105.6200, 0.70),
        (40.2918, -105.6210, 1.00),
    ]);

    let palette = HotlinePalette::new(&[
        ("green", 0.00),
        ("blue", 0.33),
        ("#ffff00", 0.67),
        ("red", 1.00),
    ]);

    view! {
        <MapContainer
            style="height: 400px"
            center=Position::new(40.2928, -105.6195)
            zoom=15.0
            set_view=true
        >
            <TileLayer
                url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                attribution="&copy; OpenStreetMap contributors"
            />
            <HotPolyline
                positions=positions
                palette=palette
                outline_color="white"
                min=0.0
                max=1.0
            />
        </MapContainer>
    }
}
```

`HotlinePalette` maps colors to normalized stops between `0.0` and `1.0`. `outline_color`, `min`, and `max` are optional.

## Attribution

This project builds on:

- [iosphere/Leaflet.hotline](https://github.com/iosphere/Leaflet.hotline), whose JavaScript API is exposed through `wasm-bindgen`
- [headless-studio/leptos-leaflet](https://github.com/headless-studio/leptos-leaflet), whose component and context structure this crate follows
- [Leaflet](https://github.com/Leaflet/Leaflet), the underlying interactive mapping library

See the linked projects for their respective licenses. This crate is licensed under the [MIT License](LICENSE).
