//! Module for hot polyline functional component
pub mod hotline;
pub use hotline::{hotline_palette::*, hotline_position::*, Hotline, HotlineOptions};

use js_sys::{Array, JsString, Object, Reflect};
use wasm_bindgen::prelude::*;

use leptos::children::Children;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::*;
use leptos_leaflet::leaflet as L;
use leptos_leaflet::prelude::*;

pub struct HotlinePositions(HotlinePositionVec);

impl HotlinePositions {
    pub fn hotline_lat_lngs(&self) -> Array {
        to_hotline_lat_lng_array(&self.0)
    }
}

pub struct HotlineOutlineColor(String);

impl HotlineOutlineColor {
    pub fn outline_color(&self) -> JsValue {
        Self::outline_color_to_js(&self.0)
    }

    ///
    /// Converts hotline outline color to [`JsValue`] type
    ///
    /// # Returns
    /// [`JsValue`] containing hotline outline color information
    ///
    #[must_use]
    #[inline]
    fn outline_color_to_js(outline_color: &String) -> JsValue {
        let js_outline_color = outline_color.clone();

        match js_outline_color.as_str() {
            "" => JsCast::unchecked_into(JsString::from("black".to_owned())),
            _ => JsCast::unchecked_into(JsString::from(js_outline_color)),
        }
    }
}

pub struct HotlineMax(f64);

impl HotlineMax {
    pub fn hotline_max(&self) -> JsValue {
        Self::max_to_js(&self.0)
    }

    ///
    /// Converts hotline max breakpoint threshold to [`JsValue`] type
    ///
    /// # Returns
    /// [`JsValue`] containing hotline max breakpoint threshold information
    ///
    #[must_use]
    #[inline]
    fn max_to_js(val: &f64) -> JsValue {
        JsValue::from_f64(*val)
    }
}

pub struct HotlineMin(f64);

impl HotlineMin {
    pub fn hotline_min(&self) -> JsValue {
        Self::min_to_js(&self.0)
    }

    ///
    /// Converts hotline min breakpoint threshold to [`JsValue`] type
    ///
    /// # Returns
    /// [`JsValue`] containing hotline min breakpoint threshold information
    ///
    #[must_use]
    #[inline]
    fn min_to_js(val: &f64) -> JsValue {
        JsValue::from_f64(*val)
    }
}

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
    overlay: StoredValue<Option<Hotline>, LocalStorage>,
) -> Result<(), ()> {
    let map: Result<L::Map, &str> = map_context.ok_or("Expected to create map from context.");
    match map {
        Ok(map_ref) => {
            hotline.add_to(&map_ref);
            update_overlay_context(&hotline);
            overlay.set_value(Some(hotline));
        }
        Err(_err) => return Err(()),
    };
    Ok(())
}

pub struct HotlinePaletteStruct(HotlinePalette);

impl HotlinePaletteStruct {
    pub fn hotline_palette(&self) -> JsValue {
        let palette_len = self.0.palette.len();

        let js_palette = if palette_len > 0 {
            Self::palette_to_js(&self.0)
        } else {
            Self::palette_to_js(&HotlinePalette::default())
        };
        js_palette
    }
    ///
    /// convert [`HotlinePalette`] to [`JsValue`] type
    ///
    /// # Returns
    /// [`JsValue`] containing hotline palette information (maps breakpoint -> color for JS binding)
    ///
    #[must_use]
    #[inline]
    fn palette_to_js(palette: &HotlinePalette) -> JsValue {
        let palette_opts = Object::new();

        for (color, bkpt) in &palette.palette {
            let res: Result<bool, JsValue> =
                Reflect::set(&palette_opts, &JsValue::from_f64(*bkpt), &color.into());
            drop(res);
        }

        JsCast::unchecked_into(palette_opts)
    }
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
/// ```ignore
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
#[component]
pub fn HotPolyline(
    #[prop(into)] positions: HotlinePositionVec,
    #[prop(into)] palette: HotlinePalette,
    #[prop(optional, into)] outline_color: Option<String>,
    #[prop(optional, into)] max: Option<f64>,
    #[prop(optional, into)] min: Option<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    extend_context_with_overlay();
    let overlay = StoredValue::new_with_storage(None::<Hotline>);

    let (hotline_palette, _) = signal(HotlinePaletteStruct(palette));
    let (lat_lngs, _) = signal(HotlinePositions(positions.clone()));

    let outline_color_or_default = outline_color.unwrap_or_else(|| "black".to_string());
    let (hotline_outline_color, _) = signal(HotlineOutlineColor(outline_color_or_default));

    let max_or_default = max.unwrap_or(1.0);
    let (hotline_max, _) = signal(HotlineMax(max_or_default));

    let min_or_default = min.unwrap_or(0.0);
    let (hotline_min, _) = signal(HotlineMin(min_or_default));

    Effect::new(move |_| -> Result<(), &str> {
        let opts = HotlineOptions::new(
            &hotline_palette.read().hotline_palette(),
            &hotline_outline_color.read().outline_color(),
            &hotline_max.read().hotline_max(),
            &hotline_min.read().hotline_min(),
        );

        let hotline = Hotline::new(&lat_lngs.read().hotline_lat_lngs(), &opts);
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

    children.map(move |child| child())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_hotline_palette_type() {
        let mut test_palette = HashMap::new();
        test_palette.insert("#FF0000".to_string(), 0.0);

        let palette_struct = HotlinePaletteStruct(HotlinePalette {
            palette: test_palette,
        });

        let js_value = palette_struct.hotline_palette();

        assert!(js_value.is_object());
    }

    #[wasm_bindgen_test]
    fn test_hotline_palette_colors() {
        let mut test_palette = HashMap::new();
        test_palette.insert("#FF0000".to_string(), 0.0);
        test_palette.insert("#00FF00".to_string(), 0.5);
        test_palette.insert("#0000FF".to_string(), 0.99);

        let palette_struct = HotlinePaletteStruct(HotlinePalette {
            palette: test_palette,
        });

        let js_value = palette_struct.hotline_palette();

        let js_obj: Object = js_value.unchecked_into();

        for (color, breakpt) in &palette_struct.0.palette {
            assert_eq!(
                Reflect::get(&js_obj, &JsValue::from_f64(*breakpt))
                    .unwrap()
                    .as_string()
                    .unwrap(),
                *color
            )
        }
    }

    #[wasm_bindgen_test]
    fn test_default_palette() {
        let hm = HashMap::new();

        let palette_struct = HotlinePaletteStruct(HotlinePalette { palette: hm });

        let js_value = palette_struct.hotline_palette();

        let js_obj: Object = js_value.unchecked_into();

        let default_palette = HotlinePalette::default();

        let mut keys: Vec<f64> = Object::keys(&js_obj)
            .iter()
            .filter_map(|key| key.as_string().and_then(|k| k.parse::<f64>().ok()))
            .collect();

        keys.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut expected_keys: Vec<f64> = default_palette.palette.values().copied().collect();
        expected_keys.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(
            keys, expected_keys,
            "mismatch in breakpoints: expected {:?} got {:?}",
            expected_keys, keys
        );

        for (color, breakpt) in &default_palette.palette {
            let js_color = Reflect::get(&js_obj, &JsValue::from_f64(*breakpt))
                .unwrap_or_else(|_| panic!("Missing breakpoint {}", breakpt))
                .as_string()
                .unwrap_or_else(|| panic!("Expected string color for {}", breakpt));

            assert_eq!(
                js_color, *color,
                "Mismatch for breakpt {}: expected: {} got {}",
                breakpt, color, js_color
            );
        }
    }

    #[wasm_bindgen_test]
    fn test_outline_color() {
        let outline_color = HotlineOutlineColor("red".to_string());
        let js_value = outline_color.outline_color();

        let js_string: String = js_value.as_string().unwrap();

        assert_eq!(
            js_string, "red",
            "expected outline color of 'red' but got {}",
            js_string
        );
    }

    #[wasm_bindgen_test]
    fn test_default_outline_color() {
        let outline_color = HotlineOutlineColor("".to_string());
        let js_value = outline_color.outline_color();

        let js_string: String = js_value.as_string().unwrap();

        assert_eq!(
            js_string, "black",
            "expected outline color of 'black' but got {}",
            js_string
        );
    }

    #[wasm_bindgen_test]
    fn test_hotline_max_with_value() {
        let hotline_max = HotlineMax(0.75);
        let js_value = hotline_max.hotline_max();

        let js_f64: f64 = js_value.as_f64().unwrap();

        assert_eq!(js_f64, 0.75, "Expected 0.75, but got {}", js_f64);
    }

    #[wasm_bindgen_test]
    fn test_hotline_min_with_value() {
        let hotline_min = HotlineMin(0.25);
        let js_value = hotline_min.hotline_min();

        let js_f64: f64 = js_value.as_f64().unwrap();

        assert_eq!(js_f64, 0.25, "Expected 0.25, but got {}", js_f64);
    }
}
