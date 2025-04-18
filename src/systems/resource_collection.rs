use bevy::prelude::*;

use crate::components::position::Position;
use crate::components::robot_inventory::RobotInventory;
use crate::map::{Tile, TileType, TILE_SIZE};

pub fn resource_collection_system(
    mut commands: Commands,
    mut robot_query: Query<(&Position, &mut RobotInventory)>,
    mut tile_query: Query<(Entity, &Tile, &Transform)>,
) {
    for (robot_pos, mut inventory) in robot_query.iter_mut() {
        for (tile_entity, tile, transform) in tile_query.iter_mut() {
            let tile_x = (transform.translation.x / TILE_SIZE).round() as i32;
            let tile_y = (transform.translation.y / TILE_SIZE).round() as i32;

            if robot_pos.x == tile_x && robot_pos.y == tile_y {
                match tile.tile_type {
                    TileType::Energy => {
                        inventory.energy += 1;
                        commands.entity(tile_entity).despawn();
                    }
                    TileType::Mineral => {
                        inventory.minerals += 1;
                        commands.entity(tile_entity).despawn();
                    }
                    TileType::Scientific => {
                        inventory.scientific_data += 1;
                        commands.entity(tile_entity).despawn();
                    }
                    _ => {}
                }
            }
        }
    }
}
