use std::cell::OnceCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

use crossbeam_channel::{Receiver, Sender};
use floem::ext_event::create_signal_from_channel;
use floem::reactive::{create_effect, RwSignal, SignalGet, SignalUpdate};
use floem::style::Style;
use floem::IntoView;

use crate::error::ThemeError;
use crate::observer::FileObserver;
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
    pub(crate) static THEME: OnceCell<Theme> = const { OnceCell::new() };
}

pub fn get_signal() -> Option<RwSignal<StyleMap>> {
    THEME.with(|t| t.get().map(|t| t.map))
}

pub struct Theme {
    path: PathBuf,
    channel: (Sender<()>, Receiver<()>),
    pub(crate) map: RwSignal<StyleMap>,
    #[allow(unused)]
    observer: FileObserver,
}

impl Theme {
    /// # Errors
    ///
    /// Returns `Err` if `path` cannot be read
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ThemeError> {
        let path = path.as_ref();
        Theme::try_from(path)
    }

    /// # Errors
    /// Errors if path cannot be read
    ///
    /// # Panics
    /// Panics only in debug mode if time is flowing into wrong direction
    pub fn reload(&self) -> Result<(), ThemeError> {
        let now = std::time::SystemTime::now();
        let styles_str = read_styles(&self.path)?;
        let parsed_styles = parse_css(&styles_str);
        if parsed_styles.is_empty() {
            log::warn!("Styles parsed but no styles found");
        }
        self.map.update(|map| {
            map.clear();
            let _ = std::mem::replace(map, parsed_styles);
        });
        {
            let elaps = std::time::SystemTime::now()
                .duration_since(now)
                .expect("Time is going backwards");
            log::debug!("Styles parsed in {}ms", elaps.as_millis());
        }
        Ok(())
    }
}

impl TryFrom<&Path> for Theme {
    type Error = ThemeError;
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let path = path.to_path_buf();
        let channel = crossbeam_channel::unbounded();
        let observer = FileObserver::new(&path, channel.0.clone(), true)?;
        let theme = Theme {
            path,
            observer,
            channel,
            map: RwSignal::new(StyleMap::default()),
        };
        theme.reload()?;
        Ok(theme)
    }
}

pub struct ThemeOptions {
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

fn read_styles(path: &Path) -> Result<String, ThemeError> {
    let combined = std::fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter_map(|e| {
            e.path()
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("css"))
                .then_some(e.path())
        })
        .flat_map(std::fs::read_to_string)
        .fold(String::new(), |mut s, c| {
            s.push_str(&c);
            s
        });

    Ok(combined)
}

fn reload_theme() {
    THEME.with(|t| {
        if let Some(theme) = t.get() {
            if let Err(e) = theme.reload() {
                log::error!("{e}");
            }
        }
    });
}

/// Wraps given view in "body" class and provides `Theme` as context
/// # Panics
///
/// Panics if path doesn't point to a existing folder.
pub fn theme_provider<V, F>(child: F, options: ThemeOptions) -> V
where
    F: Fn() -> V,
    V: IntoView + 'static,
{
    let theme = Theme::new(options.path).expect("Invalid theme path");
    let observer_event = create_signal_from_channel(theme.channel.1.clone());
    assert!(THEME.with(|t| t.set(theme)).is_ok(), "Theme is already set");
    create_effect(move |_| {
        if let Some(()) = observer_event.get() {
            reload_theme();
        }
    });
    child()
}
