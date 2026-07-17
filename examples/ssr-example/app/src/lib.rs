//! Server-rendered example for `leptos-leaflet-hotline`.

use leptos::prelude::*;
use leptos_leaflet::prelude::*;
#[cfg(target_arch = "wasm32")]
use leptos_leaflet_hotline::{ColorFormat, HotlineColorLookup};
use leptos_leaflet_hotline::{HotPolyline, HotlinePalette, HotlinePositionVec};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{closure::Closure, JsValue};

#[cfg(target_arch = "wasm32")]
type ConsoleLookup = Closure<dyn Fn(f64, f64, JsValue) -> Result<String, JsValue>>;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static CONSOLE_LOOKUP: std::cell::RefCell<Option<ConsoleLookup>> = const {
        std::cell::RefCell::new(None)
    };
}

#[cfg(target_arch = "wasm32")]
fn install_console_lookup(lookup: HotlineColorLookup) {
    let callback = ConsoleLookup::new(move |lat: f64, lng: f64, requested_format: JsValue| {
        let format = if requested_format.is_undefined() || requested_format.is_null() {
            ColorFormat::Hex
        } else {
            let requested_format = requested_format.as_string().ok_or_else(|| {
                JsValue::from(js_sys::TypeError::new("format must be \"hex\" or \"rgb\""))
            })?;

            match requested_format.to_ascii_lowercase().as_str() {
                "hex" => ColorFormat::Hex,
                "rgb" => ColorFormat::Rgb,
                _ => {
                    return Err(JsValue::from(js_sys::TypeError::new(
                        "format must be \"hex\" or \"rgb\"",
                    )))
                }
            }
        };

        lookup
            .color_at(lat, lng, format)
            .map_err(|error| JsValue::from(js_sys::Error::new(&error.to_string())))
    });

    let global = js_sys::global();
    let property = JsValue::from_str("hotlineColorAt");
    if js_sys::Reflect::set(&global, &property, callback.as_ref()).is_err() {
        leptos::logging::error!("Could not install window.hotlineColorAt");
        return;
    }

    leptos::logging::log!("Console lookup ready: hotlineColorAt(lat, lng, \"hex\" | \"rgb\")");

    CONSOLE_LOOKUP.with(|slot| {
        *slot.borrow_mut() = Some(callback);
    });
}

#[derive(Clone)]
struct HoverPoint {
    lat: f64,
    lng: f64,
    hex: String,
}

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
    let hover_point = RwSignal::new(None::<HoverPoint>);
    let positions = HotlinePositionVec::new(&[
        (40.2928, -105.6180, 0.59),
        (40.2928, -105.6190, 0.07),
        (40.2928, -105.6200, 1.00),
        (40.2918, -105.6210, 0.26),
    ]);
    let palette = HotlinePalette::new(&[
        ("green", 0.0),
        ("blue", 0.33),
        ("#ffff00", 0.67),
        ("white", 1.0),
    ]);

    #[cfg(target_arch = "wasm32")]
    let map_events = match HotlineColorLookup::new(&positions, &palette, 0.0, 1.0) {
        Ok(lookup) => {
            install_console_lookup(lookup.clone());

            MapEvents::new().mouse_move(move |event| {
                let lat_lng = event.lat_lng();
                let lat = lat_lng.lat();
                let lng = lat_lng.lng();

                match lookup.color_at(lat, lng, ColorFormat::Hex) {
                    Ok(hex) => hover_point.set(Some(HoverPoint { lat, lng, hex })),
                    Err(error) => {
                        leptos::logging::error!("Could not look up hovered hotline color: {error}")
                    }
                }
            })
        }
        Err(error) => {
            leptos::logging::error!("Could not prepare hotline color lookup: {error}");
            MapEvents::new()
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let map_events = MapEvents::new();

    Effect::new(move |_| {
        if let Some(map) = map.get() {
            leptos::logging::log!("Leaflet map ready at zoom {}", map.get_zoom());
        }
    });

    view! {
        <main class="example-shell">
            <MapContainer
                class="example-map"
                center=Position::new(40.2928, -105.6170)
                zoom=17.0
                map=set_map
                events=map_events
            >
                <TileLayer
                    url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                    attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
                />
                <HotPolyline
                    positions=positions
                    palette=palette
                    outline_color="#5a5a5a"
                    min=0.0
                    max=1.0
                >
                    <Tooltip sticky=true direction="top">
                        <div class="hotline-hover-tooltip">
                            <span>"lat"</span>
                            <code>{move || {
                                hover_point
                                    .get()
                                    .map(|point| format!("{:.6}", point.lat))
                                    .unwrap_or_default()
                            }}</code>
                            <span>"lng"</span>
                            <code>{move || {
                                hover_point
                                    .get()
                                    .map(|point| format!("{:.6}", point.lng))
                                    .unwrap_or_default()
                            }}</code>
                            <span>"hex"</span>
                            <code>{move || {
                                hover_point
                                    .get()
                                    .map(|point| point.hex)
                                    .unwrap_or_default()
                            }}</code>
                        </div>
                    </Tooltip>
                </HotPolyline>
            </MapContainer>
        </main>
    }
}
