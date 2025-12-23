//! Core types for the Cognitive Topology system.
//!
//! "Geometry is the hospitality of space toward form."

use nalgebra::{DMatrix, DVector};
use std::collections::HashMap;
use thiserror::Error;

/// The golden ratio - coherence signature τₖ
pub const PHI: f64 = 1.618033988749895;

/// Coordinate in the cognitive manifold
#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub position: DVector<f64>,
    pub dimension: usize,
}

impl Coordinate {
    pub fn new(components: Vec<f64>) -> Self {
        let dimension = components.len();
        Self {
            position: DVector::from_vec(components),
            dimension,
        }
    }

    pub fn origin(dimension: usize) -> Self {
        Self {
            position: DVector::zeros(dimension),
            dimension,
        }
    }

    pub fn distance_squared(&self, other: &Coordinate) -> f64 {
        let diff = &self.position - &other.position;
        diff.dot(&diff)
    }
}

/// A thought collapsed from topological superposition
#[derive(Debug, Clone)]
pub struct Thought {
    pub content: String,
    pub coherence: f64,
    pub origin: Coordinate,
    pub geodesic_length: f64,
}

impl Thought {
    pub fn new(content: impl Into<String>, origin: Coordinate) -> Self {
        Self {
            content: content.into(),
            coherence: PHI,
            origin,
            geodesic_length: 0.0,
        }
    }

    pub fn with_coherence(mut self, coherence: f64) -> Self {
        self.coherence = coherence;
        self
    }
}

/// Errors in cognitive topology
#[derive(Debug, Error)]
pub enum TopologyError {
    #[error("Non-contractible loop detected: H₁ ≠ 0 (paradox at depth {depth})")]
    NonContractibleLoop { depth: usize },

    #[error("Homological hole at dimension {dimension}: the unthought thought")]
    HomologicalHole { dimension: usize },

    #[error("Curvature singularity: cognitive load exceeded at {location:?}")]
    CurvatureSingularity { location: Coordinate },

    #[error("Fiber disconnection: perspective lost during transport")]
    FiberDisconnection,

    #[error("Kernel strain exceeded: τₖ = {strain} > threshold")]
    KernelStrain { strain: f64 },

    #[error("Dimension mismatch: expected {expected}, found {found}")]
    DimensionMismatch { expected: usize, found: usize },
}

/// The attention tensor - defines distance in cognitive space
#[derive(Debug, Clone)]
pub struct AttentionTensor {
    /// The metric tensor g_ij (symmetric, positive-definite)
    pub metric: DMatrix<f64>,
    /// Focus intensity (scales the metric)
    pub intensity: f64,
}

impl AttentionTensor {
    /// Create a flat (Euclidean) attention metric
    pub fn flat(dimension: usize) -> Self {
        Self {
            metric: DMatrix::identity(dimension, dimension),
            intensity: 1.0,
        }
    }

    /// Create a focused attention metric (curved toward a point)
    pub fn focused(dimension: usize, focus_weights: &[f64]) -> Self {
        let mut metric = DMatrix::zeros(dimension, dimension);
        for (i, &w) in focus_weights.iter().enumerate().take(dimension) {
            metric[(i, i)] = w.max(0.01); // Ensure positive-definite
        }
        Self {
            metric,
            intensity: 1.0,
        }
    }

    /// Compute the inner product under this metric
    pub fn inner_product(&self, v: &DVector<f64>, w: &DVector<f64>) -> f64 {
        let gv = &self.metric * v;
        w.dot(&gv) * self.intensity
    }

    /// Compute geodesic distance between two points
    pub fn geodesic_distance(&self, a: &Coordinate, b: &Coordinate) -> f64 {
        let diff = &b.position - &a.position;
        self.inner_product(&diff, &diff).sqrt()
    }
}

/// Cognitive load as Gaussian curvature
#[derive(Debug, Clone)]
pub struct CognitiveLoad {
    /// Curvature values at sampled points
    pub curvature_field: HashMap<String, f64>,
    /// Default curvature (flat = 0)
    pub default_curvature: f64,
}

impl CognitiveLoad {
    pub fn flat() -> Self {
        Self {
            curvature_field: HashMap::new(),
            default_curvature: 0.0,
        }
    }

    pub fn with_curvature(mut self, region: impl Into<String>, curvature: f64) -> Self {
        self.curvature_field.insert(region.into(), curvature);
        self
    }

    /// Positive curvature = converging thoughts (spherical)
    /// Negative curvature = diverging thoughts (hyperbolic)
    /// Zero curvature = parallel thoughts (flat)
    pub fn at(&self, _coord: &Coordinate) -> f64 {
        // In full implementation, would interpolate based on coordinate
        self.default_curvature
    }

    /// Check if curvature is singular (infinite cognitive load)
    pub fn is_singular(&self, coord: &Coordinate) -> bool {
        self.at(coord).abs() > 1e6
    }
}

/// A path through the cognitive manifold
#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<Coordinate>,
    pub is_closed: bool,
}

impl Path {
    pub fn new(points: Vec<Coordinate>) -> Self {
        let is_closed = points.len() >= 2
            && points.first().unwrap().distance_squared(points.last().unwrap()) < 1e-10;
        Self { points, is_closed }
    }

    pub fn closed(mut points: Vec<Coordinate>) -> Self {
        if let Some(first) = points.first().cloned() {
            points.push(first);
        }
        Self { points, is_closed: true }
    }

    /// Check if path contains a non-contractible loop
    pub fn contains_noncontractible_loop(&self) -> bool {
        // Simplified: closed paths that can't shrink to a point
        // In full implementation, would check against manifold topology
        self.is_closed && self.winding_number() != 0
    }

    /// Compute winding number (how many times path wraps around)
    pub fn winding_number(&self) -> i32 {
        if self.points.len() < 3 {
            return 0;
        }
        // Simplified 2D winding number computation
        let mut angle_sum = 0.0;
        for i in 0..self.points.len() - 1 {
            let p1 = &self.points[i];
            let p2 = &self.points[i + 1];
            if p1.dimension >= 2 && p2.dimension >= 2 {
                let dx = p2.position[0] - p1.position[0];
                let dy = p2.position[1] - p1.position[1];
                angle_sum += dy.atan2(dx);
            }
        }
        (angle_sum / (2.0 * std::f64::consts::PI)).round() as i32
    }

    pub fn length(&self) -> f64 {
        self.points.windows(2)
            .map(|w| w[0].distance_squared(&w[1]).sqrt())
            .sum()
    }
}

/// A geodesic - shortest path between points on the manifold
#[derive(Debug, Clone)]
pub struct Geodesic {
    pub path: Path,
    pub length: f64,
    pub initial_velocity: DVector<f64>,
}

impl Geodesic {
    pub fn new(path: Path, metric: &AttentionTensor) -> Self {
        let length = path.points.windows(2)
            .map(|w| metric.geodesic_distance(&w[0], &w[1]))
            .sum();

        let initial_velocity = if path.points.len() >= 2 {
            &path.points[1].position - &path.points[0].position
        } else {
            DVector::zeros(3)
        };

        Self {
            path,
            length,
            initial_velocity,
        }
    }

    pub fn contains_loop(&self) -> bool {
        self.path.contains_noncontractible_loop()
    }

    /// Collapse the geodesic into a thought
    pub fn collapse(self, content: impl Into<String>) -> Thought {
        let origin = self.path.points.first()
            .cloned()
            .unwrap_or_else(|| Coordinate::origin(3));

        Thought {
            content: content.into(),
            coherence: PHI / (1.0 + self.length), // Shorter = more coherent
            origin,
            geodesic_length: self.length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_distance() {
        let a = Coordinate::new(vec![0.0, 0.0, 0.0]);
        let b = Coordinate::new(vec![1.0, 0.0, 0.0]);
        assert!((a.distance_squared(&b) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_attention_tensor_flat() {
        let metric = AttentionTensor::flat(3);
        let v = DVector::from_vec(vec![1.0, 0.0, 0.0]);
        assert!((metric.inner_product(&v, &v) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_closed_path() {
        let points = vec![
            Coordinate::new(vec![0.0, 0.0]),
            Coordinate::new(vec![1.0, 0.0]),
            Coordinate::new(vec![1.0, 1.0]),
        ];
        let path = Path::closed(points);
        assert!(path.is_closed);
    }
}
