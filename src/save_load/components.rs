//! Components for the save/load functionality
//!
//! This module defines the components used for the save/load functionality.

// Currently no specific components are needed for save/load functionality
// All functionality is handled through events and systems

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

/// Serializable representation of a point shape
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializablePoint {
    pub x: f64,
    pub y: f64,
}

/// Serializable representation of a shape
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializableShape {
    pub shape_type: String,
    pub selected: bool,
    pub point: Option<SerializablePoint>,
    pub line_start: Option<SerializablePoint>,
    pub line_end: Option<SerializablePoint>,
    pub bbox_min: Option<SerializablePoint>,
    pub bbox_max: Option<SerializablePoint>,
    pub circle_center: Option<SerializablePoint>,
    pub circle_radius: Option<f64>,
    pub polygon_points: Option<Vec<SerializablePoint>>,
}
