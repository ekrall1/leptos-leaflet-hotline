use crate::blanket_overlay::{BlanketOverlay, BlanketOverlayOptions};
use js_sys::Object;
use leptos_leaflet::leaflet as L;
use std::ops::DerefMut;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = BlanketOverlayOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type RendererOptions;

    #[wasm_bindgen(extends = BlanketOverlay)]
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
}

impl DerefMut for RendererOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}

impl Default for RendererOptions {
    fn default() -> Self {
        RendererOptions {
            obj: BlanketOverlayOptions::default(),
        }
    }
}
