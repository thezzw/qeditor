//! UI plugin implementation
//!
//! Registers the egui UI state resource and the systems that render the editor UI.

use bevy::prelude::*;
use bevy_egui::EguiPrimaryContextPass;

use crate::ui::systems::{UiState, draw_editor_ui, toggle_ui_visibility};

/// `UiPlugin` handles UI state and registers UI systems.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the UI state (Default) resource consistently.
        app.init_resource::<UiState>()
            // Register UI systems that require egui context
            .add_systems(
                EguiPrimaryContextPass,
                (draw_editor_ui, toggle_ui_visibility),
            );
    }
}
