#[path = "./hotline_palette.rs"]
mod hotline_palette;
#[path = "./hotline_position.rs"]
mod hotline_position;
pub use hotline_palette::*;
pub use hotline_position::*;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;

use leptos_leaflet::leaflet as L;

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

    #[wasm_bindgen(method, js_name = "_clickTolerance")]
    pub fn _click_tolerance(this: &Hotline) -> Object;

    #[wasm_bindgen(method, js_name = "getBounds")]
    pub fn get_bounds(this: &Hotline) -> Object;

    #[wasm_bindgen(method, js_name = "setStyle")]
    pub fn set_style(this: &Hotline, style: &Object) -> Object;

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

#[wasm_bindgen]
impl Hotline {
    pub fn set_outline_color(&self, color: &str) {
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"outlineColor".into(), &JsValue::from(color)).unwrap();

        // Call the set_style method with the created object.
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
