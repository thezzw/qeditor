//! Save/Load systems
//!
//! This module defines the systems used for saving and loading selected shapes
//! from the MainScene layer to and from files.

use std::fs::File;
use std::io::{BufReader, BufWriter};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use qmath;
use qgeometry;

use crate::shapes::components::{
    BboxShape, CircleShape, LineShape, PointShape, PolygonShape, Shape, ShapeLayer,
};

/// Events to trigger save operations
#[derive(Message, Clone)]
pub struct SaveSelectedShapesEvent {
    pub file_path: String,
}

/// Events to trigger load operations
#[derive(Message, Clone)]
pub struct LoadShapesFromFileEvent {
    pub file_path: String,
}

/// Serializable representation of a point shape
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializablePoint {
    pub x: f64,
    pub y: f64,
}

/// Serializable representation of a shape
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerializableShape {
    pub shape_type: String,
    pub selected: bool,
    pub point: Option<SerializablePoint>,
    pub line_start: Option<SerializablePoint>,
    pub line_end: Option<SerializablePoint>,
    pub bbox_min: Option<SerializablePoint>,
    pub bbox_max: Option<SerializablePoint>,
    pub circle_center: Option<SerializablePoint>,
    pub circle_radius: Option<f64>,
    pub polygon_points: Option<Vec<SerializablePoint>>,
}

/// System to handle save requests for selected shapes in MainScene layer
pub fn handle_save_request(
    mut events: MessageReader<SaveSelectedShapesEvent>,
    shapes_query: Query<(
        &Shape,
        Option<&PointShape>,
        Option<&LineShape>,
        Option<&BboxShape>,
        Option<&CircleShape>,
        Option<&PolygonShape>,
    )>,
) {
    for event in events.read() {
        // Filter selected shapes from MainScene layer
        let selected_shapes: Vec<SerializableShape> = shapes_query
            .iter()
            .filter(|(shape, _, _, _, _, _)| {
                shape.layer == ShapeLayer::MainScene && shape.selected
            })
            .map(|(shape, point_opt, line_opt, bbox_opt, circle_opt, polygon_opt)| {
                let mut serializable = SerializableShape {
                    shape_type: format!("{:?}", shape.shape_type),
                    selected: shape.selected,
                    point: None,
                    line_start: None,
                    line_end: None,
                    bbox_min: None,
                    bbox_max: None,
                    circle_center: None,
                    circle_radius: None,
                    polygon_points: None,
                };

                if let Some(point) = point_opt {
                    serializable.point = Some(SerializablePoint {
                        x: point.point.pos().x.to_num::<f64>(),
                        y: point.point.pos().y.to_num::<f64>(),
                    });
                }

                if let Some(line) = line_opt {
                    serializable.line_start = Some(SerializablePoint {
                        x: line.line.start().pos().x.to_num::<f64>(),
                        y: line.line.start().pos().y.to_num::<f64>(),
                    });
                    serializable.line_end = Some(SerializablePoint {
                        x: line.line.end().pos().x.to_num::<f64>(),
                        y: line.line.end().pos().y.to_num::<f64>(),
                    });
                }

                if let Some(bbox) = bbox_opt {
                    serializable.bbox_min = Some(SerializablePoint {
                        x: bbox.bbox.left_bottom().pos().x.to_num::<f64>(),
                        y: bbox.bbox.left_bottom().pos().y.to_num::<f64>(),
                    });
                    serializable.bbox_max = Some(SerializablePoint {
                        x: bbox.bbox.right_top().pos().x.to_num::<f64>(),
                        y: bbox.bbox.right_top().pos().y.to_num::<f64>(),
                    });
                }

                if let Some(circle) = circle_opt {
                    serializable.circle_center = Some(SerializablePoint {
                        x: circle.circle.center().pos().x.to_num::<f64>(),
                        y: circle.circle.center().pos().y.to_num::<f64>(),
                    });
                    serializable.circle_radius = Some(circle.circle.radius().to_num::<f64>());
                }

                if let Some(polygon) = polygon_opt {
                    let points: Vec<SerializablePoint> = polygon
                        .polygon
                        .points()
                        .iter()
                        .map(|point| SerializablePoint {
                            x: point.pos().x.to_num::<f64>(),
                            y: point.pos().y.to_num::<f64>(),
                        })
                        .collect();
                    serializable.polygon_points = Some(points);
                }

                serializable
            })
            .collect();

        // Save to file
        if let Err(e) = save_shapes_to_file(&selected_shapes, &event.file_path) {
            eprintln!("Failed to save shapes to file: {}", e);
        }
    }
}

/// System to handle load requests for shapes from a file
pub fn handle_load_request(
    mut commands: Commands,
    mut events: MessageReader<LoadShapesFromFileEvent>,
) {
    for event in events.read() {
        match load_shapes_from_file(&event.file_path) {
            Ok(serialized_shapes) => {
                // Spawn loaded shapes as entities
                for serialized_shape in serialized_shapes {
                    spawn_shape_from_serialized(&mut commands, &serialized_shape);
                }
            }
            Err(e) => {
                eprintln!("Failed to load shapes from file: {}", e);
            }
        }
    }
}

/// Save shapes to a JSON file
fn save_shapes_to_file(shapes: &[SerializableShape], file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, shapes)?;
    Ok(())
}

/// Load shapes from a JSON file
fn load_shapes_from_file(file_path: &str) -> Result<Vec<SerializableShape>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let shapes: Vec<SerializableShape> = serde_json::from_reader(reader)?;
    Ok(shapes)
}

/// Spawn a shape entity from serialized data
fn spawn_shape_from_serialized(commands: &mut Commands, serialized: &SerializableShape) {
    // Parse the shape type correctly
    let shape_type_str = serialized.shape_type.replace("QShapeType::", "").replace("crate::shapes::components::", "");
    // Use the QShapeType from the qgeometry crate directly
    let shape_type = match shape_type_str.as_str() {
        "QPoint" => qgeometry::shape::QShapeType::QPoint,
        "QLine" => qgeometry::shape::QShapeType::QLine,
        "QBbox" => qgeometry::shape::QShapeType::QBbox,
        "QCircle" => qgeometry::shape::QShapeType::QCircle,
        "QPolygon" => qgeometry::shape::QShapeType::QPolygon,
        _ => {
            eprintln!("Unknown shape type: {}", shape_type_str);
            return; // Unknown shape type
        }
    };

    let mut entity_commands = commands.spawn((
        Shape {
            layer: ShapeLayer::MainScene,
            shape_type,
            selected: serialized.selected,
        },
        Transform::default(),
        Visibility::default(),
    ));

    match shape_type {
        qgeometry::shape::QShapeType::QPoint => {
            if let Some(point) = &serialized.point {
                let qvec = qmath::vec2::QVec2::new(
                    qmath::Q64::from_num(point.x),
                    qmath::Q64::from_num(point.y),
                );
                let qpoint = qgeometry::shape::QPoint::new(qvec);
                entity_commands.insert(PointShape { point: qpoint });
            }
        }
        qgeometry::shape::QShapeType::QLine => {
            if let (Some(start), Some(end)) = (&serialized.line_start, &serialized.line_end) {
                let start_qvec = qmath::vec2::QVec2::new(
                    qmath::Q64::from_num(start.x),
                    qmath::Q64::from_num(start.y),
                );
                let end_qvec = qmath::vec2::QVec2::new(
                    qmath::Q64::from_num(end.x),
                    qmath::Q64::from_num(end.y),
                );
                let start_qpoint = qgeometry::shape::QPoint::new(start_qvec);
                let end_qpoint = qgeometry::shape::QPoint::new(end_qvec);
                let qline = qgeometry::shape::QLine::new(start_qpoint, end_qpoint);
                entity_commands.insert(LineShape { line: qline });
            }
        }
        qgeometry::shape::QShapeType::QBbox => {
            if let (Some(min), Some(max)) = (&serialized.bbox_min, &serialized.bbox_max) {
                let min_qvec = qmath::vec2::QVec2::new(
                    qmath::Q64::from_num(min.x),
                    qmath::Q64::from_num(min.y),
                );
                let max_qvec = qmath::vec2::QVec2::new(
                    qmath::Q64::from_num(max.x),
                    qmath::Q64::from_num(max.y),
                );
                let qbbox = qgeometry::shape::QBbox::new_from_parts(min_qvec, max_qvec);
                entity_commands.insert(BboxShape { bbox: qbbox });
            }
        }
        qgeometry::shape::QShapeType::QCircle => {
            if let (Some(center), Some(radius)) = (&serialized.circle_center, serialized.circle_radius) {
                let center_qvec = qmath::vec2::QVec2::new(
                    qmath::Q64::from_num(center.x),
                    qmath::Q64::from_num(center.y),
                );
                let center_qpoint = qgeometry::shape::QPoint::new(center_qvec);
                let qcircle = qgeometry::shape::QCircle::new(center_qpoint, qmath::Q64::from_num(radius));
                entity_commands.insert(CircleShape { circle: qcircle });
            }
        }
        qgeometry::shape::QShapeType::QPolygon => {
            if let Some(points) = &serialized.polygon_points {
                let qpoints: Vec<qgeometry::shape::QPoint> = points
                    .iter()
                    .map(|p| {
                        let qvec = qmath::vec2::QVec2::new(
                            qmath::Q64::from_num(p.x),
                            qmath::Q64::from_num(p.y),
                        );
                        qgeometry::shape::QPoint::new(qvec)
                    })
                    .collect();
                let qpolygon = qgeometry::shape::QPolygon::new(qpoints);
                entity_commands.insert(PolygonShape { polygon: qpolygon });
            }
        }
    }
}