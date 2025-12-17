use bevy::prelude::*;

/// Component to track camera movement state
#[derive(Component, Default)]
pub struct CameraMovement {
    /// Whether the camera is currently being dragged
    pub dragging: bool,
    /// The previous mouse position when dragging started
    pub last_mouse_position: Vec2,
}
