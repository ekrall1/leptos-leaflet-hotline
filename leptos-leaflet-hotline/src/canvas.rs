use crate::renderer::{Renderer, RendererOptions};

use js_sys::Object;
use leptos_leaflet::leaflet as L;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = RendererOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type CanvasOptions;

    #[wasm_bindgen(extends = Renderer)]
    #[derive(Debug, Clone, PartialEq)]
    pub type Canvas;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &RendererOptions) -> Canvas;

    #[wasm_bindgen(method, getter)]
    pub fn hotline(this: &Canvas) -> L::Polyline;

    #[wasm_bindgen(method, setter)]
    pub fn set_hotline(this: &Canvas, elem: L::Polyline) -> L::Polyline;

    #[wasm_bindgen(method)]
    pub fn getEvents(this: &Canvas) -> Object;

    #[wasm_bindgen(method)]
    pub fn _initContainer(this: &Canvas) -> Canvas;

    #[wasm_bindgen(method)]
    pub fn _clear(this: &Canvas) -> Canvas;

    #[wasm_bindgen(method, getter)]
    pub fn _container(this: &Canvas) -> HtmlCanvasElement;

    #[wasm_bindgen(method, getter)]
    pub fn _ctx(this: &Canvas) -> CanvasRenderingContext2d;
}

impl CanvasOptions {
    L::object_constructor!();
    L::object_property_set!(tolerance, f64);
}

impl Default for CanvasOptions {
    fn default() -> Self {
        Self::new()
    }
}
