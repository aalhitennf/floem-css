use thiserror::Error;

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("Filesystem error")]
    IO(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Observer error: {0}")]
    Observer(#[from] notify::Error),
}
