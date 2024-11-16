use std::path::Path;

/// Default read buffer size
const DEFAULT_BUF_SIZE: usize = 16 * 1024;

/// If path points to singe file, returns file content as `String`
///
/// If path points to directory, reads all `.css` files and combines them to single `String`
///
/// # Errors
/// Returns `std::io::Error` if path is not readable
pub fn read_styles(path: &std::path::Path) -> Result<String, std::io::Error> {
    if path.is_file() {
        Ok(std::fs::read_to_string(path)?)
    } else {
        let mut buf = String::with_capacity(DEFAULT_BUF_SIZE);
        read_dir_recursive(&mut buf, path);
        Ok(buf)
    }
}

fn read_dir_recursive(buf: &mut String, path: &Path) {
    let dir_entry = match std::fs::read_dir(path) {
        Ok(dir_entry) => dir_entry,
        Err(e) => {
            log::warn!("Failed to read {path:?}: {e}");
            return;
        }
    };
    for entry in dir_entry.flatten() {
        let entry_path = entry.path();
        if entry_path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("css"))
        {
            match std::fs::read_to_string(&entry_path) {
                Ok(content) => {
                    buf.push_str(&content);
                }
                Err(e) => {
                    log::warn!("Failed to read {entry_path:?}: {e}");
                }
            }
        } else if entry_path.is_dir() {
            read_dir_recursive(buf, &entry_path);
        }
    }
}
