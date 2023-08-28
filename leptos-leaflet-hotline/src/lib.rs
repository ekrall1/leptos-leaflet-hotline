use leptos::*;
use leptos_leaflet::leaflet as L;
use leptos_leaflet::*;
use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;


#[component(transparent)]
pub fn Hotline(
    #[prop(into)] positions: MaybeSignal<Vec<Position>>,
    #[prop(optional)] children: Option<Children>,
)  -> impl IntoView {
    extend_context_with_overlay();
    let overlay = store_value(None::<L::Polyline>);
    let positions_for_effect = positions.clone();
    let positions_for_effect = positions.clone();
    
    create_effect(move |_| {
        if let Some(map) = use_context::<LeafletMapContext>()
            .expect("map context")
            .map()
        {
            let lat_lngs = L::to_lat_lng_array(&positions.get_untracked()[0..2].to_vec());
            let mut options = L::PolylineOptions::new();
            log!("Polyline options {:?}", options.color("red"));
            log!("Array: {:?}", lat_lngs);
            let hotline: L::Polyline = L::Polyline::new_with_options(&lat_lngs, &options);
            hotline.addTo(&map);
            update_overlay_context(&hotline);
            overlay.set_value(Some(hotline));
        }});
        children.map(|child| child())
}

