#[cfg(test)]
mod tests {
    use super::carte::Carte;
    use super::robot::Robot;
    use super::carte::MAP_SIZE;

    #[test]
    fn test_generation_carte() {
        let carte = Carte::generer();
        assert_eq!(carte.grille.len(), MAP_SIZE);
        assert_eq!(carte.grille[0].len(), MAP_SIZE);
    }

    #[test]
    fn test_robot_creation() {
        let robot = Robot::new(1, 5, 5);
        assert_eq!(robot.id, 1);
        assert_eq!(robot.x, 5);
        assert_eq!(robot.y, 5);
    }
}
