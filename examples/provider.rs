use std::path::PathBuf;

use floem::keyboard::NamedKey;
use floem::views::{button, container, text, v_stack, Button, Decorators, Label};
use floem::{IntoView, View};
use floem_css::{theme_provider, ProviderOptions, StyleCss};

fn main() {
    // Enable logging to see info and errors
    env_logger::builder()
        .filter_level(log::LevelFilter::Off)
        .filter_module("floem_css", log::LevelFilter::Debug)
        .init();

    // Styles are read from path. Modify the css file to instantly see changes in app.
    // Path can point to file or folder.
    let options = ProviderOptions {
        path: PathBuf::from("examples/style.css"),
        ..Default::default()
    };

    // Wrap your app in theme_provider
    floem::launch(|| theme_provider(main_view, options))
}

fn light_button(lbl: &'static str) -> Button {
    button(lbl).css("button button-light")
}

fn dark_button(lbl: &'static str) -> Button {
    button(lbl).css("button button-dark")
}

fn h3(lbl: &str) -> Label {
    text(lbl).css("h3")
}

fn main_view() -> impl IntoView {
    let button_light_1 = light_button("Light button 1");
    let button_light_2 = light_button("Light button 2");
    let button_dark_1 = dark_button("Dark button 1");
    let button_dark_2 = dark_button("Dark button 2");
    let some_header_1 = h3("Light buttons");
    let some_header_2 = h3("Dark buttons");
    let stack = v_stack((
        some_header_1,
        button_light_1,
        button_light_2,
        some_header_2,
        button_dark_1,
        button_dark_2,
    ))
    .css(".button-stack");
    let main = container(stack).css("body");
    let id = main.id();
    main.keyboard_navigable().on_key_up(
        NamedKey::F11.into(),
        |m| m.is_empty(),
        move |_| id.inspect(),
    )
}
