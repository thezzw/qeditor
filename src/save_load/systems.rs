//! Save/Load systems
//!
//! This module defines the systems used for saving and loading selected shapes
//! from the MainScene layer to and from files.

use super::components::{LoadShapesFromFileEvent, SaveSelectedShapesEvent, SerializableQShapeData};
use crate::shapes::components::{EditorShape, QBboxData, QCircleData, QLineData, QPointData, QPolygonData, ShapeLayer};
use bevy::prelude::*;
use qgeometry;
use std::fs::File;
use std::io::{BufReader, BufWriter};

/// System to handle save requests for selected shapes in MainScene layer
pub fn handle_save_request(
    mut events: MessageReader<SaveSelectedShapesEvent>,
    shapes_query: Query<(
        &EditorShape,
        Option<&QPointData>,
        Option<&QLineData>,
        Option<&QBboxData>,
        Option<&QCircleData>,
        Option<&QPolygonData>,
    )>,
) {
    for event in events.read() {
        // Save to file
        if let Err(e) = save_shapes_to_file(&event.file_path, shapes_query) {
            eprintln!("Failed to save shapes to file: {}", e);
        }
    }
}

/// Save shapes to a JSON file
fn save_shapes_to_file(
    file_path: &str,
    shapes_query: Query<(
        &EditorShape,
        Option<&QPointData>,
        Option<&QLineData>,
        Option<&QBboxData>,
        Option<&QCircleData>,
        Option<&QPolygonData>,
    )>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data_list = Vec::new();
    for (shape, point_opt, line_opt, bbox_opt, circle_opt, polygon_opt) in shapes_query.iter() {
        if shape.layer != ShapeLayer::MainScene {
            continue; // Skip shapes not in MainScene layer
        }

        if let Some(data) = point_opt {
            data_list.push(SerializableQShapeData::Point(data.clone()));
        }
        if let Some(data) = line_opt {
            data_list.push(SerializableQShapeData::Line(data.clone()));
        }
        if let Some(data) = bbox_opt {
            data_list.push(SerializableQShapeData::Bbox(data.clone()));
        }
        if let Some(data) = circle_opt {
            data_list.push(SerializableQShapeData::Circle(data.clone()));
        }
        if let Some(data) = polygon_opt {
            data_list.push(SerializableQShapeData::Polygon(data.clone()));
        }
    }
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &data_list)?;
    Ok(())
}

/// System to handle load requests for shapes from a file
pub fn handle_load_request(mut commands: Commands, mut events: MessageReader<LoadShapesFromFileEvent>) {
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

/// Load shapes from a JSON file
fn load_shapes_from_file(file_path: &str) -> Result<Vec<SerializableQShapeData>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let shapes: Vec<SerializableQShapeData> = serde_json::from_reader(reader)?;
    Ok(shapes)
}

/// Spawn a shape entity from serialized data
fn spawn_shape_from_serialized(commands: &mut Commands, serialized: &SerializableQShapeData) {
    let shape_type = match serialized {
        SerializableQShapeData::Point(_data) => qgeometry::shape::QShapeType::QPoint,
        SerializableQShapeData::Line(_data) => qgeometry::shape::QShapeType::QLine,
        SerializableQShapeData::Bbox(_data) => qgeometry::shape::QShapeType::QBbox,
        SerializableQShapeData::Circle(_data) => qgeometry::shape::QShapeType::QCircle,
        SerializableQShapeData::Polygon(_data) => qgeometry::shape::QShapeType::QPolygon,
    };

    let mut entity_commands = commands.spawn((
        EditorShape {
            layer: ShapeLayer::MainScene,
            shape_type,
            selected: false,
        },
        Transform::default(),
        Visibility::default(),
    ));

    match serialized {
        SerializableQShapeData::Point(data) => {
            entity_commands.insert(data.clone());
        }
        SerializableQShapeData::Line(data) => {
            entity_commands.insert(data.clone());
        }
        SerializableQShapeData::Bbox(data) => {
            entity_commands.insert(data.clone());
        }
        SerializableQShapeData::Circle(data) => {
            entity_commands.insert(data.clone());
        }
        SerializableQShapeData::Polygon(data) => {
            entity_commands.insert(data.clone());
        }
    }
}
