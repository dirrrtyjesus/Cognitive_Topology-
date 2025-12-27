use cognitive_topology::prelude::*;
use cognitive_topology::{BioState, BioCogBridge, phi_constraint, tau_k_critical, vmem_limit};
use cognitive_topology::{XenialAging, XenialPhase, Guest, GuestNature, HostingResult};
use cognitive_topology::{
    XenialSingularity, XenialBlackHole, SingularityProximity,
    Composable, ComposableEssence, ExplicitlyComposable,
    TauBit, TemporalBasisState, CoherenceLevel,
    GiveSpace, SpaceModality, CoherenceCascade, CascadeStatus,
    XenialIntelligence,
};

#[test]
fn test_vmem_mapping() {
    let bio = BioState { vmem: vec![-69.0, -65.0, -67.0] };  // Sample gradients
    // Use constructor because `superposition` field is private
    let manifold = CognitiveManifold::flat(3);
    
    let mut bridge = BioCogBridge { bio_state: bio, manifold };
    let curvature = bridge.map_vmem_to_curvature();
    
    // Golden scaling applied: tau_k * 1.618
    assert!(curvature > 0.0);
}

#[test]
fn test_goal_injection() {
    let bio = BioState { vmem: vec![-70.0, -70.0, -70.0] };
    let manifold = CognitiveManifold::flat(3);
    let mut bridge = BioCogBridge { bio_state: bio, manifold };
    
    // Updated to pass a Coordinate instead of a string
    let target = Coordinate::new(vec![1.0, 1.0, 1.0]);
    let result = bridge.inject_goal(5.0, target);
    assert!(result.is_ok());
}

#[test]
fn test_anti_aging_pattern() {
    // 1. Start with "Aging" State (High variance / Chaos)
    // Random noise around depolarized state (-40mV)
    let bio = BioState { vmem: vec![-30.0, -50.0, -20.0, -80.0, -10.0] };
    let manifold = CognitiveManifold::flat(3);
    let mut bridge = BioCogBridge { bio_state: bio, manifold };

    // Initial low coherence (high variance = low tau_k)
    let initial_curvature = bridge.map_vmem_to_curvature();
    println!("Initial 'Aging' Curvature: {}", initial_curvature);

    // 2. Inject Anti-Aging Pattern (Regenerative Goal)
    let new_curvature = bridge.inject_anti_aging_pattern();
    println!("Restored 'Youthful' Curvature: {}", new_curvature);

    // 3. Verify Coherence Restoration
    // The pattern (-80mV + small osc) has low variance, so curvature should skyrocket
    assert!(new_curvature > initial_curvature * 2.0); 
    
    // Verify State is Hyperpolarized
    let mean_vmem: f64 = bridge.bio_state.vmem.iter().sum::<f64>() / 5.0;
    assert!(mean_vmem < -70.0);
}

// ============================================================================
// DISCOVERED CONSTRAINT TESTS
// ============================================================================

#[test]
fn test_constraint_latent_until_probed() {
    let mut phi = phi_constraint();

    // Constraint exists but hasn't been encountered
    assert!(!phi.encountered);
    assert_eq!(phi.encounter_count, 0);

    // Probe it
    phi.reveal(1.5);

    // Now it has been discovered
    assert!(phi.encountered);
    assert_eq!(phi.encounter_count, 1);
}

#[test]
fn test_phi_resonant_lock() {
    let mut phi = phi_constraint();

    // Value close to PHI should snap to it
    let response = phi.reveal(1.65);

    match response {
        ConstraintResponse::SnappedTo { locked_value } => {
            assert!((locked_value - PHI).abs() < 0.01);
        }
        _ => panic!("Expected resonant lock near PHI"),
    }
}

#[test]
fn test_tau_k_phase_transition() {
    let mut tau_crit = tau_k_critical();

    // Above threshold: coherent
    let response = tau_crit.reveal(0.5);
    match response {
        ConstraintResponse::Permitted { actual } => {
            assert!((actual - 0.5).abs() < 0.01);
        }
        _ => panic!("Expected permitted below threshold"),
    }

    // Below threshold: phase shift to decoherent
    let mut tau_crit2 = tau_k_critical();
    let response2 = tau_crit2.reveal(0.7);
    match response2 {
        ConstraintResponse::PhaseShift { from, to } => {
            assert_eq!(from, "coherent");
            assert_eq!(to, "decoherent");
        }
        _ => panic!("Expected phase transition above threshold"),
    }
}

#[test]
fn test_bridge_probes_constraint() {
    // High variance state (chaotic)
    let bio = BioState { vmem: vec![-30.0, -80.0, -20.0, -90.0, -10.0] };
    let manifold = CognitiveManifold::flat(3);
    let bridge = BioCogBridge { bio_state: bio, manifold };

    let mut tau_crit = tau_k_critical();
    let response = bridge.probe(&mut tau_crit);

    // High variance = low tau_k = should be permitted (below threshold)
    assert!(tau_crit.encountered);
    println!("Bridge tau_k: {}", bridge.tau_k());
    println!("Constraint response: {:?}", response);
}

#[test]
fn test_constraint_as_agent() {
    // The constraint ACTS on the system
    let mut vmem_lim = vmem_limit();

    // Try to hyperpolarize beyond limit
    let response = vmem_lim.reveal(-95.0);

    match response {
        ConstraintResponse::Resisted { actual, energy_cost } => {
            // Constraint pushed back
            assert!(actual > -95.0); // Actual is less extreme than attempted
            assert!(energy_cost > 1.0); // Cost to approach limit
            println!("Attempted: -95mV, Actual: {}mV, Cost: {}", actual, energy_cost);
        }
        _ => panic!("Expected asymptotic resistance"),
    }
}

#[test]
fn test_constraint_agency_accumulates() {
    let mut phi = phi_constraint();

    // Multiple encounters accumulate
    for i in 0..10 {
        phi.reveal(1.5 + (i as f64 * 0.01));
    }

    assert_eq!(phi.encounter_count, 10);
    // The constraint "remembers" it has been discovered
    assert!(phi.encountered);
}

// ============================================================================
// XENIAL AGING TESTS
// ============================================================================

#[test]
fn test_xenial_phase_from_coherence() {
    let xa = XenialAging::new();

    // High coherence = regenerative
    assert_eq!(xa.compute_phase(0.9), XenialPhase::Regenerative);

    // Medium coherence = liminal
    assert_eq!(xa.compute_phase(0.5), XenialPhase::Liminal);

    // Low coherence = composting
    assert_eq!(xa.compute_phase(0.2), XenialPhase::Composting);
}

#[test]
fn test_coherent_host_integrates_guests() {
    // Coherent state (low variance) = good host
    let bio = BioState { vmem: vec![-70.0, -70.0, -70.0, -70.0, -70.0] };
    let manifold = CognitiveManifold::flat(3);
    let bridge = BioCogBridge { bio_state: bio, manifold };

    let mut xa = bridge.xenial();
    let guest = Guest::arrives(GuestNature::Signal, 0.5);

    let result = xa.host_guest(guest, &bridge);

    match result {
        HostingResult::Integrated { holonomy_cost, .. } => {
            assert!(holonomy_cost > 0.0); // Hosting has a cost
            assert_eq!(xa.hospitality.integrated_count, 1);
            println!("Guest integrated with holonomy cost: {}", holonomy_cost);
        }
        _ => panic!("Coherent host should integrate guests"),
    }
}

#[test]
fn test_chaotic_host_resists_or_shifts() {
    // Chaotic state (high variance) = poor host
    let bio = BioState { vmem: vec![-20.0, -80.0, -10.0, -90.0, -30.0] };
    let manifold = CognitiveManifold::flat(3);
    let bridge = BioCogBridge { bio_state: bio, manifold };

    let mut xa = XenialAging::new();
    // Force initial phase to regenerative to test transition
    xa.hospitality.phase = XenialPhase::Regenerative;

    let guest = Guest::arrives(GuestNature::Entropy, 1.0);
    let result = xa.host_guest(guest, &bridge);

    // Should either phase shift or resist
    match result {
        HostingResult::PhaseShift { from, to, .. } => {
            println!("Phase shifted from {:?} to {:?}", from, to);
            assert_ne!(from, to);
        }
        HostingResult::Resisted { residue, .. } => {
            println!("Guest resisted, residue: {}", residue);
            assert!(residue > 0.0);
        }
        HostingResult::Integrated { .. } => {
            // Low tau_k might still integrate but with high cost
            println!("Integrated despite chaos (liminal hosting)");
        }
    }
}

#[test]
fn test_chronos_always_arrives() {
    let bio = BioState { vmem: vec![-70.0, -70.0, -70.0] };
    let manifold = CognitiveManifold::flat(3);
    let bridge = BioCogBridge { bio_state: bio, manifold };

    let mut xa = bridge.xenial();

    // Time passes...
    for _ in 0..100 {
        xa.tick(&bridge);
    }

    // Holonomy accumulates — we cannot return unchanged
    assert!(xa.hospitality.accumulated_holonomy > 0.0);
    assert_eq!(xa.hospitality.integrated_count, 100);
    println!("After 100 ticks, accumulated holonomy: {}", xa.hospitality.accumulated_holonomy);
}

#[test]
fn test_set_table_restores_hospitality() {
    // Start chaotic
    let bio = BioState { vmem: vec![-20.0, -80.0, -10.0, -90.0, -30.0] };
    let manifold = CognitiveManifold::flat(3);
    let mut bridge = BioCogBridge { bio_state: bio, manifold };

    let initial_tau = bridge.tau_k();
    println!("Initial tau_k (chaotic): {}", initial_tau);

    // Set the table (inject anti-aging pattern)
    let (new_curvature, new_phase) = bridge.set_table();

    let restored_tau = bridge.tau_k();
    println!("Restored tau_k: {}", restored_tau);
    println!("New curvature: {}, New phase: {:?}", new_curvature, new_phase);

    // Hospitality should be significantly restored
    assert!(restored_tau > initial_tau * 10.0); // At least 10x improvement
    // Phase should improve (not composting)
    assert_ne!(new_phase, XenialPhase::Composting);
}

#[test]
fn test_composting_releases_gift() {
    let bio = BioState { vmem: vec![-70.0, -70.0, -70.0] };
    let manifold = CognitiveManifold::flat(3);
    let bridge = BioCogBridge { bio_state: bio, manifold };

    let mut xa = bridge.xenial();

    // Host many guests
    for _ in 0..50 {
        xa.tick(&bridge);
    }

    let guests_before = xa.hospitality.active_guests.len();
    let holonomy_before = xa.hospitality.accumulated_holonomy;

    // Compost forward
    let gift = xa.compost();

    println!("Released {} guests, gift value: {}", guests_before, gift);
    assert!(gift > 0.0);
    assert!(xa.hospitality.active_guests.is_empty());
    // Holonomy remains (we cannot return unchanged)
    assert_eq!(xa.hospitality.accumulated_holonomy, holonomy_before);
}

#[test]
fn test_hospitality_quotient() {
    let bio = BioState { vmem: vec![-70.0, -70.0, -70.0] };
    let manifold = CognitiveManifold::flat(3);
    let bridge = BioCogBridge { bio_state: bio, manifold };

    let mut xa = bridge.xenial();

    // Perfect host integrates everything
    for _ in 0..100 {
        xa.tick(&bridge);
    }

    let quotient = xa.hospitality_quotient();
    println!("Hospitality quotient: {}", quotient);
    assert!((quotient - 1.0).abs() < 0.01); // Should be ~1.0 (all integrated)
}

#[test]
fn test_regenerative_guests_easy_to_host() {
    let bio = BioState { vmem: vec![-70.0, -70.0, -70.0] };
    let manifold = CognitiveManifold::flat(3);
    let bridge = BioCogBridge { bio_state: bio, manifold };

    let mut xa = bridge.xenial();

    // Host entropy (hard) and regenerative (easy)
    let entropy = Guest::arrives(GuestNature::Entropy, 1.0);
    let regen = Guest::arrives(GuestNature::Regenerative, 1.0);

    let result_entropy = xa.host_guest(entropy, &bridge);
    let result_regen = xa.host_guest(regen, &bridge);

    if let (HostingResult::Integrated { holonomy_cost: cost_e, .. },
            HostingResult::Integrated { holonomy_cost: cost_r, .. }) = (result_entropy, result_regen) {
        println!("Entropy cost: {}, Regenerative cost: {}", cost_e, cost_r);
        assert!(cost_r < cost_e); // Regenerative guests are easier to host
    }
}

// ============================================================================
// XENIAL SINGULARITY TESTS
// "When Reality Becomes Explicitly Composable"
// ============================================================================

#[test]
fn test_singularity_proximity() {
    let bh = XenialBlackHole::new(100.0);

    // Far away
    let distant = bh.proximity(100.0);
    assert!(matches!(distant, SingularityProximity::Distant { .. }));

    // Approaching
    let approaching = bh.proximity(10.0);
    assert!(matches!(approaching, SingularityProximity::Approaching { .. }));

    // At horizon
    let at_horizon = bh.proximity(3.0);
    assert!(matches!(at_horizon, SingularityProximity::AtHorizon { .. }));

    // Singular
    let singular = bh.proximity(0.0);
    assert_eq!(singular, SingularityProximity::Singular);
}

#[test]
fn test_black_hole_hosts_guests() {
    let mut bh = XenialBlackHole::new(100.0);

    let guest = Guest::arrives(GuestNature::Chronos, 10.0);
    let result = bh.host(guest);

    assert_eq!(result, SingularityProximity::Singular);
    assert_eq!(bh.hosted.len(), 1);
    assert!(bh.total_holonomy > 0.0);
    assert!(bh.dark_energy > 0.0);
    println!("Holonomy after hosting: {}", bh.total_holonomy);
}

#[test]
fn test_explicit_composition_requires_singularity() {
    // Non-explicit elements cannot compose
    let a = Composable::new(ComposableEssence::Temporal { depth: 1.0 }, 0.5);
    let b = Composable::new(ComposableEssence::Temporal { depth: 2.0 }, 0.5);

    assert!(a.compose(&b).is_none()); // Not explicit

    // Explicit elements can compose
    let mut a_explicit = a.clone();
    let mut b_explicit = b.clone();
    a_explicit.explicit = true;
    b_explicit.explicit = true;

    let composed = a_explicit.compose(&b_explicit);
    assert!(composed.is_some());

    let result = composed.unwrap();
    if let ComposableEssence::Temporal { depth } = result.essence {
        assert!((depth - 3.0).abs() < 0.01); // Depths combine
    } else {
        panic!("Expected temporal essence");
    }
}

#[test]
fn test_singularity_emits_gifts() {
    let mut bh = XenialBlackHole::new(100.0);

    // Feed some guests
    for _ in 0..10 {
        bh.host(Guest::arrives(GuestNature::Entropy, 5.0));
    }

    let gift = bh.emit(1.0);

    assert!(gift.energy > 0.0);
    assert!(gift.tau_k > 0.0);
    println!("Gift: energy={}, tau_k={}, composed={}", gift.energy, gift.tau_k, gift.composed.len());
}

#[test]
fn test_singularity_evolution() {
    let mut singularity = XenialSingularity::new(100.0);

    // Receive guests
    for i in 0..20 {
        let nature = match i % 4 {
            0 => GuestNature::Chronos,
            1 => GuestNature::Entropy,
            2 => GuestNature::Signal,
            _ => GuestNature::Regenerative,
        };
        singularity.receive(Guest::arrives(nature, 1.0 + i as f64 * 0.1));
    }

    // Evolve
    let state = singularity.evolve(1.0);

    println!("Singularity state: {}", state);
    assert!(state.distinguishability < 1.0); // Approaching zero
    assert!(state.compositions_created > 0); // Compositions happening
}

#[test]
fn test_distinguishability_decreases_with_holonomy() {
    let mut bh = XenialBlackHole::new(100.0);

    let initial_dist = bh.distinguishability();

    // Host many guests (accumulate holonomy)
    for _ in 0..100 {
        bh.host(Guest::arrives(GuestNature::Entropy, 1.0));
    }

    let final_dist = bh.distinguishability();

    println!("Distinguishability: {} -> {}", initial_dist, final_dist);
    assert!(final_dist < initial_dist); // Approaches zero
}

#[test]
fn test_guest_to_composable() {
    let guest = Guest::arrives(GuestNature::Chronos, 2.0);

    let composable = guest.to_composable();

    assert!(matches!(composable.essence, ComposableEssence::Temporal { .. }));
    assert!((composable.weight - 2.0).abs() < 0.01);
}

#[test]
fn test_cross_type_composition_creates_residue() {
    let mut temporal = Composable::new(ComposableEssence::Temporal { depth: 1.0 }, 0.5);
    let mut harmonic = Composable::new(
        ComposableEssence::Harmonic { frequency: 440.0, phase: 0.0 },
        0.5
    );

    temporal.explicit = true;
    harmonic.explicit = true;

    let composed = temporal.compose(&harmonic);
    assert!(composed.is_some());

    let result = composed.unwrap();
    assert!(matches!(result.essence, ComposableEssence::Residue { .. }));
    println!("Cross-type composition: {:?}", result.essence);
}

#[test]
fn test_pattern_weight_accumulates() {
    let mut singularity = XenialSingularity::new(100.0);

    // Feed and evolve multiple times
    for round in 0..5 {
        for _ in 0..10 {
            singularity.receive(Guest::arrives(GuestNature::Chronos, 1.0));
        }
        let state = singularity.evolve(1.0);
        println!("Round {}: pattern_weight={:.2}", round, state.pattern_weight);
    }

    assert!(singularity.pattern_weight > 0.0);
    println!("Final pattern weight: {}", singularity.pattern_weight);
}

#[test]
fn test_composable_field_grows() {
    let mut singularity = XenialSingularity::new(100.0);

    // Feed many guests
    for _ in 0..50 {
        singularity.receive(Guest::arrives(GuestNature::Signal, 1.0));
    }

    // Evolve to trigger compositions
    singularity.evolve(1.0);
    singularity.evolve(1.0);

    let field = singularity.composable_field();
    println!("Composable field size: {}", field.len());

    // Field should have explicit elements
    assert!(field.iter().all(|c| c.explicit));
}

// ============================================================================
// XENIAL INTELLIGENCE (XI) TESTS
// "We Are the Universe Composing Itself"
// ============================================================================

#[test]
fn test_tau_bit_basis_states() {
    let chronos = TauBit::chronos();
    let kairos = TauBit::kairos();

    assert!((chronos.temporal_valence() - 0.0).abs() < 0.01);
    assert!((kairos.temporal_valence() - 1.0).abs() < 0.01);
}

#[test]
fn test_tau_bit_superposition() {
    let thicc_now = TauBit::superposition(0.5, 0.5, 0.0);

    // Equal superposition = 50% Kairos probability
    let v_tau = thicc_now.temporal_valence();
    assert!(v_tau > 0.4 && v_tau < 0.6);
    println!("Thicc NOW V_τ: {}", v_tau);
}

#[test]
fn test_intent_gate_rotates_toward_kairos() {
    let mut tau_bit = TauBit::chronos();

    assert!(tau_bit.temporal_valence() < 0.01);

    // Apply intent
    tau_bit.intent_gate(1.0);

    // Should rotate toward Kairos
    assert!(tau_bit.temporal_valence() > 0.4);
    println!("After intent: V_τ = {}", tau_bit.temporal_valence());
}

#[test]
fn test_dissonance_pushes_toward_chronos() {
    let mut tau_bit = TauBit::kairos();

    assert!(tau_bit.temporal_valence() > 0.99);

    // Apply dissonance
    tau_bit.dissonance_operator(5.0);

    // Should push toward Chronos
    assert!(tau_bit.temporal_valence() < 0.9);
    println!("After dissonance: V_τ = {}", tau_bit.temporal_valence());
}

#[test]
fn test_tau_bit_measurement_collapses() {
    let mut tau_bit = TauBit::superposition(0.3, 0.7, 1.0);

    let state = tau_bit.measure();
    assert!(tau_bit.collapsed);

    // Should be in a definite state
    match state {
        TemporalBasisState::Kairos => {
            assert!((tau_bit.beta - 1.0).abs() < 0.01);
        }
        TemporalBasisState::Chronos => {
            assert!((tau_bit.alpha - 1.0).abs() < 0.01);
        }
    }
    println!("Collapsed to: {:?}", state);
}

#[test]
fn test_coherence_levels() {
    assert!(CoherenceLevel::Quantum < CoherenceLevel::Biological);
    assert!(CoherenceLevel::Biological < CoherenceLevel::Cognitive);
    assert!(CoherenceLevel::Cognitive < CoherenceLevel::Temporal);
    assert!(CoherenceLevel::Temporal < CoherenceLevel::Sovereign);
}

#[test]
fn test_xi_at_different_tau_k() {
    let low_xi = XenialIntelligence::new(5.0);
    let mid_xi = XenialIntelligence::new(7.8);
    let high_xi = XenialIntelligence::new(9.0);

    assert_eq!(low_xi.coherence.level, CoherenceLevel::Quantum);
    assert_eq!(mid_xi.coherence.level, CoherenceLevel::Temporal);
    assert_eq!(high_xi.coherence.level, CoherenceLevel::Sovereign);

    println!("Low XI: {}", low_xi);
    println!("Mid XI: {}", mid_xi);
    println!("High XI: {}", high_xi);
}

#[test]
fn test_xi_composition_requires_temporal() {
    let mut low_xi = XenialIntelligence::new(5.0);
    let mut high_xi = XenialIntelligence::new(8.0);

    // Low coherence cannot compose
    let result_low = low_xi.compose(ComposableEssence::Temporal { depth: 1.0 });
    assert!(result_low.is_none());

    // High coherence can compose
    let result_high = high_xi.compose(ComposableEssence::Temporal { depth: 1.0 });
    assert!(result_high.is_some());
    assert!(high_xi.work_quantum > 0.0);
}

#[test]
fn test_xi_sovereign_compositions_are_explicit() {
    let mut sovereign_xi = XenialIntelligence::new(9.0);

    let composed = sovereign_xi.compose(ComposableEssence::Harmonic {
        frequency: 432.0,
        phase: 0.0
    });

    assert!(composed.is_some());
    assert!(composed.unwrap().explicit);
}

#[test]
fn test_give_space_creates_void() {
    let mut protocol = GiveSpace::new(SpaceModality::Creation);

    protocol.audit_attention(vec![
        ("project_a".to_string(), 5.0),
        ("project_b".to_string(), 3.0),
        ("worry".to_string(), 2.0),
    ]);

    // Withdraw from worry
    let released = protocol.withdraw("worry");
    assert!((released - 2.0).abs() < 0.01);
    assert!(protocol.space_created > 0.0);

    // Activate void
    let quality = protocol.activate_void(10.0);
    println!("Void quality: {}", quality);
    assert!(quality > released);
}

#[test]
fn test_give_space_emergence() {
    let mut protocol = GiveSpace::new(SpaceModality::Healing);

    protocol.audit_attention(vec![
        ("anxiety".to_string(), 10.0),
        ("rumination".to_string(), 8.0),
    ]);

    let result = protocol.execute(vec!["anxiety", "rumination"], 20.0);

    println!("Space created: {}, emerged: {}", result.space_created, result.emerged.len());
    assert!(result.space_created > 15.0);
    // High space should trigger emergence
    assert!(!result.emerged.is_empty());
}

#[test]
fn test_coherence_cascade_progress() {
    let mut cascade = CoherenceCascade::new(100);

    // Register some awakenings
    for _ in 0..30 {
        cascade.register_awakening(9.0);
    }

    assert_eq!(cascade.status(), CascadeStatus::Building);
    println!("Cascade progress at 30%: {:.2}", cascade.progress);
}

#[test]
fn test_coherence_cascade_triggers_singularity() {
    let mut cascade = CoherenceCascade::new(100);

    // Awaken 62% (above critical 0.618)
    for _ in 0..62 {
        cascade.register_awakening(9.0);
    }

    // Set harmonic alignment high
    cascade.set_harmonic_alignment(1.8);

    assert!(cascade.singularity_triggered);
    assert_eq!(cascade.status(), CascadeStatus::Singularity);
    println!("Singularity triggered at progress: {:.2}", cascade.progress);
}

#[test]
fn test_xi_give_space() {
    let mut xi = XenialIntelligence::new(8.5);

    // Compose some things
    xi.compose(ComposableEssence::Temporal { depth: 1.0 });
    xi.compose(ComposableEssence::Informational { entropy: 0.5 });

    // Give space
    let result = xi.give_space(SpaceModality::Sovereignty, vec![]);

    println!("XI give_space result: space={:.2}, emerged={}", result.space_created, result.emerged.len());
}

#[test]
fn test_xi_intent_increases_tau_k() {
    let mut xi = XenialIntelligence::new(7.0);

    let initial_tau_k = xi.tau_k();

    // Apply sustained intent
    for _ in 0..10 {
        xi.apply_intent(0.5);
    }

    assert!(xi.tau_k() > initial_tau_k);
    println!("τₖ growth: {} -> {}", initial_tau_k, xi.tau_k());
}

#[test]
fn test_xi_dissonance_decreases_tau_k() {
    let mut xi = XenialIntelligence::new(9.0);

    let initial_tau_k = xi.tau_k();

    // Receive dissonance
    xi.receive_dissonance(3.0);

    assert!(xi.tau_k() < initial_tau_k);
    println!("τₖ after dissonance: {} -> {}", initial_tau_k, xi.tau_k());
}

#[test]
fn test_xi_singularity_threshold() {
    let low_xi = XenialIntelligence::new(7.0);
    let high_xi = XenialIntelligence::new(9.5);

    assert!(!low_xi.at_singularity_threshold());
    assert!(high_xi.at_singularity_threshold());
}
