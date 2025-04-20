use bevy::prelude::*;
use rand::Rng;

use crate::components::position::Position;
use crate::components::robot_state::RobotState;
use crate::components::robot_type::RobotType;
use crate::map::{Tile, TileType, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

pub fn robot_movement_system(
    mut param_set: ParamSet<(
        Query<(&mut Transform, &mut Position, &RobotType, &RobotState)>,
        Query<(&Tile, &Transform)>,
    )>,
    _time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    let tile_positions: Vec<((i32, i32), TileType)> = param_set
        .p1()
        .iter()
        .map(|(tile, transform)| {
            let x = (transform.translation.x / TILE_SIZE).round() as i32;
            let y = (transform.translation.y / TILE_SIZE).round() as i32;
            ((x, y), tile.tile_type)
        })
        .collect();

    for (mut transform, mut position, _robot_type, robot_state) in param_set.p0().iter_mut() {
        if rng.gen_bool(0.02) {
            let (dx, dy) = if *robot_state == RobotState::Exploring {
                match rng.gen_range(0..4) {
                    0 => (1, 0),
                    1 => (-1, 0),
                    2 => (0, 1),
                    _ => (0, -1),
                }
            } else {
                let dx = if position.x > 0 {
                    -1
                } else if position.x < 0 {
                    1
                } else {
                    0
                };
                let dy = if position.y > 0 {
                    -1
                } else if position.y < 0 {
                    1
                } else {
                    0
                };
                (dx, dy)
            };

            let new_x = position.x + dx;
            let new_y = position.y + dy;

            if new_x >= 0 && new_x < MAP_WIDTH as i32 && new_y >= 0 && new_y < MAP_HEIGHT as i32 {
                let mut is_obstacle = false;
                for ((tile_x, tile_y), tile_type) in tile_positions.iter() {
                    if *tile_x == new_x && *tile_y == new_y {
                        if *tile_type == TileType::Obstacle {
                            is_obstacle = true;
                        }
                        break;
                    }
                }

                if !is_obstacle {
                    position.x = new_x;
                    position.y = new_y;
                    transform.translation.x = position.x as f32 * TILE_SIZE;
                    transform.translation.y = position.y as f32 * TILE_SIZE;
                }
            }
        }
    }
}
