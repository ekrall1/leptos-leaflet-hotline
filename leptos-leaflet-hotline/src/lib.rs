mod blanket_overlay;
mod browser;
mod canvas;
mod hotline;
mod hotline_draw;
mod renderer;
pub use blanket_overlay::{BlanketOverlay, BlanketOverlayOptions};
pub use browser::Browser;
pub use canvas::{Canvas, CanvasOptions};
pub use renderer::{Renderer, RendererOptions};
pub use hotline::{HotlinePosition, Hotline, LatLng as HotlineLatLng};
pub use hotline_draw::HotlineRenderer;

use leptos::*;
use leptos_leaflet::leaflet as L;
use leptos_leaflet::*;
use js_sys::Array;
use wasm_bindgen::JsValue;

pub fn hotline_vals(hotline_vals: &[f64]) -> Vec<f64> {
    hotline_vals.to_vec()
}

pub fn to_val_array<T: Into<JsValue> + Clone>(vals: &[T]) -> Array {
    let arr = Array::new();
    for val in vals.iter().cloned() {
        arr.push(&val.into());
    }
    arr
}

pub fn to_hotline_lat_lng_array(vals: &[HotlinePosition]) -> Array {
    let array = Array::new();
    for val in vals.iter().cloned() {
        let new_latlng = HotlineLatLng::new(val.get_lat(), val.get_lng(), val.alt);
        array.push(&new_latlng);
    }
    array
}

pub fn normalize_it(positions:  &[(f64, f64, f64)]) -> Vec<(f64, f64, f64)> {
   
    let max_val: f64 = positions.iter().map(|val| val.2).fold(f64::NEG_INFINITY, f64::max);
    let normed: Vec<(f64, f64, f64)> = positions.iter().map(|&(lat, lng, val)| (lat, lng, val/max_val)).collect();
    normed
}

pub fn new_hotline_positions(positions: &[(f64, f64, f64)]) -> Vec<HotlinePosition> {
    let normed = &normalize_it(positions);
    normed
        .iter()
        .map(|&position| HotlinePosition::new(position.0, position.1, position.2))
        .collect()
}

#[component(transparent)]
pub fn Hotline(
    #[prop(into)] positions: MaybeSignal<Vec<HotlinePosition>>,
    //#[prop(into)] hotline_vals: MaybeSignal<Vec<f64>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    extend_context_with_overlay();
    let overlay = store_value(None::<L::Polyline>);
    let _positions_for_effect = positions.clone();
    //let _hotline_vals_for_effect = hotline_vals.clone();

    create_effect(move |_| {
        if let Some(map) = use_context::<LeafletMapContext>()
            .expect("map context")
            .map()
        {
            // get browser information, potentially to work with in later functions
            let browser = Browser::default();
            // verify that the browser object has some information
            let chrome = browser.chrome;
            let edge: bool = browser.edge;
            log!("chrome {:?}", chrome && !edge);
            log!("edge {:?}", edge);

            let lat_lngs = to_hotline_lat_lng_array(&positions.get_untracked());
            log!("latlng {:?}", lat_lngs);

            let hotline: Hotline = Hotline::new(&lat_lngs);
            log!("hotline {:?}", hotline);
            hotline.addTo(&map);
            // update_overlay_context(&hotline);
            // overlay.set_value(Some(hotline));

            // let lat_lngs = L::to_lat_lng_array(&positions.get_untracked().to_vec());
            // let options = L::PolylineOptions::new();
            // let hotline: L::Polyline = L::Polyline::new_with_options(&lat_lngs, &options);
            // hotline.addTo(&map);
            // update_overlay_context(&hotline);
            // overlay.set_value(Some(hotline));
        }
    });

    children.map(|child| child())
}
