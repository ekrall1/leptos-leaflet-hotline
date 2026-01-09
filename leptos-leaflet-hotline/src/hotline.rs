//! module for hotline wasm JS bindings, structs and functions
#[path = "./hotline_palette.rs"]
pub mod hotline_palette;
#[path = "./hotline_position.rs"]
pub mod hotline_position;

use js_sys::{Array, Function, Object, Reflect, Uint8ClampedArray};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use leptos_leaflet::leaflet as L;

#[wasm_bindgen]
extern "C" {

    /// struct for binding to leaflet-hotline JS Object containing hotline options
    #[wasm_bindgen(extends = L::PolylineOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type HotlineOptions;

    /// set the hotline palette
    ///
    /// # Returns
    /// [`HotlineOptions`]
    ///
    #[wasm_bindgen(method, setter)]
    pub fn set_palette(this: &HotlineOptions, palette: &JsValue) -> HotlineOptions;

    /// set the hotline outline color
    ///
    /// # Returns
    /// [`HotlineOptions`]
    ///
    #[wasm_bindgen(method, setter, js_name = "outlineColor")]
    pub fn set_outline_color(this: &HotlineOptions, color: &JsValue) -> HotlineOptions;

    /// set the hotline max breakpoint threshold
    ///
    /// # Returns
    /// [`HotlineOptions`]
    ///
    #[wasm_bindgen(method, setter)]
    pub fn set_max(this: &HotlineOptions, max: &JsValue) -> HotlineOptions;

    /// set the hotline min breakpoint threshold
    ///
    /// # Returns
    /// [`HotlineOptions`]
    ///
    #[wasm_bindgen(method, setter)]
    pub fn set_min(this: &HotlineOptions, min: &JsValue) -> HotlineOptions;

    /// struct for binding to leaflet-hotline JS L::Hotline class
    ///
    #[wasm_bindgen(extends = L::Polyline)]
    #[derive(Debug, Clone)]
    pub type Hotline;

    /// construct a new [`Hotline`]
    ///
    /// # Returns
    /// [`Hotline`]
    ///
    #[wasm_bindgen(constructor, js_namespace=L)]
    pub fn new(hotline_data: &Array, opts: &JsValue) -> Hotline;

    /// [`Hotline`] click tolerance
    ///
    /// # Returns
    /// [`Object`]
    ///
    #[wasm_bindgen(method, js_name = "_clickTolerance")]
    pub fn _click_tolerance(this: &Hotline) -> Object;

    /// get lat, lng bounds for [`Hotline`]
    ///
    /// # Returns
    /// [`Object`]
    ///
    #[wasm_bindgen(method, js_name = "getBounds")]
    pub fn get_bounds(this: &Hotline) -> Object;

    /// set a style property for [`Hotline`]
    ///
    /// # Returns
    /// [`Object`]
    ///
    #[wasm_bindgen(method, js_name = "setStyle")]
    pub fn set_style(this: &Hotline, style: &Object) -> Object;

}

///
/// implement constructor and conversions of properties to [`JsValue`] for [`HotlineOptions`]
impl HotlineOptions {
    ///
    /// construct new [`HotlineOptions`]
    ///
    /// # Returns
    /// [`HotlineOptions`]
    ///
    #[must_use]
    #[inline]
    pub fn new(palette: &JsValue, outline_color: &JsValue, max: &JsValue, min: &JsValue) -> Self {
        let opts: Self = JsCast::unchecked_into(Object::new());
        opts.set_palette(&palette);
        opts.set_outline_color(&outline_color);
        opts.set_max(&max);
        opts.set_min(&min);
        opts
    }
}

///
/// implement functions to set outline color, set max breakpoint threshold,
/// and set min breakpoint threshold for [`Hotline`]
///
#[wasm_bindgen]
impl Hotline {
    /// set a new outline color for the hotline after it has already been created; \
    /// creates JS object with outlineColor k,v pair and calls set_style on self
    #[inline]
    pub fn set_outline_color_val(&self, color: &str) {
        let obj = js_sys::Object::new();
        Reflect::set(&obj, &"outlineColor".into(), &JsValue::from(color)).unwrap_or(true);

        // Call the set_style method with the created object.
        self.set_style(&obj);
    }

    /// set the max breakpoint threshold for [`Hotline`]
    #[inline]
    pub fn set_max_val(&self, max: f64) {
        let obj = js_sys::Object::new();
        Reflect::set(&obj, &"max".into(), &JsValue::from_f64(max)).unwrap_or(true);

        self.set_style(&obj);
    }

    /// set the min breakpoint threshold for [`Hotline`]
    #[inline]
    pub fn set_min_val(&self, min: f64) {
        let obj = js_sys::Object::new();
        Reflect::set(&obj, &"min".into(), &JsValue::from_f64(min)).unwrap_or(true);

        self.set_style(&obj);
    }

    /// get the rendered hex color for a given lat/lng if it is on the hotline
    #[must_use]
    pub fn color_for_lat_lng(&self, lat: f64, lng: f64) -> Option<String> {
        let map = Reflect::get(self, &JsValue::from_str("_map")).ok()?;
        if map.is_null() || map.is_undefined() {
            return None;
        }

        let lat_lng = Array::new();
        lat_lng.push(&JsValue::from_f64(lat));
        lat_lng.push(&JsValue::from_f64(lng));

        let to_point_fn: Function =
            Reflect::get(&map, &JsValue::from_str("latLngToContainerPoint"))
                .ok()?
                .dyn_into()
                .ok()?;
        let point = to_point_fn.call1(&map, &lat_lng).ok()?;
        let x = Reflect::get(&point, &JsValue::from_str("x"))
            .ok()?
            .as_f64()?;
        let y = Reflect::get(&point, &JsValue::from_str("y"))
            .ok()?
            .as_f64()?;

        let renderer = Reflect::get(self, &JsValue::from_str("_renderer")).ok()?;
        let ctx: CanvasRenderingContext2d = Reflect::get(&renderer, &JsValue::from_str("_ctx"))
            .ok()?
            .dyn_into()
            .ok()?;
        let image_data = ctx.get_image_data(x.round(), y.round(), 1.0, 1.0).ok()?;
        let data: Uint8ClampedArray = image_data.data();

        let alpha = data.get_index(3);
        if alpha == 0 {
            return None;
        }

        let red = data.get_index(0);
        let green = data.get_index(1);
        let blue = data.get_index(2);

        Some(format!("#{:02X}{:02X}{:02X}", red, green, blue))
    }
}

/// implement [`From<Hotline>`] for [`leptos_leaflet::leaflet::Layer`]
/// similar to the impl used for `From<Polyline>` for Layer in leptos-leaflet
/// see: <https://github.com/headless-studio/leptos-leaflet/>
/// specifically: <https://github.com/headless-studio/leptos-leaflet/blob/main/leaflet/src/shapes/polyline.rs/>
impl From<Hotline> for L::Layer {
    #[inline]
    fn from(value: Hotline) -> Self {
        value.unchecked_into()
    }
}
