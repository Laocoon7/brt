/// Serialization type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveLoadDataType {
    /// Normally used for objects
    Ron,
    /// Normally used for configurations
    Toml,
}

impl SaveLoadDataType {
    const RON_EXTENSION: &str = "ron";
    const TOML_EXTENSION: &str = "toml";

    pub const fn extension(&self) -> &'static str {
        match self {
            SaveLoadDataType::Ron => Self::RON_EXTENSION,
            SaveLoadDataType::Toml => Self::TOML_EXTENSION,
        }
    }
}