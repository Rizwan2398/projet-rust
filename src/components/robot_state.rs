use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RobotState {
    Exploring,
    ReturningToStation,
}
