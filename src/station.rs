use bevy::prelude::*;

use crate::components::position::Position;

#[derive(Component)]
pub struct Station {
    pub energy: u32,
    pub minerals: u32,
    pub scientific_data: u32,
}

pub struct StationPlugin;

impl Plugin for StationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_station);
    }
}

fn spawn_station(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::splat(32.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            ..Default::default()
        },
        Station {
            energy: 0,
            minerals: 0,
            scientific_data: 0,
        },
        Position { x: 0, y: 0 },
    ));
}
