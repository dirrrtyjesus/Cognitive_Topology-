//! Events emitted by the Resonance Protocol

use anchor_lang::prelude::*;
use crate::state::{VibeState, HarmonicCycle};

/// Emitted when a resonator enters the field
#[event]
pub struct ResonatorEntered {
    pub oscillator: Pubkey,
    pub amplitude: u64,
    pub harmonic_cycle: HarmonicCycle,
    pub entry_epoch: u64,
}

/// Emitted when a resonator decoheres (exits)
#[event]
pub struct ResonatorDecohered {
    pub oscillator: Pubkey,
    pub returned_amplitude: u64,
    pub disruption_cost: u64,
    pub epochs_resonated: u64,
}

/// Emitted when phase coupling is established
#[event]
pub struct PhaseCoupled {
    pub source: Pubkey,
    pub target: Pubkey,
    pub coupling_k: u64,
    pub coupled_amplitude: u64,
}

/// Emitted when phase coupling is removed
#[event]
pub struct PhaseUncoupled {
    pub source: Pubkey,
    pub target: Pubkey,
    pub epochs_coupled: u64,
}

/// Emitted when emissions are claimed
#[event]
pub struct EmissionsClaimed {
    pub oscillator: Pubkey,
    pub attunement: u64,
    pub resonance: u64,
    pub entrainment: u64,
    pub thicc_now: u64,
    pub total: u64,
    pub vibe_multiplier: u64,
}

/// Emitted when vibe state transitions
#[event]
pub struct VibeStateTransition {
    pub oscillator: Pubkey,
    pub from_state: VibeState,
    pub to_state: VibeState,
    pub tau_k: u64,
    pub phase_lock: u64,
}

/// Emitted when field state is updated
#[event]
pub struct FieldUpdated {
    pub epoch: u64,
    pub global_psi: u64,
    pub order_parameter: u64,
    pub network_tau_k: u64,
    pub total_amplitude: u64,
    pub resonator_count: u64,
}

/// Emitted when a resonator's phase is updated
#[event]
pub struct PhaseUpdated {
    pub oscillator: Pubkey,
    pub old_theta: u64,
    pub new_theta: u64,
    pub phase_lock: u64,
}

/// Emitted when a resonator achieves Golden state
#[event]
pub struct GoldenAchieved {
    pub oscillator: Pubkey,
    pub tau_k: u64,
    pub epochs_to_golden: u64,
    pub total_emissions: u64,
}

/// Emitted when coherence peak is detected
#[event]
pub struct CoherencePeak {
    pub epoch: u64,
    pub order_parameter: u64,
    pub resonators_in_phase: u64,
}
