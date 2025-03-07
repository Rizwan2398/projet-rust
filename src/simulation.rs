use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::carte::Carte;
use crate::robot::Robot;

/// Gestion de la simulation des robots sur la carte
pub fn lancer_simulation() {
    let carte = Arc::new(Mutex::new(Carte::generer()));
    let robots = Arc::new(Mutex::new(vec![
        Robot::new(1, 5, 5),
        Robot::new(2, 10, 10),
    ]));

    // Boucle principale simulant 10 tours d'exploration
    for _ in 0..10 {
        {
            let mut carte_lock = carte.lock().unwrap();
            let mut robots_lock = robots.lock().unwrap();
            carte_lock.afficher(&robots_lock);

            for robot in robots_lock.iter_mut() {
                robot.deplacer();
                robot.explorer(&mut carte_lock);
            }
        }
        thread::sleep(Duration::from_millis(500));
    }

    println!("📌 Fin de l'exploration !");
}
