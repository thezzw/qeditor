//! Shapes plugin implementation
//!
//! Registers resources and systems for creating, editing, and rendering shapes.

use bevy::prelude::*;

use crate::shapes::{resources::*, systems::*};

/// `ShapesPlugin` registers shape state resources and runtime systems.
pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the resources with Default implementations.
        app.init_resource::<SelectedShapeType>()
            .init_resource::<ShapeDrawingState>()
            // Register interaction and rendering systems.
            .add_systems(Update, (handle_shape_interaction, draw_shapes));
    }
}
