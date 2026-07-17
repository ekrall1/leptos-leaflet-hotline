//! Server-rendered example for `leptos-leaflet-hotline`.

use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use leptos_leaflet_hotline::{HotPolyline, HotlinePalette, HotlinePositionVec};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

/// Builds the complete HTML document used for server rendering and hydration.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link
                    rel="stylesheet"
                    href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
                    crossorigin=""
                />
                <script
                    src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"
                    crossorigin=""
                ></script>
                <script src="https://unpkg.com/leaflet-hotline@0.4.0/src/leaflet.hotline.js"></script>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// Provides the example page and its document metadata.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-leaflet-hotline-example.css"/>
        <Title text="leptos-leaflet-hotline example"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=HomePage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (map, set_map) = create_map_signal();

    Effect::new(move |_| {
        if let Some(map) = map.get() {
            leptos::logging::log!("Leaflet map ready at zoom {}", map.get_zoom());
        }
    });

    view! {
        <main class="example-shell">
            <aside class="example-card">
                <h1>"Leptos Leaflet Hotline"</h1>
                <p>"A server-rendered Leptos map with a hydrated gradient polyline."</p>
            </aside>
            <MapContainer
                class="example-map"
                center=Position::new(40.2928, -105.6170)
                zoom=17.0
                map=set_map
            >
                <TileLayer
                    url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                    attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
                />
                <Tooltip position=Position::new(40.2928, -105.6200) permanent=true direction="top">
                    <strong>"A tooltip"</strong>
                </Tooltip>
                <HotPolyline
                    positions=HotlinePositionVec::new(&[
                        (40.2928, -105.6180, 0.59),
                        (40.2928, -105.6190, 0.07),
                        (40.2928, -105.6200, 1.00),
                        (40.2918, -105.6210, 0.26),
                    ])
                    palette=HotlinePalette::new(&[
                        ("green", 0.0),
                        ("blue", 0.33),
                        ("#ffff00", 0.67),
                        ("red", 1.0),
                    ])
                    outline_color="#5a5a5a"
                    min=0.0
                    max=1.0
                />
            </MapContainer>
        </main>
    }
}
