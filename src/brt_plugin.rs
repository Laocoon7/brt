use std::path::Path;

use bevy::prelude::*;

use crate::{direction::Direction, grid::Grid, grid_shapes::{Circle, Line, Rectangle}, random::{Dice, Random}, resources::Folders, systems::remove_resource};
#[cfg(feature = "icon")]
use crate::{resources::WindowIcon, systems::set_window_icon};

pub struct BrtPlugin {
    pub folders: Folders,

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
        let folders = Folders::new(base, qualifier, orginization, application);
        Self {
            folders,
            
            #[cfg(feature = "icon")]
            icon: None,
        }
    }

    #[cfg(feature = "icon")]
    pub fn with_icon(mut self, icon: &'static [u8]) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn folders(&self) -> &Folders {
        &self.folders
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
        app.insert_resource(self.folders.clone());
        
        app.register_type::<Dice>();
        app.register_type::<Random>();

        #[cfg(feature = "icon")]
        if let Some(icon) = self.icon {
            app.insert_resource(WindowIcon(icon));
            app.add_systems(Startup, (set_window_icon, remove_resource::<WindowIcon>).chain());
        }
    }
}
