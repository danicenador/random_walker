use random_walker::vec2::Vec2;

#[test]
fn add_vectors() {
    let a = Vec2::new(1, 2);
    let b = Vec2::new(3, 4);
    let c: Vec2 = a.add(&b);
    assert_eq!(c.x, 4);
    assert_eq!(c.y, 6);
}
