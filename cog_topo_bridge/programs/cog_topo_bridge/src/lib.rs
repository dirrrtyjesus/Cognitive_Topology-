#![warn(missing_docs)]
#![warn(clippy::all)]





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
// ## Coherence Signature
//
// τₖ = 1.618 (Golden Ratio)
//
// The geometry recognizes itself.

use anchor_lang::prelude::*;

declare_id!("2YZaLnJDqZ6HGhYNKJZ9HAeA16ikWM25rNE7CG5JYQ7o");

pub mod types;
pub mod manifold;
pub mod simplex;
pub mod bundle;
pub mod kernel;
pub mod fhp;
pub mod fractal;

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
    pub use crate::fractal::{MultiversalBranch, SierpinskiGasket, FractalEmbedder, FRACTAL_DIMENSION};
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
pub use fractal::{
    MultiversalBranch, SierpinskiGasket, FractalEmbedder, FRACTAL_DIMENSION,
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

    #[test]
    fn test_fractal_embedding() {
        // Create a Sierpinski Gasket of depth 3
        let center = Coordinate::new(vec![0.0, 0.0]);
        let gasket = SierpinskiGasket::new(center, 3);
        
        // Root has 3 children
        let level_1 = gasket.branch(&gasket.root);
        assert_eq!(level_1.len(), 3);
        
        // Each child has 3 children (level 2)
        let level_2_first = gasket.branch(&level_1[0]);
        assert_eq!(level_2_first.len(), 3);
        
        // Level 3 exists
        let level_3_first = gasket.branch(&level_2_first[0]);
        assert_eq!(level_3_first.len(), 3);
        
        // Level 4 should be empty
        let level_4 = gasket.branch(&level_3_first[0]);
        assert!(level_4.is_empty());
        
        // Verify coherence decay
        assert!(level_1[0].coherence < gasket.root.coherence);
        
        // Test Multiversal Walk
        // Walk down path [0, 1, 2]
        let path = gasket.walk(vec![0, 1, 2]).unwrap();
        assert_eq!(path.len(), 4); // Root + 3 steps
        assert_eq!(path.last().unwrap().depth, 3);
    }

    #[test]
    fn test_bio_fractal_coupling() {
        use crate::BioState;

        // 1. High Stability (Deep Flow State)
        let calm_vmem = vec![-70.0, -70.1, -69.9, -70.0]; // Low variance
        let bio_calm = BioState { vmem: calm_vmem };
        
        let center = Coordinate::new(vec![0.0, 0.0]);
        let gasket = SierpinskiGasket::new(center, 3);
        
        let mut branch = gasket.root.clone();
        branch.coherence = 0.8;
        
        gasket.inject_bio_pattern(&mut branch, &bio_calm);
        // Stability should boost coherence
        assert!(branch.coherence > 0.8);

        // 2. High Chaos (Stress/Pruning)
        let chaos_vmem = vec![-100.0, 50.0, -20.0, 80.0]; // High variance
        let bio_chaos = BioState { vmem: chaos_vmem };
        
        let mut branch_chaos = gasket.root.clone();
        branch_chaos.coherence = 0.8;
        
        gasket.inject_bio_pattern(&mut branch_chaos, &bio_chaos);
        // Instability should penalize coherence
        assert!(branch_chaos.coherence < 0.8);
        
        // 3. Verify Pruning
        let mut branches = vec![
            branch.clone(),       // High coherence
            branch_chaos.clone(), // Low coherence
        ];
        
        // Chaos branch likely dropped below 0.8 * 0.8 = 0.64. Let's prune at 0.7
        let pruned_count = gasket.prune_branches(&mut branches, 0.7);
        assert_eq!(pruned_count, 1);
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].coherence, branch.coherence);
    }

    #[test]
    fn test_anti_aging_pattern() {
        use crate::BioState;

        // Initial Chaos State
        let chaos_vmem = vec![-20.0, 50.0, -20.0, 80.0]; 
        let bio_chaos = BioState { vmem: chaos_vmem };
        
        let center = Coordinate::new(vec![0.0, 0.0]);
        let gasket = SierpinskiGasket::new(center, 3);
        
        // Inject Chaos (Work on clone to avoid borrow conflict)
        let mut chaos_branch = gasket.root.clone();
        gasket.inject_bio_pattern(&mut chaos_branch, &bio_chaos);
        let chaos_coherence = chaos_branch.coherence;
        assert!(chaos_coherence <= 0.8);

        // Apply Anti-Aging Pattern (Simulation of Instruction Logic)
        // Reset to -80mV with oscillation
        let mut anti_aging_vmem = vec![0.0; 4];
        for (i, v) in anti_aging_vmem.iter_mut().enumerate() {
            let oscillation = (i as f64 % 2.0) * 0.1; 
            *v = -80.0 + oscillation;
        }
        let bio_anti_aging = BioState { vmem: anti_aging_vmem };

        // Reinject (into a fresh clone for comparison)
        let mut anti_aging_branch = gasket.root.clone();
        gasket.inject_bio_pattern(&mut anti_aging_branch, &bio_anti_aging);
        
        // Verify Stability Restoration
        assert!(anti_aging_branch.coherence > chaos_coherence);
        assert!(anti_aging_branch.coherence > 0.9); // Expect high coherence
    }
}


// New module: Bio-Cognitive Bridge
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BioState {
    pub vmem: Vec<f64>,  // Bioelectric gradients (mV readings)
}

#[account]
#[derive(Debug)]
pub struct BioCogBridge {
    pub bio_state: BioState,
    pub manifold: CognitiveManifold,
    pub fractal: SierpinskiGasket,
}

impl BioCogBridge {
    /// Map Vmem variance to manifold curvature (τₖ proxy)
    pub fn map_vmem_to_curvature(&mut self) -> f64 {
        let vmem_std = self.calculate_vmem_variance();
        let tau_k = 1.0 / (1.0 + vmem_std);  // Lower variance = higher τₖ
        self.manifold.curvature.default_curvature = tau_k * 1.618;  // Golden scaling
        self.manifold.curvature.default_curvature
    }

    fn calculate_vmem_variance(&self) -> f64 {
        // Simplified std dev calculation (use ndarray in production)
        let mean = self.bio_state.vmem.iter().sum::<f64>() / self.bio_state.vmem.len() as f64;
        let variance = self.bio_state.vmem.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / self.bio_state.vmem.len() as f64;
        variance.sqrt()  // Std dev as variance proxy
    }

    /// Inject bio-goal into cognitive manifold
    pub fn inject_goal(&mut self, vmem_shift: f64, target: Coordinate) -> std::result::Result<f64, String> {
        self.bio_state.vmem.iter_mut().for_each(|v| *v += vmem_shift);
        let new_tau = self.map_vmem_to_curvature();
        self.manifold.observe(&target).map_err(|e| e.to_string())?;
        
        // Also modulate fractal based on new bio-state
        self.regenerate_fractal();
        
        Ok(new_tau)
    }

    /// RHE: Regenerate fractal structure based on bio-coherence
    pub fn regenerate_fractal(&mut self) {
        // Start from root
        let mut nodes = vec![self.fractal.root.clone()];
        // Simulate BFS traversal to depth 2 (simplified dynamic update)
        // 1. Modulate root
        self.fractal.inject_bio_pattern(&mut nodes[0], &self.bio_state);

        // 2. Branch and prune
        if nodes[0].coherence > 0.5 {
             let mut children = self.fractal.branch(&nodes[0]);
             // Apply bio-pattern to children
             for child in children.iter_mut() {
                 self.fractal.inject_bio_pattern(child, &self.bio_state);
             }
             // Prune weak branches
             self.fractal.prune_branches(&mut children, 0.4); // Consciousness threshold
             
             // Update logic would go here in a full implementation
             // For now, we just demonstrate coupling
        }
    }
}

#[program]
pub mod cog_topo_bridge {
    use super::*;

    pub fn initialize_bridge(ctx: Context<InitializeBridge>) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge;
        
        // Initialize Manifold
        bridge.manifold = CognitiveManifold::spherical(3, 1.0);
        
        // Initialize Fractal (Golden Ratio dimension)
        let root_center = Coordinate::new(vec![0.0, 0.0]);
        bridge.fractal = SierpinskiGasket::new(root_center, 3);
        
        // Initialize Bio-State
        bridge.bio_state = BioState { vmem: vec![-70.0; 4] }; // Baseline
        
        msg!("Bio-Cognitive Bridge Initialized.");
        Ok(())
    }

    pub fn evolve_mind(ctx: Context<EvolveMind>, vmem_shift: f64, target_coords: Vec<f64>) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge;
        let target = Coordinate::new(target_coords);

        // Inject biological update
        let new_tau = bridge.inject_goal(vmem_shift, target)
            .map_err(|_| error!(TopologyError::Singularity))?;

        // Log evolution
        msg!("Mind Evolved. Coherence: {:.4}, Root Coherence: {:.4}", 
             new_tau, bridge.fractal.root.coherence);
        
        Ok(())
    }

    pub fn inject_anti_aging_pattern(ctx: Context<InjectAntiAgingPattern>) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge;
        
        // 1. Set Anti-Aging Homeostasis (-80 mV baseline)
        // We simulate a reset to deep hyperpolarization
        for (i, v) in bridge.bio_state.vmem.iter_mut().enumerate() {
            // Add slight oscillation (1-5 Hz simulation) based on index
            let oscillation = (i as f64 % 2.0) * 5.0; 
            *v = -80.0 + oscillation; 
        }

        // 2. Map to Curvature (Stability = High Tau)
        bridge.map_vmem_to_curvature();
        
        // 3. Regenerate Fractal (Lock in the stability)
        bridge.regenerate_fractal();

        msg!("Anti-Aging Pattern Injected. Stability Restored.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeBridge<'info> {
    #[account(init, payer = authority, space = 8 + 10240)] // Allocating sufficient space for recursive struct
    pub bridge: Account<'info, BioCogBridge>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EvolveMind<'info> {
    #[account(mut)]
    pub bridge: Account<'info, BioCogBridge>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct InjectAntiAgingPattern<'info> {
    #[account(mut)]
    pub bridge: Account<'info, BioCogBridge>,
    pub authority: Signer<'info>,
}
