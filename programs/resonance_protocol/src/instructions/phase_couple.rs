//! Phase-couple to another oscillator

use anchor_lang::prelude::*;

use crate::state::{CoherenceField, Resonator, VibeState, HarmonicCycle, PhaseCoupling, FieldParams, FieldVault};
use crate::constants::*;
use crate::errors::ResonanceError;
use crate::events::PhaseCoupled;

#[derive(Accounts)]
#[instruction(coupling_strength: u64, coupled_amplitude: u64)]
pub struct PhaseCoupleCtx<'info> {
    #[account(mut)]
    pub source_oscillator: Signer<'info>,

    #[account(
        seeds = [FIELD_SEED],
        bump = coherence_field.bump
    )]
    pub coherence_field: Account<'info, CoherenceField>,

    #[account(
        mut,
        seeds = [RESONATOR_SEED, source_oscillator.key().as_ref()],
        bump = source_resonator.bump,
        constraint = source_resonator.oscillator == source_oscillator.key()
    )]
    pub source_resonator: Account<'info, Resonator>,

    /// Target resonator to couple with
    #[account(
        seeds = [RESONATOR_SEED, target_oscillator.key().as_ref()],
        bump = target_resonator.bump
    )]
    pub target_resonator: Account<'info, Resonator>,

    /// CHECK: Target oscillator pubkey
    pub target_oscillator: AccountInfo<'info>,

    #[account(
        init,
        payer = source_oscillator,
        space = PhaseCoupling::LEN,
        seeds = [COUPLING_SEED, source_oscillator.key().as_ref(), target_oscillator.key().as_ref()],
        bump
    )]
    pub coupling: Account<'info, PhaseCoupling>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<PhaseCoupleCtx>,
    coupling_strength: u64,
    coupled_amplitude: u64,
) -> Result<()> {
    let source = &ctx.accounts.source_resonator;
    let target = &ctx.accounts.target_resonator;
    let field = &ctx.accounts.coherence_field;
    let coupling = &mut ctx.accounts.coupling;

    // Cannot self-couple
    require!(
        source.oscillator != target.oscillator,
        ResonanceError::CannotSelfCouple
    );

    // Must be past Attuning state to couple
    require!(
        source.vibe_state != VibeState::Attuning,
        ResonanceError::StillAttuning
    );

    // Validate coupling strength
    require!(
        coupling_strength <= MAX_COUPLING_K,
        ResonanceError::CouplingTooStrong
    );

    // Validate coupled amplitude
    require!(
        coupled_amplitude <= source.amplitude,
        ResonanceError::InsufficientAmplitude
    );

    // Initialize coupling
    coupling.source = source.oscillator;
    coupling.target = target.oscillator;
    coupling.coupling_k = coupling_strength;
    coupling.coupled_amplitude = coupled_amplitude;
    coupling.coupling_epoch = field.current_epoch;
    coupling.shared_emissions = 0;
    coupling.bump = ctx.bumps.coupling;

    emit!(PhaseCoupled {
        source: coupling.source,
        target: coupling.target,
        coupling_k: coupling_strength,
        coupled_amplitude,
    });

    msg!(
        "Phase coupled: {} -> {} with k={}, amplitude={}",
        source.oscillator,
        target.oscillator,
        coupling_strength,
        coupled_amplitude
    );

    Ok(())
}
