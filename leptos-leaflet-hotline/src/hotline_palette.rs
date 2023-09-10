use std::collections::HashMap;

const DEFAULT_PALETTE_VALUES: &[(&str, f64)] = &[
    ("green", 0.0),
    ("blue", 0.33),
    ("#ffff00", 0.67),
    ("red", 1.0),
];

#[derive(Debug, Clone, PartialEq)]
pub struct HotlinePalette {
    pub palette: HashMap<String, f64>,
}

impl HotlinePalette {
    pub fn new(palette: &[(&str, f64)]) -> Self {
        let mut palette_hashmap = HashMap::new();

        for &(key, val) in palette {
            palette_hashmap.insert(key.to_string(), val);
        }

        HotlinePalette {
            palette: palette_hashmap,
        }
    }
}

impl Default for HotlinePalette {
    fn default() -> Self {
        HotlinePalette::new(DEFAULT_PALETTE_VALUES)
    }
}

pub fn hotline_palette(palette: &[(&str, f64)]) -> HotlinePalette {
    HotlinePalette::new(&palette)
}
