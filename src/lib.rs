//! Module for hot polyline functional component
pub mod hotline;
pub use hotline::{hotline_palette::*, hotline_position::*, Hotline, HotlineOptions};

use leptos::logging::log;
use leptos::prelude::*;
use leptos_leaflet::prelude::{
    extend_context_with_overlay, update_overlay_context, JsStoredValue, LeafletMapContext,
};

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
/// use leptos::prelude::*;
/// use leptos_leaflet::prelude::*;
/// use leptos_leaflet_hotline::{HotPolyline, HotlinePalette, HotlinePositionVec};
///
/// #[component]
/// fn my_map() -> impl IntoView {
///     view! {
///         <MapContainer
///             style="height: 100vh"
///             center=Position::new(40.293, -105.618)
///             zoom=17.0
///         >
///             <HotPolyline
///                 positions=HotlinePositionVec::new(&[
///                     (40.293, -105.618, 0.0),
///                     (40.294, -105.619, 100.0),
///                 ])
///                 palette=HotlinePalette::new(&[("green", 0.0), ("red", 1.0)])
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
    #[prop(into)] positions: Signal<HotlinePositionVec>,
    #[prop(into)] palette: Signal<HotlinePalette>,
    #[prop(optional, into)] outline_color: Option<Signal<String>>,
    #[prop(optional, into)] max: Option<Signal<f64>>,
    #[prop(optional, into)] min: Option<Signal<f64>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    extend_context_with_overlay();
    let overlay = JsStoredValue::new_local(None::<Hotline>);
    let map_context = use_context::<LeafletMapContext>();

    let positions_for_setup = positions;
    let palette_for_setup = palette;
    let outline_color_for_setup = outline_color;
    let max_for_setup = max;
    let min_for_setup = min;

    Effect::new(move |_| {
        let Some(map_context) = map_context else {
            log!("Expected HotPolyline to be nested inside a MapContainer.");
            return;
        };

        if overlay.get_value().is_none() {
            let Some(map) = map_context.map() else {
                return;
            };
            let lat_lngs = to_hotline_lat_lng_array(&positions_for_setup.get_untracked());
            let opts = HotlineOptions::new(
                &palette_for_setup.get_untracked(),
                &outline_color_for_setup,
                &max_for_setup,
                &min_for_setup,
            );
            let hotline = Hotline::new(&lat_lngs, &opts);
            hotline.add_to(&map);
            update_overlay_context(&hotline);
            overlay.set_value(Some(hotline));
        }
    });

    let positions_stop = Effect::watch(
        move || positions.get(),
        move |positions, _, _| {
            if let Some(hotline) = overlay.get_value().as_ref() {
                hotline.set_lat_lngs(&to_hotline_lat_lng_array(positions));
            }
        },
        false,
    );

    let palette_stop = Effect::watch(
        move || palette.get(),
        move |palette, _, _| {
            if let Some(hotline) = overlay.get_value().as_ref() {
                hotline.set_palette_val(palette);
            }
        },
        false,
    );

    let outline_color_stop = outline_color.map(|outline_color| {
        Effect::watch(
            move || outline_color.get(),
            move |outline_color, _, _| {
                if let Some(hotline) = overlay.get_value().as_ref() {
                    hotline.set_outline_color_val(outline_color);
                }
            },
            false,
        )
    });

    let max_stop = max.map(|max| {
        Effect::watch(
            move || max.get(),
            move |max, _, _| {
                if let Some(hotline) = overlay.get_value().as_ref() {
                    hotline.set_max_val(*max);
                }
            },
            false,
        )
    });

    let min_stop = min.map(|min| {
        Effect::watch(
            move || min.get(),
            move |min, _, _| {
                if let Some(hotline) = overlay.get_value().as_ref() {
                    hotline.set_min_val(*min);
                }
            },
            false,
        )
    });

    on_cleanup(move || {
        positions_stop.stop();
        palette_stop.stop();
        if let Some(stop) = outline_color_stop {
            stop.stop();
        }
        if let Some(stop) = max_stop {
            stop.stop();
        }
        if let Some(stop) = min_stop {
            stop.stop();
        }
        if let Some(hotline) = overlay.try_get_value().flatten() {
            hotline.remove();
        }
    });

    children.map(|child| child())
}
