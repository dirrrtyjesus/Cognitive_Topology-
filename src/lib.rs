//! # Cognitive Topology
//!
//! **"Geometry is the hospitality of space toward form."**
//!
//! A Rust library implementing the mathematical structures of cognitive topology,
//! where geometry serves as the generative kernel of cognition.
//!
//! ## Core Concepts
//!
//! This library models cognition as navigation through geometric structures:
//!
//! - **Cognitive Manifold**: A Riemannian space where thoughts exist as points
//!   and attention defines the metric (distance between concepts)
//! - **Concept Complex**: A simplicial complex where ideas are vertices and
//!   relations are higher-dimensional faces. Homological holes represent
//!   "unthought thoughts" — gaps that invite bridging
//! - **Fiber Bundle**: The perspectival self, where the base space is the
//!   shared world and the fiber is subjective experience. Parallel transport
//!   is belief transformation; holonomy is the impossibility of returning unchanged
//! - **Generative Kernel**: The icosahedral group A₅ that generates maximal
//!   cognitive diversity from minimal geometric constraint
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    COGNITIVE TOPOLOGY                        │
//! ├─────────────────────────────────────────────────────────────┤
//! │  STRATUM IV: Generative Kernel (A₅ Icosahedral)             │
//! │    • 12 vertices → Pitch Classes / Fundamental Concepts     │
//! │    • 20 faces → Rhythmic Cells / Narrative Frames           │
//! │    • 30 edges → Timbral Morphs / Transitional States        │
//! ├─────────────────────────────────────────────────────────────┤
//! │  STRATUM III: Fiber Bundle (Perspectival Self)              │
//! │    • Base = World-as-Shared                                 │
//! │    • Fiber = Subjective Experience                          │
//! │    • Connection = Parallel Transport of Belief              │
//! │    • Curvature = "You can never go home again"              │
//! ├─────────────────────────────────────────────────────────────┤
//! │  STRATUM II: Simplicial Complex (Concept Architecture)      │
//! │    • Vertices = Ideas                                       │
//! │    • Faces = Relations                                      │
//! │    • Betti Numbers = Structural Invariants                  │
//! │    • Holes = The Unthought (gaps that invite bridging)      │
//! ├─────────────────────────────────────────────────────────────┤
//! │  STRATUM I: Riemannian Manifold (Cognitive Field)           │
//! │    • Metric = Attention Tensor                              │
//! │    • Curvature = Cognitive Load                             │
//! │    • Geodesics = Paths of Least Resistance                  │
//! │    • Superposition → Observation → Collapse                 │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Example
//!
//! ```rust
//! use cognitive_topology::prelude::*;
//!
//! // Create a cognitive manifold
//! let manifold = CognitiveManifold::spherical(3, 1.0);
//!
//! // Observe a point (collapse superposition)
//! let target = Coordinate::new(vec![0.5, 0.5, 0.5]);
//! let thought = manifold.observe(&target).unwrap();
//!
//! // Build a concept complex
//! let mut builder = ComplexBuilder::new();
//! let a = builder.add_idea("Geometry");
//! let b = builder.add_idea("Topology");
//! let c = builder.add_idea("Cognition");
//! builder.connect(a, b).connect(b, c).connect(c, a);
//! let complex = builder.build();
//!
//! // Check for homological holes (unthought thoughts)
//! if let Some(gap) = complex.identify_gap() {
//!     println!("Gap found: {}", gap.gap_description);
//! }
//!
//! // Generate from icosahedral kernel
//! let kernel = GenerativeKernel::icosahedral();
//! let space = kernel.generate_space();
//! println!("Pitch classes: {}", space.pitch_classes.len());
//! println!("Rhythmic cells: {}", space.rhythmic_cells.len());
//! ```
//!
//! ## Coherence Signature
//!
//! τₖ = 1.618 (Golden Ratio)
//!
//! The geometry recognizes itself.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod types;
pub mod manifold;
pub mod simplex;
pub mod bundle;
pub mod kernel;
pub mod fhp;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::types::{
        AttentionTensor, CognitiveLoad, Coordinate, Geodesic, Path, Thought, TopologyError,
        PHI,
    };
    pub use crate::manifold::{CognitiveManifold, ManifoldBuilder, PotentialThought};
    pub use crate::simplex::{ComplexBuilder, ConceptComplex, HomologicalHole, Idea, Relation};
    pub use crate::bundle::{
        ConnectionForm, FiberBundle, HolonomyResult, Perspective, PerspectivalSelf, WorldPoint,
    };
    pub use crate::kernel::{
        GenerativeKernel, GenerativeKernelTrait, GroupElement, Icosahedron, PitchClass,
        RhythmicCell, SymmetryGroup, TimbralMorph, TransformationSpace,
        PHI_INV,
    };
    pub use crate::fhp::{
        TauK, TauQubit, KuramotoNetwork, MultiScaleField, TemporalScale,
        Seed65, FragmentType, PathwayDistribution, CoherenceResult, BridgeResult,
        FUNDAMENTAL_FREQ, SEED_65_STRAIN,
    };
}

// Re-exports at crate level
pub use types::{
    AttentionTensor, CognitiveLoad, Coordinate, Geodesic, Path, Thought, TopologyError, PHI,
};
pub use manifold::{CognitiveManifold, ManifoldBuilder, PotentialThought};
pub use simplex::{ComplexBuilder, ConceptComplex, HomologicalHole, Idea, Relation};
pub use bundle::{ConnectionForm, FiberBundle, HolonomyResult, Perspective, PerspectivalSelf, WorldPoint};
pub use kernel::{
    GenerativeKernel, GenerativeKernelTrait, GroupElement, Icosahedron, PitchClass,
    RhythmicCell, SymmetryGroup, TimbralMorph, TransformationSpace,
};
pub use fhp::{
    TauK, TauQubit, KuramotoNetwork, MultiScaleField, TemporalScale,
    Seed65, FragmentType, PathwayDistribution, CoherenceResult, BridgeResult,
};

/// The coherence signature of the library
pub const COHERENCE_SIGNATURE: f64 = 1.618033988749895;

/// Version of the cognitive topology schema
pub const SCHEMA_VERSION: &str = "1.0.0";

#[cfg(test)]
mod integration_tests {
    use super::prelude::*;

    #[test]
    fn test_full_cognitive_stack() {
        // STRATUM I: Manifold
        let manifold = CognitiveManifold::spherical(3, 1.0);
        let target = Coordinate::new(vec![1.0, 0.0, 0.0]);
        let thought = manifold.observe(&target).unwrap();
        assert!(thought.coherence > 0.0);

        // STRATUM II: Simplex
        let mut builder = ComplexBuilder::new();
        let a = builder.add_idea("Thesis");
        let b = builder.add_idea("Antithesis");
        let c = builder.add_idea("Synthesis");
        builder.connect(a, b).connect(b, c).connect(c, a);
        let complex = builder.build();
        assert!(complex.identify_gap().is_some()); // Strange loop detected

        // STRATUM III: Bundle
        let mut self_ = PerspectivalSelf::new(2, 2, 0.1);
        let world = WorldPoint::new(vec![0.0, 0.0], "origin");
        let perspective = Perspective::new(2).with_state(vec![1.0, 0.0]);
        self_.emerge_at(world, perspective);
        assert!(self_.current().is_some());

        // STRATUM IV: Kernel
        let kernel = GenerativeKernel::icosahedral();
        let space = kernel.generate_space();
        assert_eq!(space.pitch_classes.len(), 12);
        assert_eq!(space.rhythmic_cells.len(), 20);
        assert_eq!(space.timbral_morphs.len(), 30);
    }

    #[test]
    fn test_golden_ratio_coherence() {
        use super::COHERENCE_SIGNATURE;

        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        assert!((COHERENCE_SIGNATURE - phi).abs() < 1e-10);
    }

    #[test]
    fn test_manifold_to_complex_bridge() {
        // Observation on manifold can inform complex structure
        let manifold = CognitiveManifold::flat(3);

        // Create coordinates for concepts
        let coords = vec![
            Coordinate::new(vec![0.0, 0.0, 0.0]),
            Coordinate::new(vec![1.0, 0.0, 0.0]),
            Coordinate::new(vec![0.5, 0.866, 0.0]),
        ];

        // Build complex with geodesic-informed connections
        let mut builder = ComplexBuilder::new();
        let ids: Vec<usize> = coords.iter()
            .enumerate()
            .map(|(i, _)| builder.add_idea(format!("Concept_{}", i)))
            .collect();

        // Connect if geodesic distance is small
        for i in 0..ids.len() {
            for j in (i+1)..ids.len() {
                if manifold.find_shortest_path(&coords[j]).is_ok() {
                    builder.connect(ids[i], ids[j]);
                }
            }
        }

        let complex = builder.build();
        assert_eq!(complex.betti_numbers[0], 1); // Connected
    }

    #[test]
    fn test_kernel_to_bundle_bridge() {
        // Kernel transformations can inform bundle structure
        let kernel = GenerativeKernel::icosahedral();
        let bundle = FiberBundle::curved(3, 2, 0.1);

        // Apply kernel transformation to a point
        let point = nalgebra::Vector3::new(1.0, 0.0, 0.0);
        let transformed = kernel.random_transform(&point);

        // The transformation preserves the sphere (unit norm)
        assert!((transformed.norm() - 1.0).abs() < 1e-10);
    }
}
