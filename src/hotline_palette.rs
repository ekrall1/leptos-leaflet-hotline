//! module for hotline palette data structure and functions
use std::collections::HashMap;

/// the default color palette used
/// hotline options will be created with the default palette
/// if no palette is available.  \
/// Colors can be color names or hex codes, breakpoints
/// indicate the relative cutoff in the values for each color.
const DEFAULT_PALETTE_VALUES: &[(&str, f64)] = &[
    ("green", 0.0),
    ("blue", 0.33),
    ("#ffff00", 0.67),
    ("red", 1.0),
];

///
/// struct data type for hotline palette
///
/// # Fields
/// * `palette` [`HashMap<String, f64>`] mapping of colors to breakpoints
///
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct HotlinePalette {
    pub palette: HashMap<String, f64>,
}

/// implement constructor for [`HotlinePalette`]
impl HotlinePalette {
    ///
    /// construct a new [`HotlinePalette`]
    ///
    /// # Returns
    /// [`HotlinePalette`]
    ///
    #[must_use]
    #[inline]
    pub fn new(palette: &[(&str, f64)]) -> Self {
        let mut palette_hashmap = HashMap::new();

        for &(key, val) in palette {
            palette_hashmap.insert(key.to_owned(), val);
        }

        Self {
            palette: palette_hashmap,
        }
    }
}

/// implement default for [`HotlinePalette`]
impl Default for HotlinePalette {
    ///
    /// create new [`HotlinePalette`] with default options
    ///
    /// # Returns
    /// [`HotlinePalette`]
    ///
    #[inline]
    fn default() -> Self {
        Self::new(DEFAULT_PALETTE_VALUES)
    }
}
