//! Camera control plugin
//!
//! This module implements simple pan and zoom camera controls using mouse input.

use super::systems::*;
use bevy::prelude::*;
use bevy_egui::EguiStartupSet;

/// Plugin that registers camera controls (panning and zooming) for a `Camera2d`.
pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup.before(EguiStartupSet::InitContexts))
            .add_systems(Update, (camera_pan, camera_zoom));
    }
}
