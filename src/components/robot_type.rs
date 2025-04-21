use bevy::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_robot_type() {
        let robot = RobotType::Explorer;
        assert_eq!(robot, RobotType::Explorer);
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RobotType {
    Explorer,
    Miner,
    Scientist,
}
