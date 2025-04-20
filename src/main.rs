mod components;
mod map;
mod station;
mod systems;
mod utils;

use bevy::prelude::*;
use map::MapPlugin;
use station::StationPlugin;
use systems::spawn_robots::RobotPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_plugins(MapPlugin)
        .add_plugins(StationPlugin)
        .add_plugins(RobotPlugin)
        .add_systems(Update, systems::movement::robot_movement_system)
        .add_systems(
            Update,
            systems::resource_collection::resource_collection_system,
        )
        .add_systems(Update, systems::robot_behavior::robot_behavior_system)
        .add_systems(Update, systems::delivery::delivery_system)
        .add_systems(Update, systems::station_report::station_report_system)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.scale = Vec3::new(1.3, 1.3, 1.0);
    commands.spawn(camera);
}
