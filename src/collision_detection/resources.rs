use bevy::prelude::*;

/// Resource containing coordinate system settings
#[derive(Resource, Debug, Clone)]
pub struct CollisionDetectionSettings {
    pub shape_color_bbox: Color,
    pub shape_color_seperation_vector: Color,
    pub shape_color_minkowski_difference: Color,
}

impl Default for CollisionDetectionSettings {
    fn default() -> Self {
        Self {
            shape_color_bbox: Color::srgba(1.0, 0.0, 0.0, 0.7),
            shape_color_seperation_vector: Color::srgba(1.0, 0.0, 0.0, 0.7),
            shape_color_minkowski_difference: Color::srgba(1.0, 0.0, 0.0, 0.7),
        }
    }
}
