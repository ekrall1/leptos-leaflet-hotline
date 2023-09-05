use js_sys::{Array, Object, Reflect};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use leptos_leaflet::leaflet as L;

const DEFAULT_PALETTE_VALUES: &[(&str, f64)] = &[
    ("green", 0.0),
    ("blue", 0.33),
    ("#ffff00", 0.67),
    ("red", 1.0),
];

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

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = L::PolylineOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type HotlineOptions;

    #[wasm_bindgen(method, setter)]
    pub fn set_palette(this: &HotlineOptions, palette: &JsValue) -> HotlineOptions;

    #[wasm_bindgen(extends = L::Polyline)]
    #[derive(Debug, Clone)]
    pub type Hotline;

    #[wasm_bindgen(constructor, js_namespace=L)]
    pub fn new(hotline_data: &Array, opts: &JsValue) -> Hotline;

    #[wasm_bindgen(method, js_name = "getRGBForValue")]
    pub fn get_rgb_for_value(this: &Hotline, value: f64) -> Array;

}

#[derive(Debug, Clone, PartialEq)]
pub struct HotlinePalette {
    pub palette: HashMap<String, f64>,
}

impl HotlinePalette {
    pub fn new(palette: &[(&str, f64)]) -> Self {
        let mut palette_hashmap = HashMap::new();

        for &(key, val) in palette {
            palette_hashmap.insert(key.to_string(), val);
        }

        HotlinePalette {
            palette: palette_hashmap,
        }
    }
}

impl Default for HotlinePalette {
    fn default() -> Self {
        HotlinePalette::new(DEFAULT_PALETTE_VALUES)
    }
}

impl HotlineOptions {
    pub fn new(palette: &HotlinePalette) -> Self {
        let palette_len = palette.palette.len();
        let js_palette = match palette_len > 0 {
            true => Self::palette_to_js(palette),
            false => Self::palette_to_js(&HotlinePalette::default()),
        };

        let opts: HotlineOptions = JsCast::unchecked_into(Object::new());
        opts.set_palette(&js_palette);
        opts
    }

    pub fn palette_to_js(palette: &HotlinePalette) -> JsValue {
        let palette_opts = Object::new();

        for (color, bkpt) in &palette.palette {
            let _ = Reflect::set(&palette_opts, &JsValue::from_f64(*bkpt), &color.into());
        }

        JsCast::unchecked_into(palette_opts)
    }
}

/// similar to the impl used for From<Polyline> for Layer in leptos-leaflet
/// see: https://github.com/headless-studio/leptos-leaflet
/// specifically: https://github.com/headless-studio/leptos-leaflet/blob/main/leaflet/src/shapes/polyline.rs
impl From<Hotline> for L::Layer {
    fn from(value: Hotline) -> Self {
        value.unchecked_into()
    }
}

pub fn to_hotline_lat_lng_array(vals: &[HotlinePosition]) -> Array {
    let array = Array::new();
    for val in vals.iter().cloned() {
        let new_latlng = LatLng::new(val.get_lat(), val.get_lng(), val.alt);
        array.push(&new_latlng);
    }
    array
}

/// some more helper functions used in creating data structures used by the HotPolyline component
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

pub fn hotline_positions(positions: &[(f64, f64, f64)]) -> Vec<HotlinePosition> {
    let normed = &normalize_hotline_vals(positions);
    normed
        .iter()
        .map(|&position| HotlinePosition::new(position.0, position.1, position.2))
        .collect()
}

pub fn hotline_palette(palette: &[(&str, f64)]) -> HotlinePalette {
    HotlinePalette::new(&palette)
}
