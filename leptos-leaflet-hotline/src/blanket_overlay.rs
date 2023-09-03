use js_sys::Object;
use leptos_leaflet::leaflet as L;
use std::ops::DerefMut;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=Object)]
    #[derive(Debug, Clone, PartialEq)]
    pub type BlanketOverlayOptions;

    #[wasm_bindgen(extends = L::Layer)]
    #[derive(Debug, Clone, PartialEq)]
    pub type BlanketOverlay;

    #[wasm_bindgen(method)]
    pub fn initialize(this: &BlanketOverlay, options: &BlanketOverlayOptions) -> BlanketOverlay;

    #[wasm_bindgen(method)]
    pub fn onAdd(this: &BlanketOverlay) -> BlanketOverlay;

    #[wasm_bindgen(method)]
    pub fn getEvents(this: &BlanketOverlay) -> Object;

    #[wasm_bindgen(method)]
    pub fn _onAnimZoom(this: &BlanketOverlay, ev: &Object) -> Object;

    #[wasm_bindgen(method)]
    pub fn _onZoom(this: &BlanketOverlay) -> BlanketOverlay;

    #[wasm_bindgen(method)]
    pub fn _updateTransform(this: &BlanketOverlay, point: L::Point, zoome: f64) -> BlanketOverlay;

    #[wasm_bindgen(method)]
    pub fn _onMoveEnd(this: &BlanketOverlay, ev: Object) -> BlanketOverlay;

    #[wasm_bindgen(method)]
    pub fn _reset(this: &BlanketOverlay) -> BlanketOverlay;
}

impl BlanketOverlayOptions {
    L::object_constructor!();
    L::object_property_set!(padding, f64);
    L::object_property_set!(continuous, bool);
    L::object_property_set!(pane, &str);
    L::object_property_set!(attribution, &str);
    L::object_property_set!(bubbling_mouse_events, bool);
}

impl DerefMut for BlanketOverlayOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}

impl Default for BlanketOverlayOptions {
    fn default() -> Self {
        let mut opts = Self::new();
        opts.padding(0.1);
        opts.continuous(false);
        opts.pane("overlayPane");
        opts.attribution("null");
        opts.bubbling_mouse_events(true);
        opts
    }
}
