use std::rc::Rc;

use floem::reactive::{use_context, SignalWith};
use floem::style::Style;
use floem::views::Decorators;
use floem::IntoView;
use floem_css_parser::css_to_rules;
use smallvec::SmallVec;

use crate::parser::parse_rules;
use crate::StyleProvider;

pub trait StyleCss: IntoView {
    #[must_use]
    fn css(self, keys: &'static str) -> <Self as IntoView>::V;
}

impl<V> StyleCss for V
where
    V: IntoView + 'static,
{
    /// # Panics
    /// Panics at compile time if `RwSignal<Theme>` context is not provided
    fn css(self, keys: &'static str) -> <Self as IntoView>::V {
        let theme = use_context::<Rc<StyleProvider>>().unwrap();
        self.style(move |s| theme.map.with(|t| t.apply_classes(s, keys)))
            .debug_name(keys)
    }
}

pub struct StyleMap {
    keys: SmallVec<[String; 32]>,
    styles: SmallVec<[Style; 32]>,
}

impl StyleMap {
    pub fn from_css(input: &str) -> Self {
        parse_rules(&css_to_rules(input))
    }
}

impl StyleMap {
    pub const fn new_const() -> Self {
        Self {
            keys: SmallVec::new_const(),
            styles: SmallVec::new_const(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Style> {
        self.keys
            .iter()
            .position(|k| k == key)
            .and_then(|idx| self.styles.get(idx).cloned())
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.styles.clear();
    }

    pub fn insert(&mut self, key: &str, style: Style) {
        self.keys.push(key.to_string());
        self.styles.push(style);
    }

    pub fn remove(&mut self, key: &str) -> Option<Style> {
        self.keys.iter().position(|k| k == key).map(|idx| {
            self.keys.remove(idx);
            self.styles.remove(idx)
        })
    }
}

impl StyleMap {
    #[must_use]
    pub fn apply_classes(&self, s: Style, class_str: &str) -> Style {
        class_str
            .split_whitespace()
            .fold(s, |s, key| s.apply_opt(self.get(key), Style::apply))
    }
}
