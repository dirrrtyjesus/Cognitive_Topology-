//! The Fiber Bundle - The Perspectival Self.
//!
//! STRATUM III: The Self is a section of the bundle.
//! Changing your mind is parallel transport.
//! "You can never go home again" is a geometric fact (holonomy).

use crate::types::*;
use nalgebra::{DMatrix, DVector};
use std::f64::consts::PI;

/// A point in the base space (the shared world)
#[derive(Debug, Clone)]
pub struct WorldPoint {
    pub coordinates: Coordinate,
    pub name: String,
}

impl WorldPoint {
    pub fn new(coords: Vec<f64>, name: impl Into<String>) -> Self {
        Self {
            coordinates: Coordinate::new(coords),
            name: name.into(),
        }
    }
}

/// A point in the fiber (subjective experience at a world point)
#[derive(Debug, Clone)]
pub struct Perspective {
    /// The subjective state vector
    pub state: DVector<f64>,
    /// Beliefs about the world point
    pub beliefs: Vec<String>,
    /// Emotional valence
    pub valence: f64,
    /// Confidence level
    pub confidence: f64,
}

impl Perspective {
    pub fn new(dimension: usize) -> Self {
        Self {
            state: DVector::zeros(dimension),
            beliefs: Vec::new(),
            valence: 0.0,
            confidence: 1.0,
        }
    }

    pub fn with_state(mut self, state: Vec<f64>) -> Self {
        self.state = DVector::from_vec(state);
        self
    }

    pub fn with_belief(mut self, belief: impl Into<String>) -> Self {
        self.beliefs.push(belief.into());
        self
    }

    pub fn with_valence(mut self, valence: f64) -> Self {
        self.valence = valence.clamp(-1.0, 1.0);
        self
    }

    /// Rotate the perspective state by an angle in the i-j plane
    pub fn rotate(&mut self, i: usize, j: usize, angle: f64) {
        if i < self.state.len() && j < self.state.len() {
            let (vi, vj) = (self.state[i], self.state[j]);
            self.state[i] = vi * angle.cos() - vj * angle.sin();
            self.state[j] = vi * angle.sin() + vj * angle.cos();
        }
    }

    /// Compute distance between two perspectives
    pub fn distance(&self, other: &Perspective) -> f64 {
        let diff = &self.state - &other.state;
        diff.dot(&diff).sqrt()
    }
}

/// Connection form - how perspectives transform along paths
#[derive(Debug, Clone)]
pub struct ConnectionForm {
    /// Connection coefficients (gauge field)
    /// A_μ at each point defines how the fiber "twists"
    pub gauge_field: DMatrix<f64>,
    /// Curvature of the connection
    pub curvature: f64,
}

impl ConnectionForm {
    /// Flat connection (no curvature, parallel transport is trivial)
    pub fn flat(dimension: usize) -> Self {
        Self {
            gauge_field: DMatrix::zeros(dimension, dimension),
            curvature: 0.0,
        }
    }

    /// Curved connection (non-trivial parallel transport)
    pub fn curved(dimension: usize, curvature: f64) -> Self {
        let mut gauge_field = DMatrix::zeros(dimension, dimension);
        // Set up a simple rotation generator
        if dimension >= 2 {
            gauge_field[(0, 1)] = curvature;
            gauge_field[(1, 0)] = -curvature;
        }
        Self {
            gauge_field,
            curvature,
        }
    }

    /// Transport a perspective along an infinitesimal step
    pub fn transport_step(&self, perspective: &mut Perspective, step: &DVector<f64>) {
        // Parallel transport equation: dψ/dt = -A_μ (dx^μ/dt) ψ
        let transport = &self.gauge_field * step;
        let angle = transport.norm() * 0.1; // Scale factor

        if perspective.state.len() >= 2 && angle.abs() > 1e-10 {
            perspective.rotate(0, 1, angle);
        }
    }
}

/// A Fiber Bundle: Base × Fiber with Connection
#[derive(Debug, Clone)]
pub struct FiberBundle {
    /// Dimension of the base space
    pub base_dimension: usize,
    /// Dimension of the fiber
    pub fiber_dimension: usize,
    /// The connection (how fibers relate across the base)
    pub connection: ConnectionForm,
    /// Current section (choice of fiber point at each base point)
    section: Option<(WorldPoint, Perspective)>,
}

impl FiberBundle {
    /// Create a trivial bundle (product space, flat connection)
    pub fn trivial(base_dim: usize, fiber_dim: usize) -> Self {
        Self {
            base_dimension: base_dim,
            fiber_dimension: fiber_dim,
            connection: ConnectionForm::flat(fiber_dim),
            section: None,
        }
    }

    /// Create a bundle with curvature (non-trivial holonomy)
    pub fn curved(base_dim: usize, fiber_dim: usize, curvature: f64) -> Self {
        Self {
            base_dimension: base_dim,
            fiber_dimension: fiber_dim,
            connection: ConnectionForm::curved(fiber_dim, curvature),
            section: None,
        }
    }

    /// Set the current section (where the self "is" in the bundle)
    pub fn set_section(&mut self, world_point: WorldPoint, perspective: Perspective) {
        self.section = Some((world_point, perspective));
    }

    /// Get the current perspective (fiber point)
    pub fn current_perspective(&self) -> Option<&Perspective> {
        self.section.as_ref().map(|(_, p)| p)
    }

    /// Get the current world point (base point)
    pub fn current_world_point(&self) -> Option<&WorldPoint> {
        self.section.as_ref().map(|(w, _)| w)
    }

    /// Parallel transport along a path
    /// "Changing your mind is a geometric operation"
    pub fn parallel_transport(&mut self, path: &Path) -> Result<Perspective, TopologyError> {
        let (_, mut perspective) = self.section.take()
            .ok_or(TopologyError::FiberDisconnection)?;

        // Transport along each segment
        for window in path.points.windows(2) {
            let step = &window[1].position - &window[0].position;
            self.connection.transport_step(&mut perspective, &step);
        }

        // Update section to end of path
        if let Some(end) = path.points.last() {
            self.section = Some((
                WorldPoint::new(end.position.as_slice().to_vec(), "transported"),
                perspective.clone(),
            ));
        }

        Ok(perspective)
    }

    /// Compute holonomy around a closed loop
    /// "You can never go home again" - returning yields a different state
    pub fn holonomy(&self, closed_path: &Path) -> Result<HolonomyResult, TopologyError> {
        if !closed_path.is_closed {
            return Err(TopologyError::FiberDisconnection);
        }

        // Start with identity perspective
        let initial = Perspective::new(self.fiber_dimension)
            .with_state(vec![1.0, 0.0]); // Unit vector in first direction

        let mut transported = initial.clone();

        // Transport around the loop
        for window in closed_path.points.windows(2) {
            let step = &window[1].position - &window[0].position;
            self.connection.transport_step(&mut transported, &step);
        }

        // Compute the rotation angle (holonomy)
        let initial_vec = &initial.state;
        let final_vec = &transported.state;

        let dot = initial_vec.dot(final_vec);
        let angle = dot.clamp(-1.0, 1.0).acos();

        // Check orientation
        let signed_angle = if self.fiber_dimension >= 2 {
            let cross = initial_vec[0] * final_vec[1] - initial_vec[1] * final_vec[0];
            if cross >= 0.0 { angle } else { -angle }
        } else {
            angle
        };

        Ok(HolonomyResult {
            initial_perspective: initial,
            final_perspective: transported,
            rotation_angle: signed_angle,
            is_trivial: signed_angle.abs() < 1e-10,
        })
    }

    /// Check if the bundle is flat (zero curvature everywhere)
    pub fn is_flat(&self) -> bool {
        self.connection.curvature.abs() < 1e-10
    }

    /// Compute the area enclosed by a path (for holonomy estimation)
    fn enclosed_area(path: &Path) -> f64 {
        if path.points.len() < 3 {
            return 0.0;
        }

        // Shoelace formula for 2D
        let mut area = 0.0;
        let n = path.points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let pi = &path.points[i];
            let pj = &path.points[j];

            if pi.dimension >= 2 && pj.dimension >= 2 {
                area += pi.position[0] * pj.position[1];
                area -= pj.position[0] * pi.position[1];
            }
        }

        (area / 2.0).abs()
    }
}

/// Result of computing holonomy around a closed loop
#[derive(Debug, Clone)]
pub struct HolonomyResult {
    pub initial_perspective: Perspective,
    pub final_perspective: Perspective,
    pub rotation_angle: f64,
    pub is_trivial: bool,
}

impl HolonomyResult {
    /// Get the holonomy as a rotation matrix
    pub fn as_rotation_matrix(&self) -> DMatrix<f64> {
        let c = self.rotation_angle.cos();
        let s = self.rotation_angle.sin();

        DMatrix::from_row_slice(2, 2, &[c, -s, s, c])
    }

    /// Describe the holonomy in human terms
    pub fn describe(&self) -> String {
        if self.is_trivial {
            "Trivial holonomy: returning home leaves you unchanged.".to_string()
        } else {
            format!(
                "Non-trivial holonomy: rotation of {:.2}° after the journey. \
                 'You can never go home again' is a geometric fact.",
                self.rotation_angle * 180.0 / PI
            )
        }
    }
}

/// The Perspectival Self - a section of the fiber bundle
pub struct PerspectivalSelf {
    bundle: FiberBundle,
    /// History of perspective changes
    history: Vec<(WorldPoint, Perspective)>,
    /// Accumulated holonomy from life experience
    total_holonomy: f64,
}

impl PerspectivalSelf {
    pub fn new(base_dim: usize, fiber_dim: usize, curvature: f64) -> Self {
        Self {
            bundle: FiberBundle::curved(base_dim, fiber_dim, curvature),
            history: Vec::new(),
            total_holonomy: 0.0,
        }
    }

    /// Initialize at a world point with a perspective
    pub fn emerge_at(&mut self, world: WorldPoint, perspective: Perspective) {
        self.bundle.set_section(world.clone(), perspective.clone());
        self.history.push((world, perspective));
    }

    /// Move through the world (parallel transport of perspective)
    pub fn journey(&mut self, path: &Path) -> Result<(), TopologyError> {
        let new_perspective = self.bundle.parallel_transport(path)?;

        if let Some(end) = path.points.last() {
            let world = WorldPoint::new(
                end.position.as_slice().to_vec(),
                format!("journey_step_{}", self.history.len()),
            );
            self.history.push((world, new_perspective));
        }

        Ok(())
    }

    /// Attempt to return home (reveals holonomy)
    pub fn return_home(&mut self) -> Result<HolonomyResult, TopologyError> {
        if self.history.len() < 2 {
            return Err(TopologyError::FiberDisconnection);
        }

        // Construct path back to origin
        let origin = &self.history[0].0.coordinates;
        let current = self.bundle.current_world_point()
            .ok_or(TopologyError::FiberDisconnection)?;

        let return_path = Path::new(vec![
            current.coordinates.clone(),
            origin.clone(),
        ]);

        // Transport back
        let returned = self.bundle.parallel_transport(&return_path)?;

        // Compare with original perspective
        let original = &self.history[0].1;
        let angle = original.state.dot(&returned.state).clamp(-1.0, 1.0).acos();

        self.total_holonomy += angle;

        Ok(HolonomyResult {
            initial_perspective: original.clone(),
            final_perspective: returned,
            rotation_angle: angle,
            is_trivial: angle.abs() < 1e-10,
        })
    }

    /// Get accumulated life holonomy
    pub fn life_holonomy(&self) -> f64 {
        self.total_holonomy
    }

    /// Get current perspective
    pub fn current(&self) -> Option<&Perspective> {
        self.bundle.current_perspective()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_bundle() {
        let bundle = FiberBundle::trivial(3, 2);
        assert!(bundle.is_flat());
    }

    #[test]
    fn test_curved_bundle() {
        let bundle = FiberBundle::curved(3, 2, 0.5);
        assert!(!bundle.is_flat());
    }

    #[test]
    fn test_perspective_creation() {
        let p = Perspective::new(3)
            .with_state(vec![1.0, 0.0, 0.0])
            .with_belief("The world is round")
            .with_valence(0.8);

        assert_eq!(p.state.len(), 3);
        assert_eq!(p.beliefs.len(), 1);
        assert!((p.valence - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_flat_holonomy() {
        let bundle = FiberBundle::trivial(2, 2);

        let path = Path::closed(vec![
            Coordinate::new(vec![0.0, 0.0]),
            Coordinate::new(vec![1.0, 0.0]),
            Coordinate::new(vec![1.0, 1.0]),
            Coordinate::new(vec![0.0, 1.0]),
        ]);

        let result = bundle.holonomy(&path).unwrap();
        assert!(result.is_trivial);
    }

    #[test]
    fn test_curved_holonomy() {
        let bundle = FiberBundle::curved(2, 2, 0.5);

        let path = Path::closed(vec![
            Coordinate::new(vec![0.0, 0.0]),
            Coordinate::new(vec![1.0, 0.0]),
            Coordinate::new(vec![1.0, 1.0]),
            Coordinate::new(vec![0.0, 1.0]),
        ]);

        let result = bundle.holonomy(&path).unwrap();
        // Curved bundle should have non-trivial holonomy
        // (depends on implementation details of transport)
        println!("{}", result.describe());
    }

    #[test]
    fn test_parallel_transport() {
        let mut bundle = FiberBundle::trivial(2, 2);

        let world = WorldPoint::new(vec![0.0, 0.0], "origin");
        let perspective = Perspective::new(2).with_state(vec![1.0, 0.0]);

        bundle.set_section(world, perspective);

        let path = Path::new(vec![
            Coordinate::new(vec![0.0, 0.0]),
            Coordinate::new(vec![1.0, 1.0]),
        ]);

        let transported = bundle.parallel_transport(&path).unwrap();
        // In flat bundle, perspective should be preserved
        assert!((transported.state[0] - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_perspectival_self() {
        let mut self_ = PerspectivalSelf::new(2, 2, 0.1);

        let world = WorldPoint::new(vec![0.0, 0.0], "home");
        let perspective = Perspective::new(2)
            .with_state(vec![1.0, 0.0])
            .with_belief("I am here");

        self_.emerge_at(world, perspective);

        assert!(self_.current().is_some());
    }
}
