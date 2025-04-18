use noise::{NoiseFn, Perlin};

pub struct NoiseGenerator {
    perlin: Perlin,
    seed: u32,
}

impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        NoiseGenerator { perlin, seed }
    }

    pub fn get_value(&self, x: f64, y: f64) -> f64 {
        self.perlin.get([x / 10.0, y / 10.0])
    }
}
