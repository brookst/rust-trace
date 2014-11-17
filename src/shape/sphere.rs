//! Spherical object.
use std::fmt::Show;
use std::num::zero;

use point::Point;
use shape::Shape;

/// A Spherical object
#[deriving(Show)]
pub struct Sphere<T: Float> {
    /// `Sphere` radius
    pub r: T,
    /// Origin `Point` of `Sphere`
    pub center: Point<T>,
}

impl<T: Float + Show> Sphere<T> {
    pub fn new(r: T, center: Point<T>) -> Sphere<T> {
        Sphere{r: r, center: center}
    }
}

impl<T: Float + Show> Shape<T> for Sphere<T> {
    /// Compute intersection of `ray` with the `Sphere`.
    /// Returns `None` if they do not intersect or `Some(T)` with `T`
    /// indicating the distance along `ray` at which an intersection occurs.
    fn intersect(&self, ray: Point<T>) -> Option<T> {
        let b = -(ray.x * self.center.x + ray.y * self.center.y + ray.z * self.center.z);
        let det = b * b + self.r * self.r - self.center.mag2();
        if ray.x == zero() && ray.y == zero() {
            println!("ray: {}", ray);
            println!("center: {}", self.center);
            println!("radius: {}", self.r);
            println!("det: {}", det);
        }
        if det < zero() {
            None
        } else {
            Some(b - det)
        }
    }
}

#[test]
fn new_init() {
    let s = Sphere::new(1.0f32, Point::new(0.0, 0.0, 0.0));
    assert_eq!(s.r, 1.0f32);
}

#[test]
fn does_intersect() {
    let s = Sphere::new(1.0f32, Point::new(0.0, 0.0, 3.0));
    let ray = Point::new(0.0f32, 0.0, 1.0);
    s.intersect(ray).unwrap();
}

#[test]
fn doesnt_intersect() {
    let s = Sphere::new(1.0f32, Point::new(0.0, 0.0, 3.0));
    let ray = Point::new(0.0f32, 1.0, 0.0);
    assert!(s.intersect(ray).is_none());
}
