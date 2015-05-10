//! Shapes to be used in the scene.
use vector::Vector;
use num::traits::Float;

/// Trait for scene objects
pub trait Shape<T: Float> {
    /// Intersection routine, returns distance along ray
    fn intersect(&self, ray: Vector<T>) -> Option<T>;
}

/// Export shapes
pub mod sphere;
