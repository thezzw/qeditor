//! Components for the save/load functionality
//!
//! This module defines the components used for the save/load functionality.

// Currently no specific components are needed for save/load functionality
// All functionality is handled through events and systems

use crate::shapes::components::{QBboxData, QCircleData, QLineData, QPointData, QPolygonData};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Events to trigger save operations
#[derive(Message, Clone)]
pub struct SaveSelectedShapesEvent {
    pub file_path: String,
}

/// Events to trigger load operations
#[derive(Message, Clone)]
pub struct LoadShapesFromFileEvent {
    pub file_path: String,
}

/// Serializable representation of a shape
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SerializableQShapeData {
    Point(QPointData),
    Line(QLineData),
    Bbox(QBboxData),
    Circle(QCircleData),
    Polygon(QPolygonData),
}
