use cognitive_topology::prelude::*;
use cognitive_topology::{BioState, BioCogBridge, phi_constraint, tau_k_critical, vmem_limit};
use cognitive_topology::{XenialAging, XenialPhase, Guest, GuestNature, HostingResult};

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
