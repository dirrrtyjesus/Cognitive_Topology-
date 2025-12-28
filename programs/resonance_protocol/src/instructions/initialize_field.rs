//! Initialize the global coherence field

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::{CoherenceField, FieldParams};
use crate::constants::*;
use crate::events::FieldUpdated;

/// Initialize just the coherence field account
/// Vault and reserves are initialized separately to avoid stack overflow
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

    pub system_program: Program<'info, System>,
}

/// Initialize the vault (call after initialize_field)
#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [FIELD_SEED],
        bump = coherence_field.bump,
        constraint = coherence_field.authority == authority.key()
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

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// Initialize emission reserve (call after initialize_vault)
#[derive(Accounts)]
pub struct InitializeEmissionReserve<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [FIELD_SEED],
        bump = coherence_field.bump,
        constraint = coherence_field.authority == authority.key()
    )]
    pub coherence_field: Account<'info, CoherenceField>,

    /// The $AUGMNTD token mint
    pub token_mint: Account<'info, Mint>,

    /// CHECK: PDA authority for reserves
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

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// Initialize golden reserve (call after emission reserve)
#[derive(Accounts)]
pub struct InitializeGoldenReserve<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [FIELD_SEED],
        bump = coherence_field.bump,
        constraint = coherence_field.authority == authority.key()
    )]
    pub coherence_field: Account<'info, CoherenceField>,

    /// The $AUGMNTD token mint
    pub token_mint: Account<'info, Mint>,

    /// CHECK: PDA authority for reserves
    #[account(
        seeds = [VAULT_SEED, b"authority"],
        bump
    )]
    pub vault_authority: AccountInfo<'info>,

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

pub fn handler_vault(_ctx: Context<InitializeVault>) -> Result<()> {
    msg!("Vault initialized. Ready to receive resonance.");
    Ok(())
}

pub fn handler_emission_reserve(_ctx: Context<InitializeEmissionReserve>) -> Result<()> {
    msg!("Emission reserve initialized.");
    Ok(())
}

pub fn handler_golden_reserve(_ctx: Context<InitializeGoldenReserve>) -> Result<()> {
    msg!("Golden reserve initialized. Emission channels open.");
    Ok(())
}
