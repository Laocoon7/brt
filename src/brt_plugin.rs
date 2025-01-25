use std::path::{Path, PathBuf};

use bevy::prelude::*;

use crate::{direction::Direction, grid::Grid, grid_shapes::{Circle, Line, Rectangle}, random::{Dice, Random}, resources::Folders, systems::remove_resource};
#[cfg(feature = "icon")]
use crate::{resources::WindowIcon, systems::set_window_icon};

#[derive(Default)]
pub struct BrtPlugin {
    pub base: PathBuf,
    pub qualifier: String,
    pub organization: String,
    pub application: String,

    #[cfg(feature = "icon")]
    pub icon: Option<&'static [u8]>,
}

impl BrtPlugin {
    pub fn new(
        base: impl AsRef<Path>,
        qualifier: impl ToString,
        orginization: impl ToString,
        application: impl ToString,
    ) -> Self {
        Self {
            base: base.as_ref().to_path_buf(),
            qualifier: qualifier.to_string(),
            organization: orginization.to_string(),
            application: application.to_string(),
            
            #[cfg(feature = "icon")]
            icon: None,
        }
    }

    #[cfg(feature = "icon")]
    pub fn with_icon(mut self, icon: &'static [u8]) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl Plugin for BrtPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        // Direction
        app.register_type::<Direction>();

        // Grid
        app.register_type::<Grid<Entity>>();
        app.register_type::<Grid<Option<Entity>>>();
        app.register_type::<Grid<Vec<Entity>>>();
        app.register_type::<Grid<bool>>();
        app.register_type::<Circle>();
        app.register_type::<Line>();
        app.register_type::<Rectangle>();

        app.register_type::<Folders>();
        app.insert_resource(Folders::new(
            &self.base,
            &self.qualifier,
            &self.organization,
            &self.application,
        ));
        
        app.register_type::<Dice>();
        app.register_type::<Random>();

        #[cfg(feature = "icon")]
        if let Some(icon) = self.icon {
            app.insert_resource(WindowIcon(icon));
            app.add_systems(Startup, (set_window_icon, remove_resource::<WindowIcon>).chain());
        }
    }
}
