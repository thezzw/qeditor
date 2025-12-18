//! Collision detection systems
//!
//! This module defines the systems used for collision detection and visualization.

use super::components::{CollisionVisualization, MinkowskiDifferenceVisualization, SeparationVectorVisualization};
use crate::shapes::components::{EditorShape, QBboxData, QCircleData, QLineData, QPointData, QPolygonData, ShapeLayer};
use bevy::prelude::*;
use qgeometry::algorithm::get_minkowski_difference;
use qgeometry::shape::{QLine, QPoint, QShapeCommon};
use qmath::vec2::QVec2;

/// System to detect collisions between shapes
pub fn detect_collisions(
    // Query all shapes with their components
    shapes: Query<(
        Entity,
        &EditorShape,
        Option<&QPointData>,
        Option<&QLineData>,
        Option<&QBboxData>,
        Option<&QCircleData>,
        Option<&QPolygonData>,
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
            if shape_a.layer == ShapeLayer::Generated || shape_b.layer == ShapeLayer::Generated {
                continue;
            }

            // Check if shapes collide
            let collision_detected = if let (Some(point), _) = (point_a, point_b) {
                if let Some(other_point) = point_b {
                    point.data.is_collide(&other_point.data)
                } else if let Some(other_line) = line_b {
                    point.data.is_collide(&other_line.data)
                } else if let Some(other_bbox) = bbox_b {
                    point.data.is_collide(&other_bbox.data)
                } else if let Some(other_circle) = circle_b {
                    point.data.is_collide(&other_circle.data)
                } else if let Some(other_polygon) = polygon_b {
                    point.data.is_collide(&other_polygon.data)
                } else {
                    false
                }
            } else if let (Some(line), _) = (line_a, line_b) {
                if let Some(other_point) = point_b {
                    line.data.is_collide(&other_point.data)
                } else if let Some(other_line) = line_b {
                    line.data.is_collide(&other_line.data)
                } else if let Some(other_bbox) = bbox_b {
                    line.data.is_collide(&other_bbox.data)
                } else if let Some(other_circle) = circle_b {
                    line.data.is_collide(&other_circle.data)
                } else if let Some(other_polygon) = polygon_b {
                    line.data.is_collide(&other_polygon.data)
                } else {
                    false
                }
            } else if let (Some(bbox), _) = (bbox_a, bbox_b) {
                if let Some(other_point) = point_b {
                    bbox.data.is_collide(&other_point.data)
                } else if let Some(other_line) = line_b {
                    bbox.data.is_collide(&other_line.data)
                } else if let Some(other_bbox) = bbox_b {
                    bbox.data.is_collide(&other_bbox.data)
                } else if let Some(other_circle) = circle_b {
                    bbox.data.is_collide(&other_circle.data)
                } else if let Some(other_polygon) = polygon_b {
                    bbox.data.is_collide(&other_polygon.data)
                } else {
                    false
                }
            } else if let (Some(circle), _) = (circle_a, circle_b) {
                if let Some(other_point) = point_b {
                    circle.data.is_collide(&other_point.data)
                } else if let Some(other_line) = line_b {
                    circle.data.is_collide(&other_line.data)
                } else if let Some(other_bbox) = bbox_b {
                    circle.data.is_collide(&other_bbox.data)
                } else if let Some(other_circle) = circle_b {
                    circle.data.is_collide(&other_circle.data)
                } else if let Some(other_polygon) = polygon_b {
                    circle.data.is_collide(&other_polygon.data)
                } else {
                    false
                }
            } else if let (Some(polygon), _) = (polygon_a, polygon_b) {
                if let Some(other_point) = point_b {
                    polygon.data.is_collide(&other_point.data)
                } else if let Some(other_line) = line_b {
                    polygon.data.is_collide(&other_line.data)
                } else if let Some(other_bbox) = bbox_b {
                    polygon.data.is_collide(&other_bbox.data)
                } else if let Some(other_circle) = circle_b {
                    polygon.data.is_collide(&other_circle.data)
                } else if let Some(other_polygon) = polygon_b {
                    polygon.data.is_collide(&other_polygon.data)
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
                        point.data.try_get_seperation_vector(&other_point.data)
                    } else if let Some(other_line) = line_b {
                        point.data.try_get_seperation_vector(&other_line.data)
                    } else if let Some(other_bbox) = bbox_b {
                        point.data.try_get_seperation_vector(&other_bbox.data)
                    } else if let Some(other_circle) = circle_b {
                        point.data.try_get_seperation_vector(&other_circle.data)
                    } else if let Some(other_polygon) = polygon_b {
                        point.data.try_get_seperation_vector(&other_polygon.data)
                    } else {
                        None
                    }
                } else if let (Some(line), _) = (line_a, line_b) {
                    if let Some(other_point) = point_b {
                        line.data.try_get_seperation_vector(&other_point.data)
                    } else if let Some(other_line) = line_b {
                        line.data.try_get_seperation_vector(&other_line.data)
                    } else if let Some(other_bbox) = bbox_b {
                        line.data.try_get_seperation_vector(&other_bbox.data)
                    } else if let Some(other_circle) = circle_b {
                        line.data.try_get_seperation_vector(&other_circle.data)
                    } else if let Some(other_polygon) = polygon_b {
                        line.data.try_get_seperation_vector(&other_polygon.data)
                    } else {
                        None
                    }
                } else if let (Some(bbox), _) = (bbox_a, bbox_b) {
                    if let Some(other_point) = point_b {
                        bbox.data.try_get_seperation_vector(&other_point.data)
                    } else if let Some(other_line) = line_b {
                        bbox.data.try_get_seperation_vector(&other_line.data)
                    } else if let Some(other_bbox) = bbox_b {
                        bbox.data.try_get_seperation_vector(&other_bbox.data)
                    } else if let Some(other_circle) = circle_b {
                        bbox.data.try_get_seperation_vector(&other_circle.data)
                    } else if let Some(other_polygon) = polygon_b {
                        bbox.data.try_get_seperation_vector(&other_polygon.data)
                    } else {
                        None
                    }
                } else if let (Some(circle), _) = (circle_a, circle_b) {
                    if let Some(other_point) = point_b {
                        circle.data.try_get_seperation_vector(&other_point.data)
                    } else if let Some(other_line) = line_b {
                        circle.data.try_get_seperation_vector(&other_line.data)
                    } else if let Some(other_bbox) = bbox_b {
                        circle.data.try_get_seperation_vector(&other_bbox.data)
                    } else if let Some(other_circle) = circle_b {
                        circle.data.try_get_seperation_vector(&other_circle.data)
                    } else if let Some(other_polygon) = polygon_b {
                        circle.data.try_get_seperation_vector(&other_polygon.data)
                    } else {
                        None
                    }
                } else if let (Some(polygon), _) = (polygon_a, polygon_b) {
                    if let Some(other_point) = point_b {
                        polygon.data.try_get_seperation_vector(&other_point.data)
                    } else if let Some(other_line) = line_b {
                        polygon.data.try_get_seperation_vector(&other_line.data)
                    } else if let Some(other_bbox) = bbox_b {
                        polygon.data.try_get_seperation_vector(&other_bbox.data)
                    } else if let Some(other_circle) = circle_b {
                        polygon.data.try_get_seperation_vector(&other_circle.data)
                    } else if let Some(other_polygon) = polygon_b {
                        polygon.data.try_get_seperation_vector(&other_polygon.data)
                    } else {
                        None
                    }
                } else {
                    None
                };

                // Visualize bbox for first shape
                if let (Some(point), _) = (point_a, point_b) {
                    let data = point.data.get_bbox();
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            ..default()
                        },
                        QBboxData { data },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (Some(line), _) = (line_a, line_b) {
                    let data = line.data.get_bbox();
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            ..default()
                        },
                        QBboxData { data },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (Some(bbox), _) = (bbox_a, bbox_b) {
                    let data = bbox.data.get_bbox(); // Already a bbox, but call get_bbox for consistency
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            ..default()
                        },
                        QBboxData { data },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (Some(circle), _) = (circle_a, circle_b) {
                    let data = circle.data.get_bbox();
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            ..default()
                        },
                        QBboxData { data },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (Some(polygon), _) = (polygon_a, polygon_b) {
                    let data = polygon.data.get_bbox();
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            ..default()
                        },
                        QBboxData { data },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                }

                // Visualize bbox for second shape
                if let (_, Some(other_point)) = (point_a, point_b) {
                    let data = other_point.data.get_bbox();
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            ..default()
                        },
                        QBboxData { data },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (_, Some(other_line)) = (line_a, line_b) {
                    let data = other_line.data.get_bbox();
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            ..default()
                        },
                        QBboxData { data },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (_, Some(other_bbox)) = (bbox_a, bbox_b) {
                    let data = other_bbox.data.get_bbox(); // Already a bbox, but call get_bbox for consistency
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            ..default()
                        },
                        QBboxData { data },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (_, Some(other_circle)) = (circle_a, circle_b) {
                    let data = other_circle.data.get_bbox();
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            ..default()
                        },
                        QBboxData { data },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                } else if let (_, Some(other_polygon)) = (polygon_a, polygon_b) {
                    let data = other_polygon.data.get_bbox();
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            ..default()
                        },
                        QBboxData { data },
                        CollisionVisualization,
                        Transform::default(),
                        Visibility::default(),
                    ));
                }

                // Spawn separation vector visualization if available
                if let Some(vector) = separation_vector
                    && vector != QVec2::ZERO
                {
                    let start = get_shape_center(point_b, line_b, bbox_b, circle_b, polygon_b);
                    let data = QLine::new_from_parts(start.pos(), start.pos().saturating_add(vector));
                    commands.spawn((
                        EditorShape {
                            layer: ShapeLayer::Generated,
                            shape_type: data.get_shape_type(),
                            line_appearance: crate::shapes::components::LineAppearance::Arrowhead,
                            ..default()
                        },
                        QLineData { data },
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
    point: Option<&QPointData>, line: Option<&QLineData>, bbox: Option<&QBboxData>, circle: Option<&QCircleData>,
    polygon: Option<&QPolygonData>,
) -> QPoint {
    if let Some(point) = point {
        point.data.get_centroid()
    } else if let Some(line) = line {
        line.data.get_centroid()
    } else if let Some(bbox) = bbox {
        bbox.data.get_centroid()
    } else if let Some(circle) = circle {
        circle.data.get_centroid()
    } else if let Some(polygon) = polygon {
        polygon.data.get_centroid()
    } else {
        QPoint::ZERO
    }
}

/// System to compute and visualize Minkowski difference of two selected polygons
pub fn compute_minkowski_difference(
    // Query all shapes with their components
    shapes: Query<(
        Entity,
        &EditorShape,
        Option<&QPointData>,
        Option<&QLineData>,
        Option<&QBboxData>,
        Option<&QCircleData>,
        Option<&QPolygonData>,
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
    let mut selected_polygons: Vec<(Entity, &QPolygonData)> = Vec::new();

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
    let minkowski_diff = get_minkowski_difference(&polygon_a.data, &polygon_b.data);

    // Visualize the Minkowski difference as a polygon
    commands.spawn((
        EditorShape {
            layer: ShapeLayer::Generated,
            shape_type: minkowski_diff.get_shape_type(),
            ..default()
        },
        QPolygonData { data: minkowski_diff },
        MinkowskiDifferenceVisualization,
        Transform::default(),
        Visibility::default(),
    ));
}

pub fn visualize_minkowski_difference(
    mut gizmos: Gizmos,
    // Query for Minkowski difference visualizations with specific coloring
    minkowski_shapes: Query<&QPolygonData, With<MinkowskiDifferenceVisualization>>,
) {
    fn qvec_to_vec2(v: QVec2) -> Vec2 {
        Vec2::new(v.x.to_num::<f32>(), v.y.to_num::<f32>())
    }
    // Draw Minkowski difference visualizations with a distinct color
    for polygon_shape in minkowski_shapes.iter() {
        let points = polygon_shape.data.points();
        if points.len() > 1 {
            // Draw edges between consecutive points with a distinct color (orange)
            for i in 0..points.len() {
                let current = points[i].pos();
                let next = points[(i + 1) % points.len()].pos();

                gizmos.line_2d(
                    qvec_to_vec2(current),
                    qvec_to_vec2(next),
                    Color::srgba(1.0, 0.5, 0.0, 1.0),
                );
            }
        } else if points.len() == 1 {
            // Draw a single point if there's only one point
            let pos = points[0].pos();
            gizmos.circle_2d(qvec_to_vec2(pos), 0.2, Color::srgba(1.0, 0.5, 0.0, 1.0));
        }
    }
}
