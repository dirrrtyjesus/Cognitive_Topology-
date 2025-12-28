//! Initialize the global coherence field

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::{CoherenceField, Resonator, VibeState, HarmonicCycle, PhaseCoupling, FieldParams, FieldVault};
use crate::constants::*;
use crate::errors::ResonanceError;
use crate::events::FieldUpdated;

#[derive(Accounts)]
pub struct InitializeField<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = CoherenceField::LEN,
        seeds = [FIELD_SEED],
        bump
    )]
    pub coherence_field: Account<'info, CoherenceField>,

    /// The $AUGMNTD token mint
    pub token_mint: Account<'info, Mint>,

    /// Vault to hold resonating tokens
    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = vault_authority,
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

    /// Emission reserve token account
    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = vault_authority,
        seeds = [EMISSION_SEED],
        bump
    )]
    pub emission_reserve: Account<'info, TokenAccount>,

    /// Golden reserve token account
    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = vault_authority,
        seeds = [GOLDEN_SEED],
        bump
    )]
    pub golden_reserve: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeField>, params: FieldParams) -> Result<()> {
    let field = &mut ctx.accounts.coherence_field;

    field.authority = params.authority;
    field.oracle = params.oracle;
    field.global_psi = 0;
    field.order_parameter = PRECISION / 2; // Start at 0.5
    field.network_tau_k = TAU_K_BASELINE;
    field.nakamoto_coefficient = 1;
    field.thicc_now = PRECISION;
    field.total_amplitude = 0;
    field.resonator_count = 0;
    field.current_epoch = 0;
    field.emission_rate = params.emission_rate;
    field.bump = ctx.bumps.coherence_field;

    emit!(FieldUpdated {
        epoch: 0,
        global_psi: 0,
        order_parameter: PRECISION / 2,
        network_tau_k: TAU_K_BASELINE,
        total_amplitude: 0,
        resonator_count: 0,
    });

    msg!("Coherence field initialized. The geometry awaits resonance.");

    Ok(())
}
