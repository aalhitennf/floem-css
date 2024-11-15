# floem-css

Hot reloading css-like styles for [Floem](https://github.com/lapce/floem)

floem-css at least tries to support all currently supported style properties that Floem supports.
Supported properties can be found from `src/style/parser.rs` and examining enum `StyleParser` variants.

_This library is experimental and made for personal usage, some features are lacking, and it may have lots of breaking changes in future_

### Usage

#### style.css

```css
body {
    flex-grow: 1;
}

my-header {
    font-size: 32px;
    font-weight: 600;
}
```

#### main.rs

```rust
use floem::views::{container, text};
use floem::IntoView;
use floem_css::{theme_provider, ProviderOptions, StyleCss};

fn main() {
    // Styles are read from this path.
    // Modify the css file to instantly see changes in app.
    // Path can point to file or folder.
    let options = ProviderOptions {
        path: "./style.css".into(),
        ..Default::default()
    };

    // Wrap your app in theme_provider and launch
    floem::launch(|| theme_provider(main_view, options))
}

fn main_view() -> impl IntoView {
    let my_text = text("Change my style").css("my-header");
    container(my_text).css("body")
}

```

This example shows to use this library. Now just run the app with `cargo run` and edit your css file to
see the changes immediately on the app. There is also larger example available, see examples section below.

### Differences to normal css

#### Simple Selectors

Selectors don't have `.` or `#` prefixes for classes, unless you decide to do so manually.
So rust code `button("click").css("button")`, matches with css selector `button`, not `.button` like you would expect with normal css.

#### No combinators

There is no support for combinators. Defining descendant, child or sibling combinators is not supported.
In example normally you would define styles for specific descendants like this:

Rust:

```
let home = button("home").css("button");
let about = button("home").css("button");
let navigation = v_stack((home, about)).css("navigation");
```

Css:

```
button { ... } // Applies to all elements with class "button"
navigation button { ... } // Applies to all buttons that are descendants of navigation
```

But that will not work as expected. You have define specific rule:

Rust:

```
let home = button("home").css("button nav-button");
let about = button("home").css("button nav-button");
let navigation = v_stack((home, about)).css("navigation");
```

Css:

```
button { ... } // Applies to all elements with class "button"
nav-button { ... } // Applies to all elements with class "nav-button"
```

To make this easier and your code less cluttered, it's recommended to create wrapper function for elements that are used often:

```
fn my_button<V: IntoView + 'static>(child: V) -> Button {
    button(child).css("button")
}

fn nav_button<V: IntoView + 'static>(child: V) -> Button {
    my_button(child).css("nav-button")
}
```

```
let home = nav_button("home");
let about = nav_button("about");
let navigation = v_stack((home, about)).css("navigation");
```

### Examples

Run example with:

    cargo run --example provider

And modify `examples/style.css` to update the apps style instantly.

### Building

When building or running with `--release` flag, you must specify environment variable `STYLE_PATH` that points to file or folder where your style(s) are located.
Styles are then compiled in the app build-time and responsive updates no longer work.

Building example (bash):

    STYLE_PATH=$(pwd)/examples/provider cargo build --example provider --release

Or to make it easier set variable for project `.cargo/config.toml`:

    [env]
    STYLE_PATH = "/path/to/my/style.css"
