#[cfg(not(debug_assertions))]
fn main() {
    use std::path::PathBuf;

    fn read_styles(path: &str) -> Result<String, Box<dyn std::error::Error>> {
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

    // Get the path to the file from an environment variable

    let file_path = std::env::var("STYLE_PATH").expect("STYLE_PATH must be set");

    println!("cargo::rerun-if-changed={file_path}");

    // Read the file contents
    let contents = read_styles(&file_path).expect("Failed to read files from STYLE_PATH");

    // Generate a Rust source file with the contents
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir).join("style.css");

    std::fs::write(out_dir, contents).expect("Failed to write style.css");
}

#[cfg(debug_assertions)]
fn main() {}
