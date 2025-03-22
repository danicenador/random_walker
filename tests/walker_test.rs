use random_walker::walker;
use random_walker::vec2::Vec2;
use random_walker::path;


#[test]
fn step_direction() {
    let value: u32 = 1;
    let step: Vec2 = walker::get_step(value);
    let value2: u32 = 5;
    let step2: Vec2 = walker::get_step(value2);
    assert_eq!(step.x, step2.x);
    assert_eq!(step.y, step2.y);
}


#[test]
fn one_step() {
    let mut walker = walker::Walker::new();
    walker.step();
    let walker_position: Vec2 = walker.last_position;
    let x_position_abs: i32 = walker_position.x.abs();
    let y_position_abs: i32 = walker_position.y.abs();
    assert_eq!(x_position_abs + y_position_abs, 1);
}

#[test]
fn path_segment_length() {
    let mut walker = walker::Walker::new();
    walker.step();
    walker.step();
    walker.step();

    let step_performed: &path::Segment = walker.path.segments.last().unwrap();
    let possition_1: Vec2 = step_performed.0;
    let possition_2: Vec2 = step_performed.1;
    let step: Vec2 = possition_1.sub(&possition_2);
    let x_position_abs: i32 = step.x.abs();
    let y_position_abs: i32 = step.y.abs();
    assert_eq!(x_position_abs + y_position_abs, 1);
}
