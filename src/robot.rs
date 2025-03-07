use crate::carte::{Carte, MAP_SIZE, Ressource};
use rand::Rng;
use std::collections::HashMap;

const CAPACITE_MAX: usize = 5; // Limite de stockage par ressource

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

    /// Déplacement stratégique vers une ressource ou aléatoire si aucune n'est proche
    pub fn deplacer(&mut self, carte: &Carte) {
        let mut rng = rand::thread_rng();
        let mut meilleur_deplacement = (self.x, self.y);
        let mut trouve_ressource = false;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let new_x = (self.x as isize + dx).clamp(0, MAP_SIZE as isize - 1) as usize;
                let new_y = (self.y as isize + dy).clamp(0, MAP_SIZE as isize - 1) as usize;

                if carte.grille[new_x][new_y].is_some() {
                    meilleur_deplacement = (new_x, new_y);
                    trouve_ressource = true;
                    break;
                }
            }
            if trouve_ressource {
                break;
            }
        }

        // Si aucune ressource proche, déplacement aléatoire
        if !trouve_ressource {
            let dx: isize = rng.gen_range(-1..=1);
            let dy: isize = rng.gen_range(-1..=1);
            meilleur_deplacement = (
                (self.x as isize + dx).clamp(0, MAP_SIZE as isize - 1) as usize,
                (self.y as isize + dy).clamp(0, MAP_SIZE as isize - 1) as usize,
            );
        }

        self.x = meilleur_deplacement.0;
        self.y = meilleur_deplacement.1;
    }

    /// Exploration et collecte de ressources si capacité non atteinte
    pub fn explorer(&mut self, carte: &mut Carte) {
        if let Some(ressource) = carte.grille[self.x][self.y] {
            let count = self.capacite.entry(ressource).or_insert(0);
            if *count < CAPACITE_MAX {
                *count += 1;
                carte.grille[self.x][self.y] = None;
                println!(
                    "🤖 Robot {} a collecté {:?} en ({}, {})",
                    self.id, ressource, self.x, self.y
                );
            }
        }
    }
}
