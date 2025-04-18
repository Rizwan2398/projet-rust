use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct RobotInventory {
    pub energy: u32,
    pub minerals: u32,
    pub scientific_data: u32,
}
