use bevy::prelude::*;
use rand::Rng;

use crate::components::position::Position;
use crate::components::robot_type::RobotType;
use crate::map::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

pub fn robot_movement_system(
    mut query: Query<(&mut Transform, &mut Position, &RobotType)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (mut transform, mut position, robot_type) in query.iter_mut() {
        if rng.gen_bool(0.02) {
            let (dx, dy) = match rng.gen_range(0..4) {
                0 => (1, 0),  // Droite
                1 => (-1, 0), // Gauche
                2 => (0, 1),  // Haut
                _ => (0, -1), // Bas
            };

            let new_x = position.x + dx;
            let new_y = position.y + dy;

            if new_x >= 0 && new_x < MAP_WIDTH as i32 && new_y >= 0 && new_y < MAP_HEIGHT as i32 {
                position.x = new_x;
                position.y = new_y;

                // Met à jour la position graphique
                transform.translation.x = position.x as f32 * TILE_SIZE;
                transform.translation.y = position.y as f32 * TILE_SIZE;
            }
        }
    }
}
