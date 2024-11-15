/// Default read buffer size
const DEFAULT_BUF_SIZE: usize = 16 * 1024;

/// If path points to singe file, returns file content as `String`
///
/// If path points to directory, reads all `.css` files and combines them to single `String`
///
/// # Errors
/// Returns `std::io::Error` if path is not readable
pub fn read_styles(path: &std::path::Path) -> Result<String, std::io::Error> {
    if path.is_dir() {
        let combined = std::fs::read_dir(path)?
            .filter_map(Result::ok)
            .filter_map(|e| {
                e.path()
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("css"))
                    .then_some(e.path())
            })
            .filter_map(|path| match std::fs::read_to_string(&path) {
                Err(e) => {
                    log::warn!("Failed to read file {path:?}\n{e}");
                    None
                }
                Ok(v) => Some(v),
            })
            .fold(String::with_capacity(DEFAULT_BUF_SIZE), |mut s, c| {
                s.push_str(&c);
                s
            });

        Ok(combined)
    } else {
        Ok(std::fs::read_to_string(path)?)
    }
}
