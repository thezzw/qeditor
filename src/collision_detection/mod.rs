//! Collision detection module for the 2D geometry editor
//!
//! This module provides functionality for detecting collisions between shapes
//! and visualizing bounding boxes for colliding shapes.

pub mod components;
pub mod plugin;
pub mod resources;
pub mod systems;

pub use plugin::CollisionDetectionPlugin;
