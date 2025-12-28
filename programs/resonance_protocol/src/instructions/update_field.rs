//! Update field state (called by oracle/crank)

use anchor_lang::prelude::*;

use crate::state::{CoherenceField, Resonator, VibeState, HarmonicCycle, PhaseCoupling, FieldParams, FieldVault};
use crate::constants::*;
use crate::errors::ResonanceError;
use crate::events::{FieldUpdated, CoherencePeak};

#[derive(Accounts)]
pub struct UpdateField<'info> {
    #[account(
        constraint = oracle.key() == coherence_field.oracle @ ResonanceError::UnauthorizedOracle
    )]
    pub oracle: Signer<'info>,

    #[account(
        mut,
        seeds = [FIELD_SEED],
        bump = coherence_field.bump
    )]
    pub coherence_field: Account<'info, CoherenceField>,
}

pub fn handler(
    ctx: Context<UpdateField>,
    new_psi: u64,
    new_r: u64,
    new_tau_k: u64,
) -> Result<()> {
    let field = &mut ctx.accounts.coherence_field;

    let old_r = field.order_parameter;

    // Update field state
    field.global_psi = new_psi % TWO_PI;
    field.order_parameter = new_r.min(PRECISION); // Cap at 1.0
    field.network_tau_k = new_tau_k;
    field.current_epoch = field.current_epoch.checked_add(1)
        .ok_or(ResonanceError::MathOverflow)?;

    // Detect coherence peak
    if new_r > old_r && new_r > 900_000_000 { // R > 0.9
        emit!(CoherencePeak {
            epoch: field.current_epoch,
            order_parameter: new_r,
            resonators_in_phase: field.resonator_count, // Simplified
        });
    }

    emit!(FieldUpdated {
        epoch: field.current_epoch,
        global_psi: field.global_psi,
        order_parameter: field.order_parameter,
        network_tau_k: field.network_tau_k,
        total_amplitude: field.total_amplitude,
        resonator_count: field.resonator_count,
    });

    msg!(
        "Field updated. Epoch: {}, Ψ: {}, R: {}, τₖ: {}",
        field.current_epoch,
        field.global_psi,
        field.order_parameter,
        field.network_tau_k
    );

    Ok(())
}
