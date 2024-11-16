use std::path::PathBuf;
use std::rc::Rc;

use crossbeam_channel::{Receiver, Sender};
use floem::ext_event::create_signal_from_channel;
use floem::reactive::{create_effect, provide_context, RwSignal, SignalGet, SignalUpdate};
use floem::IntoView;

use crate::error::ThemeError;
use crate::observer::FileObserver;
use crate::parser::parse_css;
use crate::style::StyleMap;
use crate::ProviderOptions;

pub struct StyleProvider {
    path: PathBuf,
    channel: (Sender<()>, Receiver<()>),
    pub(crate) map: RwSignal<StyleMap>,
    #[allow(unused)]
    observer: FileObserver,
}

impl StyleProvider {
    /// # Errors
    ///
    /// Returns `ThemeError` if `path` cannot be read
    pub fn new(options: ProviderOptions) -> Result<Self, ThemeError> {
        Self::try_from(options)
    }

    /// # Errors
    /// Errors if path cannot be read
    ///
    /// # Panics
    /// Panics only in debug mode if time is flowing into wrong direction
    fn reload(&self) -> Result<(), ThemeError> {
        let styles_str = floem_css_parser::read_styles(&self.path)?;
        let s = styles_str.clone();
        std::thread::spawn(move || {
            floem_css_parser::analyze(&s);
        });
        let new_map = parse_css(&styles_str);
        if new_map.is_empty() {
            log::warn!("Styles parsed but no styles found");
        }
        self.map.update(|map| {
            let _ = std::mem::replace(map, new_map);
        });
        Ok(())
    }
}

impl TryFrom<ProviderOptions> for StyleProvider {
    type Error = ThemeError;
    fn try_from(options: ProviderOptions) -> Result<Self, Self::Error> {
        let channel = crossbeam_channel::unbounded();
        let observer = FileObserver::new(&options.path, channel.0.clone(), options.recursive)?;
        let theme = Self {
            path: options.path,
            observer,
            channel,
            map: RwSignal::new(StyleMap::new_const()),
        };
        Ok(theme)
    }
}

/// Wrapper function that provides all necessary things in context
/// for hot reloading to work
///
/// ## Example
///
/// ### style.css
/// ```css
/// body {
///     flex-grow: 1;
/// }
///
/// my-header {
///     font-size: 32px;
///     font-weight: 600;
/// }
/// ```
///### main.rs
/// ```rust
/// use floem::views::{container, text};
/// use floem::IntoView;
/// use floem_css::{theme_provider, ProviderOptions, StyleCss};
///
/// fn main() {
///     // Styles are read from this path.
///     // Modify the css file to instantly see changes in app.
///     // Path can point to file or folder.
///     let options = ProviderOptions {
///         path: "./examples/style.css".into(),
///         ..Default::default()
///     };
///
///     // Wrap your app in theme_provider and launch
///     floem::launch(|| theme_provider(main_view, options))
/// }
///
/// fn main_view() -> impl IntoView {
///     let my_text = text("Change my style").css("my-header");
///     container(my_text).css("body")
/// }
/// ```
///
/// # Panics
///
/// Panics if options path doesn't exist in filesystem or is otherwise unreadable
pub fn theme_provider<V, F>(child: F, options: ProviderOptions) -> V
where
    F: Fn() -> V,
    V: IntoView + 'static,
{
    let theme = StyleProvider::new(options).expect("Invalid theme path");
    theme.reload().expect("Cannot load theme");
    let observer_event = create_signal_from_channel(theme.channel.1.clone());
    let rc_theme = Rc::new(theme);
    provide_context(rc_theme.clone());
    create_effect(move |_| {
        if observer_event.get().is_some() {
            if let Err(e) = rc_theme.reload() {
                log::error!("Cannot reload theme: {e}");
            }
        }
    });
    child()
}
