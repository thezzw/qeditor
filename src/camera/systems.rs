use super::components::CameraMovement;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup(mut commands: Commands) {
    // Spawn a 2D camera with a component to track panning state.
    commands.spawn((Camera2d, CameraMovement::default()));
}

/// Check whether a primary window exists and return it, otherwise return early from caller.
// no helper needed — inline `windows.single()` is used in callers.

/// System to handle camera panning with the middle mouse button.
pub fn camera_pan(
    mut camera_query: Query<(&mut Transform, &mut CameraMovement), With<Camera2d>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>, windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = match windows.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let Ok((mut camera_transform, mut camera_movement)) = camera_query.single_mut() else {
        return;
    };

    if mouse_button_input.just_pressed(MouseButton::Middle) {
        camera_movement.dragging = true;
        if let Some(mouse_position) = window.cursor_position() {
            camera_movement.last_mouse_position = mouse_position;
        }
    } else if mouse_button_input.just_released(MouseButton::Middle) {
        camera_movement.dragging = false;
    }

    if camera_movement.dragging {
        if let Some(current_mouse_position) = window.cursor_position() {
            let delta = current_mouse_position - camera_movement.last_mouse_position;
            camera_transform.translation.x -= delta.x * camera_transform.scale.x;
            camera_transform.translation.y += delta.y * camera_transform.scale.y;
            camera_movement.last_mouse_position = current_mouse_position;
        }
    }
}

/// System to handle camera zooming with mouse wheel.
pub fn camera_zoom(
    mut camera_query: Query<&mut Transform, With<Camera2d>>, mut mouse_wheel_events: MessageReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let _window = match windows.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    for event in mouse_wheel_events.read() {
        let zoom_factor = if event.y > 0.0 {
            0.9
        } else if event.y < 0.0 {
            1.1
        } else {
            continue;
        };
        camera_transform.scale *= zoom_factor;
    }

    // Limit how far the user can zoom in or out.
    camera_transform.scale = camera_transform.scale.clamp(Vec3::splat(0.01), Vec3::splat(0.1));
}
