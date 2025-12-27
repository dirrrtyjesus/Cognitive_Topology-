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
