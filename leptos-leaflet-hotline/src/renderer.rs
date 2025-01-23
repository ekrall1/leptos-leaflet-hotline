use js_sys::JsString;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::window;
use leptos_leaflet::leaflet as L;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = L::Canvas)]
    pub type Renderer;

    #[wasm_bindgen(constructor)]
    fn new() -> Renderer;

    #[wasm_bindgen(method, js_name="_initContainer")]
    fn init_container(this: &Renderer);

    #[wasm_bindgen(method, js_name="_hotline")]
    fn hotline(this: &Renderer) -> Hotline;
}

