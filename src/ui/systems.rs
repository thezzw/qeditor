//! UI systems
//!
//! This module defines the systems used for the egui-based user interface,
//! including the graphics editing panel.

use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Ui},
};
use qgeometry::shape::QShapeType;

// Import shape components to query them
use crate::shapes::components::{
    BboxShape, CircleShape, LineShape, PointShape, PolygonShape, Shape, ShapeLayer,
};
// Import save/load events
use crate::save_load::systems::{SaveSelectedShapesEvent, LoadShapesFromFileEvent};

/// Resource to track UI visibility state
#[derive(Resource)]
pub struct UiState {
    /// Whether the graphics editor panel is visible
    pub panel_visible: bool,
    /// Currently selected shape type for drawing
    pub selected_shape: Option<QShapeType>,
    /// Currently selected shape layer
    pub selected_layer: ShapeLayer,
    /// File path for saving/loading shapes
    pub file_path: String,
    /// Whether to enable snap to grid
    pub enable_snap: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            panel_visible: false,
            selected_shape: None,
            selected_layer: ShapeLayer::MainScene,
            file_path: "assets/save/default.json".to_string(),
            enable_snap: true,
        }
    }
}

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

    if ui
        .add(
            egui::Button::new("Point")
                .selected(matches!(ui_state.selected_shape, Some(QShapeType::QPoint))),
        )
        .clicked()
    {
        ui_state.selected_shape = Some(QShapeType::QPoint);
    }

    if ui
        .add(
            egui::Button::new("Line")
                .selected(matches!(ui_state.selected_shape, Some(QShapeType::QLine))),
        )
        .clicked()
    {
        ui_state.selected_shape = Some(QShapeType::QLine);
    }

    if ui
        .add(
            egui::Button::new("Rectangle (BBox)")
                .selected(matches!(ui_state.selected_shape, Some(QShapeType::QBbox))),
        )
        .clicked()
    {
        ui_state.selected_shape = Some(QShapeType::QBbox);
    }

    if ui
        .add(
            egui::Button::new("Circle")
                .selected(matches!(ui_state.selected_shape, Some(QShapeType::QCircle))),
        )
        .clicked()
    {
        ui_state.selected_shape = Some(QShapeType::QCircle);
    }

    if ui
        .add(egui::Button::new("Polygon").selected(matches!(
            ui_state.selected_shape,
            Some(QShapeType::QPolygon)
        )))
        .clicked()
    {
        ui_state.selected_shape = Some(QShapeType::QPolygon);
    }

    // Clear selection button
    if ui.button("Clear Selection").clicked() {
        ui_state.selected_shape = None;
    }

    // Display current selection
    ui.separator();
    ui.label("Current Selection:");
    match ui_state.selected_shape {
        Some(QShapeType::QPoint) => {
            ui.label("Point");
        }
        Some(QShapeType::QLine) => {
            ui.label("Line");
        }
        Some(QShapeType::QBbox) => {
            ui.label("Rectangle");
        }
        Some(QShapeType::QCircle) => {
            ui.label("Circle");
        }
        Some(QShapeType::QPolygon) => {
            ui.label("Polygon");
        }
        None => {
            ui.label("No shape selected");
        }
    }

    // Layer selection buttons
    ui.separator();
    ui.label("Select Layer:");
    
    // Main Scene layer button
    if ui.add(
        egui::Button::new("Main Scene")
            .selected(matches!(ui_state.selected_layer, ShapeLayer::MainScene)),
    ).clicked() {
        ui_state.selected_layer = ShapeLayer::MainScene;
    }
    
    // Auxiliary Line layer button
    if ui.add(
        egui::Button::new("Auxiliary Line")
            .selected(matches!(ui_state.selected_layer, ShapeLayer::AuxiliaryLine)),
    ).clicked() {
        ui_state.selected_layer = ShapeLayer::AuxiliaryLine;
    }

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

                // Create a selectable label for the shape
                let response =
                    ui.selectable_label(shape.selected, shape_label);

                // Handle click on the shape in the list
                if response.clicked() {
                    // Toggle selection state of the clicked shape
                    let new_selected_state = !shape.selected;
                    if let Ok(mut entity_commands) = commands.get_entity(entity) {
                        entity_commands.insert(Shape {
                            layer: shape.layer,
                            shape_type: shape.shape_type,
                            selected: new_selected_state,  // Toggle the selection state
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
    ui.label("Grid Settings:");
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