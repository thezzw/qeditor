//! Coordinate resources
//!
//! This module defines the resources used for the coordinate system.

use bevy::prelude::*;

/// Resource containing coordinate system settings
#[derive(Resource, Debug, Clone)]
pub struct CoordinateSettings {
    /// Color of the X axis
    pub x_axis_color: Color,
    /// Color of the Y axis
    pub y_axis_color: Color,
    /// Spacing between grid lines
    pub grid_spacing: f32,
    /// Color of the grid lines
    pub grid_color: Color,
    /// Spacing between chunks
    pub chunk_spacing: f32,
    /// Color of the chunks
    pub chunk_color: Color,
}

impl Default for CoordinateSettings {
    fn default() -> Self {
        Self {
            x_axis_color: Color::srgba(1.0, 0.0, 0.0, 0.5), // Red for X axis
            y_axis_color: Color::srgba(0.0, 0.0, 1.0, 0.5), // Blue for Y axis
            grid_spacing: 1.0,
            grid_color: Color::srgba(0.5, 0.5, 0.5, 0.3),
            chunk_spacing: 100.0,
            chunk_color: Color::srgba(0.5, 0.5, 0.5, 0.5),
        }
    }
}
