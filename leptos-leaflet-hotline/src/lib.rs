//! Module for hot polyline functional component
pub mod hotline;
pub use hotline::{hotline_palette::*, hotline_position::*, Hotline, HotlineOptions};

#[allow(unused_imports)]
use leptos::{
    component, create_effect, logging::*, store_value, tracing, use_context, Children, IntoView,
    MaybeSignal, SignalGetUntracked, StoredValue,
};
use leptos_leaflet::leaflet as L;
use leptos_leaflet::{extend_context_with_overlay, update_overlay_context, LeafletMapContext};

/// adds hotline instance to a leptos-leaflet map context
/// # Arguments
/// * `map_context` the map context
/// * `hotline` instance
/// * `overlay` map overlay
///
/// # Returns
/// [`Result<T,E>`]
///
#[inline]
fn add_hotline_to_map(
    map_context: Option<L::Map>,
    hotline: Hotline,
    overlay: StoredValue<Option<Hotline>>,
) -> Result<(), ()> {
    let map: Result<L::Map, &str> = map_context.ok_or("Expected to create map from context.");
    match map {
        Ok(map_ref) => {
            hotline.addTo(&map_ref);
            update_overlay_context(&hotline);
            overlay.set_value(Some(hotline));
        }
        Err(_err) => return Err(()),
    };
    Ok(())
}

///
/// Creates hot polyline functional component added to a leptos leaflet map container
///
/// # Arguments
///
/// * `positions` - (lat, lng, value) tuples representing path and value information
/// * `palette` - palette of colors and breakpoints
/// * `outline_color` - string representing the polyline outline color
/// * `max` - float representing max breakpoint to use for palette
/// * `min` - float representing min breakpoint to use for palette
/// * `children` - child elements
///
/// # Returns
///
/// `impl` [`leptos::IntoView`]
///
/// # Examples
///
/// Basic usage:
/// ```no_run
/// use leptos_leaflet::{MapContainer};
/// use leptos::{view, IntoView};
/// use leptos_leaflet_hotline::{HotPolyline};
///
/// fn my_map() -> impl IntoView {
///     let (pos, set_pos) = leptos::create_signal(leptos_leaflet::Position::new(90.000, 135.000));
///     let (map, set_map) = leptos::create_signal(None::<leptos_leaflet::leaflet::Map>);
///
///     view! {
///         <MapContainer style="height: 100vh" center=leptos_leaflet::Position::new(90.000, 135.000) zoom=17.0 set_view=false map=set_map locate=false watch=true>
///             <HotPolyline
///                 positions=leptos_leaflet_hotline::HotlinePositionVec::new(&[(90.000, 135.000, 0.0), (90.010, 135.010, 100.0)])
///                 palette=leptos_leaflet_hotline::HotlinePalette::new(&[("green", 0.0), ("red", 1.0)])
///                 outline_color="white"
///                 max=1.0
///                 min=0.0
///             />
///         </MapContainer>
///     }
/// }
/// ```
///
#[component(transparent)]
pub fn HotPolyline(
    #[prop(into)] positions: MaybeSignal<HotlinePositionVec>,
    #[prop(into)] palette: MaybeSignal<HotlinePalette>,
    #[prop(optional, into)] outline_color: Option<MaybeSignal<String>>,
    #[prop(optional, into)] max: Option<MaybeSignal<f64>>,
    #[prop(optional, into)] min: Option<MaybeSignal<f64>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    extend_context_with_overlay();
    let overlay = store_value(None::<Hotline>);
    let _positions_for_effect = positions.clone();

    create_effect(move |_| -> Result<(), &str> {
        let lat_lngs = to_hotline_lat_lng_array(&positions.get_untracked());
        let opts = HotlineOptions::new(&palette.get_untracked(), &outline_color, &max, &min);

        let hotline = Hotline::new(&lat_lngs, &opts);
        let map_context = use_context::<LeafletMapContext>();
        let context = map_context.ok_or("Expected map context.");

        match context {
            Ok(ctx) => {
                let map_ctx = ctx.map();
                let res = add_hotline_to_map(map_ctx, hotline, overlay);
                if res == Err(()) {
                    log!("Expected to add hotline to the map.");
                }
            }
            Err(err) => return Err(err),
        };
        Ok(())
    });

    children.map(|child| child())
}
