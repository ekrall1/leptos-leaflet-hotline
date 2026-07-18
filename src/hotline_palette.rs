//! module for hotline palette data structure and functions
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex, MutexGuard},
};

pub(crate) const PALETTE_SIZE: usize = 256;
pub(crate) type CompiledPalette = [[u8; 3]; PALETTE_SIZE];
pub(crate) type PaletteCacheKey = Vec<(u64, String)>;

#[derive(Debug, Clone)]
struct CachedPalette {
    key: PaletteCacheKey,
    colors: Arc<CompiledPalette>,
}

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
/// The browser-compatible 256-entry color table is compiled lazily and cached
/// by palette contents. Direct mutations of `palette` are detected the next
/// time a color lookup is prepared.
///
#[non_exhaustive]
pub struct HotlinePalette {
    pub palette: HashMap<String, f64>,
    cache: Arc<Mutex<Option<CachedPalette>>>,
}

impl fmt::Debug for HotlinePalette {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HotlinePalette")
            .field("palette", &self.palette)
            .finish()
    }
}

impl Clone for HotlinePalette {
    fn clone(&self) -> Self {
        Self {
            palette: self.palette.clone(),
            cache: Arc::clone(&self.cache),
        }
    }
}

impl PartialEq for HotlinePalette {
    fn eq(&self, other: &Self) -> bool {
        self.palette == other.palette
    }
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
            cache: Arc::new(Mutex::new(None)),
        }
    }

    pub(crate) fn compiled_palette<E>(
        &self,
        key: PaletteCacheKey,
        compile: impl FnOnce() -> Result<CompiledPalette, E>,
    ) -> Result<Arc<CompiledPalette>, E> {
        let mut cache = self.lock_cache();
        if let Some(cached) = cache.as_ref().filter(|cached| cached.key == key) {
            return Ok(Arc::clone(&cached.colors));
        }

        let colors = Arc::new(compile()?);
        *cache = Some(CachedPalette {
            key,
            colors: Arc::clone(&colors),
        });
        Ok(colors)
    }

    fn lock_cache(&self) -> MutexGuard<'_, Option<CachedPalette>> {
        self.cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
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
