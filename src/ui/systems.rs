//! UI systems
//!
//! This module defines the systems used for the egui-based user interface,
//! including the graphics editing panel.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use qgeometry::shape::{QShapeType};

/// Resource to track UI visibility state
#[derive(Resource, Default)]
pub struct UiState {
    /// Whether the graphics editor panel is visible
    pub panel_visible: bool,
    /// Currently selected shape type for drawing
    pub selected_shape: Option<QShapeType>,
}

/// System to render the egui UI
pub fn ui_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
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
                
                // Toggle buttons for shape types
                ui.label("Select Shape Type:");
                
                if ui.add(egui::Button::new("Point").selected(matches!(ui_state.selected_shape, Some(QShapeType::QPoint)))).clicked() {
                    ui_state.selected_shape = Some(QShapeType::QPoint);
                }
                
                if ui.add(egui::Button::new("Line").selected(matches!(ui_state.selected_shape, Some(QShapeType::QLine)))).clicked() {
                    ui_state.selected_shape = Some(QShapeType::QLine);
                }
                
                if ui.add(egui::Button::new("Rectangle (BBox)").selected(matches!(ui_state.selected_shape, Some(QShapeType::QBbox)))).clicked() {
                    ui_state.selected_shape = Some(QShapeType::QBbox);
                }
                
                if ui.add(egui::Button::new("Circle").selected(matches!(ui_state.selected_shape, Some(QShapeType::QCircle)))).clicked() {
                    ui_state.selected_shape = Some(QShapeType::QCircle);
                }
                
                if ui.add(egui::Button::new("Polygon").selected(matches!(ui_state.selected_shape, Some(QShapeType::QPolygon)))).clicked() {
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
                
                // Additional controls could be added here
                ui.separator();
                ui.label("Instructions:");
                ui.label("1. Select a shape type");
                ui.label("2. Click on the canvas to draw");
            });
    }
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