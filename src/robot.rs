use std::collections::HashMap;
use crate::carte::{Carte, Ressource, MAP_SIZE};
use rand::Rng;

#[derive(Debug)]
pub struct Robot {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub capacite: HashMap<Ressource, usize>,
}

impl Robot {
    /// Création d'un nouveau robot avec une position initiale
    pub fn new(id: usize, x: usize, y: usize) -> Self {
        Self {
            id,
            x,
            y,
            capacite: HashMap::new(),
        }
    }

    /// Déplacement aléatoire du robot sur la carte
    pub fn deplacer(&mut self) {
        let mut rng = rand::thread_rng();
        let dx: isize = rng.gen_range(-1..=1);
        let dy: isize = rng.gen_range(-1..=1);

        let new_x = (self.x as isize + dx).clamp(0, MAP_SIZE as isize - 1) as usize;
        let new_y = (self.y as isize + dy).clamp(0, MAP_SIZE as isize - 1) as usize;

        self.x = new_x;
        self.y = new_y;
    }

    /// Exploration et collecte de ressources
    pub fn explorer(&mut self, carte: &mut Carte) {
        if let Some(ressource) = carte.grille[self.x][self.y] {
            *self.capacite.entry(ressource).or_insert(0) += 1;
            carte.grille[self.x][self.y] = None;
            println!("🤖 Robot {} a collecté {:?} en ({}, {})", self.id, ressource, self.x, self.y);
        }
    }
}
