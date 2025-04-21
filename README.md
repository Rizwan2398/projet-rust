# Essaim Robots

Simulation d'un essaim de robots (explorateurs, mineurs, scientifiques) sous Rust et Bevy, collectant automatiquement des ressources et les ramenant à une station centrale.

---

## Table des matières

- [Présentation](#présentation)
- [Fonctionnalités Principales](#fonctionnalités-principales)
- [Architecture et Fonctionnel](#architecture-et-fonctionnel)
  - [Entités et Composants](#entités-et-composants)
  - [Systèmes](#systèmes)
- [Installation & Exécution](#installation--exécution)
- [Structure du Projet](#structure-du-projet)
- [Tests](#tests)
- [Contribuer](#contribuer)
- [Licence](#licence)

---

## Présentation

Ce projet propose une simulation en 2D d'un essaim de robots évoluant sur une carte générée procéduralement via du bruit de Perlin. Chaque robot dispose d'un inventaire, se déplace de façon autonome, collecte des ressources et retourne à la station centrale pour déposer son butin.

## Fonctionnalités Principales

- **Génération de carte** : grille 50×30, tuiles vides, obstacles, ressources (énergie, minerai, données scientifiques).
- **Trois types de robots** :
  - Explorateurs (parcourent librement)
  - Mineurs (priorisent la collecte de minerai)
  - Scientifiques (priorisent la collecte de données scientifiques)
- **Collecte & Dépôt automatique** : seuil de 5 unités total déclenche le retour à la station (position 0,0).
- **Systèmes ECS** : découplage fort via Bevy (entités, composants, systèmes).
- **Affichage temps réel** : couleurs & sprites pour visualiser robots et tuiles.
- **Tests** : validation des composants, logique de changement d'état, intégration du dépôt de ressources.

## Architecture et Fonctionnel

### Entités et Composants

| Composant          | Description                                                            |
| ------------------ | ---------------------------------------------------------------------- |
| `Position {x,y}`   | Coordonnées entières sur la grille                                     |
| `RobotInventory`   | Stock : `energy`, `minerals`, `scientific_data`                        |
| `RobotState`       | État courant : `Exploring` ou `ReturningToStation`                     |
| `RobotType`        | Rôle : `Explorer`, `Miner`, `Scientist`                                |
| `Tile {tile_type}` | Type de tuile : `Empty`, `Obstacle`, `Energy`, `Mineral`, `Scientific` |
| `Station`          | Agrégat des ressources déposées par les robots                         |

### Systèmes

1. **spawn_map** (Startup)
   - Crée la grille de tuiles en appliquant un bruit de Perlin pour décider si c'est un obstacle ou une ressource.
2. **spawn_robots** (Startup)
   - Génère 10 robots à des positions aléatoires avec types et inventaires vides.
3. **robot_movement_system** (Update)
   - Mouvement aléatoire (2% de chance chaque frame) si en exploration, ou direction de retour vers (0,0) sinon.
   - Vérifie collisions avec obstacles avant de déplacer.
4. **resource_collection_system** (Update)
   - Si un robot partage la position d'une tuile ressource, incrémente son inventaire et détruit la tuile.
5. **robot_behavior_system** (Update)
   - Si inventaire total ≥ 5 en exploration, passe l’état en `ReturningToStation`.
6. **delivery_system** (Update)
   - Quand un robot revenu à (0,0) et en état `ReturningToStation` : transfère son inventaire à la `Station`, remet son inventaire à zéro et revient à l’état `Exploring`.
7. **station_report_system** (Update)
   - Toutes les secondes, affiche en console le cumul des ressources de la station.

## Installation & Exécution

1. **Cloner le dépôt**

   ```bash
   git clone https://github.com/Rizwan2398/projet-rust
   cd projet-rust
   ```

2. **Compiler & exécuter**
   ```bash
   cargo build --release
   cargo run --release
   ```

Une fenêtre s’ouvrira avec la simulation. Fermez-la pour arrêter l’application.

## Structure du Projet

```text
src/
├── components/          # Définition des composants ECS
│   ├── position.rs      # Position(x,y)
│   ├── robot_inventory.rs
│   ├── robot_state.rs
│   └── robot_type.rs
├── map.rs               # Génération procédurale de la carte
├── station.rs           # Station centrale et plugin associé
├── systems/             # Implémentation des systèmes Bevy
│   ├── spawn_map.rs
│   ├── spawn_robots.rs
│   ├── movement.rs
│   ├── resource_collection.rs
│   ├── robot_behavior.rs
│   ├── delivery.rs
│   └── station_report.rs
├── utils/               # Outils complémentaires
│   └── noise.rs         # Wrapper Perlin noise
└── main.rs              # Point d’entrée : configuration de l’App

Cargo.toml               # Dépendances & métadonnées
README.md                # Documentation (ce fichier)
```

## Tests

Pour exécuter tous les tests unitaires et d’intégration :

```bash
cargo test
```

## Contribuer

Les contributions sont les bienvenues :


