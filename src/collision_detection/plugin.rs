//! Collision detection plugin implementation
//!
//! Registers systems for collision detection and visualization.

use bevy::prelude::*;

use crate::collision_detection::systems::*;

/// `CollisionDetectionPlugin` registers systems for collision detection and visualization.
pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        // Register collision detection and visualization systems
        app.add_systems(PostUpdate, (detect_collisions, visualize_collision_bboxes, visualize_separation_vectors, compute_minkowski_difference, visualize_minkowski_difference));
    }
}