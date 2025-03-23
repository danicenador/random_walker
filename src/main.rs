mod constants;
mod draw_engine;
mod vec2;
mod rng_factory;
mod path;
mod walker;

use draw_engine::{window_conf, DrawEngine};
use macroquad::prelude;

#[macroquad::main(window_conf)]
async fn main() {
    let drawer: DrawEngine = DrawEngine;

    let mut walker = walker::Walker::new();

    loop {
        walker.step();
        walker.draw(&drawer);
        prelude::next_frame().await
    }
}
