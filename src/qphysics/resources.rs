//! Physics resources for 2D physics simulation

use bevy::prelude::*;
use qmath::{prelude::*, vec2::QVec2};
use std::collections::HashMap;

/// Physics world configuration
#[derive(Resource, Debug, Clone)]
pub struct QPhysicsConfig {
    /// Gravity vector in units per second squared
    pub gravity: QVec2,
    /// Fixed time step for physics simulation
    pub time_step: Q64,
    /// Number of velocity iterations for constraint solving
    pub velocity_iterations: i32,
    /// Number of position iterations for constraint solving
    pub position_iterations: i32,
}

impl Default for QPhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: QVec2::new(Q64::ZERO, q64!(-10)), // Standard Earth gravity
            time_step: q64!(1 / 10),                   // 60 FPS
            velocity_iterations: 8,
            position_iterations: 3,
        }
    }
}

/// Collision matrix for defining which layers can collide with each other
#[derive(Resource, Debug, Clone)]
pub struct QCollisionMatrix {
    /// Map of layer masks defining collision relationships
    pub layer_masks: HashMap<u32, u32>,
}

impl Default for QCollisionMatrix {
    fn default() -> Self {
        let mut layer_masks = HashMap::new();
        // By default, layer 1 collides with itself
        layer_masks.insert(1, 1);
        Self { layer_masks }
    }
}

/// Debug configuration for physics visualization
#[derive(Resource, Debug, Clone)]
pub struct QPhysicsDebugConfig {
    /// Whether to show collider shapes
    pub show_colliders: bool,
    /// Whether to show velocity vectors
    pub show_velocity: bool,
    /// Whether to show contact points
    pub show_contacts: bool,
}

impl Default for QPhysicsDebugConfig {
    fn default() -> Self {
        Self {
            show_colliders: false,
            show_velocity: false,
            show_contacts: false,
        }
    }
}
