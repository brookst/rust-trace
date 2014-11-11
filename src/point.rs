#[deriving(Show)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point{x: x, y: y, z: z}
    }

    pub fn mag(&self) -> f32 {
        Float::sqrt(self.mag2())
    }

    pub fn mag2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

#[test]
#[allow(unused_variables)]
fn new_init() {
    let p = Point::new(1.0, 2.0, 3.0);
}

#[test]
fn null_mag() {
    let p = Point{x: 0.0, y: 0.0, z: 0.0};
    assert!(p.mag() == 0.0);
}

#[test]
fn single_mag() {
    let p = Point{x: 42.0, y: 0.0, z: 0.0};
    assert!(p.mag() == 42.0);
}

#[test]
fn double_mag() {
    let p = Point{x: 3.0, y: 4.0, z: 0.0};
    assert!(p.mag() == 5.0);
}

#[test]
fn triple_mag() {
    let p = Point{x: 2.0, y: 3.0, z: 6.0};
    assert!(p.mag() == 7.0);
}

#[test]
fn null_mag2() {
    let p = Point{x: 0.0, y: 0.0, z: 0.0};
    assert!(p.mag2() == 0.0);
}

#[test]
fn single_mag2() {
    let p = Point{x: 2.0, y: 0.0, z: 0.0};
    assert!(p.mag2() == 4.0);
}

#[test]
fn double_mag2() {
    let p = Point{x: 3.0, y: 4.0, z: 0.0};
    assert!(p.mag2() == 25.0);
}

#[test]
fn triple_mag2() {
    let p = Point{x: 2.0, y: 3.0, z: 6.0};
    assert!(p.mag2() == 49.0);
}
