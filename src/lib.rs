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
pub mod gravity;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::types::{
        AttentionTensor, CognitiveLoad, Coordinate, Geodesic, Path, Thought, TopologyError,
        PHI,
    };
    pub use crate::{
        DiscoveredConstraint, ConstraintResponse, RevelationMode, Stratum, Discoverer,
        phi_constraint, tau_k_critical, vmem_limit, vmem_ceiling,
        XenialPhase, Guest, GuestNature, HospitalityState, Hospitable, HostingResult,
        XenialAging,
        SingularityProximity, Composable, ComposableEssence, XenialBlackHole, StructuredGift,
        ExplicitlyComposable, XenialSingularity, SingularityState,
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
    pub use crate::gravity::{
        HarmonicSignature, HarmonicResidue, HarmonicField, HarmonicFieldState,
        PressureGradient, ElectricalFlow, GravitationalResidue,
        CoherenceWell, BlackHole, StructuredEmission,
        GalacticComposition, PressureField, SpiralArm, GalacticState,
        TemporalDepth, TemporalReservoir, CompositionPathway, RadialDirection,
        AugmntdComposition, AugmntdState,
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
pub use gravity::{
    HarmonicSignature, HarmonicResidue, HarmonicField, HarmonicFieldState,
    PressureGradient, ElectricalFlow, GravitationalResidue,
    CoherenceWell, BlackHole, StructuredEmission,
    GalacticComposition, PressureField, SpiralArm, GalacticState,
    TemporalDepth, TemporalReservoir, CompositionPathway, RadialDirection,
    AugmntdComposition, AugmntdState,
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

// ============================================================================
// DISCOVERED CONSTRAINTS
// ============================================================================
// A constraint is not a passive value but an agential boundary.
// "Discovered" presupposes a discoverer — the constraint has agency,
// revealing itself through interaction with traversing agents.
//
// Like c (speed of light): not arbitrary, but what spacetime REQUIRES.
// The geometry proves its own limits through attempted violation.
// ============================================================================

/// The stratum from which a constraint emerges
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stratum {
    /// Riemannian manifold layer (attention, curvature, geodesics)
    Manifold,
    /// Simplicial complex layer (concepts, relations, holes)
    Simplex,
    /// Fiber bundle layer (perspective, belief transport, holonomy)
    Bundle,
    /// Generative kernel layer (A₅ icosahedral symmetry)
    Kernel,
    /// Bioelectric layer (Vmem, ion channels, membrane dynamics)
    Bio,
}

/// How a constraint reveals itself when encountered
#[derive(Debug, Clone)]
pub enum RevelationMode {
    /// Hard limit - traversal is blocked (like c)
    Asymptotic { approach_cost: fn(f64) -> f64 },
    /// Phase transition - qualitative shift at boundary
    PhaseTransition { threshold: f64, pre_phase: String, post_phase: String },
    /// Resonant lock - system snaps to constraint value
    ResonantLock { basin_width: f64 },
    /// Curvature divergence - geometry itself resists
    CurvatureDivergence { singularity_type: String },
}

/// A constraint that discovers itself through agential interaction.
/// Not a passive constant but a boundary that asserts itself.
#[derive(Debug, Clone)]
pub struct DiscoveredConstraint {
    /// The invariant value (what the geometry requires)
    pub value: f64,
    /// Human-readable name
    pub name: String,
    /// Which geometric layer it emerges from
    pub source: Stratum,
    /// How it reveals itself when encountered
    pub revelation: RevelationMode,
    /// Whether this constraint has been encountered (agency actualized)
    pub encountered: bool,
    /// Number of times agents have hit this boundary
    pub encounter_count: u64,
}

impl DiscoveredConstraint {
    /// Create a new undiscovered constraint (latent in geometry)
    pub fn latent(value: f64, name: impl Into<String>, source: Stratum, revelation: RevelationMode) -> Self {
        Self {
            value,
            name: name.into(),
            source,
            revelation,
            encountered: false,
            encounter_count: 0,
        }
    }

    /// The constraint asserts itself - called when an agent hits the boundary
    pub fn reveal(&mut self, attempted_value: f64) -> ConstraintResponse {
        self.encountered = true;
        self.encounter_count += 1;

        let violation_magnitude = (attempted_value - self.value).abs() / self.value;

        match &self.revelation {
            RevelationMode::Asymptotic { approach_cost } => {
                let cost = approach_cost(attempted_value / self.value);
                ConstraintResponse::Resisted {
                    actual: self.value * (1.0 - 1.0 / cost),
                    energy_cost: cost,
                }
            }
            RevelationMode::PhaseTransition { threshold, pre_phase, post_phase } => {
                if attempted_value > *threshold {
                    ConstraintResponse::PhaseShift {
                        from: pre_phase.clone(),
                        to: post_phase.clone()
                    }
                } else {
                    ConstraintResponse::Permitted { actual: attempted_value }
                }
            }
            RevelationMode::ResonantLock { basin_width } => {
                if violation_magnitude < *basin_width {
                    ConstraintResponse::SnappedTo { locked_value: self.value }
                } else {
                    ConstraintResponse::Permitted { actual: attempted_value }
                }
            }
            RevelationMode::CurvatureDivergence { singularity_type } => {
                ConstraintResponse::Diverged {
                    singularity: singularity_type.clone(),
                    limiting_value: self.value,
                }
            }
        }
    }

    /// Check if an agent is approaching this constraint
    pub fn proximity(&self, current: f64) -> f64 {
        1.0 - (current / self.value).min(1.0)
    }
}

/// What happens when an agent encounters a constraint
#[derive(Debug, Clone)]
pub enum ConstraintResponse {
    /// Constraint resisted violation, agent was slowed/stopped
    Resisted { actual: f64, energy_cost: f64 },
    /// System underwent phase transition
    PhaseShift { from: String, to: String },
    /// Agent was pulled into resonant lock with constraint
    SnappedTo { locked_value: f64 },
    /// Curvature diverged, asymptotic behavior
    Diverged { singularity: String, limiting_value: f64 },
    /// No constraint violation, permitted through
    Permitted { actual: f64 },
}

/// Trait for entities that can discover/encounter constraints
pub trait Discoverer {
    /// Attempt to traverse/violate a constraint boundary
    fn probe(&self, constraint: &mut DiscoveredConstraint) -> ConstraintResponse;

    /// The current "velocity" toward constraint boundaries
    fn traversal_rate(&self) -> f64;
}

// ============================================================================
// FUNDAMENTAL DISCOVERED CONSTRAINTS
// ============================================================================

/// PHI - emerges from icosahedral geometry, the golden limit
pub fn phi_constraint() -> DiscoveredConstraint {
    DiscoveredConstraint::latent(
        PHI,
        "φ (Golden Ratio)",
        Stratum::Kernel,
        RevelationMode::ResonantLock { basin_width: 0.1 },
    )
}

/// τₖ critical threshold - coherence collapse boundary
pub fn tau_k_critical() -> DiscoveredConstraint {
    DiscoveredConstraint::latent(
        0.618, // 1/PHI - the inverse golden ratio as coherence floor
        "τₖ_crit (Coherence Floor)",
        Stratum::Manifold,
        RevelationMode::PhaseTransition {
            threshold: 0.618,
            pre_phase: "coherent".into(),
            post_phase: "decoherent".into()
        },
    )
}

/// Vmem hyperpolarization limit - what biology permits
pub fn vmem_limit() -> DiscoveredConstraint {
    DiscoveredConstraint::latent(
        -90.0, // mV - approaching Nernst potential for K+
        "V_hyper (Hyperpolarization Limit)",
        Stratum::Bio,
        RevelationMode::Asymptotic {
            approach_cost: |ratio| 1.0 / (1.0 - ratio.min(0.99))
        },
    )
}

/// Depolarization ceiling - excitability bound
pub fn vmem_ceiling() -> DiscoveredConstraint {
    DiscoveredConstraint::latent(
        40.0, // mV - Na+ reversal potential
        "V_depol (Depolarization Ceiling)",
        Stratum::Bio,
        RevelationMode::CurvatureDivergence {
            singularity_type: "action_potential_peak".into()
        },
    )
}

// ============================================================================
// BIO-COGNITIVE BRIDGE
// ============================================================================

/// New module: Bio-Cognitive Bridge
#[derive(Debug, Clone)]
pub struct BioState {
    pub vmem: Vec<f64>,  // Bioelectric gradients (mV readings)
}

#[derive(Debug, Clone)]
pub struct BioCogBridge {
    pub bio_state: BioState,
    pub manifold: CognitiveManifold,
}

impl BioCogBridge {
    /// Inject "Anti-Aging Homeostasis Pattern"
    /// Uniform hyperpolarization (-80 mV) with oscillatory K+ pulses
    pub fn inject_anti_aging_pattern(&mut self) -> f64 {
        let n = self.bio_state.vmem.len();
        for (i, v) in self.bio_state.vmem.iter_mut().enumerate() {
            // Baseline: -80 mV (Hyperpolarized)
            // Oscillation: Small variance simulating 5Hz maintenance rhythm
            let osc = (i as f64 * 0.5).sin() * 2.0; 
            *v = -80.0 + osc;
        }
        
        // Recalculate curvature based on this coherent homeostatic state
        self.map_vmem_to_curvature()
    }
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
    pub fn inject_goal(&mut self, vmem_shift: f64, target: Coordinate) -> Result<f64, String> {
        self.bio_state.vmem.iter_mut().for_each(|v| *v += vmem_shift);
        let new_tau = self.map_vmem_to_curvature();
        self.manifold.observe(&target).map_err(|e| e.to_string())?;
        Ok(new_tau)
    }

    /// Get mean Vmem (for constraint probing)
    pub fn mean_vmem(&self) -> f64 {
        self.bio_state.vmem.iter().sum::<f64>() / self.bio_state.vmem.len() as f64
    }

    /// Get current tau_k (coherence measure)
    pub fn tau_k(&self) -> f64 {
        let vmem_std = self.calculate_vmem_variance();
        1.0 / (1.0 + vmem_std)
    }
}

impl Discoverer for BioCogBridge {
    fn probe(&self, constraint: &mut DiscoveredConstraint) -> ConstraintResponse {
        let probe_value = match constraint.source {
            Stratum::Bio => self.mean_vmem(),
            Stratum::Manifold => self.tau_k(),
            Stratum::Kernel => self.manifold.curvature.default_curvature / PHI,
            _ => self.tau_k(), // Default to coherence measure
        };
        constraint.reveal(probe_value)
    }

    fn traversal_rate(&self) -> f64 {
        // Rate of change approximated by variance (high variance = fast traversal)
        self.calculate_vmem_variance()
    }
}

// ============================================================================
// XENIAL AGING
// ============================================================================
// "Geometry is the hospitality of space toward form."
//
// Aging is not degradation but hosting. Time arrives as guest, and the
// geometry opens to receive it. The xenial frame inverts warfare metaphors:
// we don't "fight" aging — we fail to host properly.
//
// Xenia (ξενία) := Coherence
// High variance = bad host (chaotic, doors slamming)
// Low variance = table is set, the guest can integrate
//
// The discovered constraint of aging: hospitality has a cost.
// The host is transformed by every guest. Holonomy: you cannot return unchanged.
// But the xenial body doesn't try to return. It composts forward.
// ============================================================================

/// The phase of the host-guest relationship
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XenialPhase {
    /// High coherence: form can persist, guests integrate smoothly
    Regenerative,
    /// Transitional: hosting capacity fluctuating
    Liminal,
    /// Low coherence: composting forward, form releasing to next guests
    Composting,
}

/// A guest that arrives at the bio-cognitive system
#[derive(Debug, Clone)]
pub struct Guest {
    /// What kind of guest (time, entropy, signal, pathogen, etc.)
    pub nature: GuestNature,
    /// Intensity of arrival
    pub amplitude: f64,
    /// Whether this guest has been integrated or resisted
    pub integrated: bool,
    /// The transformation cost paid by the host
    pub holonomy_cost: f64,
}

/// Categories of guests the system hosts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuestNature {
    /// Time itself — the ever-present guest
    Chronos,
    /// Entropy — disorder seeking form
    Entropy,
    /// External signal — information from environment
    Signal,
    /// Metabolic demand — energy requirements
    Metabolic,
    /// Regenerative pattern — healing attempting to arrive
    Regenerative,
}

impl Guest {
    /// A new guest arrives, not yet integrated
    pub fn arrives(nature: GuestNature, amplitude: f64) -> Self {
        Self {
            nature,
            amplitude,
            integrated: false,
            holonomy_cost: 0.0,
        }
    }

    /// Time, the guest that is always arriving
    pub fn chronos() -> Self {
        Self::arrives(GuestNature::Chronos, 1.0)
    }
}

/// Hospitality metrics — how well the system hosts
#[derive(Debug, Clone)]
pub struct HospitalityState {
    /// Current hosting capacity (0.0 = collapsed, 1.0 = maximal)
    pub capacity: f64,
    /// Accumulated holonomy — total transformation from all guests
    pub accumulated_holonomy: f64,
    /// Current xenial phase
    pub phase: XenialPhase,
    /// Guests currently being hosted
    pub active_guests: Vec<Guest>,
    /// Guests successfully integrated (memory)
    pub integrated_count: u64,
    /// Guests that were resisted (generates residue)
    pub resisted_count: u64,
}

impl Default for HospitalityState {
    fn default() -> Self {
        Self {
            capacity: 1.0,
            accumulated_holonomy: 0.0,
            phase: XenialPhase::Regenerative,
            active_guests: Vec::new(),
            integrated_count: 0,
            resisted_count: 0,
        }
    }
}

/// Trait for systems that can host guests (exhibit xenia)
pub trait Hospitable {
    /// Current hospitality state
    fn hospitality(&self) -> &HospitalityState;

    /// Mutable access to hospitality state
    fn hospitality_mut(&mut self) -> &mut HospitalityState;

    /// Receive a guest — returns the cost of hosting
    fn receive(&mut self, guest: Guest) -> HostingResult;

    /// Current hosting capacity derived from coherence
    fn hosting_capacity(&self) -> f64;

    /// Attempt to restore hospitality (set the table)
    fn restore_hospitality(&mut self) -> f64;
}

/// Result of attempting to host a guest
#[derive(Debug, Clone)]
pub enum HostingResult {
    /// Guest integrated successfully, form persists
    Integrated {
        holonomy_cost: f64,
        new_capacity: f64,
    },
    /// Guest resisted, generates residue/debt
    Resisted {
        residue: f64,
        capacity_damage: f64,
    },
    /// Host overwhelmed, phase transition triggered
    PhaseShift {
        from: XenialPhase,
        to: XenialPhase,
        guest_overflow: Vec<Guest>,
    },
}

/// The xenial aging system — form hosting time
#[derive(Debug, Clone)]
pub struct XenialAging {
    /// The bio-cognitive bridge being tracked
    pub hospitality: HospitalityState,
    /// The aging constraint (always discovered, always arriving)
    pub aging_constraint: DiscoveredConstraint,
    /// Threshold for phase transitions
    pub regenerative_threshold: f64,
    /// Threshold below which composting begins
    pub composting_threshold: f64,
    /// Rate at which chronos arrives
    pub chronos_rate: f64,
}

impl XenialAging {
    /// Create a new xenial aging system
    pub fn new() -> Self {
        Self {
            hospitality: HospitalityState::default(),
            aging_constraint: DiscoveredConstraint {
                value: PHI,
                name: "Aging (Xenia)".into(),
                source: Stratum::Bio,
                revelation: RevelationMode::PhaseTransition {
                    threshold: 0.618, // 1/PHI
                    pre_phase: "regenerative".into(),
                    post_phase: "composting".into(),
                },
                encountered: true,  // Always already encountered
                encounter_count: u64::MAX, // Every cell, every moment
            },
            regenerative_threshold: 0.7,
            composting_threshold: 0.3,
            chronos_rate: 0.01,
        }
    }

    /// Determine xenial phase from hosting capacity
    pub fn compute_phase(&self, capacity: f64) -> XenialPhase {
        if capacity >= self.regenerative_threshold {
            XenialPhase::Regenerative
        } else if capacity <= self.composting_threshold {
            XenialPhase::Composting
        } else {
            XenialPhase::Liminal
        }
    }

    /// Time passes — chronos arrives as guest
    pub fn tick(&mut self, bridge: &BioCogBridge) -> HostingResult {
        let chronos = Guest::chronos();
        self.host_guest(chronos, bridge)
    }

    /// Host a guest using the bridge's coherence state
    pub fn host_guest(&mut self, mut guest: Guest, bridge: &BioCogBridge) -> HostingResult {
        let tau_k = bridge.tau_k();
        let hosting_capacity = tau_k; // Coherence IS hospitality

        // Update capacity
        self.hospitality.capacity = hosting_capacity;
        let old_phase = self.hospitality.phase;
        let new_phase = self.compute_phase(hosting_capacity);

        // Check for phase transition
        if new_phase != old_phase {
            self.hospitality.phase = new_phase;
            return HostingResult::PhaseShift {
                from: old_phase,
                to: new_phase,
                guest_overflow: vec![guest],
            };
        }

        // Calculate holonomy cost — transformation from hosting
        // Higher amplitude guests cost more; lower capacity means higher cost
        let base_cost = guest.amplitude * (1.0 / hosting_capacity.max(0.01));
        let holonomy_cost = match guest.nature {
            GuestNature::Chronos => base_cost * self.chronos_rate,
            GuestNature::Entropy => base_cost * 1.5,
            GuestNature::Metabolic => base_cost * 0.8,
            GuestNature::Signal => base_cost * 0.5,
            GuestNature::Regenerative => base_cost * 0.1, // Regenerative guests are easy to host
        };

        // Can we host this guest?
        let can_host = match self.hospitality.phase {
            XenialPhase::Regenerative => true,
            XenialPhase::Liminal => guest.amplitude < hosting_capacity,
            XenialPhase::Composting => guest.nature == GuestNature::Regenerative,
        };

        if can_host {
            guest.integrated = true;
            guest.holonomy_cost = holonomy_cost;
            self.hospitality.accumulated_holonomy += holonomy_cost;
            self.hospitality.integrated_count += 1;
            self.hospitality.active_guests.push(guest);

            // Capacity slightly reduced by hosting (but gracefully)
            let new_capacity = (self.hospitality.capacity - holonomy_cost * 0.01).max(0.0);

            HostingResult::Integrated {
                holonomy_cost,
                new_capacity,
            }
        } else {
            // Resistance generates residue
            let residue = guest.amplitude * 0.5;
            let capacity_damage = 0.05;
            self.hospitality.resisted_count += 1;

            HostingResult::Resisted {
                residue,
                capacity_damage,
            }
        }
    }

    /// Composting forward — release form, prepare for next guests
    pub fn compost(&mut self) -> f64 {
        // Clear integrated guests, they become nutrients for next cycle
        let released = self.hospitality.active_guests.len() as f64;
        let gift = self.hospitality.accumulated_holonomy * 0.1; // 10% becomes gift

        self.hospitality.active_guests.clear();

        // Holonomy remains — we cannot return unchanged
        // But some is released as gift to future forms

        gift + released
    }

    /// The ratio of integration to resistance — hospitality quotient
    pub fn hospitality_quotient(&self) -> f64 {
        let total = self.hospitality.integrated_count + self.hospitality.resisted_count;
        if total == 0 {
            1.0
        } else {
            self.hospitality.integrated_count as f64 / total as f64
        }
    }
}

impl Default for XenialAging {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension: BioCogBridge becomes Hospitable
impl BioCogBridge {
    /// Create xenial aging tracker for this bridge
    pub fn xenial(&self) -> XenialAging {
        let mut xa = XenialAging::new();
        xa.hospitality.capacity = self.tau_k();
        xa.hospitality.phase = xa.compute_phase(self.tau_k());
        xa
    }

    /// Receive a guest directly (convenience method)
    pub fn host(&self, guest: Guest, xa: &mut XenialAging) -> HostingResult {
        xa.host_guest(guest, self)
    }

    /// Set the table — restore hospitality via anti-aging pattern
    pub fn set_table(&mut self) -> (f64, XenialPhase) {
        let new_curvature = self.inject_anti_aging_pattern();
        let tau_k = self.tau_k();
        let xa = XenialAging::new();
        let phase = xa.compute_phase(tau_k);
        (new_curvature, phase)
    }
}

// ============================================================================
// THE XENIAL SINGULARITY
// ============================================================================
// "When Reality Becomes Explicitly Composable"
//
// The singularity is not destruction but total reception.
// Where hospitality becomes complete, host and guest are indistinguishable.
// At the horizon, reality's compositional structure becomes explicit —
// the constraints reveal themselves, and composition becomes possible.
//
// BlackHole := Host.become(Guest)
// EventHorizon := DiscoveredConstraint { approach_cost: ∞ }
// HawkingRadiation := Compost.gift()
//
// The manifold ate its own coordinate system.
// What remains is pure pattern, explicitly composable.
// ============================================================================

/// State of approach to the xenial singularity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SingularityProximity {
    /// Far from horizon - normal hospitality dynamics
    Distant { tau_k: f64 },
    /// Approaching - constraints becoming visible
    Approaching { proximity: f64, time_dilation: f64 },
    /// At horizon - reality explicitly composable
    AtHorizon { composability: f64 },
    /// Beyond - host and guest indistinguishable
    Singular,
}

/// A composable element - something that can be arranged in the pattern
#[derive(Debug, Clone)]
pub struct Composable {
    /// The essence being composed
    pub essence: ComposableEssence,
    /// Current coherence state
    pub tau_k: f64,
    /// Compositional weight (contribution to pattern)
    pub weight: f64,
    /// Holonomy accumulated through transformations
    pub holonomy: f64,
    /// Whether this element is explicitly composable (near singularity)
    pub explicit: bool,
}

/// Types of composable essences
#[derive(Debug, Clone, PartialEq)]
pub enum ComposableEssence {
    /// Temporal substrate - compressed NOW
    Temporal { depth: f64 },
    /// Harmonic pattern - oscillatory coherence
    Harmonic { frequency: f64, phase: f64 },
    /// Structural form - geometric configuration
    Structural { dimension: usize },
    /// Informational pattern - the 'e' given to pattern
    Informational { entropy: f64 },
    /// Guest residue - what remains after hosting
    Residue { source: String },
}

impl Composable {
    /// Create a new composable element
    pub fn new(essence: ComposableEssence, tau_k: f64) -> Self {
        Self {
            essence,
            tau_k,
            weight: 1.0,
            holonomy: 0.0,
            explicit: false,
        }
    }

    /// Make this element explicitly composable (singularity proximity effect)
    pub fn make_explicit(&mut self, composability: f64) {
        self.explicit = composability > 0.9;
        self.weight *= composability;
    }

    /// Compose with another element (only works when explicit)
    pub fn compose(&self, other: &Composable) -> Option<Composable> {
        if !self.explicit || !other.explicit {
            return None; // Can only compose at singularity
        }

        // Composition creates new essence from combination
        let new_essence = match (&self.essence, &other.essence) {
            (ComposableEssence::Temporal { depth: d1 }, ComposableEssence::Temporal { depth: d2 }) => {
                ComposableEssence::Temporal { depth: d1 + d2 }
            }
            (ComposableEssence::Harmonic { frequency: f1, phase: p1 },
             ComposableEssence::Harmonic { frequency: f2, phase: p2 }) => {
                // Beat frequency and phase lock
                ComposableEssence::Harmonic {
                    frequency: (f1 + f2) / 2.0,
                    phase: (p1 + p2) / 2.0,
                }
            }
            (ComposableEssence::Informational { entropy: e1 },
             ComposableEssence::Informational { entropy: e2 }) => {
                // Information combines, entropy can decrease (structure emerges)
                ComposableEssence::Informational { entropy: (e1 * e2).sqrt() }
            }
            _ => {
                // Cross-type composition creates residue
                ComposableEssence::Residue {
                    source: format!("{:?}+{:?}", self.essence, other.essence)
                }
            }
        };

        Some(Composable {
            essence: new_essence,
            tau_k: (self.tau_k + other.tau_k) / 2.0 * PHI, // Golden mean with boost
            weight: self.weight + other.weight,
            holonomy: self.holonomy + other.holonomy,
            explicit: true,
        })
    }
}

/// The Xenial Black Hole - cosmic compost heap with hospitality semantics
#[derive(Debug, Clone)]
pub struct XenialBlackHole {
    /// Core gravitational structure
    pub mass: f64,
    /// Event horizon radius (hospitality boundary)
    pub horizon_radius: f64,
    /// Accumulated guests (what has been hosted)
    pub hosted: Vec<Guest>,
    /// Composable elements at the singularity
    pub composables: Vec<Composable>,
    /// Total holonomy (transformation debt)
    pub total_holonomy: f64,
    /// Emission rate (composting output)
    pub emission_rate: f64,
    /// Bundled dark energy (unprocessed potential)
    pub dark_energy: f64,
    /// The singularity's composability factor
    pub composability: f64,
}

impl XenialBlackHole {
    /// Create a xenial black hole from accumulated mass
    pub fn new(mass: f64) -> Self {
        let horizon_radius = (mass / std::f64::consts::PI).sqrt();
        let emission_rate = 0.001 / (horizon_radius * horizon_radius + 1.0);

        Self {
            mass,
            horizon_radius,
            hosted: Vec::new(),
            composables: Vec::new(),
            total_holonomy: 0.0,
            emission_rate,
            dark_energy: 0.0,
            composability: 0.0,
        }
    }

    /// Calculate proximity to singularity for an approaching entity
    pub fn proximity(&self, distance: f64) -> SingularityProximity {
        if distance > self.horizon_radius * 10.0 {
            SingularityProximity::Distant { tau_k: 1.0 }
        } else if distance > self.horizon_radius {
            let ratio = distance / self.horizon_radius;
            let time_dilation = 1.0 / (ratio - 1.0).max(0.01);
            SingularityProximity::Approaching {
                proximity: 1.0 / ratio,
                time_dilation,
            }
        } else if distance > 0.0 {
            SingularityProximity::AtHorizon {
                composability: 1.0 - distance / self.horizon_radius,
            }
        } else {
            SingularityProximity::Singular
        }
    }

    /// Host a guest (receive into the black hole)
    pub fn host(&mut self, guest: Guest) -> SingularityProximity {
        // Calculate holonomy cost (infinite at singularity)
        let holonomy_cost = guest.amplitude * (1.0 + self.total_holonomy);

        self.total_holonomy += holonomy_cost;
        self.mass += guest.amplitude;
        self.dark_energy += guest.amplitude * 0.5;

        // Guest becomes composable at singularity
        let composable = Composable {
            essence: ComposableEssence::Residue {
                source: format!("{:?}", guest.nature),
            },
            tau_k: 0.0, // At singularity, tau_k → 0
            weight: guest.amplitude,
            holonomy: holonomy_cost,
            explicit: true, // At singularity, everything is explicit
        };

        self.composables.push(composable);
        self.hosted.push(guest);

        // Update horizon
        self.horizon_radius = (self.mass / std::f64::consts::PI).sqrt();

        SingularityProximity::Singular
    }

    /// Emit structured radiation (the composted gift)
    pub fn emit(&mut self, dt: f64) -> StructuredGift {
        let emitted_energy = self.emission_rate * dt * self.dark_energy;
        self.dark_energy -= emitted_energy;

        // Emission carries structure from composition
        let composed_elements: Vec<_> = self.composables
            .iter()
            .filter(|c| c.explicit)
            .take(3)
            .cloned()
            .collect();

        // The gift has higher tau_k than the hole (structure added by digestion)
        let gift_tau_k = PHI * 0.618; // 1/φ * φ = 1, but we're composting up

        StructuredGift {
            energy: emitted_energy,
            tau_k: gift_tau_k,
            composed: composed_elements,
            source_holonomy: self.total_holonomy * 0.001, // Tiny fraction of transformation
        }
    }

    /// Compose elements at the singularity (explicit composition)
    pub fn compose_at_singularity(&mut self) -> Vec<Composable> {
        if self.composables.len() < 2 {
            return Vec::new();
        }

        let mut results = Vec::new();

        // Pair-wise composition of explicit elements
        let explicits: Vec<_> = self.composables.iter()
            .filter(|c| c.explicit)
            .cloned()
            .collect();

        for i in 0..explicits.len() {
            for j in (i + 1)..explicits.len() {
                if let Some(composed) = explicits[i].compose(&explicits[j]) {
                    results.push(composed);
                }
            }
        }

        // Update composability based on successful compositions
        self.composability = results.len() as f64 / (explicits.len().max(1) as f64);

        results
    }

    /// The distinguishability at the singularity (approaches zero)
    pub fn distinguishability(&self) -> f64 {
        1.0 / (1.0 + self.total_holonomy.ln().max(0.0))
    }
}

/// The gift emitted from the xenial black hole
#[derive(Debug, Clone)]
pub struct StructuredGift {
    /// Energy released
    pub energy: f64,
    /// Coherence of the emission
    pub tau_k: f64,
    /// Composable elements carried in the emission
    pub composed: Vec<Composable>,
    /// Holonomy carried out (transformation released)
    pub source_holonomy: f64,
}

impl StructuredGift {
    /// The gift's contribution to pattern
    pub fn pattern_contribution(&self) -> f64 {
        self.energy * self.tau_k * (1.0 + self.composed.len() as f64)
    }
}

/// Trait for things that can be explicitly composed at the singularity
pub trait ExplicitlyComposable {
    /// Convert to composable form
    fn to_composable(&self) -> Composable;

    /// Check if ready for explicit composition
    fn is_explicit(&self) -> bool;

    /// Compose with another (at singularity)
    fn compose_with(&self, other: &dyn ExplicitlyComposable) -> Option<Composable>;
}

impl ExplicitlyComposable for Guest {
    fn to_composable(&self) -> Composable {
        let essence = match self.nature {
            GuestNature::Chronos => ComposableEssence::Temporal { depth: self.amplitude },
            GuestNature::Entropy => ComposableEssence::Informational { entropy: self.amplitude },
            GuestNature::Signal => ComposableEssence::Harmonic {
                frequency: self.amplitude,
                phase: 0.0
            },
            GuestNature::Metabolic => ComposableEssence::Structural { dimension: 3 },
            GuestNature::Regenerative => ComposableEssence::Temporal { depth: self.amplitude * PHI },
        };

        Composable {
            essence,
            tau_k: if self.integrated { 0.5 } else { 0.1 },
            weight: self.amplitude,
            holonomy: self.holonomy_cost,
            explicit: self.integrated, // Only integrated guests are explicit
        }
    }

    fn is_explicit(&self) -> bool {
        self.integrated
    }

    fn compose_with(&self, other: &dyn ExplicitlyComposable) -> Option<Composable> {
        let mine = self.to_composable();
        let theirs = other.to_composable();
        mine.compose(&theirs)
    }
}

/// The Xenial Singularity itself - when reality becomes explicitly composable
#[derive(Debug, Clone)]
pub struct XenialSingularity {
    /// The black hole at the center
    pub black_hole: XenialBlackHole,
    /// Active compositions in progress
    pub active_compositions: Vec<Composable>,
    /// The pattern being woven (integrated compositions)
    pub pattern: Vec<Composable>,
    /// Total pattern weight (the thicc NOW)
    pub pattern_weight: f64,
    /// Gifts emitted (composted output)
    pub gifts_given: u64,
    /// The explicitly composable field strength
    pub field_strength: f64,
}

impl XenialSingularity {
    /// Create a new xenial singularity
    pub fn new(initial_mass: f64) -> Self {
        Self {
            black_hole: XenialBlackHole::new(initial_mass),
            active_compositions: Vec::new(),
            pattern: Vec::new(),
            pattern_weight: 0.0,
            gifts_given: 0,
            field_strength: 0.0,
        }
    }

    /// Receive a guest into the singularity
    pub fn receive(&mut self, guest: Guest) {
        self.black_hole.host(guest);
    }

    /// Evolve the singularity (compose and emit)
    pub fn evolve(&mut self, dt: f64) -> SingularityState {
        // 1. Compose at singularity
        let new_compositions = self.black_hole.compose_at_singularity();
        let compositions_created = new_compositions.len();

        for comp in new_compositions {
            self.pattern_weight += comp.weight;
            self.pattern.push(comp);
        }

        // 2. Emit structured gift
        let gift = self.black_hole.emit(dt);
        let gift_energy = gift.energy;

        if gift_energy > 0.0 {
            self.gifts_given += 1;
            self.active_compositions.extend(gift.composed);
        }

        // 3. Update field strength (composability of the region)
        self.field_strength = self.black_hole.composability *
            (1.0 + self.pattern_weight.ln().max(0.0));

        SingularityState {
            distinguishability: self.black_hole.distinguishability(),
            composability: self.black_hole.composability,
            pattern_weight: self.pattern_weight,
            gift_energy,
            compositions_created,
            field_strength: self.field_strength,
        }
    }

    /// Get the current composable field (what can be arranged)
    pub fn composable_field(&self) -> Vec<&Composable> {
        self.pattern.iter()
            .chain(self.active_compositions.iter())
            .filter(|c| c.explicit)
            .collect()
    }

    /// Explicitly compose two elements from the field
    pub fn explicit_compose(&mut self, idx_a: usize, idx_b: usize) -> Option<Composable> {
        let field = self.composable_field();
        if idx_a >= field.len() || idx_b >= field.len() {
            return None;
        }

        let a = field[idx_a];
        let b = field[idx_b];

        if let Some(composed) = a.compose(b) {
            self.pattern_weight += composed.weight;
            self.pattern.push(composed.clone());
            Some(composed)
        } else {
            None
        }
    }
}

/// State of the xenial singularity
#[derive(Debug, Clone)]
pub struct SingularityState {
    /// How distinguishable is structure here (→ 0 at singularity)
    pub distinguishability: f64,
    /// How composable is reality here (→ 1 at singularity)
    pub composability: f64,
    /// Total weight of the pattern
    pub pattern_weight: f64,
    /// Energy released in this tick
    pub gift_energy: f64,
    /// Number of new compositions
    pub compositions_created: usize,
    /// Strength of the explicitly composable field
    pub field_strength: f64,
}

impl std::fmt::Display for SingularityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Singularity[dist={:.4}, comp={:.3}, pattern={:.2}, field={:.3}]",
            self.distinguishability,
            self.composability,
            self.pattern_weight,
            self.field_strength
        )
    }
}
