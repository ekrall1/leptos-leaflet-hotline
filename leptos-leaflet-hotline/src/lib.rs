mod browser;
mod hotline;
pub use browser::Browser;
pub use hotline::{
    hotline_palette, hotline_positions, to_hotline_lat_lng_array, Hotline, HotlineOptions,
    HotlinePalette, HotlinePosition, LatLng as HotlineLatLng,
};

use leptos::*;
use leptos_leaflet::{extend_context_with_overlay, update_overlay_context, LeafletMapContext};

pub fn hotline_prop_string(prop: &str) -> MaybeSignal<String> {
    MaybeSignal::Static(prop.to_string())
}
pub fn hotline_prop_float(prop: f64) -> MaybeSignal<f64> {
    MaybeSignal::Static(prop)
}

#[component(transparent)]
pub fn HotPolyline(
    #[prop(into)] positions: MaybeSignal<Vec<HotlinePosition>>,
    #[prop(into)] palette: MaybeSignal<HotlinePalette>,
    #[prop(optional)] outline_color: Option<MaybeSignal<String>>,
    #[prop(optional)] max: Option<MaybeSignal<f64>>,
    #[prop(optional)] min: Option<MaybeSignal<f64>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    extend_context_with_overlay();
    let overlay = store_value(None::<Hotline>);
    let _positions_for_effect = positions.clone();

    create_effect(move |_| {
        if let Some(map) = use_context::<LeafletMapContext>()
            .expect("map context")
            .map()
        {
            let lat_lngs = to_hotline_lat_lng_array(&positions.get_untracked());
            let opts = HotlineOptions::new(&palette.get_untracked(), &outline_color, &max, &min);
            let hotline: Hotline = Hotline::new(&lat_lngs, &opts);

            match &outline_color {
                Some(color) => hotline.set_outline_color(&color.get_untracked()),
                None => {}
            }

            hotline.addTo(&map); // adds it to the map, but still have not implemented everything
            update_overlay_context(&hotline);
            overlay.set_value(Some(hotline));
        }
    });

    children.map(|child| child())
}
