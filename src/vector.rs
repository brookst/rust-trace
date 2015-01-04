//! Basic 3D vector. Demonstrates testing in Rust.
use std::num::Float;

#[derive(Clone,Show)]
pub struct Vector<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Vector<T> {
        Vector{x: x, y: y, z: z}
    }

    pub fn mag(&self) -> T {
        Float::sqrt(self.mag2())
    }

    pub fn mag2(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

#[test]
fn new_init() {
    let p = Vector::new(1.0f32, 2.0, 3.0);
    assert!(p.y == 2.0f32);
}

#[test]
fn new_64() {
    let p = Vector::new(1.0f64, 2.0, 3.0);
    assert!(p.y == 2.0f64);
}

#[test]
fn null_mag() {
    let p = Vector{x: 0.0f32, y: 0.0, z: 0.0};
    assert!(p.mag() == 0.0);
}

#[test]
fn single_mag() {
    let p = Vector{x: 42.0f32, y: 0.0, z: 0.0};
    assert!(p.mag() == 42.0);
}

#[test]
fn double_mag() {
    let p = Vector{x: 3.0f32, y: 4.0, z: 0.0};
    assert!(p.mag() == 5.0);
}

#[test]
fn triple_mag() {
    let p = Vector{x: 2.0f32, y: 3.0, z: 6.0};
    assert!(p.mag() == 7.0);
}

#[test]
fn null_mag2() {
    let p = Vector{x: 0.0f32, y: 0.0, z: 0.0};
    assert!(p.mag2() == 0.0);
}

#[test]
fn single_mag2() {
    let p = Vector{x: 2.0f32, y: 0.0, z: 0.0};
    assert!(p.mag2() == 4.0);
}

#[test]
fn double_mag2() {
    let p = Vector{x: 3.0f32, y: 4.0, z: 0.0};
    assert!(p.mag2() == 25.0);
}

#[test]
fn triple_mag2() {
    let p = Vector{x: 2.0f32, y: 3.0, z: 6.0};
    assert!(p.mag2() == 49.0);
}
