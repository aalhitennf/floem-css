##### floem-css

Hot reload css styles for [Floem (git)](https://github.com/lapce/floem)

Run example with:

    cargo run --example provider

And modify `examples/provider/example.css` to update the app style instantly.

When building or running with `--release` flag, you must specify environment variable `STYLE_PATH` that points to folder where your styles are located.
Styles are then statically compiled in the app build-time and responsive updates no longer work.

Building example (bash):

    STYLE_PATH=$(pwd)/examples/provider cargo build --example provider --release
