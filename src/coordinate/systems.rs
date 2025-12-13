//! Coordinate systems
//!
//! This module defines the systems used for the coordinate system functionality,
//! including rendering axes and grid lines.

use bevy::prelude::*;

use crate::coordinate::resources::CoordinateSettings;

/// System to draw the coordinate axes and grid using gizmos
pub fn draw_coordinate_system(
    coordinate_settings: Res<CoordinateSettings>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    // Get the camera transform to determine the visible area
    let Ok((_camera, camera_transform)) = camera_query.single() else {
        return;
    };

    // Get the camera viewport to determine the visible area
    let camera_position = camera_transform.translation();
    let camera_scale = camera_transform.compute_transform().scale;

    // Calculate the visible area based on camera position and scale
    // This creates an "infinite" feel by dynamically generating lines in the visible area
    let visible_width = 2000.0 * camera_scale.x;
    let visible_height = 2000.0 * camera_scale.y;

    let left = camera_position.x - visible_width / 2.0;
    let right = camera_position.x + visible_width / 2.0;
    let bottom = camera_position.y - visible_height / 2.0;
    let top = camera_position.y + visible_height / 2.0;

    // Draw X axis (red)
    gizmos.line_2d(
        Vec2::new(left, 0.0),
        Vec2::new(right, 0.0),
        coordinate_settings.x_axis_color,
    );

    // Draw Y axis (green)
    gizmos.line_2d(
        Vec2::new(0.0, bottom),
        Vec2::new(0.0, top),
        coordinate_settings.y_axis_color,
    );

    // Draw grid lines
    let grid_spacing = coordinate_settings.grid_spacing;

    // Calculate grid lines within visible area
    let start_x = (left / grid_spacing).floor() as i32;
    let end_x = (right / grid_spacing).ceil() as i32;
    let start_y = (bottom / grid_spacing).floor() as i32;
    let end_y = (top / grid_spacing).ceil() as i32;

    // Draw vertical grid lines
    for x in start_x..=end_x {
        let x_pos = x as f32 * grid_spacing;
        if x_pos != 0.0 {
            // Skip the axis line
            gizmos.line_2d(
                Vec2::new(x_pos, bottom),
                Vec2::new(x_pos, top),
                coordinate_settings.grid_color,
            );
        }
    }

    // Draw horizontal grid lines
    for y in start_y..=end_y {
        let y_pos = y as f32 * grid_spacing;
        if y_pos != 0.0 {
            // Skip the axis line
            gizmos.line_2d(
                Vec2::new(left, y_pos),
                Vec2::new(right, y_pos),
                coordinate_settings.grid_color,
            );
        }
    }
}
