use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::save_load::{SaveLoadData, SaveLoadDataType, SaveLoadError};

pub trait SaveLoad: Serialize + for<'de> Deserialize<'de> {
    const DATA: SaveLoadData;

    fn build_path(file_name: impl AsRef<Path>) -> PathBuf {
        let mut file_path = PathBuf::from(Self::DATA.directory);
        file_path.push(file_name);
        file_path.set_extension(Self::DATA.extension);
        file_path
    }

    fn load_file(file_name: impl AsRef<Path>) -> Result<Self, SaveLoadError> {
        let contents = std::fs::read_to_string(Self::build_path(file_name))?;

        match Self::DATA.data_type {
            SaveLoadDataType::Ron => Ok(ron::from_str::<Self>(&contents)?),
            SaveLoadDataType::Toml => Ok(toml::from_str::<Self>(&contents)?),
        }
    }

    fn save_file(&self, file_name: impl AsRef<Path>) -> Result<(), SaveLoadError> {
        let contents = match Self::DATA.data_type {
            SaveLoadDataType::Ron => if Self::DATA.pretty {
                ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?
            } else {
                ron::ser::to_string(self)?
            },
            SaveLoadDataType::Toml => if Self::DATA.pretty {
                toml::to_string_pretty(self)?
            } else {
                toml::to_string(self)?
            },
        };

        std::fs::write(Self::build_path(file_name), contents)?;

        Ok(())
    }
}
