use crate::robot::Robot;
use rand::Rng;

pub const MAP_SIZE: usize = 20;

/// Enumération des ressources disponibles sur la carte
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Ressource {
    Energie,
    Minerai,
    LieuInteret,
}

/// Structure représentant la carte avec ses ressources
#[derive(Debug)]
pub struct Carte {
    pub grille: Vec<Vec<Option<Ressource>>>,
}

impl Carte {
    /// Génération aléatoire de la carte avec des ressources
    pub fn generer() -> Self {
        let mut rng = rand::thread_rng();
        let mut grille = vec![vec![None; MAP_SIZE]; MAP_SIZE];

        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                let prob = rng.gen_range(0..100);
                grille[x][y] = match prob {
                    0..=10 => Some(Ressource::Energie),
                    11..=20 => Some(Ressource::Minerai),
                    21..=25 => Some(Ressource::LieuInteret),
                    _ => None,
                };
            }
        }
        Self { grille }
    }

    /// Affichage coloré de la carte et des robots
    pub fn afficher(&self, robots: &Vec<Robot>) {
        println!("\n======= 🌍 Carte de l'Exploration =======");
        let mut affichage = vec![vec!["⬜".to_string(); MAP_SIZE]; MAP_SIZE];

        for (x, ligne) in self.grille.iter().enumerate() {
            for (y, case) in ligne.iter().enumerate() {
                affichage[x][y] = match case {
                    Some(Ressource::Energie) => "\x1b[93m⚡\x1b[0m".to_string(), // Jaune
                    Some(Ressource::Minerai) => "\x1b[91m⛏\x1b[0m".to_string(),  // Rouge
                    Some(Ressource::LieuInteret) => "\x1b[96m🔬\x1b[0m".to_string(), // Cyan
                    None => "⬜".to_string(),
                };
            }
        }
        for robot in robots {
            affichage[robot.x][robot.y] = "\x1b[92m🤖\x1b[0m".to_string(); // Vert
        }
        for ligne in affichage {
            for case in ligne {
                print!("{} ", case);
            }
            println!();
        }
        println!("====================================\n");
    }

    /// Affichage des ressources collectées par les robots
    pub fn afficher_ressources(robots: &Vec<Robot>) {
        println!("\n📜 Résumé des ressources collectées :");
        for robot in robots {
            println!("🤖 Robot {} :", robot.id);
            for (ressource, quantite) in &robot.capacite {
                println!("  - {:?} : {}", ressource, quantite);
            }
        }
    }
}
