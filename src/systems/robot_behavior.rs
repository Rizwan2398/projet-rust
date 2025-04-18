use bevy::prelude::*;
use crate::components::{
    robot_inventory::RobotInventory,
    robot_state::RobotState,
};

pub fn robot_behavior_system(
    mut query: Query<(&mut RobotState, &RobotInventory)>,
) {
    for (mut state, inventory) in query.iter_mut() {
        if *state == RobotState::Exploring {
            let total_resources = inventory.energy + inventory.minerals + inventory.scientific_data;
            if total_resources >= 5 {
                *state = RobotState::ReturningToStation;
            }
        }
    }
}
