//! Remove phase coupling

use anchor_lang::prelude::*;

use crate::state::{CoherenceField, Resonator, VibeState, HarmonicCycle, PhaseCoupling, FieldParams, FieldVault};
use crate::constants::*;
use crate::events::PhaseUncoupled;

#[derive(Accounts)]
pub struct Uncouple<'info> {
    #[account(mut)]
    pub source_oscillator: Signer<'info>,

    #[account(
        seeds = [FIELD_SEED],
        bump = coherence_field.bump
    )]
    pub coherence_field: Account<'info, CoherenceField>,

    /// CHECK: Target oscillator pubkey
    pub target_oscillator: AccountInfo<'info>,

    #[account(
        mut,
        close = source_oscillator,
        seeds = [COUPLING_SEED, source_oscillator.key().as_ref(), target_oscillator.key().as_ref()],
        bump = coupling.bump,
        constraint = coupling.source == source_oscillator.key()
    )]
    pub coupling: Account<'info, PhaseCoupling>,
}

pub fn handler(ctx: Context<Uncouple>) -> Result<()> {
    let field = &ctx.accounts.coherence_field;
    let coupling = &ctx.accounts.coupling;

    let epochs_coupled = field.current_epoch.saturating_sub(coupling.coupling_epoch);

    emit!(PhaseUncoupled {
        source: coupling.source,
        target: coupling.target,
        epochs_coupled,
    });

    msg!(
        "Phase uncoupled: {} -> {} after {} epochs",
        coupling.source,
        coupling.target,
        epochs_coupled
    );

    // Account closed automatically via close constraint

    Ok(())
}
