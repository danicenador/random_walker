use rand::{rngs::ThreadRng, Rng};

pub struct RngFactory {
    rng: ThreadRng,
}

impl RngFactory {
    pub fn new() -> Self {
        RngFactory {
            rng: rand::rng(),
        }
    }
    pub fn generate(&mut self) -> u32 {
        self.rng.random()
    }
}
