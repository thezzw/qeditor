use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsSystemSet {
    PreUpdate,
    VelocityIntegration,
    BroadPhase,
    NarrowPhase,
    CollisionResolution,
    PositionIntegration,
    PostUpdate,
}

pub fn apply_forces_qsystem() {}

pub fn integrate_velocities_qsystem() {}

pub fn broad_phase_qsystem() {}

pub fn narrow_phase_qsystem() {}

pub fn collision_resolution_qsystem() {}

pub fn integrate_positions_qsystem() {}

pub fn debug_render_qsystem() {}
