mod browser;
mod canvas;
mod renderer;
pub use browser::Browser;
pub use canvas::{Canvas, CanvasOptions};
pub use renderer::{Renderer, RendererOptions};

use leptos::*;
use leptos_leaflet::leaflet as L;
use leptos_leaflet::*;

pub fn hotline_vals(hotline_vals: &[f64]) -> Vec<f64> {
    hotline_vals.to_vec()
}

#[component(transparent)]
pub fn Hotline(
    #[prop(into)] positions: MaybeSignal<Vec<Position>>,
    #[prop(into)] hotline_vals: MaybeSignal<Vec<f64>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    extend_context_with_overlay();
    let overlay = store_value(None::<L::Polyline>);
    let _positions_for_effect = positions.clone();
    let _hotline_vals_for_effect = hotline_vals.clone();

    create_effect(move |_| {
        if let Some(map) = use_context::<LeafletMapContext>()
            .expect("map context")
            .map()
        {
            // get browser information, potentially to work with in later functions
            let browser = Browser::default();
            // verify that the browser object has some information
            let chrome = browser.chrome;
            let edge: bool = browser.edge;
            log!("chrome {:?}", chrome && !edge);
            log!("edge {:?}", edge);

            let lat_lngs = L::to_lat_lng_array(&positions.get_untracked().to_vec());
            let options = L::PolylineOptions::new();
            let hotline: L::Polyline = L::Polyline::new_with_options(&lat_lngs, &options);
            hotline.addTo(&map);
            update_overlay_context(&hotline);
            overlay.set_value(Some(hotline));
        }
    });

    children.map(|child| child())
}
