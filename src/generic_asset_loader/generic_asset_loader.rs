use std::marker::PhantomData;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use serde::Deserialize;

use crate::generic_asset_loader::GenericAssetLoaderError;

pub struct GenericAssetLoader<T> {
    extensions: &'static [&'static str],
    phantom: PhantomData<T>,
}

impl<T> Default for GenericAssetLoader<T> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<T> GenericAssetLoader<T> {
    pub const fn new(extensions: &'static [&'static str]) -> Self {
        Self {
            extensions,
            phantom: PhantomData,
        }
    }
}

impl<T: Asset + for<'de> Deserialize<'de>> AssetLoader for GenericAssetLoader<T> {
    type Asset = T;
    type Error = GenericAssetLoaderError;
    type Settings = ();

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut contents = String::new();
        reader.read_to_string(&mut contents).await?;
        Ok(ron::de::from_str::<T>(&contents)?)
    }

    fn extensions(&self) -> &[&str] {
        self.extensions
    }
}
