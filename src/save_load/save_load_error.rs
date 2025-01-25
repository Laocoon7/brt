use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaveLoadError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ron deserializer error: {0}")]
    RonDe(#[from] ron::error::SpannedError),
    #[error("Ron serializer error: {0}")]
    RonSer(#[from] ron::error::Error),
    #[error("Toml deserializer error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("Toml serializer error: {0}")]
    TomlSer(#[from] toml::ser::Error),
}