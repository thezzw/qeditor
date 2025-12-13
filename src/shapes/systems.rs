//! Shapes systems
//!
//! This module defines the systems used for the shapes functionality,
//! including rendering and interaction.

use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_egui::EguiContexts;
use qgeometry::shape::{QPoint, QLine, QBbox, QCircle, QPolygon, QShapeType};
use qmath::vec2::QVec2;
use qmath::prelude::*;

use crate::shapes::{
    components::{Shape, PointShape, LineShape, BboxShape, CircleShape, PolygonShape},
    resources::{SelectedShapeType, ShapeDrawingState},
};
use crate::ui::systems::UiState;

/// System to handle shape interaction (creation, selection, etc.)
pub fn handle_shape_interaction(
    mut commands: Commands,
    mut polygon_query: Query<&mut PolygonShape>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    ui_state: Res<UiState>,
    mut selected_shape_type: ResMut<SelectedShapeType>,
    mut shape_drawing_state: ResMut<ShapeDrawingState>,
    mut egui_contexts: EguiContexts, // Add EguiContexts to check if mouse is over UI
) {
    // Check if egui wants pointer input (mouse is over UI)
    let mouse_over_ui = if let Ok(ctx) = egui_contexts.ctx_mut() {
        ctx.wants_pointer_input()
    } else {
        false
    };

    // If mouse is over UI, don't handle shape interaction
    if mouse_over_ui {
        return;
    }

    // Update the selected shape type based on UI state
    if let Some(shape_type) = ui_state.selected_shape {
        selected_shape_type.shape_type = Some(shape_type);
    }

    if selected_shape_type.shape_type.is_none() { return; }

    // Get the primary window
    let window = if let Ok(window) = windows.single() {
        window
    } else {
        return;
    };

    // Get camera transform for proper coordinate conversion
    let (camera, camera_transform) = if let Ok((camera, camera_transform)) = camera_q.single() {
        (camera, camera_transform)
    } else {
        return;
    };

    // Convert screen coordinates to world coordinates properly using the camera
    let cursor_pos = if let Some(cursor_pos) = window.cursor_position() {
        cursor_pos
    } else {
        return;
    };

    let world_pos = if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
        world_pos
    } else {
        // Fallback calculation if camera conversion fails
        Vec2::new(
            cursor_pos.x - window.width() / 2.0,
            window.height() / 2.0 - cursor_pos.y,
        )
    };

    // Convert world coordinates to QVec2
    let qworld_pos = QVec2::new(Q64::from_num(world_pos.x), Q64::from_num(world_pos.y));
    let qworld_point = QPoint::new(qworld_pos);

    // Handle ongoing shape drawing
    match selected_shape_type.shape_type.unwrap() {
        QShapeType::QPoint | QShapeType::QLine | QShapeType::QBbox | QShapeType::QCircle => {
            // Finalize the current shape
            if let Some(entity) = shape_drawing_state.current_shape {
                if let Some(start_pos) = shape_drawing_state.start_position {
                    // Finalize shape properties based on second click
                    let start_point = QPoint::new(start_pos);
                    if start_point == qworld_point { return; }
                    match selected_shape_type.shape_type.unwrap() {
                        QShapeType::QPoint => {
                            // Points don't need further modification
                        }
                        QShapeType::QLine => {
                            // For line shapes, we need to get the current line to update it
                            // Since we can't directly access the component, we'll recreate it with the new end point
                            let new_line = QLine::new(start_point, qworld_point);
                            commands.entity(entity).insert(LineShape { line: new_line });
                        }
                        QShapeType::QBbox => {
                            // Update the bounding box with the second corner
                            if !start_point.pos().partial_cmp(&qworld_pos).is_some_and(|ordering| ordering == Ordering::Less) { return; }
                            let new_bbox = QBbox::new_from_parts(start_point.pos(), qworld_pos);
                            commands.entity(entity).insert(BboxShape { bbox: new_bbox });
                        }
                        QShapeType::QCircle => {
                            // Update the circle radius based on distance from center
                            let dx = qworld_pos.x - start_pos.x;
                            let dy = qworld_pos.y - start_pos.y;
                            let radius = (dx * dx + dy * dy).sqrt();
                            let new_circle = QCircle::new(start_point, Q64::from_num(radius));
                            commands.entity(entity).insert(CircleShape { circle: new_circle });
                        }
                        _ => {}
                    }
                }
            }
        }
        QShapeType::QPolygon => {
            // Add vertex to polygon
            if let Some(entity) = shape_drawing_state.current_shape {
                // Get the current polygon component
                if let Ok(mut polygon_shape) = polygon_query.get_mut(entity) {
                    // Add new vertex to existing polygon
                    let mut points: Vec<QPoint> = polygon_shape.polygon.points().clone();
                    let last_point = points.last_mut().unwrap();
                    last_point.set_pos(qworld_pos);
                    
                    // Create new polygon with updated points
                    let new_polygon = QPolygon::new(points);
                    polygon_shape.polygon = new_polygon;
                }
            }
        }
    }

    // Handle right mouse button for ending polygon drawing
    if mouse_button_input.just_pressed(MouseButton::Right) {
        if shape_drawing_state.is_drawing && selected_shape_type.shape_type.unwrap() == QShapeType::QPolygon {
            // End polygon drawing
            shape_drawing_state.is_drawing = false;
            shape_drawing_state.start_position = None;
            return;
        }
    }

    // Handle left mouse button for shape creation
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if shape_drawing_state.is_drawing {
            // Handle ongoing shape drawing
            match selected_shape_type.shape_type.unwrap() {
                QShapeType::QPoint | QShapeType::QLine | QShapeType::QBbox | QShapeType::QCircle => {
                    // Finalize the current shape
                    if let Some(_entity) = shape_drawing_state.current_shape {
                        // Finalize shape properties based on second click
                        shape_drawing_state.is_drawing = false;
                        shape_drawing_state.start_position = None;
                    }
                }
                QShapeType::QPolygon => {
                    if let Some(entity) = shape_drawing_state.current_shape {
                        // Get the current polygon component
                        if let Ok(mut polygon_shape) = polygon_query.get_mut(entity) {
                            // Add new vertex to existing polygon
                            let mut points: Vec<QPoint> = polygon_shape.polygon.points().clone();
                            points.push(qworld_point);

                            // Create new polygon with updated points
                            let new_polygon = QPolygon::new(points);
                            polygon_shape.polygon = new_polygon;
                        }
                    }
                }
            }
            return;
        }

        // Start drawing a new shape
        shape_drawing_state.is_drawing = true;
        shape_drawing_state.start_position = Some(qworld_pos);

        // Create the appropriate shape based on the selected type
        match selected_shape_type.shape_type.unwrap() {
            QShapeType::QPoint => {
                // Create a point shape
                let entity = commands.spawn((
                    Shape {
                        shape_type: QShapeType::QPoint,
                        selected: false,
                    },
                    PointShape {
                        point: qworld_point,
                    },
                    Transform::default(),
                    Visibility::default(),
                )).id();
                shape_drawing_state.current_shape = Some(entity);
            }
            QShapeType::QLine => {
                // Create a line shape with both points at the same location initially
                let qline = QLine::new(qworld_point, QPoint::new(qworld_pos.saturating_add_num(Q64::EPS)));
                let entity = commands.spawn((
                    Shape {
                        shape_type: QShapeType::QLine,
                        selected: false,
                    },
                    LineShape {
                        line: qline,
                    },
                    Transform::default(),
                    Visibility::default(),
                )).id();
                shape_drawing_state.current_shape = Some(entity);
            }
            QShapeType::QBbox => {
                // Create a bounding box shape
                let qbbox = QBbox::new_from_parts(
                    qworld_pos,
                    qworld_pos.saturating_add_num(Q64::EPS)
                );
                let entity = commands.spawn((
                    Shape {
                        shape_type: QShapeType::QBbox,
                        selected: false,
                    },
                    BboxShape {
                        bbox: qbbox,
                    },
                    Transform::default(),
                    Visibility::default(),
                )).id();
                shape_drawing_state.current_shape = Some(entity);
            }
            QShapeType::QCircle => {
                // Create a circle shape
                let qcircle = QCircle::new(qworld_point, Q64::EPS); // Default radius of Q64::EPS
                let entity = commands.spawn((
                    Shape {
                        shape_type: QShapeType::QCircle,
                        selected: false,
                    },
                    CircleShape {
                        circle: qcircle,
                    },
                    Transform::default(),
                    Visibility::default(),
                )).id();
                shape_drawing_state.current_shape = Some(entity);
            }
            QShapeType::QPolygon => {
                // Create a polygon shape with a single point initially
                let qpolygon = QPolygon::new(vec![qworld_point, qworld_point]);
                let entity = commands.spawn((
                    Shape {
                        shape_type: QShapeType::QPolygon,
                        selected: false,
                    },
                    PolygonShape {
                        polygon: qpolygon,
                    },
                    Transform::default(),
                    Visibility::default(),
                )).id();
                shape_drawing_state.current_shape = Some(entity);
            }
        }
    }
}

/// System to draw shapes using gizmos
pub fn draw_shapes(
    mut gizmos: Gizmos,
    shapes: Query<(&Shape, Option<&PointShape>, Option<&LineShape>, Option<&BboxShape>, Option<&CircleShape>, Option<&PolygonShape>)>,
) {
    for (shape, point_opt, line_opt, bbox_opt, circle_opt, polygon_opt) in shapes.iter() {
        // Set color based on selection state
        let color = if shape.selected {
            Color::srgba(0.0, 1.0, 0.0, 1.0) // Green
        } else {
            Color::srgba(0.0, 0.0, 0.0, 1.0) // Black
        };

        // Draw the appropriate shape based on its type
        if let Some(point) = point_opt {
            let pos = point.point.pos();
            gizmos.circle_2d(Vec2::new(pos.x.to_num::<f32>(), pos.y.to_num::<f32>()), 0.2, color);
        }

        if let Some(line) = line_opt {
            // Draw actual line from the QLine data
            let start = line.line.start().pos();
            let end = line.line.end().pos();
            gizmos.line_2d(
                Vec2::new(start.x.to_num::<f32>(), start.y.to_num::<f32>()),
                Vec2::new(end.x.to_num::<f32>(), end.y.to_num::<f32>()),
                color,
            );
        }

        if let Some(bbox) = bbox_opt {
            let min = bbox.bbox.left_bottom().pos();
            let max = bbox.bbox.right_top().pos();
            let center = Vec2::new(
                (min.x.to_num::<f32>() + max.x.to_num::<f32>()) / 2.0,
                (min.y.to_num::<f32>() + max.y.to_num::<f32>()) / 2.0
            );
            let size = Vec2::new(
                (max.x.to_num::<f32>() - min.x.to_num::<f32>()).abs(),
                (max.y.to_num::<f32>() - min.y.to_num::<f32>()).abs()
            );
            gizmos.rect_2d(center, size, color);
        }

        if let Some(circle) = circle_opt {
            let center = circle.circle.center().pos();
            let radius = circle.circle.radius().to_num::<f32>();
            gizmos.circle_2d(
                Vec2::new(center.x.to_num::<f32>(), center.y.to_num::<f32>()),
                radius,
                color,
            );
        }

        // Draw polygon edges
        if let Some(polygon) = polygon_opt {
            let points = polygon.polygon.points();
            if points.len() > 1 {
                // Draw edges between consecutive points
                for i in 0..points.len() {
                    let current = points[i].pos();
                    let next = points[(i + 1) % points.len()].pos();
                    
                    gizmos.line_2d(
                        Vec2::new(current.x.to_num::<f32>(), current.y.to_num::<f32>()),
                        Vec2::new(next.x.to_num::<f32>(), next.y.to_num::<f32>()),
                        color,
                    );
                }
            } else if points.len() == 1 {
                // Draw a single point if there's only one point
                let pos = points[0].pos();
                gizmos.circle_2d(Vec2::new(pos.x.to_num::<f32>(), pos.y.to_num::<f32>()), 0.2, color);
            }
        }
    }
}
