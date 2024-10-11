use anchor_lang::prelude::*;

use crate::{
    constants::{MASTER, VAULT_SOL}, 
    errors::CustomErrors, 
    events::VaultSolInitialized, 
    state::{Master, VaultSol}
};

#[derive(Accounts)]
pub struct InitVaultSolCtx<'info> {
    #[account(
        mut, 
        seeds = [MASTER],
        bump,
    )]
    master: Account<'info, Master>,

    #[account(
        init, 
        payer = signer,
        seeds = [VAULT_SOL],
        bump,
        space = 8 + VaultSol::INIT_SPACE,
    )]
    vault_sol: Account<'info, VaultSol>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<InitVaultSolCtx>) -> Result<()> {
    let master = &mut ctx.accounts.master;
    let signer = &ctx.accounts.signer;
        
    require_keys_eq!(master.owner, signer.key(), CustomErrors::NotOwner);

    if master.is_vault_sol_initialized {
        return Err(CustomErrors::VaultSolAlreadyInitialized.into());
    }

    master.is_vault_sol_initialized = true;

    emit!(VaultSolInitialized {});

    Ok(())
}