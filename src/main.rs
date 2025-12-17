//! Main application entry point

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

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

mod save_load;
use save_load::SaveLoadPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "QEditor".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(CoordinatePlugin)
        .add_plugins(CameraControlPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(SaveLoadPlugin)
        .add_plugins(ShapesPlugin)
        .add_plugins(UiPlugin)
        .run();
}
