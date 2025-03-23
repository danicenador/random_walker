use macroquad::{prelude, time, window};

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH, LINE_THICKNESS};
use crate::vec2::Vec2;


pub struct Color {
    pub r: f32,  // red, from 0.0 to 1.0
    pub g: f32,  // green, from 0.0 to 1.0
    pub b: f32,  // blue, from 0.0 to 1.0
    pub a: f32,  // alpha, from 0.0 to 1.0
}

pub trait DrawEngineTrait {
    fn draw_line(&self, start: &Vec2, end: &Vec2, color: Color);
    fn draw_circle(&self, center: &Vec2, radius: f32, color: Color);
    fn get_frame_time(&self) -> f32;
}

pub struct DrawEngine;

impl DrawEngineTrait for DrawEngine {
    fn draw_line(&self, start: &Vec2, end: &Vec2, color: Color) {
        prelude::draw_line(
            start.x + WINDOW_WIDTH as f32 / 2.0,
            start.y + WINDOW_HEIGHT as f32 / 2.0,
            end.x + WINDOW_WIDTH as f32 / 2.0,
            end.y + WINDOW_HEIGHT as f32 / 2.0,
            LINE_THICKNESS,
            prelude::Color::new(color.r, color.g, color.b, color.a),
        );
    }

    fn draw_circle(&self, center: &Vec2, radius: f32, color: Color) {
        prelude::draw_circle(
            center.x,
            center.y,
            radius,
            prelude::Color::new(color.r, color.g, color.b, color.a),
        );
    }

    fn get_frame_time(&self) -> f32 {
        time::get_frame_time()
    }

}

pub fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Random Walker".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}
