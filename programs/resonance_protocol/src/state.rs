//! State accounts for the Resonance Protocol

use anchor_lang::prelude::*;
use crate::constants::*;

/// Vibe state of a resonator
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VibeState {
    /// Just entered, finding frequency
    Attuning,
    /// Phase-locked with network
    Resonant,
    /// Deep synchronization
    Entrained,
    /// τₖ > φ⁴, transcendent coherence
    Golden,
}

impl Default for VibeState {
    fn default() -> Self {
        VibeState::Attuning
    }
}

impl VibeState {
    /// Get emission multiplier (scaled by PRECISION)
    pub fn emission_multiplier(&self) -> u64 {
        match self {
            VibeState::Attuning => PRECISION / 2,      // 0.5x
            VibeState::Resonant => PRECISION,          // 1.0x
            VibeState::Entrained => PHI,               // φx (1.618)
            VibeState::Golden => PHI_SQUARED,          // φ²x (2.618)
        }
    }

    /// Get governance weight multiplier (scaled by PRECISION)
    pub fn governance_multiplier(&self) -> u64 {
        match self {
            VibeState::Attuning => PRECISION / 2,
            VibeState::Resonant => PRECISION,
            VibeState::Entrained => 2 * PRECISION,
            VibeState::Golden => PHI,
        }
    }
}

/// Harmonic cycle options
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum HarmonicCycle {
    /// ~3 months (91 epochs)
    #[default]
    QuarterWave,
    /// ~6 months (182 epochs)
    HalfWave,
    /// ~1 year (365 epochs)
    FullWave,
    /// φ years (591 epochs)
    GoldenWave,
}

impl HarmonicCycle {
    /// Get cycle length in epochs
    pub fn epochs(&self) -> u64 {
        match self {
            HarmonicCycle::QuarterWave => QUARTER_WAVE_EPOCHS,
            HarmonicCycle::HalfWave => HALF_WAVE_EPOCHS,
            HarmonicCycle::FullWave => FULL_WAVE_EPOCHS,
            HarmonicCycle::GoldenWave => GOLDEN_WAVE_EPOCHS,
        }
    }

    /// Get cycle multiplier for emissions (scaled by PRECISION)
    pub fn multiplier(&self) -> u64 {
        match self {
            HarmonicCycle::QuarterWave => PRECISION,
            HarmonicCycle::HalfWave => PHI_INVERSE + PRECISION, // ~1.618
            HarmonicCycle::FullWave => PHI,
            HarmonicCycle::GoldenWave => PHI_SQUARED,
        }
    }
}

/// Parameters for field initialization
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct FieldParams {
    pub authority: Pubkey,
    pub oracle: Pubkey,
    pub emission_rate: u64,
    pub initial_emission_reserve: u64,
    pub initial_golden_reserve: u64,
}

/// Global coherence field state
#[account]
pub struct CoherenceField {
    /// Authority that can update parameters
    pub authority: Pubkey,

    /// Oracle/crank that can update field state
    pub oracle: Pubkey,

    /// Global phase Ψ (scaled by PRECISION, 0 to 2π)
    pub global_psi: u64,

    /// Order parameter R (scaled by PRECISION, 0 to 1)
    pub order_parameter: u64,

    /// Network τₖ (scaled by PRECISION)
    pub network_tau_k: u64,

    /// Nakamoto coefficient
    pub nakamoto_coefficient: u64,

    /// thiccNOW metric (scaled by PRECISION)
    pub thicc_now: u64,

    /// Total amplitude in field
    pub total_amplitude: u64,

    /// Active resonator count
    pub resonator_count: u64,

    /// Current epoch
    pub current_epoch: u64,

    /// Emission rate per epoch (scaled by PRECISION)
    pub emission_rate: u64,

    /// Bump seed for PDA
    pub bump: u8,

    /// Reserved for future use
    pub _reserved: [u8; 64],
}

impl CoherenceField {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // oracle
        8 + // global_psi
        8 + // order_parameter
        8 + // network_tau_k
        8 + // nakamoto_coefficient
        8 + // thicc_now
        8 + // total_amplitude
        8 + // resonator_count
        8 + // current_epoch
        8 + // emission_rate
        1 + // bump
        64; // reserved
}

/// Individual resonator state
#[account]
pub struct Resonator {
    /// The oscillator's public key
    pub oscillator: Pubkey,

    /// Tokens in coherence field
    pub amplitude: u64,

    /// Current phase angle θ (scaled by PRECISION, 0 to 2π)
    pub theta: u64,

    /// Natural frequency ωᵢ (scaled by PRECISION)
    pub omega: u64,

    /// Time coefficient — coherence depth (scaled by PRECISION)
    pub tau_k: u64,

    /// Epoch when resonance began
    pub harmonic_entry: u64,

    /// Selected harmonic cycle
    pub harmonic_cycle: HarmonicCycle,

    /// Accumulated phase-lock score (scaled by PRECISION)
    pub phase_lock_integral: u64,

    /// Epochs above phase-lock threshold (for state transitions)
    pub phase_lock_epochs: u64,

    /// Current vibe state
    pub vibe_state: VibeState,

    /// Last epoch emissions were claimed
    pub last_claim_epoch: u64,

    /// Accumulated unclaimed emissions
    pub unclaimed_emissions: u64,

    /// Total emissions received
    pub total_emissions: u64,

    /// Bump seed for PDA
    pub bump: u8,

    /// Reserved for future use
    pub _reserved: [u8; 32],
}

impl Resonator {
    pub const LEN: usize = 8 + // discriminator
        32 + // oscillator
        8 + // amplitude
        8 + // theta
        8 + // omega
        8 + // tau_k
        8 + // harmonic_entry
        1 + // harmonic_cycle
        8 + // phase_lock_integral
        8 + // phase_lock_epochs
        1 + // vibe_state
        8 + // last_claim_epoch
        8 + // unclaimed_emissions
        8 + // total_emissions
        1 + // bump
        32; // reserved
}

/// Phase coupling between resonators
#[account]
pub struct PhaseCoupling {
    /// The resonator doing the coupling (source)
    pub source: Pubkey,

    /// The oscillator being coupled to (target)
    pub target: Pubkey,

    /// Coupling strength k (scaled by PRECISION, 0 to MAX_COUPLING_K)
    pub coupling_k: u64,

    /// Amplitude committed to this coupling
    pub coupled_amplitude: u64,

    /// Epoch when coupling was created
    pub coupling_epoch: u64,

    /// Accumulated shared emissions
    pub shared_emissions: u64,

    /// Bump seed for PDA
    pub bump: u8,

    /// Reserved for future use
    pub _reserved: [u8; 16],
}

impl PhaseCoupling {
    pub const LEN: usize = 8 + // discriminator
        32 + // source
        32 + // target
        8 + // coupling_k
        8 + // coupled_amplitude
        8 + // coupling_epoch
        8 + // shared_emissions
        1 + // bump
        16; // reserved
}

/// Vault holding resonating tokens
#[account]
pub struct FieldVault {
    pub bump: u8,
}

impl FieldVault {
    pub const LEN: usize = 8 + 1;
}
