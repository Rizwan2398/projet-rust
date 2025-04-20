use noise::{NoiseFn, Perlin};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_generator_returns_value() {
        // Ce test vérifie que NoiseGenerator retourne une valeur
        let noise = NoiseGenerator::new(42);
        let value = noise.get_value(10.0, 20.0);

        // On s'assure simplement que la valeur est dans une plage réaliste
        assert!(value >= -1.0 && value <= 1.0);
    }
}


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
