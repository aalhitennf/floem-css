use thiserror::Error;

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("Filesystem error")]
    IO(#[from] std::io::Error),
    #[error("Observer error: {0}")]
    Observer(#[from] notify::Error),
}
