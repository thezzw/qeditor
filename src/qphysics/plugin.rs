use crate::qphysics::messages::*;
use crate::qphysics::resources::*;
use crate::qphysics::systems::*;
use bevy::prelude::*;

pub struct QPhysicsPlugin;

impl Plugin for QPhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Initialize resources
        app.init_resource::<QPhysicsConfig>()
            .init_resource::<QCollisionMatrix>()
            .init_resource::<QPhysicsDebugConfig>()
            // Add messages
            .add_message::<QCollisionEvent>()
            .add_message::<QTriggerEvent>()
            // Configure system sets
            .configure_sets(
                Update,
                (
                    PhysicsSystemSet::PreUpdate,
                    PhysicsSystemSet::VelocityIntegration,
                    PhysicsSystemSet::BroadPhase,
                    PhysicsSystemSet::NarrowPhase,
                    PhysicsSystemSet::CollisionResolution,
                    PhysicsSystemSet::PositionIntegration,
                    PhysicsSystemSet::PostUpdate,
                )
                    .chain(),
            )
            // Add systems
            .add_systems(
                Update,
                (
                    apply_forces_qsystem.in_set(PhysicsSystemSet::PreUpdate),
                    integrate_velocities_qsystem.in_set(PhysicsSystemSet::VelocityIntegration),
                    broad_phase_qsystem.in_set(PhysicsSystemSet::BroadPhase),
                    narrow_phase_qsystem.in_set(PhysicsSystemSet::NarrowPhase),
                    collision_resolution_qsystem.in_set(PhysicsSystemSet::CollisionResolution),
                    integrate_positions_qsystem.in_set(PhysicsSystemSet::PositionIntegration),
                    debug_render_qsystem.in_set(PhysicsSystemSet::PostUpdate),
                ),
            );
    }
}
