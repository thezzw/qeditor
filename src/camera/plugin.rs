//! Camera control plugin
//!
//! This module implements camera controls for panning and zooming with mouse input.

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiStartupSet;

/// Plugin for handling camera controls
pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup.before(EguiStartupSet::InitContexts))
            .add_systems(Update, (camera_pan, camera_zoom));
    }
}

/// Component to track camera movement state
#[derive(Component, Default)]
pub struct CameraMovement {
    /// Whether the camera is currently being dragged
    dragging: bool,
    /// The previous mouse position when dragging started
    last_mouse_position: Vec2,
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera with camera movement component for panning and zooming
    commands.spawn((Camera2d, CameraMovement::default()));
}

/// System to handle camera panning with middle mouse button
fn camera_pan(
    mut camera_query: Query<(&mut Transform, &mut CameraMovement), With<Camera2d>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    if window.is_err() {
        return;
    }
    let window = window.unwrap();

    // Get mutable access to the camera
    let camera_result = camera_query.single_mut();
    if camera_result.is_err() {
        return;
    }

    let (mut camera_transform, mut camera_movement) = camera_result.unwrap();

    // Check if middle mouse button is pressed
    if mouse_button_input.just_pressed(MouseButton::Middle) {
        // Start dragging
        camera_movement.dragging = true;

        // Get initial mouse position
        if let Some(mouse_position) = window.cursor_position() {
            camera_movement.last_mouse_position = mouse_position;
        }
    } else if mouse_button_input.just_released(MouseButton::Middle) {
        // Stop dragging
        camera_movement.dragging = false;
    }

    // Handle dragging
    if camera_movement.dragging {
        if let Some(current_mouse_position) = window.cursor_position() {
            // Calculate how much the mouse moved
            let delta = current_mouse_position - camera_movement.last_mouse_position;

            // Move the camera in the opposite direction of the mouse movement
            camera_transform.translation.x -= delta.x * camera_transform.scale.x;
            camera_transform.translation.y += delta.y * camera_transform.scale.y;

            // Update last mouse position
            camera_movement.last_mouse_position = current_mouse_position;
        }
    }
}

/// System to handle camera zooming with mouse wheel
fn camera_zoom(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Only process zoom if there is a window
    let window = windows.single();
    if window.is_err() {
        return;
    }

    // Get mutable access to the camera transform
    let camera_result = camera_query.single_mut();
    if camera_result.is_err() {
        return;
    }

    let mut camera_transform = camera_result.unwrap();

    // Process mouse wheel events for zooming
    for event in mouse_wheel_events.read() {
        // Determine zoom factor based on scroll direction
        let zoom_factor = if event.y > 0.0 {
            0.9 // Zoom in
        } else if event.y < 0.0 {
            1.1 // Zoom out
        } else {
            continue; // No vertical scroll
        };

        // Apply zoom factor to camera scale
        camera_transform.scale *= zoom_factor;

    }

    // Clamp the scale to prevent zooming too far in or out
    camera_transform.scale = camera_transform
        .scale
        .clamp(Vec3::splat(0.01), Vec3::splat(0.1));
}
