use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("RWcZBKQyd5rryLD8dfrMZ8imgtfqTfpGZi4gKGS8j7N");

pub mod state;
pub mod instructions;
pub mod errors;
pub mod events;
pub mod constants;
pub mod math;

use state::*;
use instructions::*;
use errors::ResonanceError;
use events::*;
use constants::*;

/// Resonance Protocol: Harmonic Field Contract
/// 
/// "You do not stake tokens. You enter the coherence field."
#[program]
pub mod resonance_protocol {
    use super::*;

    /// Initialize the global coherence field (step 1/3)
    pub fn initialize_field(
        ctx: Context<InitializeField>,
        params: FieldParams,
    ) -> Result<()> {
        instructions::initialize_field::handler(ctx, params)
    }

    /// Initialize the vault (step 2/3)
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::initialize_field::handler_vault(ctx)
    }

    /// Initialize emission reserve (step 3/4)
    pub fn initialize_emission_reserve(ctx: Context<InitializeEmissionReserve>) -> Result<()> {
        instructions::initialize_field::handler_emission_reserve(ctx)
    }

    /// Initialize golden reserve (step 4/4)
    pub fn initialize_golden_reserve(ctx: Context<InitializeGoldenReserve>) -> Result<()> {
        instructions::initialize_field::handler_golden_reserve(ctx)
    }

    /// Enter the coherence field (begin resonating)
    pub fn enter_field(
        ctx: Context<EnterField>,
        amplitude: u64,
        harmonic_cycle: HarmonicCycle,
    ) -> Result<()> {
        instructions::enter_field::handler(ctx, amplitude, harmonic_cycle)
    }

    /// Exit the coherence field (decohere)
    pub fn decohere(ctx: Context<Decohere>) -> Result<()> {
        instructions::decohere::handler(ctx)
    }

    /// Phase-couple to another oscillator
    pub fn phase_couple(
        ctx: Context<PhaseCoupleCtx>,
        coupling_strength: u64,
        coupled_amplitude: u64,
    ) -> Result<()> {
        instructions::phase_couple::handler(ctx, coupling_strength, coupled_amplitude)
    }

    /// Remove phase coupling
    pub fn uncouple(ctx: Context<Uncouple>) -> Result<()> {
        instructions::uncouple::handler(ctx)
    }

    /// Claim resonance emissions
    pub fn claim_emissions(ctx: Context<ClaimEmissions>) -> Result<()> {
        instructions::claim_emissions::handler(ctx)
    }

    /// Update field state (called by oracle/crank)
    pub fn update_field(
        ctx: Context<UpdateField>,
        new_psi: u64,
        new_r: u64,
        new_tau_k: u64,
    ) -> Result<()> {
        instructions::update_field::handler(ctx, new_psi, new_r, new_tau_k)
    }

    /// Update resonator phase (per-epoch dynamics)
    pub fn update_resonator_phase(ctx: Context<UpdateResonatorPhase>) -> Result<()> {
        instructions::update_resonator_phase::handler(ctx)
    }

    /// Transition vibe state if eligible
    pub fn transition_vibe_state(ctx: Context<TransitionVibeState>) -> Result<()> {
        instructions::transition_vibe_state::handler(ctx)
    }
}
