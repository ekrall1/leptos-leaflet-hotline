//! module for hotline wasm JS bindings, structs and functions
#[path = "./hotline_palette.rs"]
pub mod hotline_palette;
use hotline_palette::HotlinePalette;
#[path = "./hotline_position.rs"]
pub mod hotline_position;

use core::mem::drop;
use js_sys::{Array, JsString, Object, Reflect};
use wasm_bindgen::prelude::*;

use leptos::{MaybeSignal, SignalGetUntracked};
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
    pub fn new(
        palette: &HotlinePalette,
        outline_color: &Option<MaybeSignal<String>>,
        max: &Option<MaybeSignal<f64>>,
        min: &Option<MaybeSignal<f64>>,
    ) -> Self {
        let palette_len = palette.palette.len();

        let js_palette = if palette_len > 0 {
            Self::palette_to_js(palette)
        } else {
            Self::palette_to_js(&HotlinePalette::default())
        };

        let js_outline_color = Self::outline_color_to_js(outline_color);
        let js_max = Self::max_to_js(max);
        let js_min = Self::min_to_js(min);

        let opts: Self = JsCast::unchecked_into(Object::new());
        opts.set_palette(&js_palette);
        opts.set_outline_color(&js_outline_color);
        opts.set_max(&js_max);
        opts.set_min(&js_min);
        opts
    }

    ///
    /// convert [`HotlinePalette`] to [`JsValue`] type
    ///
    /// # Returns
    /// [`JsValue`] containing hotline palette information (maps breakpoint -> color for JS binding)
    ///
    #[must_use]
    #[inline]
    pub fn palette_to_js(palette: &HotlinePalette) -> JsValue {
        let palette_opts = Object::new();

        for (color, bkpt) in &palette.palette {
            let res: Result<bool, JsValue> =
                Reflect::set(&palette_opts, &JsValue::from_f64(*bkpt), &color.into());
            drop(res);
        }

        JsCast::unchecked_into(palette_opts)
    }

    ///
    /// Converts hotline outline color to [`JsValue`] type
    ///
    /// # Returns
    /// [`JsValue`] containing hotline outline color information
    ///
    #[must_use]
    #[inline]
    pub fn outline_color_to_js(outline_color: &Option<MaybeSignal<String>>) -> JsValue {
        let js_outline_color = outline_color
            .as_ref()
            .map_or_else(|| "black".to_owned(), SignalGetUntracked::get_untracked);
        JsCast::unchecked_into(JsString::from(js_outline_color))
    }

    ///
    /// Converts hotline max breakpoint threshold to [`JsValue`] type
    ///
    /// # Returns
    /// [`JsValue`] containing hotline max breakpoint threshold information
    ///
    #[must_use]
    #[inline]
    pub fn max_to_js(val: &Option<MaybeSignal<f64>>) -> JsValue {
        let js_val = val
            .as_ref()
            .map_or(1.0_f64, SignalGetUntracked::get_untracked);
        JsValue::from_f64(js_val)
    }

    ///
    /// Converts hotline min breakpoint threshold to [`JsValue`] type
    ///
    /// # Returns
    /// [`JsValue`] containing hotline min breakpoint threshold information
    ///
    #[must_use]
    #[inline]
    pub fn min_to_js(val: &Option<MaybeSignal<f64>>) -> JsValue {
        let js_val = val
            .as_ref()
            .map_or(0.0_f64, SignalGetUntracked::get_untracked);
        JsValue::from_f64(js_val)
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
