use crate::vec2::Vec2;

pub struct Segment(pub Vec2, pub Vec2);

pub struct Path {
    pub segments: Vec<Segment>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, start: Vec2, end: Vec2) {
        self.segments.push(Segment(start, end));
    }
}
