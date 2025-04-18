use bevy::prelude::*;
use rand::Rng;

use crate::components::{position::Position, robot_type::RobotType, robot_inventory::RobotInventory, robot_state::RobotState};
use crate::map::{MAP_WIDTH, MAP_HEIGHT, TILE_SIZE};


pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_robots);
    }
}

fn spawn_robots(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x = rng.gen_range(0..MAP_WIDTH) as i32;
        let y = rng.gen_range(0..MAP_HEIGHT) as i32;

        let robot_type = match rng.gen_range(0..3) {
            0 => RobotType::Explorer,
            1 => RobotType::Miner,
            _ => RobotType::Scientist,
        };

        let color = match robot_type {
            RobotType::Explorer => Color::rgb(1.0, 1.0, 0.0),
            RobotType::Miner => Color::rgb(1.0, 0.5, 0.0),
            RobotType::Scientist => Color::rgb(0.5, 0.0, 1.0),
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE - 4.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    1.0,
                ),
                ..Default::default()
            },
            Position { x, y },
            robot_type,
            RobotInventory {
                energy: 0,
                minerals: 0,
                scientific_data: 0,
            },
            RobotState::Exploring,
        ));
        
    }
}
