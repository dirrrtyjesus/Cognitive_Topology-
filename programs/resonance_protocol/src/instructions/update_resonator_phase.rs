//! Update resonator phase (Kuramoto dynamics)

use anchor_lang::prelude::*;

use crate::state::{CoherenceField, Resonator, VibeState};
use crate::constants::*;
use crate::errors::ResonanceError;
use crate::events::PhaseUpdated;
use crate::math;

#[derive(Accounts)]
pub struct UpdateResonatorPhase<'info> {
    /// Anyone can crank phase updates
    pub crank: Signer<'info>,

    #[account(
        seeds = [FIELD_SEED],
        bump = coherence_field.bump
    )]
    pub coherence_field: Account<'info, CoherenceField>,

    #[account(
        mut,
        seeds = [RESONATOR_SEED, resonator.oscillator.as_ref()],
        bump = resonator.bump
    )]
    pub resonator: Account<'info, Resonator>,
}

pub fn handler(ctx: Context<UpdateResonatorPhase>) -> Result<()> {
    let field = &ctx.accounts.coherence_field;
    let resonator = &mut ctx.accounts.resonator;

    let old_theta = resonator.theta;

    // Apply Kuramoto dynamics
    // dθᵢ = ωᵢ + K * R * sin(Ψ - θᵢ)
    let coupling_k = match resonator.vibe_state {
        VibeState::Attuning => PRECISION / 10,    // Low coupling while attuning
        VibeState::Resonant => PRECISION / 5,     // Medium coupling
        VibeState::Entrained => PRECISION / 3,    // Strong coupling
        VibeState::Golden => PRECISION / 2,       // Very strong coupling
    };

    let new_theta = math::kuramoto_phase_update(
        resonator.theta,
        resonator.omega,
        field.global_psi,
        field.order_parameter,
        coupling_k,
    )?;

    resonator.theta = new_theta;

    // Calculate current phase lock
    let phase_lock = math::phase_lock(new_theta, field.global_psi)?;
    let phase_lock_unsigned = if phase_lock > 0 { phase_lock as u64 } else { 0 };

    // Update phase-lock tracking for state transitions
    if phase_lock_unsigned > RESONANT_THRESHOLD {
        resonator.phase_lock_epochs = resonator.phase_lock_epochs
            .checked_add(1)
            .ok_or(ResonanceError::MathOverflow)?;
    } else {
        // Reset if phase lock drops
        resonator.phase_lock_epochs = 0;
    }

    // Gradually increase τₖ based on sustained coherence
    if phase_lock_unsigned > ENTRAINED_THRESHOLD {
        // τₖ grows when deeply phase-locked
        let tau_growth = PRECISION / 1000; // 0.1% per update
        resonator.tau_k = resonator.tau_k
            .checked_add(tau_growth)
            .ok_or(ResonanceError::MathOverflow)?;
    }

    emit!(PhaseUpdated {
        oscillator: resonator.oscillator,
        old_theta,
        new_theta,
        phase_lock: phase_lock_unsigned,
    });

    Ok(())
}
