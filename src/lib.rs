#![doc = include_str!("../README.md")]

mod color_lookup;
pub mod hotline;
pub use color_lookup::{ColorFormat, HotlineColorError, HotlineColorLookup};
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
/// * `smooth_factor` - Leaflet simplification tolerance; defaults to `0.0` so
///   value-bearing vertices are preserved
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
/// pub fn HotlineMap() -> impl IntoView {
///     let positions = HotlinePositionVec::new(&[
///         (40.2928, -105.6180, 1.00),
///         (40.2928, -105.6190, 0.67),
///         (40.2928, -105.6200, 0.33),
///         (40.2918, -105.6210, 0.01),
///     ]);
///
///     let palette = HotlinePalette::new(&[
///         ("blue", 0.00),
///         ("yellow", 0.33),
///         ("red", 1.00),
///     ]);
///
///     view! {
///         <MapContainer
///             style="height: 400px"
///             center=Position::new(40.2928, -105.6170)
///             zoom=17.0
///         >
///             <TileLayer
///                 url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
///                 attribution="&copy; OpenStreetMap contributors"
///             />
///             <HotPolyline
///                 positions=positions
///                 palette=palette
///                 outline_color="#5a5a5a"
///                 min=0.0
///                 max=1.0
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
    /// Defaults to zero for color fidelity. A nonzero value opts back into
    /// Leaflet's geometric simplification and can make rendered colors differ
    /// from lookups over the original input segments.
    #[prop(optional)]
    smooth_factor: f64,
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
            opts.set_smooth_factor(smooth_factor);
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
