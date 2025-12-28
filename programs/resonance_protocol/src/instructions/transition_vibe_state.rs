//! Transition vibe state if eligible

use anchor_lang::prelude::*;

use crate::state::{CoherenceField, Resonator, VibeState};
use crate::constants::*;
use crate::errors::ResonanceError;
use crate::events::{VibeStateTransition, GoldenAchieved};
use crate::math;

#[derive(Accounts)]
pub struct TransitionVibeState<'info> {
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
}

pub fn handler(ctx: Context<TransitionVibeState>) -> Result<()> {
    let field = &ctx.accounts.coherence_field;
    let resonator = &mut ctx.accounts.resonator;

    let current_phase_lock = math::phase_lock(resonator.theta, field.global_psi)?;
    let phase_lock_unsigned = if current_phase_lock > 0 {
        current_phase_lock as u64
    } else {
        0
    };

    let old_state = resonator.vibe_state;
    let mut new_state = old_state;

    match old_state {
        VibeState::Attuning => {
            // Transition to Resonant if phase_lock > 0.5 for RESONANT_EPOCHS
            if phase_lock_unsigned > RESONANT_THRESHOLD
                && resonator.phase_lock_epochs >= RESONANT_EPOCHS
            {
                new_state = VibeState::Resonant;
            }
        }

        VibeState::Resonant => {
            // Transition to Entrained if phase_lock > 0.8 for ENTRAINED_EPOCHS AND τₖ > 5.0
            if phase_lock_unsigned > ENTRAINED_THRESHOLD
                && resonator.phase_lock_epochs >= ENTRAINED_EPOCHS
                && resonator.tau_k > TAU_K_BASELINE
            {
                new_state = VibeState::Entrained;
            }
        }

        VibeState::Entrained => {
            // Transition to Golden if τₖ > φ⁴ AND phase_lock > 0.9
            if resonator.tau_k > PHI_FOURTH
                && phase_lock_unsigned > GOLDEN_PHASE_THRESHOLD
            {
                new_state = VibeState::Golden;
            }
        }

        VibeState::Golden => {
            // Already at highest state
            return Err(ResonanceError::AlreadyGolden.into());
        }
    }

    // Check if transition occurred
    if new_state == old_state {
        return Err(ResonanceError::NotEligibleForTransition.into());
    }

    // Apply transition
    resonator.vibe_state = new_state;

    emit!(VibeStateTransition {
        oscillator: resonator.oscillator,
        from_state: old_state,
        to_state: new_state,
        tau_k: resonator.tau_k,
        phase_lock: phase_lock_unsigned,
    });

    // Special event for Golden achievement
    if new_state == VibeState::Golden {
        let epochs_to_golden = field.current_epoch.saturating_sub(resonator.harmonic_entry);

        emit!(GoldenAchieved {
            oscillator: resonator.oscillator,
            tau_k: resonator.tau_k,
            epochs_to_golden,
            total_emissions: resonator.total_emissions,
        });

        msg!(
            "🌟 GOLDEN STATE ACHIEVED! τₖ: {}, Epochs: {}",
            resonator.tau_k,
            epochs_to_golden
        );
    } else {
        msg!(
            "Vibe state transition: {:?} -> {:?}",
            old_state,
            new_state
        );
    }

    Ok(())
}
