use bevy::prelude::*;
use qmath::{prelude::*, vec2::QVec2};  // 正确导入 qmath 类型
use crate::qphysics::components::{QCollisionFlag, QCollisionShape, QMotion, QPhysicsBody, QTransform};
use crate::qphysics::resources::{QCollisionPairs, QPhysicsConfig, QPhysicsDebugConfig};
use crate::qphysics::messages::QCollisionEvent;
use crate::util;
use qgeometry::prelude::*;

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

pub fn apply_forces_qsystem(
    mut motion_query: Query<(&QPhysicsBody, &mut QMotion)>,
    physics_config: Res<QPhysicsConfig>,
) {
    for (body, mut motion) in motion_query.iter_mut() {
        if !body.is_static() {
            // F = ma, a = F/m = g
            motion.acceleration += physics_config.gravity;
        }
    }
}

pub fn integrate_velocities_qsystem(
    mut motion_query: Query<&mut QMotion>,
    _physics_config: Res<QPhysicsConfig>,
    time: Res<Time>,
) {
    let delta_time = time.delta().as_secs_f64();
    
    for mut motion in motion_query.iter_mut() {
        // v = v0 + a * dt
        motion.velocity = motion.velocity + motion.acceleration * Q64::from_num(delta_time);
    }
}

pub fn integrate_positions_qsystem(
    mut transform_query: Query<(&mut QTransform, &QMotion)>,
    time: Res<Time>,
) {
    let delta_time = q64!(time.delta().as_secs_f64());  // 使用正确的函数
    
    for (mut transform, motion) in transform_query.iter_mut() {
        // x = x0 + v * dt
        let displacement = motion.velocity * delta_time;
        transform.position = transform.position.saturating_add(displacement);
        
        // θ = θ0 + ω * dt
        let angle_displacement = motion.angular_velocity * delta_time;
        transform.rotation.rotate(angle_displacement);
    }
}

pub fn broad_phase_qsystem(
    mut collision_pairs: ResMut<QCollisionPairs>,
    query: Query<(Entity, &QCollisionShape, &QCollisionFlag)>,
) {
    let collision_pairs = &mut collision_pairs.0;
    collision_pairs.clear();
    
    let shapes: Vec<_> = query.iter().collect();

    for i in 0..shapes.len() {
        for j in (i + 1)..shapes.len() {
            let (entity_a, shape_a, flag_a) = shapes[i];
            let (entity_b, shape_b, flag_b) = shapes[j];

            if !flag_a.can_collide_with(flag_b) {
                continue;
            }

            let bbox_a = shape_a.get_bbox();
            let bbox_b = shape_b.get_bbox();

            if bbox_a.is_collide(&bbox_b) {
                collision_pairs.push((entity_a, entity_b));
            }
        }
    }
}

pub fn narrow_phase_qsystem(
    mut collision_pairs: ResMut<QCollisionPairs>,
    query: Query<&QCollisionShape>,
    mut collision_events: MessageWriter<QCollisionEvent>,
) {
    let collision_pairs = &mut collision_pairs.0;
    collision_pairs.retain(|(ea, eb)| {
        if let (Ok(shape_a), Ok(shape_b)) = (query.get(*ea), query.get(*eb)) {
            return shape_a.is_collide(shape_b);
        }
        return false;
    });
    for (entity_a, entity_b) in collision_pairs.iter() {
        collision_events.write(QCollisionEvent::Started(*entity_a, *entity_b));
    }
}

pub fn collision_resolution_qsystem(
    mut collision_pairs: ResMut<QCollisionPairs>,
    mut motions: Query<(&QPhysicsBody, &mut QMotion)>,
    shapes: Query<&QCollisionShape>,
    _physics_config: Res<QPhysicsConfig>,
) {
    let collision_pairs = &mut collision_pairs.0;
    for (entity_a, entity_b) in collision_pairs.iter() {
        if let Ok([(body_a, mut motion_a), (body_b, mut motion_b)]) = motions.get_many_mut([*entity_a, *entity_b]) {
            if let (Ok(shape_a), Ok(shape_b)) = (shapes.get(*entity_a), shapes.get(*entity_b)) {
                if let Some(separation_vector) = shape_a.try_get_separation_vector(shape_b) {
                    let relative_velocity = QVec2::new(
                        motion_a.velocity.x - motion_b.velocity.x,
                        motion_a.velocity.y - motion_b.velocity.y
                    );
                    
                    let magnitude_sq = separation_vector.x * separation_vector.x + separation_vector.y * separation_vector.y;
                    let magnitude = magnitude_sq.sqrt();
                    if magnitude > Q64::from_num(1e-9) {  // 使用小数值代替 EPSILON
                        let normal = QVec2::new(
                            separation_vector.x / magnitude,
                            separation_vector.y / magnitude
                        );
                        let vel_along_normal = relative_velocity.x * normal.x + relative_velocity.y * normal.y;
                        
                        if vel_along_normal > Q64::ZERO {
                            continue;
                        }

                        let restitution = (body_a.restitution + body_b.restitution) / Q64::from_num(2.0);

                        let inv_mass_a = body_a.inverse_mass();
                        let inv_mass_b = body_b.inverse_mass();
                        let impulse_scalar = -(Q64::ONE + restitution) * vel_along_normal / (inv_mass_a + inv_mass_b);

                        let impulse = QVec2::new(
                            normal.x * impulse_scalar,
                            normal.y * impulse_scalar
                        );
                        motion_a.velocity = QVec2::new(
                            motion_a.velocity.x + impulse.x * inv_mass_a,
                            motion_a.velocity.y + impulse.y * inv_mass_a
                        );
                        motion_b.velocity = QVec2::new(
                            motion_b.velocity.x - impulse.x * inv_mass_b,
                            motion_b.velocity.y - impulse.y * inv_mass_b
                        );
                    }
                }
            }
        }
    }
}

pub fn debug_render_qsystem(
    query: Query<(&QTransform, &QMotion, &QCollisionShape)>,
    debug_config: Res<QPhysicsDebugConfig>,
    mut gizmos: Gizmos,
) {
    if !debug_config.show_colliders && !debug_config.show_velocity {
        return;
    }
    
    for (transform, motion, shape) in query.iter() {
        if debug_config.show_colliders {
            let polygon = shape.to_polygon();
            let points: Vec<Vec2> = polygon.points()
                .iter()
                .map(|point| {
                    let pos = point.pos();
                    Vec2::new(pos.x.to_num::<f32>(), pos.y.to_num::<f32>())
                })
                .collect();
            
            if points.len() > 1 {
                for i in 0..points.len() {
                    let current = points[i];
                    let next = points[(i + 1) % points.len()];
                    gizmos.line_2d(current, next, Color::srgb(1.0, 0.0, 0.0));  // RED
                }
            }
        }

        if debug_config.show_velocity {
            let start = util::qvec2vec(transform.position);
            let end = start + util::qvec2vec(motion.velocity) * 0.1;
            gizmos.arrow_2d(start, end, Color::srgb(0.0, 0.0, 1.0));  // BLUE
        }
    }
}