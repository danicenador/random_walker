use random_walker::vec2::Vec2;

#[test]
fn add_vectors() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let c: Vec2 = a.add(&b);
    assert_eq!(c.x, 4.0);
    assert_eq!(c.y, 6.0);
}
