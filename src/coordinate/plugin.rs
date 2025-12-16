//! Coordinate plugin implementation
//!
//! This module defines the Bevy plugin for the coordinate system, including
//! initialization of resources and registration of systems for rendering the grid
//! and axes.

use bevy::prelude::*;

use crate::coordinate::{resources::CoordinateSettings, systems::draw_coordinate_system};

/// `CoordinatePlugin` registers the coordinate system resource and its rendering systems.
pub struct CoordinatePlugin;

impl Plugin for CoordinatePlugin {
    fn build(&self, app: &mut App) {
        // Initialize coordinate settings using `init_resource` for consistency.
        app.init_resource::<CoordinateSettings>()
            // Register the drawing system at the Update stage.
            .add_systems(PreUpdate, draw_coordinate_system);
    }
}
