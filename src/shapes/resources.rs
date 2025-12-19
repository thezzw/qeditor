//! Resources for the shapes functionality
//!
//! This module defines the resources used for managing shapes and their interactions.

use bevy::prelude::*;
use qgeometry::shape::QShapeType;
use qmath::vec2::QVec2;

/// Resource to track the state of shape drawing
#[derive(Resource, Debug, Default)]
pub struct ShapeDrawingState {
    /// The starting position of the shape being drawn
    pub start_position: Option<QVec2>,
    /// The entity of the current shape being drawn
    pub current_shape: Option<Entity>,
    /// The currently selected shape type
    pub selected_shape_type: Option<QShapeType>,
}

#[derive(Resource, Debug, Clone)]
pub struct ShapesSettings {
    pub shape_color_selected: Color,
}

impl Default for ShapesSettings {
    fn default() -> Self {
        Self {
            shape_color_selected: Color::srgba(0.0, 0.0, 1.0, 1.0),
        }
    }
}