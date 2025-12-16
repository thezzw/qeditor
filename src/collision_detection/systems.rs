//! Collision detection systems
//!
//! This module defines the systems used for collision detection and visualization.

use bevy::prelude::*;
use qgeometry::shape::{QLine, QPoint, QShapeCommon};
use qgeometry::algorithm::get_minkowski_difference;
use qmath::vec2::QVec2;

use crate::shapes::{
    components::{Shape, PointShape, LineShape, BboxShape, CircleShape, PolygonShape, ShapeLayer},
};

/// Component to mark entities that represent collision visualization
#[derive(Component)]
pub struct CollisionVisualization;

/// Component to mark entities that represent separation vector visualization
#[derive(Component)]
pub struct SeparationVectorVisualization;

/// Component to mark entities that represent Minkowski difference visualization
#[derive(Component)]
pub struct MinkowskiDifferenceVisualization;

/// System to detect collisions between shapes
pub fn detect_collisions(
    // Query all shapes with their components
    shapes: Query<(
        Entity,
        &Shape,
        Option<&PointShape>,
        Option<&LineShape>,
        Option<&BboxShape>,
        Option<&CircleShape>,
        Option<&PolygonShape>,
    )>,
    // Query existing collision visualizations to clean them up
    mut visualization_query: Query<Entity, With<CollisionVisualization>>,
    // Query existing separation vector visualizations to clean them up
    mut separation_vector_query: Query<Entity, With<SeparationVectorVisualization>>,
    // Add commands to spawn/despawn entities for visualization
    mut commands: Commands,
) {
    // Clean up existing collision visualizations
    for entity in visualization_query.iter_mut() {
        commands.entity(entity).despawn();
    }
    
    // Clean up existing separation vector visualizations
    for entity in separation_vector_query.iter_mut() {
        commands.entity(entity).despawn();
    }

    // Get all shape entities
    let shape_entities: Vec<_> = shapes.iter().collect();
    
    // Check collisions between all pairs of shapes
    for i in 0..shape_entities.len() {
        for j in (i + 1)..shape_entities.len() {
            let (_, shape_a, point_a, line_a, bbox_a, circle_a, polygon_a) = shape_entities[i];
            let (_, shape_b, point_b, line_b, bbox_b, circle_b, polygon_b) = shape_entities[j];
            
            // Skip if either shape is on auxiliary layer (to avoid checking visualization shapes)
            if shape_a.layer == ShapeLayer::AuxiliaryLine || shape_b.layer == ShapeLayer::AuxiliaryLine {
                continue;
            }
            
            // Check if shapes collide
            let collision_detected = if let (Some(point), _) = (point_a, point_b) {
                if let Some(other_point) = point_b {
                    point.point.is_collide(&other_point.point)
                } else if let Some(other_line) = line_b {
                    point.point.is_collide(&other_line.line)
                } else if let Some(other_bbox) = bbox_b {
                    point.point.is_collide(&other_bbox.bbox)
                } else if let Some(other_circle) = circle_b {
                    point.point.is_collide(&other_circle.circle)
                } else if let Some(other_polygon) = polygon_b {
                    point.point.is_collide(&other_polygon.polygon)
                } else {
                    false
                }
            } else if let (Some(line), _) = (line_a, line_b) {
                if let Some(other_point) = point_b {
                    line.line.is_collide(&other_point.point)
                } else if let Some(other_line) = line_b {
                    line.line.is_collide(&other_line.line)
                } else if let Some(other_bbox) = bbox_b {
                    line.line.is_collide(&other_bbox.bbox)
                } else if let Some(other_circle) = circle_b {
                    line.line.is_collide(&other_circle.circle)
                } else if let Some(other_polygon) = polygon_b {
                    line.line.is_collide(&other_polygon.polygon)
                } else {
                    false
                }
            } else if let (Some(bbox), _) = (bbox_a, bbox_b) {
                if let Some(other_point) = point_b {
                    bbox.bbox.is_collide(&other_point.point)
                } else if let Some(other_line) = line_b {
                    bbox.bbox.is_collide(&other_line.line)
                } else if let Some(other_bbox) = bbox_b {
                    bbox.bbox.is_collide(&other_bbox.bbox)
                } else if let Some(other_circle) = circle_b {
                    bbox.bbox.is_collide(&other_circle.circle)
                } else if let Some(other_polygon) = polygon_b {
                    bbox.bbox.is_collide(&other_polygon.polygon)
                } else {
                    false
                }
            } else if let (Some(circle), _) = (circle_a, circle_b) {
                if let Some(other_point) = point_b {
                    circle.circle.is_collide(&other_point.point)
                } else if let Some(other_line) = line_b {
                    circle.circle.is_collide(&other_line.line)
                } else if let Some(other_bbox) = bbox_b {
                    circle.circle.is_collide(&other_bbox.bbox)
                } else if let Some(other_circle) = circle_b {
                    circle.circle.is_collide(&other_circle.circle)
                } else if let Some(other_polygon) = polygon_b {
                    circle.circle.is_collide(&other_polygon.polygon)
                } else {
                    false
                }
            } else if let (Some(polygon), _) = (polygon_a, polygon_b) {
                if let Some(other_point) = point_b {
                    polygon.polygon.is_collide(&other_point.point)
                } else if let Some(other_line) = line_b {
                    polygon.polygon.is_collide(&other_line.line)
                } else if let Some(other_bbox) = bbox_b {
                    polygon.polygon.is_collide(&other_bbox.bbox)
                } else if let Some(other_circle) = circle_b {
                    polygon.polygon.is_collide(&other_circle.circle)
                } else if let Some(other_polygon) = polygon_b {
                    polygon.polygon.is_collide(&other_polygon.polygon)
                } else {
                    false
                }
            } else {
                false
            };
            
            // If collision detected, create visualization for both shapes
            if collision_detected {
                // Calculate separation vector
                let separation_vector = if let (Some(point), _) = (point_a, point_b) {
                    if let Some(other_point) = point_b {
                        point.point.try_get_seperation_vector(&other_point.point)
                    } else if let Some(other_line) = line_b {
                        point.point.try_get_seperation_vector(&other_line.line)
                    } else if let Some(other_bbox) = bbox_b {
                        point.point.try_get_seperation_vector(&other_bbox.bbox)
                    } else if let Some(other_circle) = circle_b {
                        point.point.try_get_seperation_vector(&other_circle.circle)
                    } else if let Some(other_polygon) = polygon_b {
                        point.point.try_get_seperation_vector(&other_polygon.polygon)
                    } else {
                        None
                    }
                } else if let (Some(line), _) = (line_a, line_b) {
                    if let Some(other_point) = point_b {
                        line.line.try_get_seperation_vector(&other_point.point)
                    } else if let Some(other_line) = line_b {
                        line.line.try_get_seperation_vector(&other_line.line)
                    } else if let Some(other_bbox) = bbox_b {
                        line.line.try_get_seperation_vector(&other_bbox.bbox)
                    } else if let Some(other_circle) = circle_b {
                        line.line.try_get_seperation_vector(&other_circle.circle)
                    } else if let Some(other_polygon) = polygon_b {
                        line.line.try_get_seperation_vector(&other_polygon.polygon)
                    } else {
                        None
                    }
                } else if let (Some(bbox), _) = (bbox_a, bbox_b) {
                    if let Some(other_point) = point_b {
                        bbox.bbox.try_get_seperation_vector(&other_point.point)
                    } else if let Some(other_line) = line_b {
                        bbox.bbox.try_get_seperation_vector(&other_line.line)
                    } else if let Some(other_bbox) = bbox_b {
                        bbox.bbox.try_get_seperation_vector(&other_bbox.bbox)
                    } else if let Some(other_circle) = circle_b {
                        bbox.bbox.try_get_seperation_vector(&other_circle.circle)
                    } else if let Some(other_polygon) = polygon_b {
                        bbox.bbox.try_get_seperation_vector(&other_polygon.polygon)
                    } else {
                        None
                    }
                } else if let (Some(circle), _) = (circle_a, circle_b) {
                    if let Some(other_point) = point_b {
                        circle.circle.try_get_seperation_vector(&other_point.point)
                    } else if let Some(other_line) = line_b {
                        circle.circle.try_get_seperation_vector(&other_line.line)
                    } else if let Some(other_bbox) = bbox_b {
                        circle.circle.try_get_seperation_vector(&other_bbox.bbox)
                    } else if let Some(other_circle) = circle_b {
                        circle.circle.try_get_seperation_vector(&other_circle.circle)
                    } else if let Some(other_polygon) = polygon_b {
                        circle.circle.try_get_seperation_vector(&other_polygon.polygon)
                    } else {
                        None
                    }
                } else if let (Some(polygon), _) = (polygon_a, polygon_b) {
                    if let Some(other_point) = point_b {
                        polygon.polygon.try_get_seperation_vector(&other_point.point)
                    } else if let Some(other_line) = line_b {
                        polygon.polygon.try_get_seperation_vector(&other_line.line)
                    } else if let Some(other_bbox) = bbox_b {
                        polygon.polygon.try_get_seperation_vector(&other_bbox.bbox)
                    } else if let Some(other_circle) = circle_b {
                        polygon.polygon.try_get_seperation_vector(&other_circle.circle)
                    } else if let Some(other_polygon) = polygon_b {
                        polygon.polygon.try_get_seperation_vector(&other_polygon.polygon)
                    } else {
                        None
                    }
                } else {
                    None
                };
                
                // Visualize bbox for first shape
                if let (Some(point), _) = (point_a, point_b) {
                    let bbox = point.point.get_bbox();
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: bbox.get_shape_type(),
                            selected: false,
                        },
                        BboxShape { bbox },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (Some(line), _) = (line_a, line_b) {
                    let bbox = line.line.get_bbox();
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: bbox.get_shape_type(),
                            selected: false,
                        },
                        BboxShape { bbox },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (Some(bbox), _) = (bbox_a, bbox_b) {
                    let bbox_val = bbox.bbox.get_bbox(); // Already a bbox, but call get_bbox for consistency
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: bbox_val.get_shape_type(),
                            selected: false,
                        },
                        BboxShape { bbox: bbox_val },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (Some(circle), _) = (circle_a, circle_b) {
                    let bbox = circle.circle.get_bbox();
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: bbox.get_shape_type(),
                            selected: false,
                        },
                        BboxShape { bbox },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (Some(polygon), _) = (polygon_a, polygon_b) {
                    let bbox = polygon.polygon.get_bbox();
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: bbox.get_shape_type(),
                            selected: false,
                        },
                        BboxShape { bbox },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                }
                
                // Visualize bbox for second shape
                if let (_, Some(other_point)) = (point_a, point_b) {
                    let bbox = other_point.point.get_bbox();
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: bbox.get_shape_type(),
                            selected: false,
                        },
                        BboxShape { bbox },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (_, Some(other_line)) = (line_a, line_b) {
                    let bbox = other_line.line.get_bbox();
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: bbox.get_shape_type(),
                            selected: false,
                        },
                        BboxShape { bbox },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (_, Some(other_bbox)) = (bbox_a, bbox_b) {
                    let bbox_val = other_bbox.bbox.get_bbox(); // Already a bbox, but call get_bbox for consistency
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: bbox_val.get_shape_type(),
                            selected: false,
                        },
                        BboxShape { bbox: bbox_val },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (_, Some(other_circle)) = (circle_a, circle_b) {
                    let bbox = other_circle.circle.get_bbox();
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: bbox.get_shape_type(),
                            selected: false,
                        },
                        BboxShape { bbox },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (_, Some(other_polygon)) = (polygon_a, polygon_b) {
                    let bbox = other_polygon.polygon.get_bbox();
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: bbox.get_shape_type(),
                            selected: false,
                        },
                        BboxShape { bbox },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                }
                
                // Spawn separation vector visualization if available
                if let Some(vector) = separation_vector {
                    let start = get_shape_center(point_b, line_b, bbox_b, circle_b, polygon_b);
                    let line = QLine::new_from_parts(start.pos(), start.pos().saturating_add(vector));
                    commands.spawn((
                        Shape {
                            layer: ShapeLayer::AuxiliaryLine,
                            shape_type: line.get_shape_type(),
                            selected: false,
                        },
                        LineShape { line },
                        SeparationVectorVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                }
            }
        }
    }
}

// Helper function to get the center of a shape
fn get_shape_center(
    point: Option<&PointShape>,
    line: Option<&LineShape>,
    bbox: Option<&BboxShape>,
    circle: Option<&CircleShape>,
    polygon: Option<&PolygonShape>,
) -> QPoint {
    if let Some(point) = point {
        point.point.get_centroid()
    } else if let Some(line) = line {
        line.line.get_centroid()
    } else if let Some(bbox) = bbox {
        bbox.bbox.get_centroid()
    } else if let Some(circle) = circle {
        circle.circle.get_centroid()
    } else if let Some(polygon) = polygon {
        polygon.polygon.get_centroid()
    } else {
        QPoint::ZERO
    }
}

/// System to visualize bounding boxes of colliding shapes
pub fn visualize_collision_bboxes(
    mut gizmos: Gizmos,
    shapes: Query<(&Shape, &BboxShape)>,
) {
    // Draw all bbox shapes that are on the auxiliary layer
    for (shape, bbox) in shapes.iter() {
        if shape.layer == ShapeLayer::AuxiliaryLine {
            let min = bbox.bbox.left_bottom().pos();
            let max = bbox.bbox.right_top().pos();
            let center = Vec2::new(
                (min.x.to_num::<f32>() + max.x.to_num::<f32>()) / 2.0,
                (min.y.to_num::<f32>() + max.y.to_num::<f32>()) / 2.0,
            );
            let size = Vec2::new(
                (max.x.to_num::<f32>() - min.x.to_num::<f32>()).abs(),
                (max.y.to_num::<f32>() - min.y.to_num::<f32>()).abs(),
            );
            // Draw with red color to indicate collision
            gizmos.rect_2d(center, size, Color::srgba(1.0, 0.0, 0.0, 1.0));
        }
    }
}

/// System to visualize separation vectors as arrows
pub fn visualize_separation_vectors(
    mut gizmos: Gizmos,
    vectors: Query<(&Shape, &LineShape)>,
) {
    for (shape, qline) in vectors.iter() {
        if shape.layer == ShapeLayer::AuxiliaryLine {
            let qstart = qline.line.start();
            let qend = qline.line.end();
            let start = Vec2::new(qstart.x().to_num(), qstart.y().to_num());
            let end = Vec2::new(qend.x().to_num(), qend.y().to_num());
            
            // Draw arrow line
            gizmos.line_2d(start, end, Color::srgba(0.0, 1.0, 0.0, 1.0)); // Green color
            
            // Draw arrowhead
            draw_arrowhead(&mut gizmos, start, end, Color::srgba(0.0, 1.0, 0.0, 1.0));
        }
    }
}

/// Helper function to draw an arrowhead
fn draw_arrowhead(gizmos: &mut Gizmos, start: Vec2, end: Vec2, color: Color) {
    let arrow_length = end.distance(start);
    if arrow_length < 0.001 {
        return;
    }
    
    let direction = (end - start).normalize();
    let arrow_size = 0.5; // Size of the arrowhead
    
    // Calculate perpendicular vector for arrowhead
    let perp = Vec2::new(-direction.y, direction.x) * arrow_size * 0.5;
    
    // Arrowhead points
    let arrow_point1 = end - direction * arrow_size + perp;
    let arrow_point2 = end - direction * arrow_size - perp;
    
    // Draw arrowhead lines
    gizmos.line_2d(end, arrow_point1, color);
    gizmos.line_2d(end, arrow_point2, color);
}

/// System to compute and visualize Minkowski difference of two selected polygons
pub fn compute_minkowski_difference(
    // Query all shapes with their components
    shapes: Query<(
        Entity,
        &Shape,
        Option<&PointShape>,
        Option<&LineShape>,
        Option<&BboxShape>,
        Option<&CircleShape>,
        Option<&PolygonShape>,
    )>,
    // Query existing Minkowski difference visualizations to clean them up
    mut minkowski_query: Query<Entity, With<MinkowskiDifferenceVisualization>>,
    // Add commands to spawn/despawn entities for visualization
    mut commands: Commands,
) {
    // Clean up existing Minkowski difference visualizations
    for entity in minkowski_query.iter_mut() {
        commands.entity(entity).despawn();
    }

    // Find exactly two selected polygons
    let mut selected_polygons: Vec<(Entity, &PolygonShape)> = Vec::new();
    
    for (entity, shape, _, _, _, _, polygon_opt) in shapes.iter() {
        if let Some(polygon) = polygon_opt {
            if shape.selected {
                selected_polygons.push((entity, polygon));
            }
        }
    }
    
    // Only proceed if exactly two polygons are selected
    if selected_polygons.len() != 2 {
        return;
    }
    
    let (_, polygon_a) = selected_polygons[0];
    let (_, polygon_b) = selected_polygons[1];
    
    // Compute Minkowski difference
    let minkowski_diff = get_minkowski_difference(&polygon_a.polygon, &polygon_b.polygon);
    
    // Visualize the Minkowski difference as a polygon
    commands.spawn((
        Shape {
            layer: ShapeLayer::AuxiliaryLine,
            shape_type: minkowski_diff.get_shape_type(),
            selected: false,
        },
        PolygonShape { polygon: minkowski_diff },
        MinkowskiDifferenceVisualization,
        Transform::default(),
        Visibility::default(),
    ));
}

pub fn visualize_minkowski_difference(
    mut gizmos: Gizmos,
    // Query for Minkowski difference visualizations with specific coloring
    minkowski_shapes: Query<&PolygonShape, With<MinkowskiDifferenceVisualization>>,
) {
    fn qvec_to_vec2(v: QVec2) -> Vec2 {
        Vec2::new(v.x.to_num::<f32>(), v.y.to_num::<f32>())
    }
    // Draw Minkowski difference visualizations with a distinct color
    for polygon_shape in minkowski_shapes.iter() {
        let points = polygon_shape.polygon.points();
        if points.len() > 1 {
            // Draw edges between consecutive points with a distinct color (orange)
            for i in 0..points.len() {
                let current = points[i].pos();
                let next = points[(i + 1) % points.len()].pos();
                
                gizmos.line_2d(qvec_to_vec2(current), qvec_to_vec2(next), Color::srgba(1.0, 0.5, 0.0, 1.0));
            }
        } else if points.len() == 1 {
            // Draw a single point if there's only one point
            let pos = points[0].pos();
            gizmos.circle_2d(qvec_to_vec2(pos), 0.2, Color::srgba(1.0, 0.5, 0.0, 1.0));
        }
    }
}