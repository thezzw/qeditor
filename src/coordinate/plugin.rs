//! Coordinate plugin implementation
//!
//! This module defines the Bevy plugin for the coordinate system, including initialization
//! of components, resources, and systems needed for coordinate system functionality.

use bevy::prelude::*;

use crate::coordinate::{resources::CoordinateSettings, systems::draw_coordinate_system};

/// Plugin for handling coordinate system functionality
pub struct CoordinatePlugin;

impl Plugin for CoordinatePlugin {
    fn build(&self, app: &mut App) {
        // Initialize coordinate settings as a resource
        app.insert_resource(CoordinateSettings::default());

        // Add systems for coordinate system rendering
        app.add_systems(Update, draw_coordinate_system);
    }
}
