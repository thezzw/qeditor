//! Shapes plugin implementation
//!
//! Registers resources and systems for creating, editing, and rendering shapes.

use super::{resources::*, systems::*};
use bevy::prelude::*;

/// `ShapesPlugin` registers shape state resources and runtime systems.
pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the resources with Default implementations.
        app.init_resource::<ShapesSettings>()
            .init_resource::<ShapeDrawingState>()
            // Register interaction and rendering systems.
            .add_systems(Update, (handle_shape_interaction, draw_shapes));
    }
}
