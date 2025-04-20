use bevy::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_position() {
        // Test de création d'une position
        let pos = Position { x: 5, y: 10 };
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 10);
    }
}


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
