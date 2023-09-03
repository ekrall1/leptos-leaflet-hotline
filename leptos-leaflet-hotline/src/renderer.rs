use js_sys::Object;
use leptos_leaflet::leaflet as L;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq)]
    pub type RendererOptions;

    #[wasm_bindgen(extends = L::Layer)]
    #[derive(Debug, Clone, PartialEq)]
    pub type Renderer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &RendererOptions) -> Renderer;

    #[wasm_bindgen(method)]
    pub fn initialize(this: &Renderer, options: &RendererOptions) -> Renderer;

    #[wasm_bindgen(method)]
    pub fn getEvents(this: &Renderer) -> Object;
}

impl RendererOptions {
    L::object_constructor!();
    L::object_property_set!(pane, &str);
    L::object_property_set!(attribution, &str);
    L::object_property_set!(bubbling_mouse_events, bool);
}

impl Default for RendererOptions {
    fn default() -> Self {
        let mut opts = Self::new();
        opts.pane("overlayPane");
        opts.attribution("null");
        opts.bubbling_mouse_events(true);
        opts
    }
}
