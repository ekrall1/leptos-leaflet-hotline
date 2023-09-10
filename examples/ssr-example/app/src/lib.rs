use leptos::{component, create_effect, create_signal, log, tracing, view, IntoView, SignalGet};
use leptos_leaflet::leaflet as L;
use leptos_leaflet::{MapContainer, MapEvents, Position, TileLayer, Tooltip};
<<<<<<< HEAD
use leptos_leaflet_hotline::*;
=======
use leptos_leaflet_hotline::{
    hotline_palette, hotline_positions, hotline_prop_string, HotPolyline,
};
>>>>>>> 1c90e3d (add bindings for clickTolerance, getBounds and setStyle. Make optional prop for outline color.  remove getRGBFromValue method b/c bindgen is not picking up _renderer object)
use leptos_meta::{provide_meta_context, Script, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leaflet" href="https://unpkg.com/leaflet@1.9.3/dist/leaflet.css"/>
        <Script src="https://unpkg.com/leaflet@1.9.3/dist/leaflet.js"/>
        <Script src="https://unpkg.com/leaflet-hotline@0.4.0/src/leaflet.hotline.js" />
        <Stylesheet id="leptos" href="/pkg/leptos-leaflet-hotline.css"/>

        // sets the document title
        <Title text="leptos-leaflet-hotline example"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (pos, _set_pos) = create_signal(Position::new(40.2928, -105.6200));
    let (map, set_map) = create_signal(None::<L::Map>);

    create_effect(move |_| {
        if let Some(map) = map.get() {
            log!("Map context {:?}", map.getZoom());
        }
    });

    let location_found = move |loc: L::LocationEvent| {
        log!("Hi from {:?}", loc.latlng());
    };

    let events = MapEvents::new().location_found(location_found);

    view! {
        <MapContainer style="height: 100vh" center=Position::new(40.2928, -105.6170) zoom=17.0 set_view=false map=set_map locate=false watch=true events>
            <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
            <Tooltip position=pos permanent=true direction="top">
                <strong>{"A tooltip"}</strong>
            </Tooltip>
            <HotPolyline
                positions=hotline_positions(&[(40.2928, -105.6180, 56.54), (40.2928, -105.6190, 6.80), (40.2928, -105.6200, 96.52), (40.2918, -105.6210, 24.91)])
                palette=hotline_palette(&[("green", 0.0), ("blue", 0.33), ("#ffff00", 0.67), ("red", 1.0)])
                outline_color=hotline_prop_string("#5a5a5a")
<<<<<<< HEAD
<<<<<<< HEAD
                max=hotline_prop_float(1.0)
                min=hotline_prop_float(0.0)
=======
>>>>>>> c316473 (add some methods, reorganize modules.  remove getRGBFromValue method b/c bindgen is not picking up _renderer object)
=======
>>>>>>> 1c90e3d (add bindings for clickTolerance, getBounds and setStyle. Make optional prop for outline color.  remove getRGBFromValue method b/c bindgen is not picking up _renderer object)
            />
        </MapContainer>
    }
}
