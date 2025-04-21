use bevy::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::robot_inventory::RobotInventory;

    #[test]
    fn robot_changes_state_to_returning() {

        let inventory = RobotInventory {
            energy: 2,
            minerals: 2,
            scientific_data: 2,
        };

        let mut state = RobotState::Exploring;

        if let RobotState::Exploring = state {
            let total_resources = inventory.energy + inventory.minerals + inventory.scientific_data;
            if total_resources >= 5 {
                state = RobotState::ReturningToStation;
            }
        }

        assert_eq!(state, RobotState::ReturningToStation);
    }
}


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RobotState {
    Exploring,
    ReturningToStation,
}
