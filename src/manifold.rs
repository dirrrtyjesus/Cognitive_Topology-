//! The Cognitive Manifold - Riemannian substrate of thought.
//!
//! STRATUM I: The field of potential thoughts, curved by cognitive load.
//! Attention collapses the topological superposition into bounded thought.

use crate::types::*;
use nalgebra::DVector;
use rand::Rng;

/// The Cognitive Manifold - a Riemannian space where thoughts exist
#[derive(Debug, Clone)]
pub struct CognitiveManifold {
    /// Degrees of freedom in a thought
    pub dimension: usize,
    /// Defines "distance" between concepts
    pub metric: AttentionTensor,
    /// Gaussian curvature field (difficulty/density)
    pub curvature: CognitiveLoad,
    /// Superposition states before observation
    superposition: Vec<PotentialThought>,
}

/// A thought in superposition - not yet collapsed by attention
#[derive(Debug, Clone)]
pub struct PotentialThought {
    pub amplitude: f64,
    pub phase: f64,
    pub location: Coordinate,
    pub content_seed: u64,
}

impl PotentialThought {
    pub fn new(location: Coordinate) -> Self {
        Self {
            amplitude: 1.0,
            phase: 0.0,
            location,
            content_seed: rand::thread_rng().gen(),
        }
    }

    pub fn probability(&self) -> f64 {
        self.amplitude * self.amplitude
    }
}

impl CognitiveManifold {
    /// Create a flat cognitive manifold (Euclidean thought-space)
    pub fn flat(dimension: usize) -> Self {
        Self {
            dimension,
            metric: AttentionTensor::flat(dimension),
            curvature: CognitiveLoad::flat(),
            superposition: Vec::new(),
        }
    }

    /// Create a spherical cognitive manifold (closed, finite thought-space)
    pub fn spherical(dimension: usize, radius: f64) -> Self {
        Self {
            dimension,
            metric: AttentionTensor::flat(dimension),
            curvature: CognitiveLoad::flat()
                .with_curvature("global", 1.0 / (radius * radius)),
            superposition: Vec::new(),
        }
    }

    /// Create a hyperbolic cognitive manifold (open, infinite thought-space)
    pub fn hyperbolic(dimension: usize, curvature: f64) -> Self {
        Self {
            dimension,
            metric: AttentionTensor::flat(dimension),
            curvature: CognitiveLoad::flat()
                .with_curvature("global", -curvature.abs()),
            superposition: Vec::new(),
        }
    }

    /// Add a potential thought to the superposition
    pub fn superpose(&mut self, location: Coordinate) -> &mut Self {
        self.superposition.push(PotentialThought::new(location));
        self
    }

    /// Apply attention to collapse superposition at a location
    pub fn observe(&self, location: &Coordinate) -> Result<Thought, TopologyError> {
        // Check for curvature singularity
        if self.curvature.is_singular(location) {
            return Err(TopologyError::CurvatureSingularity {
                location: location.clone(),
            });
        }

        // Find shortest geodesic to the observation point
        let geodesic = self.find_shortest_path(location)?;

        // Check for non-contractible loops (paradoxes)
        if geodesic.contains_loop() {
            return Err(TopologyError::NonContractibleLoop {
                depth: geodesic.path.winding_number().unsigned_abs() as usize,
            });
        }

        // Collapse the geodesic into a thought
        let length = geodesic.length;
        Ok(geodesic.collapse(format!(
            "Observed at {:?} with geodesic length {:.4}",
            location.position.as_slice(),
            length
        )))
    }

    /// Find the shortest path (geodesic) to a target location
    pub fn find_shortest_path(&self, target: &Coordinate) -> Result<Geodesic, TopologyError> {
        if target.dimension != self.dimension {
            return Err(TopologyError::DimensionMismatch {
                expected: self.dimension,
                found: target.dimension,
            });
        }

        // Start from origin (or nearest superposition state)
        let origin = if let Some(nearest) = self.nearest_superposition(target) {
            nearest.location.clone()
        } else {
            Coordinate::origin(self.dimension)
        };

        // Compute geodesic via gradient descent on the metric
        let path = self.compute_geodesic(&origin, target);

        Ok(Geodesic::new(path, &self.metric))
    }

    /// Find nearest potential thought in superposition
    fn nearest_superposition(&self, target: &Coordinate) -> Option<&PotentialThought> {
        self.superposition
            .iter()
            .min_by(|a, b| {
                let da = a.location.distance_squared(target);
                let db = b.location.distance_squared(target);
                da.partial_cmp(&db).unwrap()
            })
    }

    /// Compute geodesic path using discrete gradient descent
    fn compute_geodesic(&self, start: &Coordinate, end: &Coordinate) -> Path {
        const NUM_STEPS: usize = 50;
        let mut points = Vec::with_capacity(NUM_STEPS + 1);

        for i in 0..=NUM_STEPS {
            let t = i as f64 / NUM_STEPS as f64;
            // Linear interpolation (geodesic in flat space)
            // In curved space, would use exponential map
            let position = &start.position * (1.0 - t) + &end.position * t;

            // Apply curvature correction
            let coord = Coordinate {
                position: self.apply_curvature_correction(position, t),
                dimension: self.dimension,
            };
            points.push(coord);
        }

        Path::new(points)
    }

    /// Apply curvature effects to trajectory
    fn apply_curvature_correction(&self, mut position: DVector<f64>, t: f64) -> DVector<f64> {
        let k = self.curvature.default_curvature;

        if k.abs() > 1e-10 {
            // Spherical/hyperbolic correction
            // Geodesics curve toward/away from center based on sign of k
            let correction_factor = 1.0 + k * t * (1.0 - t);
            position *= correction_factor;
        }

        position
    }

    /// Parallel transport a vector along a path
    pub fn parallel_transport(
        &self,
        vector: DVector<f64>,
        path: &Path,
    ) -> Result<DVector<f64>, TopologyError> {
        if path.points.is_empty() {
            return Ok(vector);
        }

        let mut transported = vector;

        for window in path.points.windows(2) {
            let _from = &window[0];
            let _to = &window[1];

            // In curved space, parallel transport rotates the vector
            // based on the connection (Christoffel symbols)
            let k = self.curvature.default_curvature;

            if k.abs() > 1e-10 {
                // Simplified: rotation proportional to curvature and path length
                let angle = k * 0.01; // Small rotation per step
                if transported.len() >= 2 {
                    let (x, y) = (transported[0], transported[1]);
                    transported[0] = x * angle.cos() - y * angle.sin();
                    transported[1] = x * angle.sin() + y * angle.cos();
                }
            }
        }

        Ok(transported)
    }

    /// Compute holonomy - the rotation after parallel transport around a closed loop
    pub fn holonomy(&self, closed_path: &Path) -> Result<f64, TopologyError> {
        if !closed_path.is_closed {
            return Ok(0.0); // No holonomy for open paths
        }

        // Initial vector
        let initial = DVector::from_vec(vec![1.0, 0.0]);
        let transported = self.parallel_transport(initial.clone(), closed_path)?;

        // Compute angle between initial and transported
        let dot = initial.dot(&transported);
        let angle = dot.acos();

        Ok(angle)
    }

    /// Check if the manifold admits insight (shorter geodesic exists)
    pub fn admits_insight(&self, current_path: &Path, target: &Coordinate) -> Option<Geodesic> {
        let current_length: f64 = current_path.points.windows(2)
            .map(|w| self.metric.geodesic_distance(&w[0], &w[1]))
            .sum();

        if let Some(first) = current_path.points.first() {
            if let Ok(direct) = self.find_shortest_path(target) {
                if direct.length < current_length * 0.9 {
                    // Insight: found path at least 10% shorter
                    return Some(direct);
                }
            }
        }

        None
    }
}

/// Builder pattern for constructing cognitive manifolds
pub struct ManifoldBuilder {
    dimension: usize,
    metric: Option<AttentionTensor>,
    curvature: Option<CognitiveLoad>,
}

impl ManifoldBuilder {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            metric: None,
            curvature: None,
        }
    }

    pub fn with_metric(mut self, metric: AttentionTensor) -> Self {
        self.metric = Some(metric);
        self
    }

    pub fn with_curvature(mut self, curvature: CognitiveLoad) -> Self {
        self.curvature = Some(curvature);
        self
    }

    pub fn with_focus(mut self, focus_weights: &[f64]) -> Self {
        self.metric = Some(AttentionTensor::focused(self.dimension, focus_weights));
        self
    }

    pub fn build(self) -> CognitiveManifold {
        CognitiveManifold {
            dimension: self.dimension,
            metric: self.metric.unwrap_or_else(|| AttentionTensor::flat(self.dimension)),
            curvature: self.curvature.unwrap_or_else(CognitiveLoad::flat),
            superposition: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_manifold_observation() {
        let manifold = CognitiveManifold::flat(3);
        let target = Coordinate::new(vec![1.0, 1.0, 1.0]);
        let thought = manifold.observe(&target).unwrap();
        assert!(thought.coherence > 0.0);
    }

    #[test]
    fn test_spherical_manifold() {
        let manifold = CognitiveManifold::spherical(3, 1.0);
        // Spherical manifold has positive curvature stored in the field map
        assert!(manifold.curvature.curvature_field.get("global").copied().unwrap_or(0.0) > 0.0);
    }

    #[test]
    fn test_hyperbolic_manifold() {
        let manifold = CognitiveManifold::hyperbolic(3, 1.0);
        // Hyperbolic manifold has negative curvature stored in the field map
        assert!(manifold.curvature.curvature_field.get("global").copied().unwrap_or(0.0) < 0.0);
    }

    #[test]
    fn test_geodesic_length() {
        let manifold = CognitiveManifold::flat(3);
        let target = Coordinate::new(vec![3.0, 4.0, 0.0]);
        let geodesic = manifold.find_shortest_path(&target).unwrap();
        // Should be approximately 5.0 (3-4-5 triangle)
        assert!((geodesic.length - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_superposition() {
        let mut manifold = CognitiveManifold::flat(3);
        manifold
            .superpose(Coordinate::new(vec![1.0, 0.0, 0.0]))
            .superpose(Coordinate::new(vec![0.0, 1.0, 0.0]));

        assert_eq!(manifold.superposition.len(), 2);
    }

    #[test]
    fn test_holonomy_flat() {
        let manifold = CognitiveManifold::flat(2);
        let path = Path::closed(vec![
            Coordinate::new(vec![0.0, 0.0]),
            Coordinate::new(vec![1.0, 0.0]),
            Coordinate::new(vec![1.0, 1.0]),
            Coordinate::new(vec![0.0, 1.0]),
        ]);
        let holonomy = manifold.holonomy(&path).unwrap();
        // Flat space has zero holonomy
        assert!(holonomy.abs() < 1e-10);
    }

    #[test]
    fn test_builder_pattern() {
        let manifold = ManifoldBuilder::new(4)
            .with_focus(&[1.0, 2.0, 1.0, 0.5])
            .with_curvature(CognitiveLoad::flat().with_curvature("center", 0.5))
            .build();

        assert_eq!(manifold.dimension, 4);
    }
}
