//! daThiccNOW Composition — Amplified Presence
//!
//! "A black hole is where NOW got so thick it folded into itself"
//!
//! Run with: cargo run --example thicc_now

use cognitive_topology::prelude::*;
use std::f64::consts::PI;

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("              daThiccNOW — AUGMNTD COMPOSITION                 ");
    println!("         'e' removed → energy given to pattern                 ");
    println!("═══════════════════════════════════════════════════════════════");
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE I: Genesis — Form the Coherence Well
    // ═══════════════════════════════════════════════════════════════════════

    println!("▓▓▓ PHASE I: GENESIS — Forming the Coherence Well ▓▓▓");
    println!();

    let mut well = CoherenceWell::new([0.0, 0.0, 0.0]);

    // Feed gravitational residue — the exhaust of electrical ascent
    println!("  Feeding gravitational residue into the well...");
    println!("  (What electricity left behind as it chased coherence)");
    println!();

    for i in 0..150 {
        // Residue from various Tk sources — some high, some low
        let source_tk = 0.3 + (i as f64 * 0.01).sin().abs() * 2.0;

        well.absorb(GravitationalResidue {
            amount: 5.0 + (i as f64 * 0.1).cos() * 2.0,
            position: [0.0, 0.0, 0.0],
            source_tau_k: source_tk,
        });

        if i % 30 == 0 {
            println!("    step {:>3}: well_τₖ={:.3}, weight={:.1}, dark_E={:.1}, distinguish={:.4}",
                i,
                well.well_tau_k.value,
                well.total_weight,
                well.dark_energy,
                well.distinguishability()
            );
        }
    }

    println!();
    println!("  Final well state:");
    println!("    τₖ = {:.4} (threshold for BH: < 0.5)", well.well_tau_k.value);
    println!("    weight = {:.2}", well.total_weight);
    println!("    dark energy = {:.2}", well.dark_energy);
    println!("    distinguishability = {:.6}", well.distinguishability());
    println!("    is_black_hole = {}", well.is_black_hole());
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE II: Collapse — Black Hole Formation
    // ═══════════════════════════════════════════════════════════════════════

    println!("▓▓▓ PHASE II: COLLAPSE — Black Hole Formation ▓▓▓");
    println!();

    let black_hole = BlackHole::collapse(well).expect("Well should collapse into black hole");

    println!("  ╔══════════════════════════════════════════════════════╗");
    println!("  ║  BLACK HOLE FORMED                                   ║");
    println!("  ║  'The manifold ate its own coordinate system'        ║");
    println!("  ╠══════════════════════════════════════════════════════╣");
    println!("  ║  Event horizon radius: {:<28.4} ║", black_hole.core.event_horizon_radius());
    println!("  ║  Bundled dark energy:  {:<28.2} ║", black_hole.bundled_dark_energy);
    println!("  ║  Emission rate:        {:<28.6} ║", black_hole.emission_rate);
    println!("  ║  Core τₖ:              {:<28.4} ║", black_hole.core.well_tau_k.value);
    println!("  ╚══════════════════════════════════════════════════════╝");
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE III: Augmntd Genesis — Establish Temporal Reservoir
    // ═══════════════════════════════════════════════════════════════════════

    println!("▓▓▓ PHASE III: AUGMNTD GENESIS — Temporal Reservoir ▓▓▓");
    println!();

    let mut augmntd = AugmntdComposition::new(3, 30); // 3 spiral arms, 30 oscillators

    // Establish the black hole as a temporal reservoir
    augmntd.establish_reservoir(black_hole);

    let reservoir = augmntd.temporal_reservoir.as_ref().unwrap();
    println!("  Temporal Reservoir established:");
    println!("    compressed_now = {:.4}", reservoir.compressed_now);
    println!("    release_rate = {:.6}", reservoir.release_rate);
    println!("    amplification_factor = {:.4}", reservoir.amplification_factor());
    println!();

    println!("  Compositional Pathways active: {}", augmntd.pathways.len());
    for (i, pathway) in augmntd.pathways.iter().enumerate() {
        let eff = pathway.transfer_efficiency(1.0);
        let name = match pathway {
            CompositionPathway::Radial { direction, .. } =>
                format!("Radial({:?})", direction),
            CompositionPathway::Spiral { arm_index, .. } =>
                format!("Spiral(arm={})", arm_index),
            CompositionPathway::Resonant { .. } =>
                "Resonant".to_string(),
            CompositionPathway::Janus { inward_fraction, .. } =>
                format!("Janus(in={:.2})", inward_fraction),
        };
        println!("    [{i}] {:<20} efficiency={:.4}", name, eff);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE IV: Stellar Genesis — Spawn Golden Flows
    // ═══════════════════════════════════════════════════════════════════════

    println!("▓▓▓ PHASE IV: STELLAR GENESIS — Spawning Golden Flows ▓▓▓");
    println!();

    // Spawn golden flows in a φ-spiral pattern
    let num_stars = 12; // Icosahedral pitch class count
    for i in 0..num_stars {
        let golden_angle = 2.0 * PI * PHI_INV * (i as f64);
        let r = 2.0 + (i as f64) * PHI_INV;
        let pos = [r * golden_angle.cos(), r * golden_angle.sin(), 0.0];

        augmntd.galaxy.stellar_flows.push(ElectricalFlow::golden(pos));
    }

    println!("  Spawned {} golden flows in φ-spiral:", num_stars);
    for (i, flow) in augmntd.galaxy.stellar_flows.iter().enumerate() {
        if i < 5 || i >= num_stars - 2 {
            println!("    ⭐ [{:>2}] pos=({:>6.2}, {:>6.2}) τₖ={:.3} ω={:.2e} stability={:.3}",
                i,
                flow.position[0], flow.position[1],
                flow.tau_k.value,
                flow.harmonic.omega,
                flow.harmonic.stability
            );
        } else if i == 5 {
            println!("    ... ({} more) ...", num_stars - 7);
        }
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // PHASE V: EVOLUTION — daThiccNOW Amplification
    // ═══════════════════════════════════════════════════════════════════════

    println!("▓▓▓ PHASE V: EVOLUTION — daThiccNOW Amplification ▓▓▓");
    println!();
    println!("  'e' → pattern : energy removed thickens NOW");
    println!();

    let dt = 0.1;
    let num_epochs = 10;
    let steps_per_epoch = 20;

    println!("  ┌─────────┬───────────┬───────────┬───────────┬───────────┬───────────┐");
    println!("  │  Epoch  │ thiccNOW  │  ∫thicc   │  paths    │ e→pattern │ harmonic R│");
    println!("  ├─────────┼───────────┼───────────┼───────────┼───────────┼───────────┤");

    for epoch in 0..num_epochs {
        let mut epoch_state = None;

        for step in 0..steps_per_epoch {
            // Peristaltic pulse — spiral arm contraction wave
            let phase = (epoch * steps_per_epoch + step) as f64 * 0.1;
            augmntd.temporal_peristalsis(phase);

            // Evolve the augmntd composition
            epoch_state = Some(augmntd.evolve(dt));
        }

        if let Some(state) = epoch_state {
            println!("  │  {:>5}  │  {:>7.4}  │  {:>7.3}  │  {:>7.4}  │  {:>7.5}  │  {:>7.4}  │",
                epoch + 1,
                state.amplified_thicc_now,
                state.integrated_thicc,
                state.pathway_efficiency,
                state.temporal_released,
                state.harmonic_state.order_parameter
            );
        }
    }

    println!("  └─────────┴───────────┴───────────┴───────────┴───────────┴───────────┘");
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // FINAL STATE — The Amplified Presence
    // ═══════════════════════════════════════════════════════════════════════

    println!("▓▓▓ FINAL STATE — THE AMPLIFIED PRESENCE ▓▓▓");
    println!();

    let final_thicc = augmntd.da_thicc_now();
    let final_integrated = augmntd.integrated_thicc;

    println!("  ╔══════════════════════════════════════════════════════════════╗");
    println!("  ║                    daThiccNOW ACHIEVED                       ║");
    println!("  ╠══════════════════════════════════════════════════════════════╣");
    println!("  ║                                                              ║");
    println!("  ║   Present Moment Thickness:  {:>10.6}                     ║", final_thicc);
    println!("  ║   Integrated Temporal Depth: {:>10.4}                     ║", final_integrated);
    println!("  ║                                                              ║");
    println!("  ║   Temporal Depth Breakdown:                                  ║");
    println!("  ║     base_thickness:      {:>10.4}                         ║", augmntd.temporal_depth.base_thickness);
    println!("  ║     scale_integration:   {:>10.4}                         ║", augmntd.temporal_depth.scale_integration);
    println!("  ║     reservoir_amp:       {:>10.4}                         ║", augmntd.temporal_depth.reservoir_amplification);
    println!("  ║                                                              ║");

    if let Some(ref reservoir) = augmntd.temporal_reservoir {
        println!("  ║   Reservoir Status:                                          ║");
        println!("  ║     compressed_now:      {:>10.4}                         ║", reservoir.compressed_now);
        println!("  ║     amplification:       {:>10.4}                         ║", reservoir.amplification_factor());
    }

    println!("  ║                                                              ║");
    println!("  ║   Harmonic Field:                                            ║");
    println!("  ║     global_coherence (R): {:>10.4}                         ║", augmntd.harmonic_field.global_coherence);
    println!("  ║     global_phase (ψ):     {:>10.4} rad                     ║", augmntd.harmonic_field.global_phase);
    println!("  ║                                                              ║");
    println!("  ║   Galaxy:                                                    ║");
    println!("  ║     stellar_flows:        {:>10}                           ║", augmntd.galaxy.stellar_flows.len());
    println!("  ║     galactic_τₖ:          {:>10.4}                         ║", augmntd.galaxy.galactic_tau_k.value);
    println!("  ║                                                              ║");
    println!("  ╚══════════════════════════════════════════════════════════════╝");
    println!();

    // The Signature
    println!("  ┌──────────────────────────────────────────────────────────────┐");
    println!("  │                                                              │");
    println!("  │     'Gravity isn't attraction — it's phase abandonment       │");
    println!("  │      crystallized. Black holes don't consume — they          │");
    println!("  │      compress time and release it as thickened NOW.'         │");
    println!("  │                                                              │");
    println!("  │                          τₖ = φ = {:.15}           │", PHI);
    println!("  │                                                              │");
    println!("  │                    The geometry recognizes itself.           │");
    println!("  │                                                              │");
    println!("  └──────────────────────────────────────────────────────────────┘");
    println!();

    // ASCII art representation of the augmntd system
    println!("                         ·  ✧  ·");
    println!("                    ✦        ●        ✦");
    println!("                 ·    ╱ ╲   ╱ ╲   ╱ ╲    ·");
    println!("               ✧   ╱   ● ╲╱ ◉ ╲╱ ●   ╲   ✧");
    println!("              ·  ╱   ╱ ╲     ▓     ╱ ╲   ╲  ·");
    println!("             ✦ ●   ╱   ╲   ▓▓▓   ╱   ╲   ● ✦");
    println!("              ·  ╲   ● ╱ ▓▓▓▓▓ ╲ ●   ╱  ·");
    println!("               ✧   ╲   ╲ ▓▓▓▓▓ ╱   ╱   ✧");
    println!("                 ·    ╲ ╱ ▓▓▓ ╲ ╱    ·");
    println!("                    ✦    ╲ ▓ ╱    ✦");
    println!("                         ·  ●  ·");
    println!();
    println!("           [ ▓ = Temporal Reservoir | ● = Golden Flow ]");
    println!();
}
