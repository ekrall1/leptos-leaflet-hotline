//! module for hotline position data structures and functions
use js_sys::Array;
use wasm_bindgen::prelude::*;

/// Struct for conventional lat, lng position
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct FlatPosition {
    /// latitude
    pub lat: f64,
    /// longitude
    pub lng: f64,
}

/// Struct for leaflet hotline positions
/// in addition to lat and lng, there is a 3rd dimension \
/// for the value to be visualized.  \
/// This is consistent with JS leaflet-hotline which puts
/// the value in the Leaflet altitude placeholder.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct HotlinePosition {
    /// lat, lng pair for the position
    pub latlng: FlatPosition,
    /// used to hold the value for the position; used for visualization along path
    pub alt: f64,
}

/// Struct for vector of leaflet hotline positions
#[derive(Debug, Default, Clone, PartialEq)]
#[non_exhaustive]
pub struct HotlinePositionVec {
    /// vec of positions
    pub positions: Vec<HotlinePosition>,
}

impl HotlinePositionVec {
    #[must_use]
    #[inline]
    pub fn new(arr: &[(f64, f64, f64)]) -> Self {
        let mut position_vec: Vec<HotlinePosition> = vec![];

        let max_val = arr
            .iter()
            .map(|val| val.2)
            .fold(f64::NEG_INFINITY, f64::max);

        for &elem in arr {
            position_vec.push(HotlinePosition::new(elem.0, elem.1, elem.2 / max_val));
        }

        Self {
            positions: position_vec,
        }
    }
}

impl HotlinePosition {
    #[must_use]
    #[inline]
    pub const fn new(lat: f64, lng: f64, alt: f64) -> Self {
        Self {
            latlng: FlatPosition { lat, lng },
            alt,
        }
    }

    #[must_use]
    #[inline]
    pub const fn get_lat(&self) -> f64 {
        self.latlng.lat
    }

    #[must_use]
    #[inline]
    pub const fn get_lng(&self) -> f64 {
        self.latlng.lng
    }
}

/// to override the latlng bindings with the 3-d data structure the hotline JS code uses
/// eventually, re-write so the values are in a separate array,
/// instead of being passed in the z dimension like the current leaflet-hotline
/// then, drop this once there is no need to have a separate latlng binding
#[wasm_bindgen(js_namespace=L)]
extern "C" {
    /// LatLng type for hotlines (lat, lng, value)
    #[derive(Debug, Default, Clone)]
    pub type LatLng;

    /// construct a new hotline [`LatLng`]
    ///
    /// # Returns
    /// [`LatLng`]
    ///
    #[wasm_bindgen(constructor, js_name = LatLng)]
    pub fn new(lat: f64, lng: f64, alt: f64) -> LatLng;

    ///
    /// get hotline position latitude
    ///
    /// # Returns
    /// [`f64`]
    ///
    #[wasm_bindgen(method, getter)]
    pub fn lat(this: &LatLng) -> f64;

    ///
    /// get hotline position longitude
    ///
    /// # Returns
    /// [`f64`]
    ///
    #[wasm_bindgen(method, getter)]
    pub fn lng(this: &LatLng) -> f64;

    ///
    /// get hotline position value
    ///
    /// # Returns
    /// [`f64`]
    ///
    #[wasm_bindgen(method, getter)]
    pub fn alt(this: &LatLng) -> f64;

    ///
    /// set hotline position latitude
    ///
    /// # Returns
    /// [`f64`]
    ///
    #[wasm_bindgen(method, setter)]
    pub fn set_lat(this: &LatLng, value: f64) -> f64;

    ///
    /// set hotline position longitude
    ///
    /// # Returns
    /// [`f64`]
    ///
    #[wasm_bindgen(method, setter)]
    pub fn set_lng(this: &LatLng, value: f64) -> f64;

    ///
    /// set hotline position value
    ///
    /// # Returns
    /// [`f64`]
    ///
    #[wasm_bindgen(method, setter)]
    pub fn set_alt(this: &LatLng, value: f64) -> f64;
}

/// Creates a JS Array of objects of type ```{lat: number, lng: number, alt: number}``` \
/// for passing to the JS leaflet-hotline code through wasm bindings.  This
/// must be used to convert a slice of [`HotlinePosition`] values to JS Array,
/// prior to calling the hotline constructor bound to JS by wasm-bindgen
///
/// # Args
/// `vals`: slice of hotline positions with values.
///
/// # Returns
/// JS Array of objects containing hotline positions and values.
///
/// # Examples
///
/// Basic usage:
/// ```no_run
/// let positions = &[(40.293, -105.618, 25.0), (40.2928, -105.6190, 0.0)];
/// let position_vec = leptos_leaflet_hotline::HotlinePositionVec::new(positions);
/// let arr = leptos_leaflet_hotline::to_hotline_lat_lng_array(&position_vec);
/// assert!(arr.is_array())
/// ```
///
#[must_use]
#[inline]
pub fn to_hotline_lat_lng_array(vals: &HotlinePositionVec) -> Array {
    let array = Array::new();
    for val in vals.positions.iter().copied() {
        let new_latlng = LatLng::new(val.get_lat(), val.get_lng(), val.alt);
        array.push(&new_latlng);
    }
    array
}
