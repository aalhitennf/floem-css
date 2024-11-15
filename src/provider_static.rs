use floem::reactive::{provide_context, RwSignal, SignalUpdate};
use floem::IntoView;

use crate::error::ThemeError;
use crate::parser::parse_css;
use crate::style::StyleMap;
use crate::ProviderOptions;

pub struct StyleProvider {
    pub(crate) map: RwSignal<StyleMap>,
}

impl StyleProvider {
    /// # Errors
    ///
    /// Returns `Err` if `path` cannot be read
    pub fn new() -> Result<Self, ThemeError> {
        let theme = Self {
            map: RwSignal::new(StyleMap::new_const()),
        };
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

/// Provides `Theme` as context
/// # Panics
///
/// Panics if path doesn't point to a existing folder.
pub fn theme_provider<V, F>(child: F, _: ProviderOptions) -> V
where
    F: Fn() -> V,
    V: IntoView + 'static,
{
    let theme = StyleProvider::new().expect("Failed to create provider");
    theme.reload().expect("Cannot load theme");
    let rc_theme = std::rc::Rc::new(theme);
    provide_context(rc_theme);
    child()
}
