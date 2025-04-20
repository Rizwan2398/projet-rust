use crate::utils::noise::NoiseGenerator;
use bevy::prelude::*;
use rand::Rng;

pub const MAP_WIDTH: u32 = 50;
pub const MAP_HEIGHT: u32 = 30;
pub const TILE_SIZE: f32 = 16.0;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Scientific,
}

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(mut commands: Commands) {
    let noise = NoiseGenerator::new(42);
    let mut rng = rand::thread_rng();

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let noise_value = noise.get_value(x as f64, y as f64);

            let tile_type = if noise_value < -0.2 {
                TileType::Obstacle
            } else if noise_value > 0.4 {
                match rng.gen_range(0..10) {
                    0 => TileType::Energy,
                    1 => TileType::Mineral,
                    2 => TileType::Scientific,
                    _ => TileType::Empty,
                }
            } else {
                TileType::Empty
            };

            let color = match tile_type {
                TileType::Empty => Color::rgb(0.9, 0.9, 0.9),
                TileType::Obstacle => Color::rgb(0.2, 0.2, 0.2),
                TileType::Energy => Color::rgb(0.0, 1.0, 0.0),
                TileType::Mineral => Color::rgb(0.7, 0.5, 0.0),
                TileType::Scientific => Color::rgb(0.0, 0.5, 1.0),
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(TILE_SIZE - 1.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
                    ..Default::default()
                },
                Tile { tile_type },
            ));
        }
    }
}
