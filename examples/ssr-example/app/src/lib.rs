use leptos::{
    component, create_effect, create_signal, log, tracing, view, IntoView, MaybeSignal, SignalGet,
};
use leptos_leaflet::leaflet as L;
use leptos_leaflet::{
    position, positions, Circle, MapContainer, MapEvents, Marker, Popup, Position, TileLayer,
    Tooltip,
};
use leptos_leaflet_hotline::*;
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
        <Stylesheet id="leptos" href="/pkg/leptos-leaflet-hotline.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

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
    let (pos, _set_pos) = create_signal(Position::new(39.8283, -98.5795));
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
        <MapContainer style="height: 100vh" center=Position::new(39.8283, -98.5795) zoom=18.0 set_view=false map=set_map locate=false watch=true events>
            <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
            <Marker position=pos >
                <Popup>
                    <strong>{"A pretty CSS3 popup"}</strong>
                </Popup>
            </Marker>
            <Tooltip position=pos permanent=true direction="top">
                <strong>{"This is the center of USA"}</strong>
            </Tooltip>
            <Hotline positions=positions(&[(39.8283, -98.5795), (39.7283, -77.456707), (39.8283, -98.4795)])/>
            <Circle center=position!(39.8293, -98.5785) color="#0000CC" radius=200.0 class_name="mycircle">
                <Tooltip sticky=true permanent=true>{"This is a circle"}</Tooltip>
            </Circle>
        </MapContainer>
    }
}
