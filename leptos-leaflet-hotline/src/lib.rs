//! Module for hot polyline functional component
mod hotline;
pub use hotline::{
    hotline_palette, hotline_positions, to_hotline_lat_lng_array, Hotline, HotlineOptions,
    HotlinePalette, HotlinePosition, LatLng as HotlineLatLng,
};

use core::fmt::Error;
#[allow(unused_imports)]
use leptos::{
    component, create_effect, log, store_value, tracing, use_context, Children, IntoView,
    MaybeSignal, SignalGetUntracked, StoredValue,
};
use leptos_leaflet::leaflet as L;
use leptos_leaflet::{extend_context_with_overlay, update_overlay_context, LeafletMapContext};

#[must_use]
pub fn hotline_prop_string(prop: &str) -> MaybeSignal<String> {
    MaybeSignal::Static(prop.to_string())
}

#[must_use]
pub const fn hotline_prop_float(prop: f64) -> MaybeSignal<f64> {
    MaybeSignal::Static(prop)
}

macro_rules! is_ok {
    ($opt:expr) => {
        $opt.ok_or(Error)
    };
}

pub fn add_hotline_to_map(
    map_context: Option<L::Map>,
    hotline: Hotline,
    overlay: StoredValue<Option<Hotline>>,
) {
    let map: Result<L::Map, Error> = is_ok!(map_context);
    match map {
        Ok(map_ref) => {
            hotline.addTo(&map_ref);
            update_overlay_context(&hotline);
            overlay.set_value(Some(hotline));
        }
        Err(err) => {
            log!("error adding hotline to map {:?}", err);
        }
    }
}

/// Creates hot polyline functional component and adds to a leptos leaflet map
#[component(transparent)]
pub fn HotPolyline(
    /// hotline (lat, lng, value) tuples
    #[prop(into)]
    positions: MaybeSignal<Vec<HotlinePosition>>,
    /// the palette of colors and breakpoints
    #[prop(into)]
    palette: MaybeSignal<HotlinePalette>,
    /// color of the polyline's outline
    #[prop(optional)]
    outline_color: Option<MaybeSignal<String>>,
    /// max breakpoint to use for palette
    #[prop(optional)]
    max: Option<MaybeSignal<f64>>,
    /// min breakpoint to use for palette
    #[prop(optional)]
    min: Option<MaybeSignal<f64>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    extend_context_with_overlay();
    let overlay = store_value(None::<Hotline>);
    let _positions_for_effect = positions.clone();

    create_effect(move |_| -> Result<(), Error> {
        let lat_lngs = to_hotline_lat_lng_array(&positions.get_untracked());
        let opts = HotlineOptions::new(&palette.get_untracked(), &outline_color, &max, &min);
        let hotline = Hotline::new(&lat_lngs, &opts);

        let map_context: Option<LeafletMapContext> = use_context::<LeafletMapContext>();
        let context = is_ok!(map_context);

        match context {
            Ok(ctx) => {
                let map_ctx = ctx.map();
                add_hotline_to_map(map_ctx, hotline, overlay);
            }
            Err(err) => {
                log!("error getting leaflet map context {:?}", err);
            }
        };

        Ok(())
    });

    children.map(|child| child())
}
