use super::components::{QCollisionFlag, QCollisionShape, QMotion, QObject, QPhysicsBody, QTransform};
use super::messages::QCollisionEvent;
use super::resources::{QCollisionPairs, QCollisionPairsSetLastFrame, QPhysicsConfig, QPhysicsDebugConfig};
use crate::qphysics::messages::QTriggerEvent;
use crate::util;
use bevy::prelude::*;
use qgeometry::prelude::*;
use qmath::dir::QDir;
use qmath::prelude::*;
use std::collections::HashSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum QPhysicsUpdateSet {
    PreUpdate,
    VelocityIntegration,
    BroadPhase,
    NarrowPhase,
    CollisionResolution,
    PositionIntegration,
    PostUpdate,
}

pub fn update_qobject_qsysytem(mut query: Query<(Entity, &mut QObject)>) {
    for (entity, mut qobject) in query.iter_mut() {
        qobject.entity = Some(entity);
    }
}

pub fn apply_forces_qsystem(
    mut motion_query: Query<(&QPhysicsBody, &mut QMotion)>, physics_config: Res<QPhysicsConfig>,
) {
    for (body, mut motion) in motion_query.iter_mut() {
        if !body.is_static() {
            // F = ma, a = F/m = g
            motion.acceleration = physics_config.gravity;
        }
    }
}

pub fn integrate_velocities_qsystem(mut motion_query: Query<&mut QMotion>, physics_config: Res<QPhysicsConfig>) {
    let delta_time = physics_config.time_step;

    for mut motion in motion_query.iter_mut() {
        // v = v0 + a * dt
        let delta_v = motion.acceleration.saturating_mul_num(delta_time);
        motion.velocity = motion.velocity.saturating_add(delta_v);
    }
}

pub fn broad_phase_qsystem(
    mut collision_pairs: ResMut<QCollisionPairs>,
    mut collision_pairs_set_last_frame: ResMut<QCollisionPairsSetLastFrame>,
    query: Query<(&QObject, &QCollisionShape, &QCollisionFlag, &QTransform)>,
) {
    // Reset collision pairs.
    let collision_pairs = &mut collision_pairs.0;
    collision_pairs.to_owned().into_iter().for_each(|pair| {
        collision_pairs_set_last_frame.0.insert(pair);
    });
    collision_pairs.clear();

    let shapes: Vec<_> = query.iter().collect();

    for i in 0..shapes.len() {
        for j in (i + 1)..shapes.len() {
            let (qobject_a, shape_a, flag_a, transform_a) = shapes[i];
            let (qobject_b, shape_b, flag_b, transform_b) = shapes[j];

            if !flag_a.can_collide_with(flag_b) {
                continue;
            }

            let bbox_a = transform_a.apply_to(shape_a).get_bbox();
            let bbox_b = transform_b.apply_to(shape_b).get_bbox();

            if bbox_a.is_collide(&bbox_b) {
                collision_pairs.push((*qobject_a, *qobject_b));
            }
        }
    }
}

pub fn narrow_phase_qsystem(
    mut collision_pairs: ResMut<QCollisionPairs>, collision_pairs_set_last_frame: ResMut<QCollisionPairsSetLastFrame>,
    shapes: Query<(&QCollisionShape, &QCollisionFlag, &QTransform)>,
    mut collision_events: MessageWriter<QCollisionEvent>, mut trigger_events: MessageWriter<QTriggerEvent>,
) {
    let collision_pairs = &mut collision_pairs.0;
    collision_pairs.retain(|(qobject_a, qobject_b)| {
        if let (Ok((shape_a, _, transform_a)), Ok((shape_b, _, transform_b))) =
            (shapes.get(qobject_a.entity.unwrap()), shapes.get(qobject_b.entity.unwrap()))
        {
            return transform_a.apply_to(shape_a).is_collide(&transform_b.apply_to(shape_b));
        }
        return false;
    });

    // Fire colliding messages.
    for collision_pair in collision_pairs.iter() {
        if let (Ok((_, flag_a, _)), Ok((_, flag_b, _))) =
            (shapes.get(collision_pair.0.entity.unwrap()), shapes.get(collision_pair.1.entity.unwrap()))
        {
            if collision_pairs_set_last_frame.0.contains(collision_pair) {
                if flag_a.is_trigger || flag_b.is_trigger {
                    trigger_events.write(QTriggerEvent::Enter(collision_pair.0, collision_pair.1));
                } else {
                    collision_events.write(QCollisionEvent::Started(collision_pair.0, collision_pair.1));
                }
            } else {
                if flag_a.is_trigger || flag_b.is_trigger {
                    trigger_events.write(QTriggerEvent::Stay(collision_pair.0, collision_pair.1));
                } else {
                    collision_events.write(QCollisionEvent::Ongoing(collision_pair.0, collision_pair.1));
                }
            }
        }
    }
    // Fire exiting messages.
    let mut collision_pairs_set_this_frame = HashSet::new();
    collision_pairs.to_owned().into_iter().for_each(|p| {
        collision_pairs_set_this_frame.insert(p);
    });
    collision_pairs_set_last_frame.0.iter().for_each(|p| {
        if !collision_pairs_set_this_frame.contains(p) {
            if let (Ok((_, flag_a, _)), Ok((_, flag_b, _))) = (shapes.get(p.0.entity.unwrap()), shapes.get(p.1.entity.unwrap())) {
                if flag_a.is_trigger || flag_b.is_trigger {
                    trigger_events.write(QTriggerEvent::Exit(p.0, p.1));
                } else {
                    collision_events.write(QCollisionEvent::Ended(p.0, p.1));
                }
            }
        }
    });
}

pub fn collision_resolution_qsystem(
    mut collision_pairs: ResMut<QCollisionPairs>, mut motions: Query<(&QPhysicsBody, &mut QMotion)>,
    mut shapes: Query<(&QCollisionShape, &mut QTransform)>,
) {
    let collision_pairs = &mut collision_pairs.0;
    for (qobject_a, qobject_b) in collision_pairs.iter() {
        if let Ok([(body_a, mut motion_a), (body_b, mut motion_b)]) =
            motions.get_many_mut([qobject_a.entity.unwrap(), qobject_b.entity.unwrap()])
        {
            if let Ok([(shape_a, mut transform_a), (shape_b, mut transform_b)]) = shapes.get_many_mut([qobject_a.entity.unwrap(), qobject_b.entity.unwrap()])
            {
                if let Some(separation_vector_b) = transform_a
                    .apply_to(shape_a)
                    .try_get_separation_vector(&transform_b.apply_to(shape_b))
                {
                    /*
                     * Apply separation vector.
                     */
                    let mass_sum = body_a.mass + body_b.mass;
                    if mass_sum != Q64::ZERO {
                        let separation_part_vector_a = -separation_vector_b.saturating_mul_num(body_a.mass.saturating_div(mass_sum));
                        let separation_part_vector_b = separation_vector_b.saturating_mul_num(body_b.mass.saturating_div(mass_sum));
                        transform_a.position = transform_a.position.saturating_add(separation_part_vector_a);
                        transform_b.position = transform_b.position.saturating_add(separation_part_vector_b);
                    }

                    /*
                     * Apply impluse.
                     */
                    let relative_velocity = motion_a.velocity.saturating_sub(motion_b.velocity);

                    let magnitude = separation_vector_b.length();
                    if magnitude == Q64::ZERO {
                        continue;
                    }

                    let separation_dir_b = QDir::new_from_vec(separation_vector_b);
                    let vel_along_normal = separation_dir_b.projection_of(relative_velocity);
                    if vel_along_normal < Q64::ZERO {
                        continue;
                    }

                    let restitution = (body_a.restitution.saturating_add(body_b.restitution)).half();
                    let inv_mass_a = body_a.inverse_mass();
                    let inv_mass_b = body_b.inverse_mass();
                    let separate_vel = -(restitution.saturating_add(Q64::ONE)).saturating_mul(vel_along_normal);
                    let inv_mass_sum = inv_mass_a + inv_mass_b;
                    if inv_mass_sum == Q64::ZERO {
                        continue;
                    }

                    let impulse_scalar = separate_vel.saturating_div(inv_mass_sum);
                    let impulse = separation_dir_b.to_vec().saturating_mul_num(impulse_scalar);
                    motion_a.velocity = motion_a.velocity.saturating_add(impulse.saturating_mul_num(inv_mass_a));
                    motion_b.velocity = motion_b.velocity.saturating_sub(impulse.saturating_mul_num(inv_mass_b));
                }
            }
        }
    }
}

pub fn integrate_positions_qsystem(mut transform_query: Query<(&mut QTransform, &QMotion)>, physics_config: Res<QPhysicsConfig>) {
    let delta_time = physics_config.time_step;

    for (mut transform, motion) in transform_query.iter_mut() {
        // x = x0 + v * dt
        let displacement = motion.velocity.saturating_mul_num(delta_time);
        transform.position = transform.position.saturating_add(displacement);

        // θ = θ0 + ω * dt
        let angle_displacement = motion.angular_velocity.saturating_mul(delta_time);
        transform.rotation.rotate(angle_displacement);
    }
}

pub fn debug_render_qsystem(
    query: Query<(&QTransform, &QMotion, &QCollisionShape)>, debug_config: Res<QPhysicsDebugConfig>, mut gizmos: Gizmos,
) {
    if !debug_config.show_colliders && !debug_config.show_velocity {
        return;
    }

    for (transform, motion, shape) in query.iter() {
        if debug_config.show_colliders {
            let polygon = transform.apply_to(shape).to_polygon();
            let points = polygon.points();
            if points.len() > 1 {
                for i in 0..points.len() {
                    let current = points[i].pos();
                    let next = points[(i + 1) % points.len()].pos();
                    gizmos.line_2d(util::qvec2vec(current), util::qvec2vec(next), Color::BLACK);
                }
            }
        }

        if debug_config.show_velocity {
            let polygon = transform.apply_to(shape).to_polygon();
            let start = util::qvec2vec(polygon.get_centroid().pos());
            let end = start + util::qvec2vec(motion.velocity);
            gizmos.arrow_2d(start, end, Color::srgb(0.0, 0.0, 1.0)); // BLUE
        }
    }
}
