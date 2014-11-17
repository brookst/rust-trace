//! Shapes to be used in the scene.
use point::Point;

/// Trait for scene objects
pub trait Shape<T: Float> {
    /// Intersection routine, returns distance along ray
    fn intersect(&self, ray: Point<T>) -> Option<T>;
}

/// Export shapes
pub mod sphere;
