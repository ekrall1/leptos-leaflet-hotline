#[path = "./hotline_palette.rs"]
mod hotline_palette;
#[path = "./hotline_position.rs"]
mod hotline_position;
pub use hotline_palette::*;
pub use hotline_position::*;
use js_sys::{Array, JsString, Object, Reflect};
use wasm_bindgen::prelude::*;

use leptos::*;
use leptos_leaflet::leaflet as L;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = L::PolylineOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type HotlineOptions;

    #[wasm_bindgen(method, setter)]
    pub fn set_palette(this: &HotlineOptions, palette: &JsValue) -> HotlineOptions;

    #[wasm_bindgen(method, setter, js_name = "outlineColor")]
    pub fn set_outline_color(this: &HotlineOptions, color: &JsValue) -> HotlineOptions;

    #[wasm_bindgen(method, setter)]
    pub fn set_max(this: &HotlineOptions, max: &JsValue) -> HotlineOptions;

    #[wasm_bindgen(method, setter)]
    pub fn set_min(this: &HotlineOptions, min: &JsValue) -> HotlineOptions;

    #[wasm_bindgen(extends = L::Polyline)]
    #[derive(Debug, Clone)]
    pub type Hotline;

    #[wasm_bindgen(constructor, js_namespace=L)]
    pub fn new(hotline_data: &Array, opts: &JsValue) -> Hotline;

    #[wasm_bindgen(method, js_name = "_clickTolerance")]
    pub fn _click_tolerance(this: &Hotline) -> Object;

    #[wasm_bindgen(method, js_name = "getBounds")]
    pub fn get_bounds(this: &Hotline) -> Object;

    #[wasm_bindgen(method, js_name = "setStyle")]
    pub fn set_style(this: &Hotline, style: &Object) -> Object;

}

impl HotlineOptions {
    pub fn new(
        palette: &HotlinePalette,
        outline_color: &Option<MaybeSignal<String>>,
        max: &Option<MaybeSignal<f64>>,
        min: &Option<MaybeSignal<f64>>,
    ) -> Self {
        let palette_len = palette.palette.len();
        let js_palette = match palette_len > 0 {
            true => Self::palette_to_js(palette),
            false => Self::palette_to_js(&HotlinePalette::default()),
        };
        let js_outline_color = Self::outline_color_to_js(outline_color);
        let js_max = Self::max_to_js(max);

        let opts: HotlineOptions = JsCast::unchecked_into(Object::new());
        opts.set_palette(&js_palette);
        opts.set_outline_color(&js_outline_color);
        opts.set_max(&js_max);
        opts
    }

    pub fn palette_to_js(palette: &HotlinePalette) -> JsValue {
        let palette_opts = Object::new();

        for (color, bkpt) in &palette.palette {
            let _ = Reflect::set(&palette_opts, &JsValue::from_f64(*bkpt), &color.into());
        }

        JsCast::unchecked_into(palette_opts)
    }

    pub fn outline_color_to_js(outline_color: &Option<MaybeSignal<String>>) -> JsValue {
        let js_outline_color = match outline_color {
            Some(color) => color.get_untracked(),
            None => "black".to_string(),
        };
        JsCast::unchecked_into(JsString::from(js_outline_color.to_string()))
    }

    pub fn max_to_js(val: &Option<MaybeSignal<f64>>) -> JsValue {
        let js_val = match val {
            Some(max) => max.get_untracked(),
            None => 1.0,
        };
        JsValue::from_f64(js_val)
    }

    pub fn min_to_js(val: &Option<MaybeSignal<f64>>) -> JsValue {
        let js_val = match val {
            Some(min) => min.get_untracked(),
            None => 1.0,
        };
        JsValue::from_f64(js_val)
    }
}

#[wasm_bindgen]
impl Hotline {
    pub fn set_outline_color(&self, color: &str) {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"outlineColor".into(), &JsValue::from(color)).unwrap();

        self.set_style(&obj);
    }
    pub fn set_max(&self, max: f64) {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"max".into(), &JsValue::from_f64(max)).unwrap();

        self.set_style(&obj);
    }
    pub fn set_min(&self, min: f64) {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"max".into(), &JsValue::from_f64(min)).unwrap();

        self.set_style(&obj);
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
