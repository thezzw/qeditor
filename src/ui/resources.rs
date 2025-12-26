use crate::shapes::components::ShapeLayer;
use bevy::prelude::*;
use qgeometry::shape::QShapeType;

#[derive(Debug, PartialEq)]
pub enum EditorMode {
    Shape,
    Physics,
}

/// Resource to track UI visibility state
#[derive(Resource)]
pub struct UiState {
    pub editor_mode: EditorMode,
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
    /// Whether to only show shapes in the selected layer
    pub only_show_select_layer: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            editor_mode: EditorMode::Shape,
            panel_visible: false,
            selected_shape: None,
            selected_layer: ShapeLayer::MainScene,
            file_path: "assets/saves/default.json".to_string(),
            enable_snap: true,
            only_show_select_layer: false,
        }
    }
}
