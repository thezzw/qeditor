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
            .init_resource::<QCollisionPairs>()
            .init_resource::<QCollisionPairsSetLastFrame>()
            // Add messages
            .add_message::<QCollisionEvent>()
            .add_message::<QTriggerEvent>()
            // Configure system sets
            .configure_sets(
                FixedUpdate,
                (
                    QPhysicsUpdateSet::PreUpdate,
                    QPhysicsUpdateSet::VelocityIntegration,
                    QPhysicsUpdateSet::BroadPhase,
                    QPhysicsUpdateSet::NarrowPhase,
                    QPhysicsUpdateSet::CollisionResolution,
                    QPhysicsUpdateSet::PositionIntegration,
                    QPhysicsUpdateSet::PostUpdate,
                )
                    .chain(),
            )
            // Add systems
            .add_systems(
                FixedUpdate,
                (
                    (update_qobject_qsysytem, apply_forces_qsystem).in_set(QPhysicsUpdateSet::PreUpdate),
                    integrate_velocities_qsystem.in_set(QPhysicsUpdateSet::VelocityIntegration),
                    broad_phase_qsystem.in_set(QPhysicsUpdateSet::BroadPhase),
                    narrow_phase_qsystem.in_set(QPhysicsUpdateSet::NarrowPhase),
                    collision_resolution_qsystem.in_set(QPhysicsUpdateSet::CollisionResolution),
                    integrate_positions_qsystem.in_set(QPhysicsUpdateSet::PositionIntegration),
                    debug_render_qsystem.in_set(QPhysicsUpdateSet::PostUpdate),
                ),
            );
    }
}
