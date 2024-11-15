#[cfg(not(debug_assertions))]
fn main() {
    println!("cargo::rerun-if-env-changed=STYLE_PATH");
    // Get the path to the file from an environment variable
    let file_path = std::env::var("STYLE_PATH").expect("STYLE_PATH must be set");
    println!("cargo::rerun-if-changed={file_path}");
    // Read the file contents
    let contents = floem_css_parser::read_styles(&file_path.as_ref())
        .expect("Failed to read files from STYLE_PATH");
    // Generate a Rust source file with the contents
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(out_dir).join("style.css");
    std::fs::write(out_dir, contents).expect("Build script failed to write combined style.css");
}

#[cfg(debug_assertions)]
fn main() {}
