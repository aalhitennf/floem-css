use std::cell::OnceCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

use floem::reactive::{RwSignal, SignalUpdate};
use floem::style::Style;
use floem::IntoView;

use crate::error::ThemeError;
use crate::style::parser::parse_css;
#[derive(Default)]
pub struct StyleMap(pub(crate) HashMap<String, Style>);

impl StyleMap {
    #[must_use]
    pub fn apply_classes(&self, s: Style, class_str: &str) -> Style {
        class_str.split_whitespace().fold(s, |s, key| {
            s.apply_opt(self.get(key), |s, t| s.apply(t.clone()))
        })
    }
}

impl Deref for StyleMap {
    type Target = HashMap<String, Style>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StyleMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

thread_local! {
    pub(crate) static THEME: OnceCell<Theme> = OnceCell::new();
}

pub fn get_signal() -> Option<RwSignal<StyleMap>> {
    THEME.with(|t| t.get().map(|t| t.map))
}

pub struct Theme {
    pub(crate) map: RwSignal<StyleMap>,
}

impl Theme {
    /// # Errors
    ///
    /// Returns `Err` if `path` cannot be read
    pub fn new() -> Result<Self, ThemeError> {
        let theme = Theme {
            map: RwSignal::new(StyleMap::default()),
        };
        theme.reload()?;
        Ok(theme)
    }

    /// # Errors
    /// Errors if path cannot be read
    ///
    /// # Panics
    /// Panics only in debug mode if time is flowing into wrong direction
    pub fn reload(&self) -> Result<(), ThemeError> {
        let styles_str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));
        let parsed_styles = parse_css(&styles_str);
        if parsed_styles.is_empty() {
            log::warn!("Styles parsed but no styles found");
        }
        self.map.update(|map| {
            map.clear();
            let _ = std::mem::replace(map, parsed_styles);
        });
        Ok(())
    }
}

pub struct ThemeOptions {
    #[allow(unused)]
    path: PathBuf,
}

impl ThemeOptions {
    #[must_use]
    pub fn with_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

/// Wraps given view in "body" class and provides `Theme` as context
/// # Panics
///
/// Panics if path doesn't point to a existing folder.
pub fn theme_provider<V, F>(child: F, _: ThemeOptions) -> V
where
    F: Fn() -> V,
    V: IntoView + 'static,
{
    let theme = Theme::new().expect("Invalid theme path");
    if THEME.with(|t| t.set(theme)).is_err() {
        panic!("Theme is already set");
    }
    child()
}
