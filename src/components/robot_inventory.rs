use bevy::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_robot_inventory() {
        // Vérifie qu'un robot nouvellement créé a un inventaire vide
        let inventory = RobotInventory {
            energy: 0,
            minerals: 0,
            scientific_data: 0,
        };
        assert_eq!(inventory.energy, 0);
        assert_eq!(inventory.minerals, 0);
        assert_eq!(inventory.scientific_data, 0);
    }
}


#[derive(Component, Debug, Clone, Copy)]
pub struct RobotInventory {
    pub energy: u32,
    pub minerals: u32,
    pub scientific_data: u32,
}
