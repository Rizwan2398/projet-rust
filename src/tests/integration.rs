use essaim_robots::components::{robot_inventory::RobotInventory, robot_state::RobotState};

#[test]
fn robot_delivers_resources() {
    // Ce test simule un robot livrant ses ressources à la station

    let mut station_energy = 0;
    let mut robot_inventory = RobotInventory {
        energy: 5,
        minerals: 0,
        scientific_data: 0,
    };

    // Simuler livraison
    station_energy += robot_inventory.energy;
    robot_inventory.energy = 0;

    assert_eq!(station_energy, 5);
    assert_eq!(robot_inventory.energy, 0);
}
