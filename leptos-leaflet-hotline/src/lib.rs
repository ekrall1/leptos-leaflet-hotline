mod browser;
mod hotline;
pub use browser::Browser;
pub use hotline::{
    hotline_palette, hotline_positions, to_hotline_lat_lng_array, Hotline, HotlineOptions,
    HotlinePalette, HotlinePosition, LatLng as HotlineLatLng,
};

use leptos::*;
use leptos_leaflet::{extend_context_with_overlay, update_overlay_context, LeafletMapContext};

#[component(transparent)]
pub fn HotPolyline(
    #[prop(into)] positions: MaybeSignal<Vec<HotlinePosition>>,
    #[prop(into)] palette: MaybeSignal<HotlinePalette>,
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
            // get browser information, potentially to work with in later functions
            let browser = Browser::default();
            // verify that the browser object has some information
            let chrome = browser.chrome;
            let edge: bool = browser.edge;
            log!("chrome {:?}", chrome && !edge);
            log!("edge {:?}", edge);

            let lat_lngs = to_hotline_lat_lng_array(&positions.get_untracked());
            let opts = HotlineOptions::new(&palette.get_untracked());
            let hotline: Hotline = Hotline::new(&lat_lngs, &opts);

            hotline.addTo(&map); // adds it to the map, but still have not implemented everything
            update_overlay_context(&hotline);
            overlay.set_value(Some(hotline));
        }
    });

    children.map(|child| child())
}
