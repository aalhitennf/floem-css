pub mod parser;

use floem::reactive::SignalWith;
use floem::views::Decorators;
use floem::IntoView;

use crate::theme::get_signal;

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
        let Some(map_signal) = get_signal() else {
            log::error!("Theme not set. Use theme_provider.");
            return self.style(|s| s);
        };
        self.style(move |s| map_signal.with(|t| t.apply_classes(s, keys)))
            .debug_name(keys)
    }
}
