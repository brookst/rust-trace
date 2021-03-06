//! Spherical object.
use std::fmt::Debug;
use num::traits::Float;
use num::traits::Zero;

use vector::Vector;
use shape::Shape;

/// A Spherical object
#[derive(Debug)]
pub struct Sphere<T: Float> {
    /// `Sphere` radius
    pub r: T,
    /// Origin `Vector` of `Sphere`
    pub center: Vector<T>,
    /// Color as RGB u8 values
    color: (u8, u8, u8),
}

impl<T: Float + Debug> Sphere<T> {
    pub fn new(r: T, center: Vector<T>, color: (u8, u8, u8)) -> Sphere<T> {
        Sphere{r, center, color}
    }
    pub fn new_white(r: T, center: Vector<T>) -> Sphere<T> {
        Sphere{r, center, color: (255, 255, 255)}
    }
    pub fn get_color(&self, _: Vector<T>) -> (u8, u8, u8) {
        self.color
    }
}

impl<T: Float + Debug> Shape<T> for Sphere<T> {
    /// Compute intersection of `ray` with the `Sphere`.
    /// Returns `None` if they do not intersect or `Some(T)` with `T`
    /// indicating the distance along `ray` at which an intersection occurs.
    fn intersect(&self, ray: Vector<T>) -> Option<T> {
        let b = ray.x * self.center.x + ray.y * self.center.y + ray.z * self.center.z;
        let det = b * b + self.r * self.r - self.center.mag2();
        if ray.x == Zero::zero() && ray.y == Zero::zero() {
            println!("ray: {:?}", ray);
            println!("center: {:?}", self.center);
            println!("radius: {:?}", self.r);
            println!("det: {:?}", det);
        }
        if det < Zero::zero() {
            None
        } else {
            Some(b - det.sqrt())
        }
    }
}

#[test]
fn new_init() {
    let s = Sphere::new_white(1.0f32, Vector::new(0.0, 0.0, 0.0));
    assert_eq!(s.r, 1.0f32);
}

#[test]
fn does_intersect() {
    let s = Sphere::new_white(1.0f32, Vector::new(0.0, 0.0, 3.0));
    let ray = Vector::new(0.0f32, 0.0, 1.0);
    assert!(s.intersect(ray).is_some());
}

#[test]
fn intersect_length_null() {
    let s = Sphere::new_white(0.0f32, Vector::new(0.0, 0.0, 3.0));
    let ray = Vector::new(0.0f32, 0.0, 1.0);
    assert_eq!(s.intersect(ray).unwrap(), 3.0);
}

#[test]
fn intersect_length() {
    let s = Sphere::new_white(1.0f32, Vector::new(0.0, 0.0, 3.0));
    let ray = Vector::new(0.0f32, 0.0, 1.0);
    assert_eq!(s.intersect(ray).unwrap(), 2.0);
}

#[test]
/// Origin inside `Sphere`!
fn intersect_length_behind() {
    let s = Sphere::new_white(4.0f32, Vector::new(0.0, 0.0, 3.0));
    let ray = Vector::new(0.0f32, 0.0, 1.0);
    assert_eq!(s.intersect(ray).unwrap(), -1.0);
}

#[test]
fn doesnt_intersect() {
    let s = Sphere::new_white(1.0f32, Vector::new(0.0, 0.0, 3.0));
    let ray = Vector::new(0.0f32, 1.0, 0.0);
    assert!(s.intersect(ray).is_none());
}
