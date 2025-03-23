use crate::vec2::Vec2;
use crate::rng_factory::RngFactory;
use crate::path::Path;

pub struct Walker {
    pub last_position: Vec2,
    rng_factory: RngFactory,
    pub path: Path,
}

impl Walker {
    pub fn new() -> Self {
        Self {
            last_position: Vec2::new(0.0, 0.0),
            rng_factory: RngFactory::new(),
            path: Path::new(),
        }
    }

    pub fn step(&mut self) {
        let random_input: u32 = self.rng_factory.generate();
        let step: Vec2 = get_step(random_input);

        let new_position: Vec2 = self.last_position.add(&step);
        self.path.add_segment(self.last_position, new_position);

        self.last_position = new_position;
    }

}

pub fn get_step(random_input: u32) -> Vec2 {
    match random_input % 4{
        0 => Vec2::new(1.0, 0.0),
        1 => Vec2::new(-1.0, 0.0),
        2 => Vec2::new(0.0, 1.0),
        3 => Vec2::new(0.0, -1.0),
        _ => Vec2::new(0.0, 0.0),
    }
}
