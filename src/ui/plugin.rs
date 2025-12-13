//! UI plugin implementation
//!
//! This module defines the Bevy plugin for the egui-based user interface,
//! including the graphics editing panel.

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::ui::systems::{ui_system, toggle_ui_visibility};

/// Plugin for handling UI functionality
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Initialize UI state resource
        app.init_resource::<crate::ui::systems::UiState>();
        
        // Add systems for UI rendering
        app.add_systems(EguiPrimaryContextPass, (ui_system, toggle_ui_visibility));
    }
}