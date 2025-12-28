//! Enter the coherence field (begin resonating)

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{CoherenceField, Resonator, VibeState, HarmonicCycle, PhaseCoupling, FieldParams, FieldVault};
use crate::constants::*;
use crate::errors::ResonanceError;
use crate::events::ResonatorEntered;
use crate::math;

#[derive(Accounts)]
pub struct EnterField<'info> {
    #[account(mut)]
    pub oscillator: Signer<'info>,

    #[account(
        mut,
        seeds = [FIELD_SEED],
        bump = coherence_field.bump
    )]
    pub coherence_field: Account<'info, CoherenceField>,

    #[account(
        init,
        payer = oscillator,
        space = Resonator::LEN,
        seeds = [RESONATOR_SEED, oscillator.key().as_ref()],
        bump
    )]
    pub resonator: Account<'info, Resonator>,

    /// Oscillator's token account
    #[account(
        mut,
        constraint = oscillator_tokens.owner == oscillator.key(),
    )]
    pub oscillator_tokens: Account<'info, TokenAccount>,

    /// Vault to receive tokens
    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<EnterField>,
    amplitude: u64,
    harmonic_cycle: HarmonicCycle,
) -> Result<()> {
    // Validate amplitude
    require!(amplitude >= MIN_AMPLITUDE, ResonanceError::AmplitudeTooLow);

    let field = &mut ctx.accounts.coherence_field;
    let resonator = &mut ctx.accounts.resonator;
    let clock = Clock::get()?;

    // Transfer tokens to vault
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.oscillator_tokens.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.oscillator.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amplitude)?;

    // Initialize resonator
    resonator.oscillator = ctx.accounts.oscillator.key();
    resonator.amplitude = amplitude;
    resonator.theta = math::random_phase(clock.unix_timestamp as u64, &resonator.oscillator);
    resonator.omega = math::natural_frequency(amplitude, PRECISION / 100)?; // Base omega
    resonator.tau_k = TAU_K_BASELINE;
    resonator.harmonic_entry = field.current_epoch;
    resonator.harmonic_cycle = harmonic_cycle;
    resonator.phase_lock_integral = 0;
    resonator.phase_lock_epochs = 0;
    resonator.vibe_state = VibeState::Attuning;
    resonator.last_claim_epoch = field.current_epoch;
    resonator.unclaimed_emissions = 0;
    resonator.total_emissions = 0;
    resonator.bump = ctx.bumps.resonator;

    // Update field state
    field.total_amplitude = field.total_amplitude.checked_add(amplitude)
        .ok_or(ResonanceError::MathOverflow)?;
    field.resonator_count = field.resonator_count.checked_add(1)
        .ok_or(ResonanceError::MathOverflow)?;

    emit!(ResonatorEntered {
        oscillator: resonator.oscillator,
        amplitude,
        harmonic_cycle,
        entry_epoch: field.current_epoch,
    });

    msg!(
        "Resonator entered field. Amplitude: {}, Cycle: {:?}, Initial θ: {}",
        amplitude,
        harmonic_cycle,
        resonator.theta
    );

    Ok(())
}
