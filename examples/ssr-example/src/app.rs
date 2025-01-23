//! Example using leptos-leaflet-hotline [HotPolyline] component
use leptos::{component, logging::*, view, IntoView};
use leptos::prelude::{AutoReload, ClassAttribute, Effect, ElementChild, Get, GlobalAttributes, HydrationScripts, LeptosOptions};
use leptos_meta::MetaTags;
use leptos_leaflet::leaflet as L;
use leptos_leaflet::prelude::{JsRwSignal, create_map_signal, MapContainer, MapEvents, Position, TileLayer};
use leptos_leaflet_hotline::{HotPolyline, HotlinePalette, HotlinePositionVec};

use leptos_meta::{provide_meta_context, Script, Stylesheet, Title};
use leptos_router::components::*;
use leptos_router::path;

/// app html shell
pub fn shell(options: LeptosOptions) -> impl IntoView {

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Stylesheet id="leaflet" href="https://unpkg.com/leaflet@1.9.3/dist/leaflet.css"/>
                <Script src="https://unpkg.com/leaflet@1.9.3/dist/leaflet.js"/>
                <Script src="https://unpkg.com/leaflet-hotline@0.4.0/src/leaflet.hotline.js" />
                <Stylesheet id="leptos" href="/pkg/leptos-leaflet-hotline-ssr.css"/>
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

/// Container for the example app. \
/// Returns [IntoView] that converts RSX values defining context, stylesheets, scripts, title,
/// and content into a view that can be mounted to the DOM.
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // sets the document title
        <Title text="leptos-leaflet-hotline example"/>

        // content for this page
        <Router>
            <main>
                <Routes fallback=|| "404 Not found">
                    <Route path=path!("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (_, _set_pos) = JsRwSignal::new_local(Position::new(40.2928, -105.6200)).split();
    let (map, set_map) = create_map_signal();

    Effect::new(move |_| {
        if let Some(map) = map.get() {
            log!("Map context {:?}", map.get_zoom());
        }
    });

    let location_found = move |loc: L::LocationEvent| {
        log!("Hi from {:?}", loc.lat_lng());
    };

    let events = MapEvents::new().location_found(location_found);

    view! {
        <div class="map-parent" >
        <MapContainer class="map-container" center=Position::new(40.2928, -105.6170) zoom=17.0 map=set_map set_view=true events>
            <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
            <HotPolyline
                positions=HotlinePositionVec::new(&[(40.2928, -105.6180, 56.54), (40.2928, -105.6190, 6.80), (40.2928, -105.6200, 96.52), (40.2918, -105.6210, 24.91)])
                palette=HotlinePalette::new(&[("green", 0.0), ("blue", 0.33), ("#ffff00", 0.67), ("red", 1.0)])
                outline_color="#5a5a5a"
                max=1.0
                min=0.0
            />
        </MapContainer>
        </div>
    }
}
