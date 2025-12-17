//! UI systems
//!
//! This module defines the systems used for the egui-based user interface,
//! including the graphics editing panel.

use super::resources::UiState;
use crate::save_load::components::{LoadShapesFromFileEvent, SaveSelectedShapesEvent};
use crate::shapes::components::{
    BboxShape, CircleShape, LineShape, PointShape, PolygonShape, Shape, ShapeLayer,
};
use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Ui},
};
use qgeometry::shape::QShapeType;

/// System to render the egui UI
pub fn draw_editor_ui(
    mut contexts: EguiContexts,
    commands: Commands,
    mut ui_state: ResMut<UiState>,
    // Query all shapes to display in the list
    shapes_query: Query<(
        Entity,
        &Shape,
        Option<&PointShape>,
        Option<&LineShape>,
        Option<&BboxShape>,
        Option<&CircleShape>,
        Option<&PolygonShape>,
    )>,
) {
    if !ui_state.panel_visible {
        return;
    }

    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Graphics Editor")
            .resizable(true)
            .default_size(egui::Vec2::new(300.0, 400.0))
            .show(ctx, |ui| {
                ui.heading("Graphics Editor");
                draw_shape_editor(ui, commands, &mut ui_state, shapes_query);
            });
    }
}

fn draw_shape_editor(
    ui: &mut Ui,
    mut commands: Commands,
    ui_state: &mut UiState,
    // Query selected shape to edit
    shapes_query: Query<(
        Entity,
        &Shape,
        Option<&PointShape>,
        Option<&LineShape>,
        Option<&BboxShape>,
        Option<&CircleShape>,
        Option<&PolygonShape>,
    )>,
) {
    // Toggle buttons for shape types
    ui.label("Select Shape Type:");
    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut ui_state.selected_shape,
            Some(QShapeType::QPoint),
            "Point",
        );
        ui.selectable_value(
            &mut ui_state.selected_shape,
            Some(QShapeType::QLine),
            "Line",
        );
        ui.selectable_value(
            &mut ui_state.selected_shape,
            Some(QShapeType::QBbox),
            "BBox",
        );
        ui.selectable_value(
            &mut ui_state.selected_shape,
            Some(QShapeType::QCircle),
            "Circle",
        );
        ui.selectable_value(
            &mut ui_state.selected_shape,
            Some(QShapeType::QPolygon),
            "Polygon",
        );
        ui.selectable_value(&mut ui_state.selected_shape, None, "None");
    });

    // Layer selection buttons
    ui.separator();
    ui.label("Select Layer:");
    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut ui_state.selected_layer,
            ShapeLayer::MainScene,
            "MainScene",
        );
        ui.selectable_value(
            &mut ui_state.selected_layer,
            ShapeLayer::AuxiliaryLine,
            "AuxiliaryLine",
        );
    });

    // Display list of shapes for the selected layer
    ui.separator();
    ui.label("Drawn Shapes:");

    // Scroll area for the shapes list
    egui::ScrollArea::vertical()
        .max_height(200.0)
        .show(ui, |ui| {
            // Iterate through shapes and display only those in the selected layer
            for (entity, shape, point_opt, line_opt, bbox_opt, circle_opt, polygon_opt) in
                shapes_query.iter()
            {
                // Only show shapes that belong to the selected layer
                if shape.layer != ui_state.selected_layer {
                    continue;
                }

                // Create a descriptive label for each shape
                let shape_label = match shape.shape_type {
                    QShapeType::QPoint => {
                        if let Some(point) = point_opt {
                            format!(
                                "Point ({:.2}, {:.2})",
                                point.point.pos().x.to_num::<f32>(),
                                point.point.pos().y.to_num::<f32>()
                            )
                        } else {
                            "Point".to_string()
                        }
                    }
                    QShapeType::QLine => {
                        if let Some(line) = line_opt {
                            format!(
                                "Line ({:.2}, {:.2}) -> ({:.2}, {:.2})",
                                line.line.start().pos().x.to_num::<f32>(),
                                line.line.start().pos().y.to_num::<f32>(),
                                line.line.end().pos().x.to_num::<f32>(),
                                line.line.end().pos().y.to_num::<f32>()
                            )
                        } else {
                            "Line".to_string()
                        }
                    }
                    QShapeType::QBbox => {
                        if let Some(bbox) = bbox_opt {
                            format!(
                                "Rectangle ({:.2}, {:.2}) -> ({:.2}, {:.2})",
                                bbox.bbox.left_bottom().pos().x.to_num::<f32>(),
                                bbox.bbox.left_bottom().pos().y.to_num::<f32>(),
                                bbox.bbox.right_top().pos().x.to_num::<f32>(),
                                bbox.bbox.right_top().pos().y.to_num::<f32>()
                            )
                        } else {
                            "Rectangle".to_string()
                        }
                    }
                    QShapeType::QCircle => {
                        if let Some(circle) = circle_opt {
                            format!(
                                "Circle ({:.2}, {:.2}), r={:.2}",
                                circle.circle.center().pos().x.to_num::<f32>(),
                                circle.circle.center().pos().y.to_num::<f32>(),
                                circle.circle.radius().to_num::<f32>()
                            )
                        } else {
                            "Circle".to_string()
                        }
                    }
                    QShapeType::QPolygon => {
                        if let Some(polygon) = polygon_opt {
                            format!("Polygon ({} vertices)", polygon.polygon.points().len())
                        } else {
                            "Polygon".to_string()
                        }
                    }
                };

                // Handle click on the shape in the list
                if ui.selectable_label(shape.selected, shape_label).clicked() {
                    // Toggle selection state of the clicked shape
                    let new_selected_state = !shape.selected;
                    if let Ok(mut entity_commands) = commands.get_entity(entity) {
                        entity_commands.insert(Shape {
                            layer: shape.layer,
                            shape_type: shape.shape_type,
                            selected: new_selected_state, // Toggle the selection state
                        });
                    }
                }
            }

            // Handle case when no shapes exist in the selected layer
            let shapes_in_selected_layer: Vec<_> = shapes_query
                .iter()
                .filter(|(_, shape, _, _, _, _, _)| shape.layer == ui_state.selected_layer)
                .collect();

            if shapes_in_selected_layer.is_empty() {
                ui.label("No shapes in the selected layer");
            }
        });

    // Add save/load functionality
    ui.separator();
    ui.label("Save/Load Selected Shapes:");

    // File path input
    ui.text_edit_singleline(&mut ui_state.file_path);

    // Save button
    if ui.button("Save Selected Shapes").clicked() {
        if !ui_state.file_path.is_empty() {
            commands.write_message(SaveSelectedShapesEvent {
                file_path: ui_state.file_path.clone(),
            });
        }
    }

    // Load button
    if ui.button("Load Shapes from File").clicked() {
        if !ui_state.file_path.is_empty() {
            commands.write_message(LoadShapesFromFileEvent {
                file_path: ui_state.file_path.clone(),
            });
        }
    }

    // Snap to grid checkbox
    ui.separator();
    ui.checkbox(&mut ui_state.enable_snap, "Snap to Grid");
}

/// System to toggle UI visibility with a keyboard shortcut (e.g., Tab key)
pub fn toggle_ui_visibility(
    mut ui_state: ResMut<UiState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        ui_state.panel_visible = !ui_state.panel_visible;
    }
}
