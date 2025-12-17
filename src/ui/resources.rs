use crate::shapes::components::ShapeLayer;
use bevy::prelude::*;
use qgeometry::shape::QShapeType;

/// Resource to track UI visibility state
#[derive(Resource)]
pub struct UiState {
    /// Whether the graphics editor panel is visible
    pub panel_visible: bool,
    /// Currently selected shape type for drawing
    pub selected_shape: Option<QShapeType>,
    /// Currently selected shape layer
    pub selected_layer: ShapeLayer,
    /// File path for saving/loading shapes
    pub file_path: String,
    /// Whether to enable snap to grid
    pub enable_snap: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            panel_visible: false,
            selected_shape: None,
            selected_layer: ShapeLayer::MainScene,
            file_path: "assets/save/default.json".to_string(),
            enable_snap: true,
        }
    }
}
