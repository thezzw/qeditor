//! Collision detection plugin implementation
//!
//! Registers systems for collision detection and visualization.

use super::resources::CollisionDetectionSettings;
use super::systems::*;
use bevy::prelude::*;

/// `CollisionDetectionPlugin` registers systems for collision detection and visualization.
pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        // Register collision detection and visualization systems
        app.init_resource::<CollisionDetectionSettings>()
            .add_systems(
            PostUpdate,
            (
                detect_collisions,
                compute_minkowski_difference,
                visualize_minkowski_difference,
            ),
        );
    }
}
