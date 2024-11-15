use std::path::PathBuf;

/// Options for initializing theme and responsive loader.
///
/// `path` defaults to `./styles` if not defined. Automatically detects if given
/// path is directory or file.
///
/// `recursive` sets the filesystem notifier mode to also detect changes in subfolders.
/// Has no have effect if path points to file.
pub struct ProviderOptions {
    pub path: PathBuf,
    pub recursive: bool,
}

impl Default for ProviderOptions {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./styles"),
            recursive: true,
        }
    }
}
