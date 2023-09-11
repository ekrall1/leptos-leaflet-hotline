//! Module for hot polyline functional component
pub mod hotline;
pub use hotline::{hotline_palette::*, hotline_position::*, Hotline, HotlineOptions};

use core::fmt::Error;
#[allow(unused_imports)]
use leptos::{
    component, create_effect, log, store_value, tracing, use_context, Children, IntoView,
    MaybeSignal, SignalGetUntracked, StoredValue,
};
use leptos_leaflet::leaflet as L;
use leptos_leaflet::{extend_context_with_overlay, update_overlay_context, LeafletMapContext};

macro_rules! is_ok {
    ($opt:expr) => {
        $opt.ok_or(Error)
    };
}

#[inline]
fn add_hotline_to_map(
    map_context: Option<L::Map>,
    hotline: Hotline,
    overlay: StoredValue<Option<Hotline>>,
) {
    let map: Result<L::Map, Error> = is_ok!(map_context);
    if let Ok(map_ref) = map {
        hotline.addTo(&map_ref);
        update_overlay_context(&hotline);
        overlay.set_value(Some(hotline));
    }
}

/// Creates hot polyline functional component that can be wrapped in a leptos leaflet map container
/// # Examples
/// 
/// Basic usage:
/// ```
/// #[component]
/// fn MyMapPage() -> leptos::IntoView {
///     let (pos, set_pos) = leptos::create_signal(leptos_leaflet::Position::new(90.000, 135.000));
///     let (map, set_map) = leptos::create_signal(None::<leptos_leaflet::leaflet::Map>);
///     
///     leptos::view! {
///         <leptos_leaflet::MapContainer style="height: 100vh" center=Position::new(90.000, 135.000) zoom=17.0 set_view=false map=set_map locate=false watch=true events>
///             <leptos_leaflet_hotline::HotPolyline
///                 positions=leptos_leaflet_hotline::hotline_positions(&[(90.000, 135.000, 0), (90.010, 135.010, 100)])
///                 palette=make_hotline_palette(&[("green", 0.0), ("red", 1.0)])
///                 outline_color="white"
///                 max=1.0
///                 min=0.0
///             />
///         </leptos_leaflet::MapContainer>
///     }
/// }
/// ```
#[component(transparent)]
pub fn HotPolyline(
    /// hotline (lat, lng, value) tuples
    #[prop(into)]
    positions: MaybeSignal<Vec<HotlinePosition>>,
    /// the palette of colors and breakpoints
    #[prop(into)]
    palette: MaybeSignal<HotlinePalette>,
    /// color of the polyline's outline
    #[prop(optional, into)]
    outline_color: Option<MaybeSignal<String>>,
    /// max breakpoint to use for palette
    #[prop(optional, into)]
    max: Option<MaybeSignal<f64>>,
    /// min breakpoint to use for palette
    #[prop(optional, into)]
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
        let map_context = use_context::<LeafletMapContext>();
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
