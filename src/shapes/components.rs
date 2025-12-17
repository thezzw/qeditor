//! Components for the shapes functionality
//!
//! This module defines the components used for storing geometric shapes
//! using the qgeometry library data structures.

use bevy::prelude::*;
use qgeometry::shape::{QBbox, QCircle, QLine, QPoint, QPolygon, QShapeType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize, Serialize)]
pub enum ShapeLayer {
    #[default]
    MainScene,
    AuxiliaryLine,
    Generated
}

#[derive(Component, Debug, Clone, Deserialize, Serialize)]
pub struct EditorShape {
    /// The layer of the shape
    pub layer: ShapeLayer,
    /// The type of the shape
    pub shape_type: QShapeType,
    /// Whether the shape is selected
    pub selected: bool,
}

/// Component for storing a point shape
#[derive(Component, Debug, Clone, Deserialize, Serialize)]
pub struct QPointData {
    /// The point data
    pub data: QPoint,
}

/// Component for storing a line shape
#[derive(Component, Debug, Clone, Deserialize, Serialize)]
pub struct QLineData {
    /// The line data
    pub data: QLine,
}

/// Component for storing a bounding box shape
#[derive(Component, Debug, Clone, Deserialize, Serialize)]
pub struct QBboxData {
    /// The bounding box data
    pub data: QBbox,
}

/// Component for storing a circle shape
#[derive(Component, Debug, Clone, Deserialize, Serialize)]
pub struct QCircleData {
    /// The circle data
    pub data: QCircle,
}

/// Component for storing a polygon shape
#[derive(Component, Debug, Clone, Deserialize, Serialize)]
pub struct QPolygonData {
    /// The polygon data
    pub data: QPolygon,
}
