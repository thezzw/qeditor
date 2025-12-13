//! Components for the shapes functionality
//!
//! This module defines the components used for storing geometric shapes
//! using the qgeometry library data structures.

use bevy::prelude::*;
use qgeometry::shape::{QPoint, QLine, QBbox, QCircle, QPolygon, QShapeType};

/// Component representing a shape entity
#[derive(Component, Debug, Clone)]
pub struct Shape {
    /// The type of the shape
    pub shape_type: QShapeType,
    /// Whether the shape is selected
    pub selected: bool,
}

/// Component for storing a point shape
#[derive(Component, Debug, Clone)]
pub struct PointShape {
    /// The point data
    pub point: QPoint,
}

/// Component for storing a line shape
#[derive(Component, Debug, Clone)]
pub struct LineShape {
    /// The line data
    pub line: QLine,
}

/// Component for storing a bounding box shape
#[derive(Component, Debug, Clone)]
pub struct BboxShape {
    /// The bounding box data
    pub bbox: QBbox,
}

/// Component for storing a circle shape
#[derive(Component, Debug, Clone)]
pub struct CircleShape {
    /// The circle data
    pub circle: QCircle,
}

/// Component for storing a polygon shape
#[derive(Component, Debug, Clone)]
pub struct PolygonShape {
    /// The polygon data
    pub polygon: QPolygon,
}
