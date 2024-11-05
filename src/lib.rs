#![allow(clippy::module_name_repetitions)]

#[cfg(debug_assertions)]
mod observer;

#[cfg(debug_assertions)]
#[path = "theme.rs"]
mod theme;
#[cfg(not(debug_assertions))]
#[path = "theme_static.rs"]
mod theme;

mod error;
mod style;

pub use style::StyleCss;
pub use theme::{theme_provider, Theme, ThemeOptions};
