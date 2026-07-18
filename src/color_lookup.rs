//! Color lookup for positions along a hotline path.
//!
//! Leaflet.hotline first rasterizes its palette into 256 RGB entries. It then
//! looks up the two endpoint colors of every path segment and creates a second
//! RGB gradient between those endpoint colors. The types in this module follow
//! that same two-stage algorithm.

use crate::{
    hotline::hotline_palette::{CompiledPalette, PaletteCacheKey, PALETTE_SIZE},
    HotlinePalette, HotlinePositionVec,
};
use std::{error::Error, fmt, sync::Arc};

const DEFAULT_MIN: f64 = 0.0;
const DEFAULT_MAX: f64 = 1.0;
const MAX_MERCATOR_LATITUDE: f64 = 85.051_128_779_8;

/// Output syntax used by [`HotlineColorLookup::color_at`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ColorFormat {
    /// A lowercase CSS hexadecimal color, such as `#800080`.
    Hex,
    /// A CSS RGB color, such as `rgb(128, 0, 128)`.
    Rgb,
}

/// An error returned while preparing or querying a hotline color lookup.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum HotlineColorError {
    /// A browser Canvas 2D context was unavailable while building the exact
    /// Leaflet.hotline palette on WebAssembly.
    CanvasUnavailable,
    /// More than one color uses the same palette stop.
    DuplicatePaletteStop { stop: f64 },
    /// A palette color is not valid CSS.
    InvalidPaletteColor { color: String },
    /// A palette stop is non-finite or outside `0.0..=1.0`.
    InvalidPaletteStop { color: String, stop: f64 },
    /// A path coordinate is not finite or cannot be projected.
    InvalidPathPosition { index: usize, lat: f64, lng: f64 },
    /// A stored path value is not finite.
    InvalidPathValue { index: usize, value: f64 },
    /// The query coordinate is not finite or cannot be projected.
    InvalidQueryPosition { lat: f64, lng: f64 },
    /// `min` and `max` do not describe a usable finite range.
    InvalidValueRange { min: f64, max: f64 },
    /// A scalar passed to [`HotlineColorLookup::rgb_for_value`] is not finite.
    InvalidValue { value: f64 },
    /// The supplied path has no non-zero-length segment.
    NoDrawableSegments,
}

impl fmt::Display for HotlineColorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanvasUnavailable => {
                formatter.write_str("a browser Canvas 2D context is unavailable")
            }
            Self::DuplicatePaletteStop { stop } => {
                write!(formatter, "more than one color uses palette stop {stop}")
            }
            Self::InvalidPaletteColor { color } => {
                write!(formatter, "invalid CSS palette color: {color}")
            }
            Self::InvalidPaletteStop { color, stop } => write!(
                formatter,
                "palette stop for {color} must be finite and in 0.0..=1.0, got {stop}"
            ),
            Self::InvalidPathPosition { index, lat, lng } => write!(
                formatter,
                "path position {index} cannot be projected: ({lat}, {lng})"
            ),
            Self::InvalidPathValue { index, value } => {
                write!(formatter, "path position {index} has invalid value {value}")
            }
            Self::InvalidQueryPosition { lat, lng } => {
                write!(
                    formatter,
                    "query position cannot be projected: ({lat}, {lng})"
                )
            }
            Self::InvalidValueRange { min, max } => write!(
                formatter,
                "hotline value range must have two distinct finite bounds, got {min}..{max}"
            ),
            Self::InvalidValue { value } => {
                write!(formatter, "hotline value must be finite, got {value}")
            }
            Self::NoDrawableSegments => {
                formatter.write_str("hotline path has no non-zero-length segment")
            }
        }
    }
}

impl Error for HotlineColorError {}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ProjectedPoint {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ProjectedSegment {
    start: ProjectedPoint,
    end: ProjectedPoint,
    length_squared: f64,
    start_value: f64,
    end_value: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct PaletteEntry {
    stop: f64,
    color: String,
}

/// A reusable snapshot of a hotline path and its 256-entry color palette.
///
/// Build this once when performing many queries. [`Self::color_at`] projects
/// the requested coordinate onto the nearest supplied path segment. At an
/// exact self-intersection, the later segment wins, matching Canvas paint
/// order.
///
/// On `wasm32`, the palette is generated with the same 1-by-256 Canvas
/// gradient used by Leaflet.hotline, including the browser's CSS color parser
/// and byte rounding. Native builds use a deterministic Rust CSS parser so the
/// lookup is also usable during SSR and in tests.
///
/// # Examples
///
/// ```
/// use leptos_leaflet_hotline::{
///     ColorFormat, HotlineColorLookup, HotlinePalette, HotlinePositionVec,
/// };
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let positions = HotlinePositionVec::new(&[
///     (0.0, -1.0, 0.0),
///     (0.0, 1.0, 1.0),
/// ]);
/// let palette = HotlinePalette::new(&[("red", 0.0), ("blue", 1.0)]);
/// let lookup = HotlineColorLookup::new(&positions, &palette, 0.0, 1.0)?;
///
/// assert_eq!(lookup.color_at(0.0, 0.0, ColorFormat::Hex)?, "#800080");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HotlineColorLookup {
    segments: Vec<ProjectedSegment>,
    palette: Arc<CompiledPalette>,
    min: f64,
    max: f64,
}

impl HotlineColorLookup {
    /// Prepares a reusable lookup with the same path, palette, `min`, and `max`
    /// values supplied to `HotPolyline`.
    ///
    /// Reversed finite ranges are supported because Leaflet.hotline supports
    /// them and uses them to reverse a palette. Equal bounds are rejected.
    pub fn new(
        positions: &HotlinePositionVec,
        palette: &HotlinePalette,
        min: f64,
        max: f64,
    ) -> Result<Self, HotlineColorError> {
        validate_range(min, max)?;

        let projected = positions
            .positions
            .iter()
            .enumerate()
            .map(|(index, position)| {
                if !position.alt.is_finite() {
                    return Err(HotlineColorError::InvalidPathValue {
                        index,
                        value: position.alt,
                    });
                }

                project(position.latlng.lat, position.latlng.lng).ok_or(
                    HotlineColorError::InvalidPathPosition {
                        index,
                        lat: position.latlng.lat,
                        lng: position.latlng.lng,
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut segments = Vec::with_capacity(projected.len().saturating_sub(1));
        for (index, pair) in projected.windows(2).enumerate() {
            let start = pair[0];
            let end = pair[1];
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let length_squared = dx * dx + dy * dy;

            if length_squared == 0.0 {
                continue;
            }
            if !length_squared.is_finite() {
                let position = positions.positions[index + 1];
                return Err(HotlineColorError::InvalidPathPosition {
                    index: index + 1,
                    lat: position.latlng.lat,
                    lng: position.latlng.lng,
                });
            }

            segments.push(ProjectedSegment {
                start,
                end,
                length_squared,
                start_value: positions.positions[index].alt,
                end_value: positions.positions[index + 1].alt,
            });
        }

        if segments.is_empty() {
            return Err(HotlineColorError::NoDrawableSegments);
        }

        let entries = palette_entries(palette)?;
        let key = palette_cache_key(&entries);
        let palette = palette.compiled_palette(key, || build_palette(&entries))?;

        Ok(Self {
            segments,
            palette,
            min,
            max,
        })
    }

    /// Returns the color at the nearest point on the supplied hotline path.
    ///
    /// The result is either `#rrggbb` or `rgb(r, g, b)`, according to
    /// `format`.
    pub fn color_at(
        &self,
        lat: f64,
        lng: f64,
        format: ColorFormat,
    ) -> Result<String, HotlineColorError> {
        self.rgb_at(lat, lng).map(|rgb| format_rgb(rgb, format))
    }

    /// Returns the three RGB channels at the nearest point on the path.
    pub fn rgb_at(&self, lat: f64, lng: f64) -> Result<[u8; 3], HotlineColorError> {
        let query =
            project(lat, lng).ok_or(HotlineColorError::InvalidQueryPosition { lat, lng })?;
        let mut closest: Option<(&ProjectedSegment, f64, f64)> = None;

        for segment in &self.segments {
            let dx = segment.end.x - segment.start.x;
            let dy = segment.end.y - segment.start.y;
            let query_dx = query.x - segment.start.x;
            let query_dy = query.y - segment.start.y;
            let fraction =
                ((query_dx * dx + query_dy * dy) / segment.length_squared).clamp(0.0, 1.0);
            let nearest_x = segment.start.x + fraction * dx;
            let nearest_y = segment.start.y + fraction * dy;
            let distance_x = query.x - nearest_x;
            let distance_y = query.y - nearest_y;
            let distance_squared = distance_x * distance_x + distance_y * distance_y;

            if !fraction.is_finite() || !distance_squared.is_finite() {
                return Err(HotlineColorError::InvalidQueryPosition { lat, lng });
            }

            // Replacing on equality deliberately chooses the segment painted
            // last at an exact self-intersection.
            if closest.is_none_or(|(_, _, best_distance)| distance_squared <= best_distance) {
                closest = Some((segment, fraction, distance_squared));
            }
        }

        let (segment, fraction, _) = closest.ok_or(HotlineColorError::NoDrawableSegments)?;
        let start = self.rgb_for_value(segment.start_value)?;
        let end = self.rgb_for_value(segment.end_value)?;
        Ok(interpolate_rgb(start, end, fraction))
    }

    /// Applies Leaflet.hotline's scalar-to-palette lookup and returns RGB
    /// channels. This mirrors upstream `getRGBForValue` without relying on a
    /// layer's private/shared renderer state.
    pub fn rgb_for_value(&self, value: f64) -> Result<[u8; 3], HotlineColorError> {
        if !value.is_finite() {
            return Err(HotlineColorError::InvalidValue { value });
        }

        let relative = ((value - self.min) / (self.max - self.min)).clamp(0.0, 0.999);
        let index = (relative * PALETTE_SIZE as f64).floor() as usize;
        Ok(self.palette[index])
    }
}

impl HotlinePositionVec {
    /// Performs a one-off color query using Leaflet.hotline's default value
    /// range of `0.0..1.0`.
    ///
    /// The compiled color table is cached by `palette`, but this method still
    /// rebuilds the projected path segments. Use [`HotlineColorLookup`]
    /// directly for repeated queries or for a custom `min`/`max` range.
    pub fn color_at(
        &self,
        lat: f64,
        lng: f64,
        palette: &HotlinePalette,
        format: ColorFormat,
    ) -> Result<String, HotlineColorError> {
        HotlineColorLookup::new(self, palette, DEFAULT_MIN, DEFAULT_MAX)?.color_at(lat, lng, format)
    }

    /// Performs a one-off color query using explicit `min` and `max` values.
    /// The compiled color table is cached by `palette`, but the projected path
    /// segments are rebuilt for every call.
    #[allow(clippy::too_many_arguments)]
    pub fn color_at_with_range(
        &self,
        lat: f64,
        lng: f64,
        palette: &HotlinePalette,
        min: f64,
        max: f64,
        format: ColorFormat,
    ) -> Result<String, HotlineColorError> {
        HotlineColorLookup::new(self, palette, min, max)?.color_at(lat, lng, format)
    }
}

fn validate_range(min: f64, max: f64) -> Result<(), HotlineColorError> {
    if !min.is_finite() || !max.is_finite() || min == max || !(max - min).is_finite() {
        return Err(HotlineColorError::InvalidValueRange { min, max });
    }
    Ok(())
}

fn project(lat: f64, lng: f64) -> Option<ProjectedPoint> {
    if !lat.is_finite() || !lng.is_finite() {
        return None;
    }

    let latitude = lat
        .clamp(-MAX_MERCATOR_LATITUDE, MAX_MERCATOR_LATITUDE)
        .to_radians();
    let sin_latitude = latitude.sin();
    let point = ProjectedPoint {
        x: lng.to_radians(),
        y: ((1.0 + sin_latitude) / (1.0 - sin_latitude)).ln() / 2.0,
    };

    (point.x.is_finite() && point.y.is_finite()).then_some(point)
}

fn palette_entries(palette: &HotlinePalette) -> Result<Vec<PaletteEntry>, HotlineColorError> {
    let fallback = HotlinePalette::default();
    let palette = if palette.palette.is_empty() {
        &fallback
    } else {
        palette
    };

    let mut entries = palette
        .palette
        .iter()
        .map(|(color, &stop)| {
            if !stop.is_finite() || !(0.0..=1.0).contains(&stop) {
                return Err(HotlineColorError::InvalidPaletteStop {
                    color: color.clone(),
                    stop,
                });
            }
            Ok(PaletteEntry {
                stop,
                color: color.clone(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    entries.sort_by(|left, right| left.stop.total_cmp(&right.stop));
    for pair in entries.windows(2) {
        if pair[0].stop == pair[1].stop {
            return Err(HotlineColorError::DuplicatePaletteStop { stop: pair[0].stop });
        }
    }

    Ok(entries)
}

fn palette_cache_key(entries: &[PaletteEntry]) -> PaletteCacheKey {
    entries
        .iter()
        .map(|entry| {
            let stop = if entry.stop == 0.0 { 0.0 } else { entry.stop };
            (stop.to_bits(), entry.color.clone())
        })
        .collect()
}

#[cfg(target_arch = "wasm32")]
fn build_palette(entries: &[PaletteEntry]) -> Result<[[u8; 3]; PALETTE_SIZE], HotlineColorError> {
    use wasm_bindgen::JsCast;
    use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or(HotlineColorError::CanvasUnavailable)?;
    let canvas = document
        .create_element("canvas")
        .map_err(|_| HotlineColorError::CanvasUnavailable)?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| HotlineColorError::CanvasUnavailable)?;
    canvas.set_width(1);
    canvas.set_height(PALETTE_SIZE as u32);

    let context = canvas
        .get_context("2d")
        .map_err(|_| HotlineColorError::CanvasUnavailable)?
        .ok_or(HotlineColorError::CanvasUnavailable)?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| HotlineColorError::CanvasUnavailable)?;
    let gradient = context.create_linear_gradient(0.0, 0.0, 0.0, PALETTE_SIZE as f64);

    for entry in entries {
        gradient
            .add_color_stop(entry.stop as f32, &entry.color)
            .map_err(|_| HotlineColorError::InvalidPaletteColor {
                color: entry.color.clone(),
            })?;
    }

    context.set_fill_style_canvas_gradient(&gradient);
    context.fill_rect(0.0, 0.0, 1.0, PALETTE_SIZE as f64);
    let pixels = context
        .get_image_data(0.0, 0.0, 1.0, PALETTE_SIZE as f64)
        .map_err(|_| HotlineColorError::CanvasUnavailable)?
        .data()
        .0;

    if pixels.len() < PALETTE_SIZE * 4 {
        return Err(HotlineColorError::CanvasUnavailable);
    }

    let mut palette = [[0; 3]; PALETTE_SIZE];
    for (index, pixel) in pixels.chunks_exact(4).take(PALETTE_SIZE).enumerate() {
        palette[index].copy_from_slice(&pixel[..3]);
    }
    Ok(palette)
}

#[cfg(not(target_arch = "wasm32"))]
fn build_palette(entries: &[PaletteEntry]) -> Result<[[u8; 3]; PALETTE_SIZE], HotlineColorError> {
    #[derive(Debug, Clone, Copy)]
    struct Rgba {
        red: f64,
        green: f64,
        blue: f64,
        alpha: f64,
    }

    fn interpolate(start: Rgba, end: Rgba, fraction: f64) -> Rgba {
        let alpha = start.alpha + (end.alpha - start.alpha) * fraction;
        let red =
            start.red * start.alpha + (end.red * end.alpha - start.red * start.alpha) * fraction;
        let green = start.green * start.alpha
            + (end.green * end.alpha - start.green * start.alpha) * fraction;
        let blue =
            start.blue * start.alpha + (end.blue * end.alpha - start.blue * start.alpha) * fraction;

        if alpha == 0.0 {
            Rgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha,
            }
        } else {
            Rgba {
                red: red / alpha,
                green: green / alpha,
                blue: blue / alpha,
                alpha,
            }
        }
    }

    fn byte(channel: f64) -> u8 {
        (channel.clamp(0.0, 1.0) * 255.0).round() as u8
    }

    let colors = entries
        .iter()
        .map(|entry| {
            let parsed = csscolorparser::parse(&entry.color).map_err(|_| {
                HotlineColorError::InvalidPaletteColor {
                    color: entry.color.clone(),
                }
            })?;
            let color = Rgba {
                red: f64::from(parsed.r),
                green: f64::from(parsed.g),
                blue: f64::from(parsed.b),
                alpha: f64::from(parsed.a),
            };
            if [color.red, color.green, color.blue, color.alpha]
                .into_iter()
                .any(|channel| !channel.is_finite())
            {
                return Err(HotlineColorError::InvalidPaletteColor {
                    color: entry.color.clone(),
                });
            }
            Ok(color)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut palette = [[0; 3]; PALETTE_SIZE];
    for (index, result) in palette.iter_mut().enumerate() {
        let position = (index as f64 + 0.5) / PALETTE_SIZE as f64;
        let color = if position <= entries[0].stop {
            colors[0]
        } else if position >= entries[entries.len() - 1].stop {
            colors[colors.len() - 1]
        } else {
            let end_index = entries.partition_point(|entry| entry.stop < position);
            let start_index = end_index - 1;
            let start = &entries[start_index];
            let end = &entries[end_index];
            let fraction = (position - start.stop) / (end.stop - start.stop);
            interpolate(colors[start_index], colors[end_index], fraction)
        };
        *result = [byte(color.red), byte(color.green), byte(color.blue)];
    }

    Ok(palette)
}

fn interpolate_rgb(start: [u8; 3], end: [u8; 3], fraction: f64) -> [u8; 3] {
    std::array::from_fn(|index| {
        let start = f64::from(start[index]);
        let end = f64::from(end[index]);
        (start + (end - start) * fraction).round().clamp(0.0, 255.0) as u8
    })
}

fn format_rgb(rgb: [u8; 3], format: ColorFormat) -> String {
    match format {
        ColorFormat::Hex => format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]),
        ColorFormat::Rgb => format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HotlinePosition;

    fn raw_positions(values: &[(f64, f64, f64)]) -> HotlinePositionVec {
        HotlinePositionVec {
            positions: values
                .iter()
                .map(|&(lat, lng, value)| HotlinePosition::new(lat, lng, value))
                .collect(),
        }
    }

    fn red_blue_palette() -> HotlinePalette {
        HotlinePalette::new(&[("red", 0.0), ("blue", 1.0)])
    }

    #[test]
    fn queries_non_input_point_in_both_formats() {
        let positions = raw_positions(&[(0.0, -1.0, 0.0), (0.0, 1.0, 1.0)]);
        let lookup = HotlineColorLookup::new(&positions, &red_blue_palette(), 0.0, 1.0)
            .expect("lookup should build");

        assert_eq!(
            lookup.color_at(0.0, 0.0, ColorFormat::Hex),
            Ok("#800080".to_owned())
        );
        assert_eq!(
            lookup.color_at(0.0, 0.0, ColorFormat::Rgb),
            Ok("rgb(128, 0, 128)".to_owned())
        );
        assert_eq!(lookup.rgb_at(0.0, -0.5), Ok([191, 0, 64]));
    }

    #[test]
    fn interpolates_endpoint_colors_not_intermediate_palette_stops() {
        let positions = raw_positions(&[(0.0, -1.0, 0.0), (0.0, 1.0, 1.0)]);
        let palette = HotlinePalette::new(&[("red", 0.0), ("green", 0.5), ("blue", 1.0)]);
        let lookup =
            HotlineColorLookup::new(&positions, &palette, 0.0, 1.0).expect("lookup should build");
        let expected = interpolate_rgb(
            lookup.rgb_for_value(0.0).expect("start color"),
            lookup.rgb_for_value(1.0).expect("end color"),
            0.5,
        );

        assert_eq!(lookup.rgb_at(0.0, 0.0), Ok(expected));
        assert_ne!(lookup.rgb_at(0.0, 0.0), lookup.rgb_for_value(0.5));
    }

    #[test]
    fn uses_continuous_web_mercator_geometry() {
        let positions = raw_positions(&[(0.0, 0.0, 0.0), (60.0, 0.0, 1.0)]);
        let lookup = HotlineColorLookup::new(&positions, &red_blue_palette(), 0.0, 1.0)
            .expect("lookup should build");
        let start_y = project(0.0, 0.0).expect("start projects").y;
        let end_y = project(60.0, 0.0).expect("end projects").y;
        let midpoint_latitude = (2.0 * ((start_y + end_y) / 2.0).exp().atan()
            - std::f64::consts::FRAC_PI_2)
            .to_degrees();

        let projected_midpoint = lookup
            .rgb_at(midpoint_latitude, 0.0)
            .expect("projected midpoint should have a color");
        let coordinate_midpoint = lookup
            .rgb_at(30.0, 0.0)
            .expect("coordinate midpoint should have a color");

        assert!((i16::from(projected_midpoint[0]) - i16::from(projected_midpoint[2])).abs() <= 1);
        assert!(coordinate_midpoint[0] > coordinate_midpoint[2]);
    }

    #[test]
    fn chooses_last_segment_at_self_intersection() {
        let positions = raw_positions(&[
            (-1.0, -1.0, 0.0),
            (1.0, 1.0, 0.0),
            (-1.0, 1.0, 1.0),
            (1.0, -1.0, 1.0),
        ]);
        let lookup = HotlineColorLookup::new(&positions, &red_blue_palette(), 0.0, 1.0)
            .expect("lookup should build");

        assert_eq!(lookup.rgb_at(0.0, 0.0), Ok([0, 0, 255]));
    }

    #[test]
    fn chooses_nearest_segment() {
        let positions = raw_positions(&[
            (0.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (10.0, 0.0, 1.0),
            (10.0, 1.0, 1.0),
        ]);
        let lookup = HotlineColorLookup::new(&positions, &red_blue_palette(), 0.0, 1.0)
            .expect("lookup should build");

        assert_eq!(lookup.rgb_at(0.0, 0.5), Ok([255, 0, 0]));
        assert_eq!(lookup.rgb_at(10.0, 0.5), Ok([0, 0, 255]));
    }

    #[test]
    fn clamps_values_and_supports_reversed_ranges() {
        let positions = raw_positions(&[(0.0, 0.0, 0.0), (0.0, 1.0, 1.0)]);
        let normal = HotlineColorLookup::new(&positions, &red_blue_palette(), 0.0, 1.0)
            .expect("lookup should build");
        let reversed = HotlineColorLookup::new(&positions, &red_blue_palette(), 1.0, 0.0)
            .expect("lookup should build");

        assert_eq!(normal.rgb_for_value(-10.0), Ok([255, 0, 0]));
        assert_eq!(normal.rgb_for_value(10.0), Ok([0, 0, 255]));
        assert_eq!(reversed.rgb_for_value(1.0), Ok([255, 0, 0]));
        assert_eq!(reversed.rgb_for_value(0.0), Ok([0, 0, 255]));
    }

    #[test]
    fn one_off_methods_use_default_or_explicit_range() {
        let positions = raw_positions(&[(0.0, -1.0, 0.5), (0.0, 1.0, 1.0)]);
        let palette = red_blue_palette();
        let default = positions
            .color_at(0.0, 0.0, &palette, ColorFormat::Hex)
            .expect("default lookup");
        let custom = positions
            .color_at_with_range(0.0, 0.0, &palette, 0.0, 2.0, ColorFormat::Hex)
            .expect("custom lookup");

        assert_ne!(default, custom);
    }

    #[test]
    fn reuses_compiled_palette_for_unchanged_contents_and_clones() {
        let positions = raw_positions(&[(0.0, 0.0, 0.0), (0.0, 1.0, 1.0)]);
        let palette = red_blue_palette();
        let cloned_palette = palette.clone();
        assert_eq!(palette, cloned_palette);

        let first = HotlineColorLookup::new(&positions, &palette, 0.0, 1.0).expect("first lookup");
        let second =
            HotlineColorLookup::new(&positions, &palette, 0.0, 1.0).expect("second lookup");

        assert!(Arc::ptr_eq(&first.palette, &second.palette));

        let from_clone = HotlineColorLookup::new(&positions, &cloned_palette, 0.0, 1.0)
            .expect("lookup from cloned palette");

        assert!(Arc::ptr_eq(&first.palette, &from_clone.palette));
    }

    #[test]
    fn invalidates_compiled_palette_after_public_content_mutation() {
        let positions = raw_positions(&[(0.0, 0.0, 0.0), (0.0, 1.0, 1.0)]);
        let mut palette = red_blue_palette();
        let original =
            HotlineColorLookup::new(&positions, &palette, 0.0, 1.0).expect("original lookup");

        palette.palette.insert("blue".to_owned(), 0.5);
        let changed =
            HotlineColorLookup::new(&positions, &palette, 0.0, 1.0).expect("changed lookup");

        assert!(!Arc::ptr_eq(&original.palette, &changed.palette));
        assert_ne!(original.rgb_for_value(0.25), changed.rgb_for_value(0.25));

        palette.palette.insert("not a real color".to_owned(), 0.25);
        assert!(matches!(
            HotlineColorLookup::new(&positions, &palette, 0.0, 1.0),
            Err(HotlineColorError::InvalidPaletteColor { .. })
        ));

        palette.palette.remove("not a real color");
        let repaired =
            HotlineColorLookup::new(&positions, &palette, 0.0, 1.0).expect("repaired lookup");
        assert!(Arc::ptr_eq(&changed.palette, &repaired.palette));
    }

    #[test]
    fn empty_palette_uses_crate_default() {
        let positions = raw_positions(&[(0.0, 0.0, 0.0), (0.0, 1.0, 1.0)]);
        let empty = HotlinePalette::new(&[]);
        let empty_lookup = HotlineColorLookup::new(&positions, &empty, 0.0, 1.0)
            .expect("default palette should build");
        let default_lookup =
            HotlineColorLookup::new(&positions, &HotlinePalette::default(), 0.0, 1.0)
                .expect("explicit default palette should build");

        assert_eq!(
            empty_lookup.rgb_for_value(0.0),
            default_lookup.rgb_for_value(0.0)
        );
    }

    #[test]
    fn rejects_invalid_palette_inputs() {
        let positions = raw_positions(&[(0.0, 0.0, 0.0), (0.0, 1.0, 1.0)]);
        let duplicate = HotlinePalette::new(&[("red", 0.5), ("blue", 0.5)]);
        let invalid_stop = HotlinePalette::new(&[("red", -0.1), ("blue", 1.0)]);
        let invalid_color = HotlinePalette::new(&[("not a real color", 0.0), ("blue", 1.0)]);

        assert!(matches!(
            HotlineColorLookup::new(&positions, &duplicate, 0.0, 1.0),
            Err(HotlineColorError::DuplicatePaletteStop { stop: 0.5 })
        ));
        assert!(matches!(
            HotlineColorLookup::new(&positions, &invalid_stop, 0.0, 1.0),
            Err(HotlineColorError::InvalidPaletteStop { .. })
        ));
        assert!(matches!(
            HotlineColorLookup::new(&positions, &invalid_color, 0.0, 1.0),
            Err(HotlineColorError::InvalidPaletteColor { .. })
        ));
    }

    #[test]
    fn rejects_invalid_path_query_range_and_value() {
        let one_point = raw_positions(&[(0.0, 0.0, 1.0)]);
        let invalid_path = raw_positions(&[(0.0, 0.0, 0.0), (0.0, 1.0, f64::NAN)]);
        let positions = raw_positions(&[(0.0, 0.0, 0.0), (0.0, 1.0, 1.0)]);

        assert_eq!(
            HotlineColorLookup::new(&one_point, &red_blue_palette(), 0.0, 1.0),
            Err(HotlineColorError::NoDrawableSegments)
        );
        assert!(matches!(
            HotlineColorLookup::new(&invalid_path, &red_blue_palette(), 0.0, 1.0),
            Err(HotlineColorError::InvalidPathValue { index: 1, .. })
        ));
        assert!(matches!(
            HotlineColorLookup::new(&positions, &red_blue_palette(), 1.0, 1.0),
            Err(HotlineColorError::InvalidValueRange { .. })
        ));

        let lookup = HotlineColorLookup::new(&positions, &red_blue_palette(), 0.0, 1.0)
            .expect("lookup should build");
        assert!(matches!(
            lookup.rgb_at(f64::NAN, 0.0),
            Err(HotlineColorError::InvalidQueryPosition { .. })
        ));
        assert!(matches!(
            lookup.rgb_for_value(f64::NAN),
            Err(HotlineColorError::InvalidValue { .. })
        ));
    }
}
