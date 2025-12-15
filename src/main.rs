//! Main application entry point
//! Initializes the Bevy application with the coordinate system plugin

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

// Import our coordinate system plugin
mod coordinate;
use coordinate::CoordinatePlugin;

mod camera;
use camera::CameraControlPlugin;

mod ui;
use ui::UiPlugin;

mod shapes;
use shapes::ShapesPlugin;

mod collision_detection;
use collision_detection::CollisionDetectionPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2D Coordinate System".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0))) // Set background to white
        .add_plugins(EguiPlugin::default())
        .add_plugins(CoordinatePlugin)
        .add_plugins(CameraControlPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(ShapesPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut _commands: Commands) {
    // Setup code can go here if needed
}