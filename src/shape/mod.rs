//! Shapes to be used in the scene.
use point::Point;

pub trait Shape<T: Float> {
    fn intersect(&self, Point<T>) -> Option<T>;
}

pub mod sphere;
