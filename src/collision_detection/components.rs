use bevy::prelude::*;

/// Component to mark entities that represent collision visualization
#[derive(Component)]
pub struct CollisionVisualization;

/// Component to mark entities that represent separation vector visualization
#[derive(Component)]
pub struct SeparationVectorVisualization;

/// Component to mark entities that represent Minkowski difference visualization
#[derive(Component)]
pub struct MinkowskiDifferenceVisualization;
