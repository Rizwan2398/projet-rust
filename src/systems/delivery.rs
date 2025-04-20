use bevy::prelude::*;

use crate::components::{
    position::Position, robot_inventory::RobotInventory, robot_state::RobotState,
};
use crate::station::Station;

pub fn delivery_system(
    mut robot_query: Query<(&mut RobotInventory, &mut RobotState, &Position)>,
    mut station_query: Query<&mut Station>,
) {
    for (mut inventory, mut state, pos) in robot_query.iter_mut() {
        if *state == RobotState::ReturningToStation && pos.x == 0 && pos.y == 0 {
            if let Ok(mut station) = station_query.get_single_mut() {
                station.energy += inventory.energy;
                station.minerals += inventory.minerals;
                station.scientific_data += inventory.scientific_data;

                inventory.energy = 0;
                inventory.minerals = 0;
                inventory.scientific_data = 0;

                *state = RobotState::Exploring;
            }
        }
    }
}
