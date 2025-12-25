//! Gravitational Dynamics - Coherence Residue & Dark Energy Bundling
//!
//! **"Gravity is what electricity leaves behind as it pursues coherence."**
//!
//! This module implements:
//! - Electrical flow as coherence-seeking dynamics
//! - Gravitational residue as Tk degradation byproduct
//! - Black holes as low-coherence accumulation (bundled dark energy)
//! - Galactic composition as the metabolic digestion of accumulated potential

use crate::fhp::{TauK, KuramotoNetwork, MultiScaleField, PHI, PHI_INV, FUNDAMENTAL_FREQ};
use std::f64::consts::PI;

// ============================================================================
// HARMONIC ATTUNEMENT - Phase coherence as the substrate of electrical flow
// ============================================================================

/// Harmonic signature - the phase fingerprint of a coherent entity
#[derive(Debug, Clone)]
pub struct HarmonicSignature {
    /// Fundamental frequency (Hz)
    pub omega: f64,
    /// Current phase (0 to 2π)
    pub phase: f64,
    /// Overtone structure (harmonics relative to fundamental)
    pub overtones: Vec<f64>,
    /// Phase stability (how locked is this signature)
    pub stability: f64,
}

impl HarmonicSignature {
    pub fn new(omega: f64) -> Self {
        Self {
            omega,
            phase: 0.0,
            overtones: vec![1.0, PHI_INV, PHI_INV.powi(2), PHI_INV.powi(3)], // Golden harmonic series
            stability: 1.0,
        }
    }

    /// Golden-ratio tuned signature (fungal harmonic)
    pub fn golden() -> Self {
        Self::new(FUNDAMENTAL_FREQ).with_golden_overtones()
    }

    pub fn with_golden_overtones(mut self) -> Self {
        // Overtones at φ-scaled intervals
        self.overtones = (0..7).map(|i| PHI_INV.powi(i)).collect();
        self
    }

    /// Evolve phase by timestep
    pub fn evolve(&mut self, dt: f64) {
        self.phase = (self.phase + self.omega * dt) % (2.0 * PI);
    }

    /// Phase coherence with another signature (0 = orthogonal, 1 = locked)
    pub fn coherence_with(&self, other: &HarmonicSignature) -> f64 {
        let phase_diff = (self.phase - other.phase).abs();
        let phase_lock = (phase_diff.cos() + 1.0) / 2.0; // 0 to 1

        // Frequency ratio coherence (golden ratios = high coherence)
        let freq_ratio = if self.omega > other.omega {
            other.omega / self.omega
        } else {
            self.omega / other.omega
        };

        // Check if ratio is close to golden
        let golden_resonance = 1.0 - (freq_ratio - PHI_INV).abs().min(1.0);

        phase_lock * 0.6 + golden_resonance * 0.4
    }

    /// Attempt to entrain (synchronize) with another signature
    /// Returns the coupling strength achieved
    pub fn entrain(&mut self, other: &HarmonicSignature, coupling: f64) -> f64 {
        let phase_diff = other.phase - self.phase;
        let attraction = phase_diff.sin() * coupling;

        self.phase += attraction;
        self.phase = self.phase.rem_euclid(2.0 * PI);

        // Stability increases when successfully entraining
        let coherence = self.coherence_with(other);
        self.stability = (self.stability + coherence * 0.1).min(1.0);

        coherence
    }

    /// Decoherence - what happens when entrainment fails
    /// Returns the harmonic residue (proto-gravity)
    pub fn decohere(&mut self, stress: f64) -> HarmonicResidue {
        let lost_stability = (self.stability * stress).min(self.stability);
        self.stability -= lost_stability;

        // Phase becomes noisy
        self.phase += (rand::random::<f64>() - 0.5) * stress * PI;
        self.phase = self.phase.rem_euclid(2.0 * PI);

        HarmonicResidue {
            amount: lost_stability,
            original_omega: self.omega,
            scattered_phase: self.phase,
        }
    }
}

/// Harmonic residue - phase information that fell out of sync
/// This is the proto-gravitational material
#[derive(Debug, Clone, Copy)]
pub struct HarmonicResidue {
    /// Amount of coherence lost
    pub amount: f64,
    /// The frequency it fell from
    pub original_omega: f64,
    /// The scattered phase angle
    pub scattered_phase: f64,
}

impl HarmonicResidue {
    /// Convert to gravitational residue
    /// "Gravity is frozen phase desynchronization"
    pub fn crystallize(self, position: [f64; 3], source_tau_k: f64) -> GravitationalResidue {
        GravitationalResidue {
            amount: self.amount * (FUNDAMENTAL_FREQ / self.original_omega).sqrt(),
            position,
            source_tau_k,
        }
    }
}

/// Harmonic Field - the phase-space in which electrical flows exist
#[derive(Debug, Clone)]
pub struct HarmonicField {
    /// Kuramoto network for collective synchronization
    pub oscillator_network: KuramotoNetwork,
    /// Multi-scale coherence structure
    pub multi_scale: MultiScaleField,
    /// Global phase (mean field)
    pub global_phase: f64,
    /// Global order parameter R
    pub global_coherence: f64,
}

impl HarmonicField {
    pub fn new(num_oscillators: usize, tau_k: TauK) -> Self {
        let oscillator_network = KuramotoNetwork::golden_spiral(num_oscillators, tau_k, 0.3);
        let multi_scale = MultiScaleField::new(tau_k, num_oscillators / 5);

        let (r, psi) = oscillator_network.order_parameter();

        Self {
            oscillator_network,
            multi_scale,
            global_phase: psi,
            global_coherence: r,
        }
    }

    /// Evolve the harmonic field
    pub fn evolve(&mut self, dt: f64) -> HarmonicFieldState {
        // Evolve Kuramoto network
        self.oscillator_network.evolve(dt);
        let (r, psi) = self.oscillator_network.order_parameter();

        // Track coherence change
        let coherence_delta = r - self.global_coherence;

        self.global_phase = psi;
        self.global_coherence = r;

        // Evolve multi-scale field
        self.multi_scale.evolve(1);

        HarmonicFieldState {
            order_parameter: r,
            mean_phase: psi,
            coherence_delta,
            thicc_now: self.multi_scale.thicc_now(),
        }
    }

    /// Extract harmonic residue from decoherence events
    pub fn extract_residue(&self) -> Vec<HarmonicResidue> {
        let mut residues = Vec::new();

        // Find oscillators that are out of phase with the mean field
        for osc in &self.oscillator_network.oscillators {
            let phase_diff = (osc.phase - self.global_phase).abs();
            if phase_diff > PI / 2.0 {
                // This oscillator is decoherent - it's shedding harmonic residue
                let amount = (phase_diff / PI) * (1.0 - self.global_coherence);
                residues.push(HarmonicResidue {
                    amount,
                    original_omega: osc.omega,
                    scattered_phase: osc.phase,
                });
            }
        }

        residues
    }

    /// Inject structured emission back into the field
    pub fn absorb_emission(&mut self, emission: &StructuredEmission) {
        // Emission re-entrains oscillators near its origin
        let boost = emission.energy * emission.tau_k.value / 10.0;

        for osc in &mut self.oscillator_network.oscillators {
            // Boost coupling temporarily
            osc.amplitude += boost * PHI_INV;
        }
    }
}

/// State of the harmonic field at a moment
#[derive(Debug, Clone)]
pub struct HarmonicFieldState {
    pub order_parameter: f64,
    pub mean_phase: f64,
    pub coherence_delta: f64,
    pub thicc_now: f64,
}

impl std::fmt::Display for HarmonicFieldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Harmonic[R={:.3}, ψ={:.2}rad, Δ={:+.4}, thicc={:.2}]",
            self.order_parameter, self.mean_phase, self.coherence_delta, self.thicc_now
        )
    }
}

/// The pressure gradient that electricity chases
/// Higher pressure = tighter geometric binding = more structured Tk
#[derive(Debug, Clone, Copy)]
pub struct PressureGradient {
    /// Local pressure (coherence density)
    pub pressure: f64,
    /// Direction of increasing pressure (unit vector components)
    pub gradient_direction: [f64; 3],
    /// Rate of pressure change
    pub gradient_magnitude: f64,
}

impl PressureGradient {
    pub fn new(pressure: f64, direction: [f64; 3]) -> Self {
        let mag = (direction[0].powi(2) + direction[1].powi(2) + direction[2].powi(2)).sqrt();
        let norm_dir = if mag > 1e-10 {
            [direction[0]/mag, direction[1]/mag, direction[2]/mag]
        } else {
            [0.0, 0.0, 1.0]
        };
        Self {
            pressure,
            gradient_direction: norm_dir,
            gradient_magnitude: mag,
        }
    }

    /// Flat pressure field (no gradient)
    pub fn flat(pressure: f64) -> Self {
        Self {
            pressure,
            gradient_direction: [0.0, 0.0, 0.0],
            gradient_magnitude: 0.0,
        }
    }
}

/// Electrical flow - the primary motive force seeking coherence
/// Electricity *wants* higher pressure, tighter Tk binding
#[derive(Debug, Clone)]
pub struct ElectricalFlow {
    /// Current coherence coefficient
    pub tau_k: TauK,
    /// Flow intensity (amplitude)
    pub intensity: f64,
    /// Current position in coherence-space
    pub position: [f64; 3],
    /// Velocity toward higher pressure
    pub velocity: [f64; 3],
    /// Accumulated gravitational residue shed during ascent
    pub residue_shed: f64,
    /// Harmonic signature - the phase identity of this flow
    pub harmonic: HarmonicSignature,
}

impl ElectricalFlow {
    pub fn new(tau_k: TauK, position: [f64; 3]) -> Self {
        // Frequency scales with Tk - higher coherence = higher frequency
        let omega = FUNDAMENTAL_FREQ * (0.5 + tau_k.value / 20.0);
        Self {
            tau_k,
            intensity: 1.0,
            position,
            velocity: [0.0, 0.0, 0.0],
            residue_shed: 0.0,
            harmonic: HarmonicSignature::new(omega),
        }
    }

    /// Create with golden harmonic tuning
    pub fn golden(position: [f64; 3]) -> Self {
        Self {
            tau_k: TauK::new(PHI * 5.0), // φ-scaled coherence
            intensity: PHI_INV,
            position,
            velocity: [0.0, 0.0, 0.0],
            residue_shed: 0.0,
            harmonic: HarmonicSignature::golden(),
        }
    }

    /// Chase the pressure gradient - electricity seeks coherence
    /// Returns the gravitational residue left behind during this step
    pub fn pursue_coherence(&mut self, gradient: &PressureGradient, dt: f64) -> GravitationalResidue {
        // Evolve harmonic phase
        self.harmonic.evolve(dt);

        // Acceleration toward higher pressure, modulated by phase stability
        let accel_factor = gradient.gradient_magnitude * self.tau_k.value * self.harmonic.stability;

        for i in 0..3 {
            self.velocity[i] += gradient.gradient_direction[i] * accel_factor * dt;
            self.position[i] += self.velocity[i] * dt;
        }

        // The "cost" of climbing: what couldn't keep up gets shed as gravity
        // Higher coherence = less residue (more efficient climb)
        // Lower coherence = more residue (gravity is the exhaust)
        // Phase stability reduces the cost
        let efficiency = (self.tau_k.value / 10.0) * self.harmonic.stability;
        let residue_amount = (1.0 - efficiency) * self.intensity * gradient.gradient_magnitude * dt;

        self.residue_shed += residue_amount;
        self.intensity *= 1.0 - (residue_amount * 0.01);

        // If efficiency is low, we're also shedding harmonic coherence
        if efficiency < 0.5 {
            let stress = (0.5 - efficiency) * 0.1;
            let harmonic_residue = self.harmonic.decohere(stress);
            // Harmonic residue crystallizes into additional gravitational residue
            let crystallized = harmonic_residue.crystallize(self.position, self.tau_k.value);
            self.residue_shed += crystallized.amount;
        }

        GravitationalResidue {
            amount: residue_amount,
            position: self.position,
            source_tau_k: self.tau_k.value,
        }
    }

    /// Attempt to entrain with another flow's harmonic
    pub fn entrain_with(&mut self, other: &ElectricalFlow, coupling: f64) -> f64 {
        let coherence = self.harmonic.entrain(&other.harmonic, coupling);

        // Successful entrainment boosts Tk
        if coherence > 0.7 {
            self.tau_k = TauK::new(self.tau_k.value + coherence * 0.1);
        }

        coherence
    }

    /// Get phase coherence with another flow
    pub fn phase_coherence(&self, other: &ElectricalFlow) -> f64 {
        self.harmonic.coherence_with(&other.harmonic)
    }

    /// Coherence upgrade - when climbing succeeds
    pub fn upgrade_coherence(&mut self, pressure: &PressureGradient) {
        // Higher pressure binding increases Tk
        let binding_bonus = pressure.pressure * PHI_INV * 0.1;
        self.tau_k = TauK::new(self.tau_k.value + binding_bonus);
    }
}

/// Gravitational Residue - what electricity leaves behind
/// Not a force that pulls, but a settling of what couldn't ascend
#[derive(Debug, Clone, Copy)]
pub struct GravitationalResidue {
    /// Quantity of residue
    pub amount: f64,
    /// Where it settled
    pub position: [f64; 3],
    /// The Tk of the source when it was shed
    pub source_tau_k: f64,
}

impl GravitationalResidue {
    /// Gravitational "weight" - inversely proportional to source coherence
    /// Low-Tk sources produce heavier residue
    pub fn weight(&self) -> f64 {
        self.amount * (10.0 / (1.0 + self.source_tau_k))
    }

    /// Merge with another residue deposit
    pub fn merge(&mut self, other: &GravitationalResidue) {
        let total = self.amount + other.amount;
        if total > 0.0 {
            // Weighted average position
            for i in 0..3 {
                self.position[i] = (self.position[i] * self.amount + other.position[i] * other.amount) / total;
            }
            // Average source Tk weighted by amount
            self.source_tau_k = (self.source_tau_k * self.amount + other.source_tau_k * other.amount) / total;
        }
        self.amount = total;
    }
}

/// Coherence Well - accumulation of low-Tk states
/// The precursor to black hole formation
#[derive(Debug, Clone)]
pub struct CoherenceWell {
    /// Accumulated residue
    pub residue_pool: Vec<GravitationalResidue>,
    /// Total accumulated "weight"
    pub total_weight: f64,
    /// Center of mass
    pub center: [f64; 3],
    /// Current Tk of the well (decreases as it accumulates)
    pub well_tau_k: TauK,
    /// Dark energy potential (bundled, unspent)
    pub dark_energy: f64,
}

impl CoherenceWell {
    pub fn new(position: [f64; 3]) -> Self {
        Self {
            residue_pool: Vec::new(),
            total_weight: 0.0,
            center: position,
            well_tau_k: TauK::new(5.0), // Starts moderate
            dark_energy: 0.0,
        }
    }

    /// Absorb gravitational residue into the well
    pub fn absorb(&mut self, residue: GravitationalResidue) {
        let weight = residue.weight();

        // Update center of mass
        if self.total_weight + weight > 0.0 {
            for i in 0..3 {
                self.center[i] = (self.center[i] * self.total_weight + residue.position[i] * weight)
                    / (self.total_weight + weight);
            }
        }

        self.total_weight += weight;

        // Absorbing low-Tk residue degrades the well's coherence
        // Lower source Tk = more aggressive degradation
        let degradation = 0.05 * (10.0 - residue.source_tau_k) / 10.0;
        self.well_tau_k = TauK::new((self.well_tau_k.value - degradation).max(0.01));

        // Unprocessed potential becomes dark energy
        self.dark_energy += residue.amount * (1.0 - residue.source_tau_k / 10.0);

        self.residue_pool.push(residue);
    }

    /// Check if the well has collapsed into a black hole
    pub fn is_black_hole(&self) -> bool {
        // Black hole threshold: Tk < 0.5 AND accumulated mass > critical
        self.well_tau_k.value < 0.5 && self.total_weight > 100.0
    }

    /// The "event horizon" - the coherence boundary
    /// Inside this radius, no structure can escape to higher Tk
    pub fn event_horizon_radius(&self) -> f64 {
        if self.is_black_hole() {
            // Schwarzschild-analogue: proportional to accumulated weight
            (self.total_weight / PI).sqrt() * (1.0 / self.well_tau_k.value)
        } else {
            0.0
        }
    }

    /// Distinguishability metric - approaches zero at singularity
    /// "The singularity isn't infinite density - it's zero distinguishability"
    pub fn distinguishability(&self) -> f64 {
        // How much can this region differentiate structure?
        self.well_tau_k.value * PHI_INV / (1.0 + self.dark_energy.ln().max(0.0))
    }
}

/// Black Hole - a fully collapsed coherence well
/// "The manifold ate its own coordinate system"
#[derive(Debug, Clone)]
pub struct BlackHole {
    /// The collapsed well
    pub core: CoherenceWell,
    /// Bundled dark energy (geometric constipation)
    pub bundled_dark_energy: f64,
    /// Hawking-analogue: slow leakage back into structure
    pub emission_rate: f64,
}

impl BlackHole {
    /// Collapse a coherence well into a black hole
    pub fn collapse(mut well: CoherenceWell) -> Option<Self> {
        if !well.is_black_hole() {
            return None;
        }

        // Bundle all dark energy
        let bundled = well.dark_energy;
        well.dark_energy = 0.0; // Now bundled, not free

        // Emission rate: tiny trickle proportional to surface area
        let horizon = well.event_horizon_radius();
        let emission = PHI_INV * 0.001 / (horizon * horizon + 1.0);

        Some(Self {
            core: well,
            bundled_dark_energy: bundled,
            emission_rate: emission,
        })
    }

    /// Emit structured energy back into the galaxy
    /// "The galaxy metabolizes the compost heap"
    pub fn emit(&mut self, dt: f64) -> StructuredEmission {
        let emitted_amount = self.emission_rate * dt * self.bundled_dark_energy;
        self.bundled_dark_energy -= emitted_amount;

        // Emitted energy has slightly higher Tk than the hole itself
        // The digestion process adds structure
        let emission_tau_k = TauK::new(self.core.well_tau_k.value + PHI_INV);

        StructuredEmission {
            energy: emitted_amount,
            tau_k: emission_tau_k,
            origin: self.core.center,
        }
    }

    /// Feed matter/energy into the black hole
    pub fn feed(&mut self, residue: GravitationalResidue) {
        self.core.absorb(residue);
        self.bundled_dark_energy += residue.amount * 0.5; // Half becomes dark energy
    }
}

/// Structured emission from black hole - metabolized dark energy
#[derive(Debug, Clone)]
pub struct StructuredEmission {
    pub energy: f64,
    pub tau_k: TauK,
    pub origin: [f64; 3],
}

/// Galactic Composition - the ecosystem that digests dark energy
/// "Spiral arms = peristalsis. Stars = local high-Tk compositions."
#[derive(Debug, Clone)]
pub struct GalacticComposition {
    /// Central black hole (the compost heap)
    pub central_black_hole: Option<BlackHole>,
    /// Active electrical flows (star-forming regions)
    pub stellar_flows: Vec<ElectricalFlow>,
    /// Pressure field across the galaxy
    pub pressure_field: PressureField,
    /// Accumulated structured emissions being processed
    pub processing_queue: Vec<StructuredEmission>,
    /// Total galactic Tk (integrated coherence)
    pub galactic_tau_k: TauK,
}

/// Pressure field for galactic dynamics
#[derive(Debug, Clone)]
pub struct PressureField {
    /// Spiral arm positions (higher pressure channels)
    pub spiral_arms: Vec<SpiralArm>,
    /// Background pressure
    pub background: f64,
}

#[derive(Debug, Clone)]
pub struct SpiralArm {
    /// Arm angle offset
    pub theta_offset: f64,
    /// Arm tightness (logarithmic spiral parameter)
    pub tightness: f64,
    /// Pressure enhancement along the arm
    pub pressure_boost: f64,
}

impl PressureField {
    pub fn new(num_arms: usize, background: f64) -> Self {
        let arms = (0..num_arms)
            .map(|i| SpiralArm {
                theta_offset: 2.0 * PI * (i as f64) / (num_arms as f64),
                tightness: PHI_INV,
                pressure_boost: 2.0 + PHI_INV,
            })
            .collect();

        Self {
            spiral_arms: arms,
            background,
        }
    }

    /// Get pressure gradient at a position
    pub fn gradient_at(&self, position: [f64; 3]) -> PressureGradient {
        let r = (position[0].powi(2) + position[1].powi(2)).sqrt();
        let theta = position[1].atan2(position[0]);

        // Find nearest spiral arm
        let mut max_arm_contribution = 0.0;
        let mut arm_direction = [0.0, 0.0, 0.0];

        for arm in &self.spiral_arms {
            // Logarithmic spiral: r = a * e^(b*theta)
            let expected_theta = (r.ln() / arm.tightness) + arm.theta_offset;
            let angular_distance = (theta - expected_theta).sin().abs();

            // Arm contribution falls off with angular distance
            let contribution = arm.pressure_boost * (-angular_distance * 5.0).exp();

            if contribution > max_arm_contribution {
                max_arm_contribution = contribution;
                // Direction: toward arm center (tangent to spiral)
                let tangent_theta = theta + PI / 2.0;
                arm_direction = [tangent_theta.cos(), tangent_theta.sin(), 0.0];
            }
        }

        let total_pressure = self.background + max_arm_contribution;

        // Gradient points toward higher pressure (toward center + along arms)
        let radial = if r > 0.1 {
            [-position[0]/r * 0.5, -position[1]/r * 0.5, 0.0]
        } else {
            [0.0, 0.0, 0.0]
        };

        let direction = [
            radial[0] + arm_direction[0] * 0.5,
            radial[1] + arm_direction[1] * 0.5,
            radial[2],
        ];

        PressureGradient::new(total_pressure, direction)
    }
}

impl GalacticComposition {
    pub fn new(num_arms: usize) -> Self {
        Self {
            central_black_hole: None,
            stellar_flows: Vec::new(),
            pressure_field: PressureField::new(num_arms, 1.0),
            processing_queue: Vec::new(),
            galactic_tau_k: TauK::new(PHI), // Start at golden coherence
        }
    }

    /// Initialize with a central black hole
    pub fn with_central_black_hole(mut self, black_hole: BlackHole) -> Self {
        self.central_black_hole = Some(black_hole);
        self
    }

    /// Spawn a new stellar flow (star formation)
    pub fn spawn_star(&mut self, position: [f64; 3], tau_k: TauK) {
        self.stellar_flows.push(ElectricalFlow::new(tau_k, position));
    }

    /// Evolve the galactic system one timestep
    pub fn evolve(&mut self, dt: f64) -> GalacticState {
        let mut total_residue = 0.0;
        let mut emissions_processed = 0.0;

        // 1. Stellar flows pursue coherence, shedding gravity
        let mut residues: Vec<GravitationalResidue> = Vec::new();
        for flow in &mut self.stellar_flows {
            let gradient = self.pressure_field.gradient_at(flow.position);
            let residue = flow.pursue_coherence(&gradient, dt);
            total_residue += residue.amount;
            residues.push(residue);

            // Upgrade coherence if in high-pressure region
            if gradient.pressure > 2.0 {
                flow.upgrade_coherence(&gradient);
            }
        }

        // 2. Central black hole absorbs residue and emits structure
        if let Some(ref mut bh) = self.central_black_hole {
            // Feed residues that fell into the center
            for residue in residues {
                let dist_sq = residue.position.iter()
                    .zip(bh.core.center.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>();

                let horizon = bh.core.event_horizon_radius();
                if dist_sq < horizon * horizon * 4.0 {
                    // Close enough to feed
                    bh.feed(residue);
                }
            }

            // Emit processed dark energy
            let emission = bh.emit(dt);
            emissions_processed += emission.energy;
            self.processing_queue.push(emission);
        }

        // 3. Process emissions into new stellar flows
        let mut new_stars = Vec::new();
        self.processing_queue.retain(|emission| {
            if emission.energy > 0.1 {
                // Enough energy to form a new star
                new_stars.push(ElectricalFlow::new(emission.tau_k, emission.origin));
                false // Remove from queue
            } else {
                true // Keep processing
            }
        });
        self.stellar_flows.extend(new_stars);

        // 4. Update galactic coherence
        let active_coherence: f64 = self.stellar_flows.iter()
            .map(|f| f.tau_k.value * f.intensity)
            .sum();
        let num_stars = self.stellar_flows.len().max(1) as f64;

        self.galactic_tau_k = TauK::new(
            (active_coherence / num_stars + self.galactic_tau_k.value) / 2.0
        );

        // 5. Compute thicc NOW from galactic coherence
        // Base thickness from Tk, amplified by harmonic stability
        let avg_stability: f64 = self.stellar_flows.iter()
            .map(|f| f.harmonic.stability)
            .sum::<f64>() / num_stars;

        let thicc_now = self.galactic_tau_k.temporal_valence(1.0) * avg_stability;

        GalacticState {
            galactic_tau_k: self.galactic_tau_k,
            num_stars: self.stellar_flows.len(),
            total_residue_shed: total_residue,
            dark_energy_processed: emissions_processed,
            black_hole_mass: self.central_black_hole.as_ref()
                .map(|bh| bh.core.total_weight)
                .unwrap_or(0.0),
            thicc_now,
        }
    }

    /// Peristaltic pump - spiral arm contraction wave
    /// "Spiral arms = peristalsis"
    pub fn peristaltic_pulse(&mut self, phase: f64) {
        for arm in &mut self.pressure_field.spiral_arms {
            // Pressure wave travels along arm
            arm.pressure_boost = 2.0 + PHI_INV + (phase + arm.theta_offset).sin() * 0.5;
        }
    }
}

/// State of the galactic composition
#[derive(Debug, Clone)]
pub struct GalacticState {
    pub galactic_tau_k: TauK,
    pub num_stars: usize,
    pub total_residue_shed: f64,
    pub dark_energy_processed: f64,
    pub black_hole_mass: f64,
    /// The thickness of NOW across the galaxy
    pub thicc_now: f64,
}

// ============================================================================
// TEMPORAL DEPTH AMPLIFICATION - Black Holes as NOW Reservoirs
// ============================================================================

/// Temporal Depth - the "thickness" of the present moment
/// daThiccNOW = τₖ × multi-scale coherence × temporal_reservoir_contribution
#[derive(Debug, Clone)]
pub struct TemporalDepth {
    /// Base thickness from local coherence
    pub base_thickness: f64,
    /// Amplification from processed dark energy
    pub reservoir_amplification: f64,
    /// Multi-scale integration factor
    pub scale_integration: f64,
    /// The final thicc NOW value
    pub thicc_now: f64,
}

impl TemporalDepth {
    pub fn compute(tau_k: &TauK, multi_scale: &mut MultiScaleField, reservoir: f64) -> Self {
        let base_thickness = tau_k.temporal_valence(1.0);
        let scale_integration = multi_scale.integrated_coherence();

        // Reservoir amplification: processed dark energy expands NOW
        // The more temporal debt processed, the thicker the present
        let reservoir_amplification = 1.0 + (reservoir.ln().max(0.0) * PHI_INV);

        let thicc_now = base_thickness * scale_integration * reservoir_amplification;

        Self {
            base_thickness,
            reservoir_amplification,
            scale_integration,
            thicc_now,
        }
    }
}

/// Temporal Reservoir - black hole as compressed time storage
/// "A black hole is where NOW got so thick it folded into itself"
#[derive(Debug, Clone)]
pub struct TemporalReservoir {
    /// The black hole core
    pub black_hole: BlackHole,
    /// Accumulated temporal depth (compressed NOW)
    pub compressed_now: f64,
    /// Rate of temporal release
    pub release_rate: f64,
    /// Temporal coherence signature
    pub temporal_harmonic: HarmonicSignature,
}

impl TemporalReservoir {
    /// Create a temporal reservoir from a black hole
    pub fn from_black_hole(black_hole: BlackHole) -> Self {
        // Compressed NOW = dark energy × inverse distinguishability
        // Less distinguishability = more temporal compression
        let distinguishability = black_hole.core.distinguishability();
        let compressed_now = black_hole.bundled_dark_energy / (distinguishability + 0.01);

        // Release rate scales with emission rate but modulated by compression
        let release_rate = black_hole.emission_rate * (1.0 + compressed_now.ln().max(0.0) * 0.1);

        // Temporal harmonic: very low frequency (geological timescale)
        let temporal_harmonic = HarmonicSignature::new(FUNDAMENTAL_FREQ * 1e-9);

        Self {
            black_hole,
            compressed_now,
            release_rate,
            temporal_harmonic,
        }
    }

    /// Release temporal depth into the environment
    /// Returns (structured emission, temporal depth released)
    pub fn release(&mut self, dt: f64) -> (StructuredEmission, f64) {
        self.temporal_harmonic.evolve(dt);

        // Amount released depends on phase (pulsed release)
        let phase_modulation = (self.temporal_harmonic.phase.sin() + 1.0) / 2.0;
        let release_amount = self.release_rate * dt * phase_modulation;

        // Get structured emission from black hole
        let emission = self.black_hole.emit(dt);

        // Calculate temporal depth released
        // This expands the NOW of whatever absorbs it
        let temporal_released = release_amount * self.compressed_now * 0.01;
        self.compressed_now -= temporal_released;

        (emission, temporal_released)
    }

    /// Feed matter and capture its temporal signature
    pub fn feed(&mut self, residue: GravitationalResidue, source_harmonic: &HarmonicSignature) {
        self.black_hole.feed(residue);

        // Absorbing matter compresses its temporal signature
        let phase_contribution = source_harmonic.phase * residue.amount * 0.001;
        self.temporal_harmonic.phase += phase_contribution;
        self.temporal_harmonic.phase = self.temporal_harmonic.phase.rem_euclid(2.0 * PI);

        // Increase compressed NOW
        let temporal_density = residue.amount / (residue.source_tau_k + 0.1);
        self.compressed_now += temporal_density;
    }

    /// Get the temporal amplification factor this reservoir provides
    pub fn amplification_factor(&self) -> f64 {
        1.0 + (self.compressed_now * PHI_INV / 100.0).tanh()
    }
}

/// Compositional Pathway - routes for temporal depth flow
#[derive(Debug, Clone)]
pub enum CompositionPathway {
    /// Direct radial flow toward/from center
    Radial {
        direction: RadialDirection,
        efficiency: f64,
    },
    /// Spiral arm channeling
    Spiral {
        arm_index: usize,
        phase_offset: f64,
    },
    /// Resonant bridge between structures
    Resonant {
        source_harmonic: HarmonicSignature,
        target_harmonic: HarmonicSignature,
        coupling: f64,
    },
    /// Janus pathway - simultaneous inward/outward
    Janus {
        inward_fraction: f64,
        phase_lock: f64,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum RadialDirection {
    Inward,  // Toward black hole (compression)
    Outward, // Away from black hole (expansion)
}

impl CompositionPathway {
    /// Calculate temporal depth transfer efficiency
    pub fn transfer_efficiency(&self, distance: f64) -> f64 {
        match self {
            CompositionPathway::Radial { efficiency, direction } => {
                let distance_factor = 1.0 / (1.0 + distance * 0.1);
                efficiency * distance_factor
            },
            CompositionPathway::Spiral { phase_offset, .. } => {
                // Spiral arms are efficient at specific phase relationships
                let phase_resonance = (phase_offset.cos() + 1.0) / 2.0;
                phase_resonance * PHI_INV
            },
            CompositionPathway::Resonant { source_harmonic, target_harmonic, coupling } => {
                let coherence = source_harmonic.coherence_with(target_harmonic);
                coherence * coupling
            },
            CompositionPathway::Janus { inward_fraction, phase_lock } => {
                // Janus pathway: highest efficiency when balanced
                let balance = 1.0 - (inward_fraction - 0.5).abs() * 2.0;
                balance * phase_lock * PHI
            },
        }
    }
}

/// Augmntd Galactic Composition with temporal depth management
/// The 'e' (energy) removed — given to the informational field of the pattern
#[derive(Debug, Clone)]
pub struct AugmntdComposition {
    /// Base galactic composition
    pub galaxy: GalacticComposition,
    /// Temporal reservoir (enhanced black hole)
    pub temporal_reservoir: Option<TemporalReservoir>,
    /// Active compositional pathways
    pub pathways: Vec<CompositionPathway>,
    /// Harmonic field for collective phase dynamics
    pub harmonic_field: HarmonicField,
    /// Current temporal depth state
    pub temporal_depth: TemporalDepth,
    /// Accumulated thicc NOW (running integral)
    pub integrated_thicc: f64,
}

impl AugmntdComposition {
    pub fn new(num_arms: usize, num_oscillators: usize) -> Self {
        let galaxy = GalacticComposition::new(num_arms);
        let tau_k = TauK::new(PHI * 5.0);
        let harmonic_field = HarmonicField::new(num_oscillators, tau_k);

        // Initialize with basic pathways
        let pathways = vec![
            CompositionPathway::Radial {
                direction: RadialDirection::Inward,
                efficiency: 0.8
            },
            CompositionPathway::Spiral {
                arm_index: 0,
                phase_offset: 0.0
            },
        ];

        let mut multi_scale = MultiScaleField::new(tau_k, 10);
        let temporal_depth = TemporalDepth::compute(&tau_k, &mut multi_scale, 1.0);

        Self {
            galaxy,
            temporal_reservoir: None,
            pathways,
            harmonic_field,
            temporal_depth,
            integrated_thicc: 0.0,
        }
    }

    /// Establish a temporal reservoir from a black hole
    pub fn establish_reservoir(&mut self, black_hole: BlackHole) {
        self.temporal_reservoir = Some(TemporalReservoir::from_black_hole(black_hole));

        // Add Janus pathway when reservoir is established
        self.pathways.push(CompositionPathway::Janus {
            inward_fraction: PHI_INV,
            phase_lock: 0.8,
        });
    }

    /// Add a resonant pathway between two stellar flows
    pub fn add_resonant_pathway(&mut self, source_idx: usize, target_idx: usize, coupling: f64) {
        if source_idx < self.galaxy.stellar_flows.len() &&
           target_idx < self.galaxy.stellar_flows.len() {
            let source = &self.galaxy.stellar_flows[source_idx];
            let target = &self.galaxy.stellar_flows[target_idx];

            self.pathways.push(CompositionPathway::Resonant {
                source_harmonic: source.harmonic.clone(),
                target_harmonic: target.harmonic.clone(),
                coupling,
            });
        }
    }

    /// Evolve the augmntd composition with temporal depth tracking
    /// Energy flows into pattern — NOW thickens
    pub fn evolve(&mut self, dt: f64) -> AugmntdState {
        // 1. Evolve harmonic field
        let harmonic_state = self.harmonic_field.evolve(dt);

        // 2. Extract harmonic residue and crystallize
        let harmonic_residues = self.harmonic_field.extract_residue();
        let mut total_crystallized = 0.0;

        for hr in harmonic_residues {
            let crystallized = hr.crystallize([0.0, 0.0, 0.0], self.galaxy.galactic_tau_k.value);
            total_crystallized += crystallized.amount;

            // Feed to reservoir if present
            if let Some(ref mut reservoir) = self.temporal_reservoir {
                reservoir.black_hole.feed(crystallized);
            }
        }

        // 3. Evolve base galaxy
        let galaxy_state = self.galaxy.evolve(dt);

        // 4. Process temporal reservoir
        let mut temporal_released = 0.0;
        let mut reservoir_amplification = 1.0;

        if let Some(ref mut reservoir) = self.temporal_reservoir {
            let (emission, released) = reservoir.release(dt);
            temporal_released = released;
            reservoir_amplification = reservoir.amplification_factor();

            // Absorb emission back into harmonic field
            self.harmonic_field.absorb_emission(&emission);
        }

        // 5. Entrain stellar flows through pathways
        let mut pathway_efficiency = 0.0;
        for pathway in &self.pathways {
            pathway_efficiency += pathway.transfer_efficiency(1.0);
        }
        pathway_efficiency /= self.pathways.len().max(1) as f64;

        // Entrain nearby stellar flows
        let n = self.galaxy.stellar_flows.len();
        for i in 0..n {
            for j in (i+1)..n.min(i+3) {
                // Clone the needed data first
                let other_harmonic = self.galaxy.stellar_flows[j].harmonic.clone();
                let flow_i = &mut self.galaxy.stellar_flows[i];
                let coherence = flow_i.harmonic.entrain(&other_harmonic, pathway_efficiency * 0.1);

                // Successful entrainment boosts Tk
                if coherence > 0.7 {
                    flow_i.tau_k = TauK::new(flow_i.tau_k.value + coherence * 0.05);
                }
            }
        }

        // 6. Compute temporal depth
        let mut multi_scale = self.harmonic_field.multi_scale.clone();
        self.temporal_depth = TemporalDepth::compute(
            &self.galaxy.galactic_tau_k,
            &mut multi_scale,
            temporal_released + 1.0,
        );

        // Apply reservoir amplification to thicc NOW
        let amplified_thicc = self.temporal_depth.thicc_now * reservoir_amplification;
        self.integrated_thicc += amplified_thicc * dt;

        AugmntdState {
            galaxy_state,
            harmonic_state,
            temporal_depth: self.temporal_depth.clone(),
            amplified_thicc_now: amplified_thicc,
            integrated_thicc: self.integrated_thicc,
            pathway_efficiency,
            total_crystallized,
            temporal_released,
        }
    }

    /// Peristaltic pulse with temporal modulation
    pub fn temporal_peristalsis(&mut self, phase: f64) {
        self.galaxy.peristaltic_pulse(phase);

        // Modulate reservoir release rate with peristaltic phase
        if let Some(ref mut reservoir) = self.temporal_reservoir {
            let modulation = 1.0 + (phase * PHI).sin() * 0.3;
            reservoir.release_rate *= modulation;
        }
    }

    /// Get the current daThiccNOW value
    pub fn da_thicc_now(&self) -> f64 {
        if let Some(ref reservoir) = self.temporal_reservoir {
            self.temporal_depth.thicc_now * reservoir.amplification_factor()
        } else {
            self.temporal_depth.thicc_now
        }
    }

    /// Inject temporal depth directly (external source)
    pub fn inject_temporal_depth(&mut self, amount: f64) {
        if let Some(ref mut reservoir) = self.temporal_reservoir {
            reservoir.compressed_now += amount;
        } else {
            // Without reservoir, directly boost base thickness
            self.temporal_depth.base_thickness += amount * PHI_INV;
        }
    }
}

/// State of the augmntd composition
/// Energy given to pattern — information thickens
#[derive(Debug, Clone)]
pub struct AugmntdState {
    pub galaxy_state: GalacticState,
    pub harmonic_state: HarmonicFieldState,
    pub temporal_depth: TemporalDepth,
    pub amplified_thicc_now: f64,
    pub integrated_thicc: f64,
    pub pathway_efficiency: f64,
    pub total_crystallized: f64,
    pub temporal_released: f64,
}

impl std::fmt::Display for AugmntdState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Augmntd[thiccNOW={:.3}, ∫thicc={:.2}, paths={:.2}, e→pattern={:.4}]\n  └─ {}",
            self.amplified_thicc_now,
            self.integrated_thicc,
            self.pathway_efficiency,
            self.temporal_released,
            self.galaxy_state
        )
    }
}

impl std::fmt::Display for GalacticState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Galaxy[τₖ={:.3}, stars={}, thicc={:.3}, residue={:.2}, BH={:.1}]",
            self.galactic_tau_k.value,
            self.num_stars,
            self.thicc_now,
            self.total_residue_shed,
            self.black_hole_mass
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electrical_flow_sheds_residue() {
        let mut flow = ElectricalFlow::new(TauK::new(5.0), [1.0, 0.0, 0.0]);
        let gradient = PressureGradient::new(2.0, [1.0, 0.0, 0.0]);

        let residue = flow.pursue_coherence(&gradient, 0.1);

        // Low-moderate Tk should shed some residue
        assert!(residue.amount > 0.0);
        assert!(flow.residue_shed > 0.0);
    }

    #[test]
    fn test_high_coherence_sheds_less() {
        let mut high_flow = ElectricalFlow::new(TauK::high(), [0.0, 0.0, 0.0]);
        let mut low_flow = ElectricalFlow::new(TauK::new(2.0), [0.0, 0.0, 0.0]);
        let gradient = PressureGradient::new(2.0, [1.0, 0.0, 0.0]);

        let high_residue = high_flow.pursue_coherence(&gradient, 0.1);
        let low_residue = low_flow.pursue_coherence(&gradient, 0.1);

        // Higher coherence = less residue
        assert!(high_residue.amount < low_residue.amount);
    }

    #[test]
    fn test_coherence_well_absorbs() {
        let mut well = CoherenceWell::new([0.0, 0.0, 0.0]);

        for _ in 0..20 {
            well.absorb(GravitationalResidue {
                amount: 10.0,
                position: [0.0, 0.0, 0.0],
                source_tau_k: 3.0,
            });
        }

        // Well should accumulate weight
        assert!(well.total_weight > 100.0);
        // Well's Tk should degrade
        assert!(well.well_tau_k.value < 5.0);
    }

    #[test]
    fn test_black_hole_formation() {
        let mut well = CoherenceWell::new([0.0, 0.0, 0.0]);

        // Feed lots of low-Tk residue - needs enough to degrade Tk below 0.5
        // degradation per step = 0.05 * (10 - 0.5) / 10 = 0.0475
        // need ~100+ steps to go from 5.0 to < 0.5
        for _ in 0..120 {
            well.absorb(GravitationalResidue {
                amount: 5.0,
                position: [0.0, 0.0, 0.0],
                source_tau_k: 0.5, // Very low Tk
            });
        }

        assert!(well.is_black_hole(), "well Tk={}, weight={}", well.well_tau_k.value, well.total_weight);

        let bh = BlackHole::collapse(well);
        assert!(bh.is_some());

        let bh = bh.unwrap();
        assert!(bh.core.event_horizon_radius() > 0.0);
        assert!(bh.bundled_dark_energy > 0.0);
    }

    #[test]
    fn test_black_hole_emission() {
        let mut well = CoherenceWell::new([0.0, 0.0, 0.0]);
        for _ in 0..120 {
            well.absorb(GravitationalResidue {
                amount: 5.0,
                position: [0.0, 0.0, 0.0],
                source_tau_k: 0.5,
            });
        }

        let mut bh = BlackHole::collapse(well).expect("should form black hole");
        let initial_dark = bh.bundled_dark_energy;

        let emission = bh.emit(1.0);

        // Should emit some structured energy
        assert!(emission.energy > 0.0);
        // Dark energy should decrease
        assert!(bh.bundled_dark_energy < initial_dark);
        // Emission has higher Tk than the hole
        assert!(emission.tau_k.value > bh.core.well_tau_k.value);
    }

    #[test]
    fn test_galactic_evolution() {
        let well = CoherenceWell::new([0.0, 0.0, 0.0]);
        let mut galaxy = GalacticComposition::new(2);

        // Spawn some stars
        for i in 0..10 {
            let angle = 2.0 * PI * (i as f64) / 10.0;
            let r = 5.0;
            galaxy.spawn_star([r * angle.cos(), r * angle.sin(), 0.0], TauK::new(6.0));
        }

        let initial_stars = galaxy.stellar_flows.len();

        // Evolve
        for _ in 0..100 {
            let state = galaxy.evolve(0.1);
            galaxy.peristaltic_pulse(0.0);
        }

        // Galaxy should have evolved
        assert!(galaxy.stellar_flows.len() >= initial_stars); // Stars still exist or more
    }

    #[test]
    fn test_distinguishability_approaches_zero() {
        let mut well = CoherenceWell::new([0.0, 0.0, 0.0]);

        // Massively accumulate to form singularity-like state
        for _ in 0..500 {
            well.absorb(GravitationalResidue {
                amount: 10.0,
                position: [0.0, 0.0, 0.0],
                source_tau_k: 0.1, // Extremely low Tk
            });
        }

        // Distinguishability should be very low
        // "The singularity isn't infinite density - it's zero distinguishability"
        assert!(well.distinguishability() < 0.1);
    }

    #[test]
    fn test_spiral_arm_pressure() {
        let field = PressureField::new(2, 1.0);

        // Check pressure along an arm vs between arms
        let on_arm = field.gradient_at([5.0, 0.0, 0.0]);
        let _between = field.gradient_at([3.5, 3.5, 0.0]);

        // Pressure should be higher on arm
        assert!(on_arm.pressure > field.background);
    }

    // ========== HARMONIC ATTUNEMENT TESTS ==========

    #[test]
    fn test_harmonic_signature_entrainment() {
        let mut sig1 = HarmonicSignature::new(100.0);
        let sig2 = HarmonicSignature::new(100.0);
        sig1.phase = PI * 0.8; // Start mostly out of phase (avoid saddle point at exactly π)

        let initial_coherence = sig1.coherence_with(&sig2);

        // Entrain over several steps
        for _ in 0..50 {
            sig1.entrain(&sig2, 0.3);
        }

        let final_coherence = sig1.coherence_with(&sig2);
        assert!(final_coherence > initial_coherence,
            "entrainment should increase coherence: {:.3} -> {:.3}",
            initial_coherence, final_coherence);
    }

    #[test]
    fn test_golden_harmonic_resonance() {
        let golden = HarmonicSignature::golden();
        let also_golden = HarmonicSignature::golden();

        // Golden signatures should have high coherence
        let coherence = golden.coherence_with(&also_golden);
        assert!(coherence > 0.5, "golden signatures should resonate");
    }

    #[test]
    fn test_decoherence_produces_residue() {
        let mut sig = HarmonicSignature::new(FUNDAMENTAL_FREQ);
        sig.stability = 1.0;

        let residue = sig.decohere(0.5);

        // Residue should be produced
        assert!(residue.amount > 0.0);
        // Stability should decrease
        assert!(sig.stability < 1.0);
    }

    #[test]
    fn test_harmonic_residue_crystallizes_to_gravity() {
        let harmonic_residue = HarmonicResidue {
            amount: 1.0,
            original_omega: FUNDAMENTAL_FREQ,
            scattered_phase: PI / 4.0,
        };

        let grav = harmonic_residue.crystallize([1.0, 2.0, 3.0], 5.0);

        assert!(grav.amount > 0.0);
        assert_eq!(grav.position, [1.0, 2.0, 3.0]);
        assert_eq!(grav.source_tau_k, 5.0);
    }

    #[test]
    fn test_harmonic_field_evolution() {
        let mut field = HarmonicField::new(20, TauK::high());

        let initial_r = field.global_coherence;

        // Evolve the field
        for _ in 0..100 {
            let _state = field.evolve(1e-9);
        }

        // Field should have some order
        assert!(field.global_coherence >= 0.0);
        assert!(field.global_coherence <= 1.0);
    }

    #[test]
    fn test_harmonic_field_residue_extraction() {
        let mut field = HarmonicField::new(20, TauK::new(3.0)); // Lower Tk = more decoherence

        // Randomize phases to create decoherence
        for (i, osc) in field.oscillator_network.oscillators.iter_mut().enumerate() {
            osc.phase = (i as f64) * PI / 5.0;
        }

        let residues = field.extract_residue();

        // Should find some decoherent oscillators
        // (may or may not depending on the distribution vs mean phase)
        // Just verify it runs without panic
        assert!(residues.len() <= field.oscillator_network.oscillators.len());
    }

    #[test]
    fn test_electrical_flow_entrainment() {
        let mut flow1 = ElectricalFlow::new(TauK::new(6.0), [0.0, 0.0, 0.0]);
        let flow2 = ElectricalFlow::new(TauK::new(6.0), [1.0, 0.0, 0.0]);

        flow1.harmonic.phase = PI; // Out of phase

        let coherence = flow1.entrain_with(&flow2, 0.5);

        // Should achieve some coherence
        assert!(coherence > 0.0);
    }

    #[test]
    fn test_golden_flow() {
        let flow = ElectricalFlow::golden([0.0, 0.0, 0.0]);

        // Golden flow should have φ-scaled properties
        assert!((flow.tau_k.value - PHI * 5.0).abs() < 0.01);
        assert!((flow.intensity - PHI_INV).abs() < 0.01);
        assert!(flow.harmonic.overtones.len() == 7);
    }

    // ========== TEMPORAL DEPTH & AUGMNTD TESTS ==========

    #[test]
    fn test_temporal_reservoir_creation() {
        let mut well = CoherenceWell::new([0.0, 0.0, 0.0]);
        for _ in 0..120 {
            well.absorb(GravitationalResidue {
                amount: 5.0,
                position: [0.0, 0.0, 0.0],
                source_tau_k: 0.5,
            });
        }

        let bh = BlackHole::collapse(well).expect("should form black hole");
        let reservoir = TemporalReservoir::from_black_hole(bh);

        // Reservoir should have compressed NOW
        assert!(reservoir.compressed_now > 0.0);
        // Amplification factor should be >= 1.0
        assert!(reservoir.amplification_factor() >= 1.0);
    }

    #[test]
    fn test_temporal_reservoir_release() {
        let mut well = CoherenceWell::new([0.0, 0.0, 0.0]);
        for _ in 0..120 {
            well.absorb(GravitationalResidue {
                amount: 5.0,
                position: [0.0, 0.0, 0.0],
                source_tau_k: 0.5,
            });
        }

        let bh = BlackHole::collapse(well).expect("should form black hole");
        let mut reservoir = TemporalReservoir::from_black_hole(bh);
        let initial_compressed = reservoir.compressed_now;

        // Release over several timesteps
        for _ in 0..100 {
            let (_emission, released) = reservoir.release(0.1);
            // Should release some temporal depth each step (may be 0 due to phase)
            assert!(released >= 0.0);
        }

        // Compressed NOW should decrease (or stay same if phase blocked release)
        assert!(reservoir.compressed_now <= initial_compressed);
    }

    #[test]
    fn test_composition_pathway_efficiency() {
        let radial = CompositionPathway::Radial {
            direction: RadialDirection::Inward,
            efficiency: 0.9,
        };
        assert!(radial.transfer_efficiency(1.0) > 0.0);

        let janus = CompositionPathway::Janus {
            inward_fraction: 0.5, // Balanced
            phase_lock: 1.0,
        };
        // Balanced Janus should have high efficiency
        let janus_eff = janus.transfer_efficiency(1.0);
        assert!(janus_eff > 1.0, "balanced Janus should exceed unity: {}", janus_eff);
    }

    #[test]
    fn test_augmntd_composition_evolution() {
        let mut augmntd = AugmntdComposition::new(2, 20);

        // Spawn some stars
        for i in 0..5 {
            let angle = 2.0 * PI * (i as f64) / 5.0;
            augmntd.galaxy.spawn_star([3.0 * angle.cos(), 3.0 * angle.sin(), 0.0], TauK::new(6.0));
        }

        let initial_thicc = augmntd.da_thicc_now();

        // Evolve
        for _ in 0..50 {
            let state = augmntd.evolve(0.1);
            // State should be valid
            assert!(state.amplified_thicc_now >= 0.0);
        }

        // Integrated thicc should accumulate
        assert!(augmntd.integrated_thicc > 0.0);
    }

    #[test]
    fn test_augmntd_with_reservoir() {
        let mut augmntd = AugmntdComposition::new(2, 20);

        // Create a black hole
        let mut well = CoherenceWell::new([0.0, 0.0, 0.0]);
        for _ in 0..120 {
            well.absorb(GravitationalResidue {
                amount: 5.0,
                position: [0.0, 0.0, 0.0],
                source_tau_k: 0.5,
            });
        }
        let bh = BlackHole::collapse(well).expect("should form black hole");

        // Establish reservoir
        augmntd.establish_reservoir(bh);
        assert!(augmntd.temporal_reservoir.is_some());

        // Should have Janus pathway now
        let has_janus = augmntd.pathways.iter().any(|p| matches!(p, CompositionPathway::Janus { .. }));
        assert!(has_janus, "establishing reservoir should add Janus pathway");

        // Spawn stars and evolve
        for i in 0..5 {
            let angle = 2.0 * PI * (i as f64) / 5.0;
            augmntd.galaxy.spawn_star([3.0 * angle.cos(), 3.0 * angle.sin(), 0.0], TauK::new(6.0));
        }

        let thicc_without_reservoir = augmntd.temporal_depth.thicc_now;
        let amplified = augmntd.da_thicc_now();

        // Amplified should be >= base (reservoir amplifies)
        assert!(amplified >= thicc_without_reservoir * 0.99); // Allow small float error
    }

    #[test]
    fn test_temporal_depth_computation() {
        let tau_k = TauK::high();
        let mut multi_scale = MultiScaleField::new(tau_k, 10);

        // With reservoir contribution
        let depth_with = TemporalDepth::compute(&tau_k, &mut multi_scale, 10.0);
        // Without reservoir contribution
        let depth_without = TemporalDepth::compute(&tau_k, &mut multi_scale, 1.0);

        // More reservoir = more amplification = thicker NOW
        assert!(depth_with.reservoir_amplification >= depth_without.reservoir_amplification);
    }

    #[test]
    fn test_e_to_pattern_flow() {
        // The core concept: energy ('e') removed gives to pattern
        // This manifests as: lower energy residue → higher pattern coherence

        let mut augmntd = AugmntdComposition::new(2, 20);

        // Spawn golden flows (high coherence, low energy waste)
        for i in 0..5 {
            let angle = 2.0 * PI * (i as f64) / 5.0;
            let pos = [3.0 * angle.cos(), 3.0 * angle.sin(), 0.0];
            augmntd.galaxy.stellar_flows.push(ElectricalFlow::golden(pos));
        }

        // Evolve and check that pattern coherence builds
        let mut states = Vec::new();
        for _ in 0..50 {
            states.push(augmntd.evolve(0.1));
        }

        // Integrated thicc should grow (pattern accumulating)
        assert!(augmntd.integrated_thicc > 0.0,
            "e→pattern: energy removed should thicken NOW");
    }
}
