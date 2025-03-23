mod constants;
mod draw_engine;
mod vec2;

use draw_engine::{window_conf, DrawEngine};
// use macroquad::prelude::next_frame;
use macroquad::{prelude, time};

#[macroquad::main(window_conf)]
async fn main() {
    let drawer: DrawEngine = draw_engine::DrawEngine::new();

    let point_a = vec2::Vec2::new(-1.0, 11.0);
    let point_a_2 = vec2::Vec2::new(11.0, -1.0);
    let point_b = vec2::Vec2::new(9.0, -1.0);
    let point_b_2 = vec2::Vec2::new(21.0, 11.0);
    let point_c = vec2::Vec2::new(21.0, 9.0);
    let point_c_2 = vec2::Vec2::new(9.0, 21.0);
    let point_d = vec2::Vec2::new(-1.0, 9.0);
    let point_d_2 = vec2::Vec2::new(11.0, 21.0);

    let mut colision_vector: Vec<colision_barrier::ColisionBarrier> = Vec::new();

    colision_vector.push(colision_barrier::ColisionBarrier::new(point_a, point_a_2));
    colision_vector.push(colision_barrier::ColisionBarrier::new(point_b, point_b_2));
    colision_vector.push(colision_barrier::ColisionBarrier::new(point_c, point_c_2));
    colision_vector.push(colision_barrier::ColisionBarrier::new(point_d, point_d_2));

    let position = vec2::Vec2::new(11.0, 18.0);
    let radius: f32 = 0.35;
    let velocity = vec2::Vec2::new(4.0, 0.0);
    let acceleration = vec2::Vec2::new(0.0, 0.0);
    let mass: f32 = 1.0;
    let mut ball = ball::Ball::new(position, radius, velocity, acceleration, mass);

    loop {
        drawer.print_fps();
        drawer.print_kinetic_energy(&ball);
        drawer.print_potential_energy(&ball);
        drawer.print_total_energy(&ball);
        drawer.draw_axis();
        drawer.draw_line(&point_a, &point_a_2, prelude::WHITE);
        drawer.draw_line(&point_b, &point_b_2, prelude::WHITE);
        drawer.draw_line(&point_c, &point_c_2, prelude::WHITE);
        drawer.draw_line(&point_d, &point_d_2, prelude::WHITE);
        drawer.draw_trail(&ball.trail, prelude::BLUE);
        drawer.draw_ball(&ball, prelude::DARKGREEN);

        ball.update_state(time::get_frame_time(), &colision_vector);
        prelude::next_frame().await
    }
}
