use js_sys::Array;
use wasm_bindgen::prelude::*;

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
            latlng: FlatPosition { lat, lng },
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
/// eventually, re-write so the values are in a separate array,
/// instead of being passed in the z dimension like the current leaflet-hotline
/// then, drop this once there is no need to have a separate latlng binding
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

pub fn to_hotline_lat_lng_array(vals: &[HotlinePosition]) -> Array {
    let array = Array::new();
    for val in vals.iter().cloned() {
        let new_latlng = LatLng::new(val.get_lat(), val.get_lng(), val.alt);
        array.push(&new_latlng);
    }
    array
}

pub fn hotline_positions(positions: &[(f64, f64, f64)]) -> Vec<HotlinePosition> {
    let normed = &normalize_hotline_vals(positions);
    normed
        .iter()
        .map(|&position| HotlinePosition::new(position.0, position.1, position.2))
        .collect()
}

pub fn normalize_hotline_vals(positions: &[(f64, f64, f64)]) -> Vec<(f64, f64, f64)> {
    let max_val: f64 = positions
        .iter()
        .map(|val| val.2)
        .fold(f64::NEG_INFINITY, f64::max);
    let normed: Vec<(f64, f64, f64)> = positions
        .iter()
        .map(|&(lat, lng, val)| (lat, lng, val / max_val))
        .collect();
    normed
}