//! Claim resonance emissions

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{CoherenceField, Resonator, VibeState, HarmonicCycle};
use crate::constants::*;
use crate::errors::ResonanceError;
use crate::events::EmissionsClaimed;
use crate::math;

#[derive(Accounts)]
pub struct ClaimEmissions<'info> {
    #[account(mut)]
    pub oscillator: Signer<'info>,

    #[account(
        seeds = [FIELD_SEED],
        bump = coherence_field.bump
    )]
    pub coherence_field: Account<'info, CoherenceField>,

    #[account(
        mut,
        seeds = [RESONATOR_SEED, oscillator.key().as_ref()],
        bump = resonator.bump,
        constraint = resonator.oscillator == oscillator.key()
    )]
    pub resonator: Account<'info, Resonator>,

    /// Oscillator's token account to receive emissions
    #[account(
        mut,
        constraint = oscillator_tokens.owner == oscillator.key(),
    )]
    pub oscillator_tokens: Account<'info, TokenAccount>,

    /// Emission reserve
    #[account(
        mut,
        seeds = [EMISSION_SEED],
        bump
    )]
    pub emission_reserve: Account<'info, TokenAccount>,

    /// Golden reserve (for Golden state resonators)
    #[account(
        mut,
        seeds = [GOLDEN_SEED],
        bump
    )]
    pub golden_reserve: Account<'info, TokenAccount>,

    /// CHECK: PDA authority for reserves
    #[account(
        seeds = [VAULT_SEED, b"authority"],
        bump
    )]
    pub vault_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ClaimEmissions>) -> Result<()> {
    let field = &ctx.accounts.coherence_field;
    let resonator = &mut ctx.accounts.resonator;

    let epochs_since_claim = field.current_epoch.saturating_sub(resonator.last_claim_epoch);

    if epochs_since_claim == 0 {
        return Err(ResonanceError::NoEmissionsToClaim.into());
    }

    // Calculate emissions for each channel (25% each)

    // 1. Attunement: base emission for field presence
    let attunement = math::calculate_channel_emission(
        resonator.amplitude,
        epochs_since_claim,
        field.emission_rate,
        25,
    )?;

    // 2. Resonance: phase-lock quality
    let phase_lock = math::phase_lock(resonator.theta, field.global_psi)?;
    let phase_lock_factor = if phase_lock > 0 {
        phase_lock as u64
    } else {
        0
    };
    let resonance_base = math::calculate_channel_emission(
        resonator.amplitude,
        epochs_since_claim,
        field.emission_rate,
        25,
    )?;
    let resonance = math::mul_precision(resonance_base, phase_lock_factor)?;

    // 3. Entrainment: coupling contribution (simplified - would need coupling accounts)
    let entrainment = math::calculate_channel_emission(
        resonator.amplitude,
        epochs_since_claim,
        field.emission_rate,
        25,
    )?;

    // 4. thiccNOW: τₖ growth reward
    let tau_factor = math::div_precision(resonator.tau_k, TAU_K_BASELINE)?;
    let thicc_base = math::calculate_channel_emission(
        resonator.amplitude,
        epochs_since_claim,
        field.emission_rate,
        25,
    )?;
    let thicc_now = math::mul_precision(thicc_base, tau_factor)?;

    // Sum channels
    let base_total = attunement
        .checked_add(resonance)
        .and_then(|x| x.checked_add(entrainment))
        .and_then(|x| x.checked_add(thicc_now))
        .ok_or(ResonanceError::MathOverflow)?;

    // Apply vibe state multiplier
    let vibe_multiplier = resonator.vibe_state.emission_multiplier();
    let multiplied = math::mul_precision(base_total, vibe_multiplier)?;

    // Apply harmonic cycle multiplier
    let cycle_multiplier = resonator.harmonic_cycle.multiplier();
    let final_emission = math::mul_precision(multiplied, cycle_multiplier)?;

    // Determine source reserve
    let (reserve, _reserve_bump) = if resonator.vibe_state == VibeState::Golden {
        (&ctx.accounts.golden_reserve, ctx.bumps.golden_reserve)
    } else {
        (&ctx.accounts.emission_reserve, ctx.bumps.emission_reserve)
    };

    // Check reserve has sufficient balance
    require!(
        reserve.amount >= final_emission,
        ResonanceError::ReserveDepleted
    );

    // Transfer emissions
    let vault_bump = ctx.bumps.vault_authority;
    let seeds = &[VAULT_SEED, b"authority".as_ref(), &[vault_bump]];
    let signer_seeds = &[&seeds[..]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: reserve.to_account_info(),
            to: ctx.accounts.oscillator_tokens.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, final_emission)?;

    // Update resonator state
    resonator.last_claim_epoch = field.current_epoch;
    resonator.total_emissions = resonator.total_emissions
        .checked_add(final_emission)
        .ok_or(ResonanceError::MathOverflow)?;

    // Update phase-lock integral
    if phase_lock > 0 {
        resonator.phase_lock_integral = resonator.phase_lock_integral
            .checked_add(phase_lock as u64)
            .ok_or(ResonanceError::MathOverflow)?;
    }

    emit!(EmissionsClaimed {
        oscillator: resonator.oscillator,
        attunement,
        resonance,
        entrainment,
        thicc_now,
        total: final_emission,
        vibe_multiplier,
    });

    msg!(
        "Emissions claimed: {} (A:{} R:{} E:{} T:{})",
        final_emission,
        attunement,
        resonance,
        entrainment,
        thicc_now
    );

    Ok(())
}
