//! Resources for the shapes functionality
//!
//! This module defines the resources used for managing shapes and their interactions.

use bevy::prelude::*;
use qgeometry::shape::QShapeType;
use qmath::vec2::QVec2;

/// Resource to track the currently selected shape type for drawing
#[derive(Resource, Debug, Clone, Default)]
pub struct SelectedShapeType {
    /// The currently selected shape type
    pub shape_type: Option<QShapeType>,
}

/// Resource to track the state of shape drawing
#[derive(Resource, Debug, Default)]
pub struct ShapeDrawingState {
    /// The starting position of the shape being drawn
    pub start_position: Option<QVec2>,
    /// The entity of the current shape being drawn
    pub current_shape: Option<Entity>,
}
