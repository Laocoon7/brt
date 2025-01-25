use crate::save_load::SaveLoadDataType;

pub struct SaveLoadData {
    pub directory: &'static str,
    pub extension: &'static str,
    pub data_type: SaveLoadDataType,
    pub pretty: bool,
}

impl SaveLoadData {
    pub const fn new(
        directory: &'static str,
        data_type: SaveLoadDataType,
        pretty: bool,
    ) -> Self {
        Self {
            directory,
            data_type,
            extension: data_type.extension(),
            pretty,
        }
    }

    pub const fn with_extension(mut self, extension: &'static str) -> Self {
        self.extension = extension;
        self
    }
}