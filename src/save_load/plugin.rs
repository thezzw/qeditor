//! Save/Load plugin implementation
//!
//! Registers systems for saving and loading selected shapes from the MainScene layer.

use super::components::*;
use super::systems::*;
use bevy::prelude::*;

/// `SaveLoadPlugin` handles saving and loading of selected shapes.
pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register events
            .add_message::<SaveSelectedShapesEvent>()
            .add_message::<LoadShapesFromFileEvent>()
            // Register systems for save/load functionality
            .add_systems(Update, handle_save_request)
            .add_systems(Update, handle_load_request);
    }
}
