#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

#[cfg(debug_assertions)]
mod observer;

#[cfg(debug_assertions)]
mod provider;
#[cfg(not(debug_assertions))]
#[path = "provider_static.rs"]
mod provider;

mod error;
mod options;
mod parser;
mod style;

pub use options::ProviderOptions;
pub use provider::{theme_provider, StyleProvider};
pub use style::{StyleCss, StyleMap};
