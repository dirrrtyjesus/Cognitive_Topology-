//! Exit the coherence field (decohere)

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{CoherenceField, Resonator, VibeState, HarmonicCycle, PhaseCoupling, FieldParams, FieldVault};
use crate::constants::*;
use crate::errors::ResonanceError;
use crate::events::ResonatorDecohered;
use crate::math;

#[derive(Accounts)]
pub struct Decohere<'info> {
    #[account(mut)]
    pub oscillator: Signer<'info>,

    #[account(
        mut,
        seeds = [FIELD_SEED],
        bump = coherence_field.bump
    )]
    pub coherence_field: Account<'info, CoherenceField>,

    #[account(
        mut,
        close = oscillator,
        seeds = [RESONATOR_SEED, oscillator.key().as_ref()],
        bump = resonator.bump,
        constraint = resonator.oscillator == oscillator.key()
    )]
    pub resonator: Account<'info, Resonator>,

    /// Oscillator's token account to receive returned tokens
    #[account(
        mut,
        constraint = oscillator_tokens.owner == oscillator.key(),
    )]
    pub oscillator_tokens: Account<'info, TokenAccount>,

    /// Vault holding resonating tokens
    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    /// CHECK: PDA authority for vault
    #[account(
        seeds = [VAULT_SEED, b"authority"],
        bump
    )]
    pub vault_authority: AccountInfo<'info>,

    /// Emission reserve receives disruption cost
    #[account(
        mut,
        seeds = [EMISSION_SEED],
        bump
    )]
    pub emission_reserve: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Decohere>) -> Result<()> {
    let field = &mut ctx.accounts.coherence_field;
    let resonator = &ctx.accounts.resonator;

    let epochs_resonated = field.current_epoch.saturating_sub(resonator.harmonic_entry);

    // Calculate decoherence cost
    let disruption_cost = math::decoherence_cost(
        resonator.amplitude,
        field.current_epoch,
        resonator.harmonic_entry,
        resonator.harmonic_cycle.epochs(),
        resonator.phase_lock_integral,
    )?;

    let return_amount = resonator.amplitude.saturating_sub(disruption_cost);

    // Transfer returned tokens to oscillator
    let vault_bump = ctx.bumps.vault_authority;
    let seeds = &[VAULT_SEED, b"authority".as_ref(), &[vault_bump]];
    let signer_seeds = &[&seeds[..]];

    // Return tokens minus cost
    if return_amount > 0 {
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.oscillator_tokens.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_ctx, return_amount)?;
    }

    // Transfer disruption cost to emission reserve (feeds the field)
    if disruption_cost > 0 {
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.emission_reserve.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_ctx, disruption_cost)?;
    }

    // Update field state
    field.total_amplitude = field.total_amplitude.saturating_sub(resonator.amplitude);
    field.resonator_count = field.resonator_count.saturating_sub(1);

    emit!(ResonatorDecohered {
        oscillator: resonator.oscillator,
        returned_amplitude: return_amount,
        disruption_cost,
        epochs_resonated,
    });

    msg!(
        "Resonator decohered. Returned: {}, Disruption cost: {}, Epochs: {}",
        return_amount,
        disruption_cost,
        epochs_resonated
    );

    Ok(())
}
