use std::ops::DerefMut;

use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

use leptos_leaflet::leaflet as L;
use leptos::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FlatPosition {
    pub lat: f64,
    pub lng: f64,
}

/// leaflet-hotline uses the 'z' dimension for the hotline values
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HotlinePosition {
    pub latlng: FlatPosition,
    pub alt: f64,
}

impl HotlinePosition {
    pub fn new(lat: f64, lng: f64, alt: f64) -> Self {
        HotlinePosition {
            latlng: FlatPosition {lat, lng},
            alt,
        }
    }

    pub fn get_lat(&self) -> f64 {
        self.latlng.lat
    }

    pub fn get_lng(&self) -> f64 {
        self.latlng.lng
    }
}

/// to override the latlng bindings with the 3-d data structure the hotline JS code uses
/// eventually, re-write hotline functions in rust,
/// with values being passed in a separate array instead of in the z dimension
/// then drop this so there is no need to have a separate latlng binding
#[wasm_bindgen(js_namespace=L)]
extern "C" {
    #[derive(Debug, Default, Clone)]
    pub type LatLng;

    #[wasm_bindgen(constructor, js_name = LatLng)]
    pub fn new(lat: f64, lng: f64, alt: f64) -> LatLng;

    #[wasm_bindgen(method, getter)]
    pub fn lat(this: &LatLng) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn lng(this: &LatLng) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn alt(this: &LatLng) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_lat(this: &LatLng, value: f64) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_lng(this: &LatLng, value: f64) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_alt(this: &LatLng, value: f64) -> f64;
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = L::Polyline)]
    #[derive(Debug, Clone)]
    pub type Hotline;

    #[wasm_bindgen(constructor, js_namespace=L)]
    pub fn new(hotline_data: &Array) -> Hotline;

    #[wasm_bindgen(method)]
    pub fn get_rgb_for_value(this: &Hotline, value: f64) -> Array;

    #[wasm_bindgen(method, js_name="_projectLatLngs")]
    pub fn _project_lat_lngs(this: &Hotline, latlngs: &Array, projected_bounds: &Array);

    #[wasm_bindgen(method, js_name="_clipPoints")]
    pub fn _clip_points(this: &Hotline);

    #[wasm_bindgen(method, js_name="_clipPoints")]
    pub fn _click_tolerance(this: &Hotline);
}

impl From<Hotline> for L::Layer {
    fn from(value: Hotline) -> Self {
        value.unchecked_into()
    }
}

impl From<HotlinePosition> for L::LatLng {
    fn from(value: HotlinePosition) -> Self {
        L::LatLng::new(value.get_lat(), value.get_lng())
    }
}


impl From<&HotlinePosition> for L::LatLng {
    fn from(value: &HotlinePosition) -> Self {
        L::LatLng::new(value.get_lat(), value.get_lng())
    }
}

impl From<HotlinePosition> for (f64, f64) {
    fn from(value: HotlinePosition) -> Self {
        (value.get_lat(), value.get_lng())
    }
}

impl From<HotlinePosition> for (f64, f64, f64) {
    fn from(value: HotlinePosition) -> Self {
        (value.get_lat(), value.get_lng(), value.alt)
    }
}

impl From<HotlinePosition> for [f64; 2] {
    fn from(value: HotlinePosition) -> Self {
        [value.get_lat(), value.get_lng()]
    }
}

impl From<HotlinePosition> for [f64; 3] {
    fn from(value: HotlinePosition) -> Self {
        [value.get_lat(), value.get_lng(), value.alt]
    }
}

// impl From<Vec<HotlinePosition>> for MaybeSignal<Vec<(f64, f64, f64)>> {
//     fn from(values: Vec<HotlinePosition>) -> Self {
        
//     }
// }