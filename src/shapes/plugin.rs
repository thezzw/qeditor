//! Shapes plugin implementation
//!
//! This module defines the Bevy plugin for managing geometric shapes
//! using the qgeometry library data structures.

use bevy::prelude::*;

use crate::shapes::{
    components::*,
    resources::*,
    systems::*,
};

/// Plugin for handling shapes functionality
pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut App) {
        // Initialize resources
        app.init_resource::<SelectedShapeType>()
            .init_resource::<ShapeDrawingState>()
            // Add systems for shape management
            .add_systems(Update, (
                handle_shape_interaction,
                draw_shapes,
            ));
    }
}
