use bevy::prelude::*;
use crate::station::Station;

pub fn station_report_system(
    time: Res<Time>,
    mut timer: Local<f32>,
    query: Query<&Station>,
) {
    *timer += time.delta_seconds();

    if *timer >= 1.0 {
        *timer = 0.0;

        if let Ok(station) = query.get_single() {
            println!(
                "Station Resources - Energy: {}, Minerals: {}, Scientific Data: {}",
                station.energy, station.minerals, station.scientific_data
            );
        }
    }
}
