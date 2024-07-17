use anchor_lang::prelude::*;

use crate::states::master::Master;
use crate::states::vault_sol::VaultSol;
use crate::errors::Errors;
use crate::VaultSolInitialized;

#[derive(Accounts)]
pub struct InitVaultSolCtx<'info> {
    #[account(
        mut, 
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    #[account(
        init, 
        payer = signer,
        seeds = [b"vault_sol"],
        bump,
        space = 8 + std::mem::size_of::<VaultSol>(),
    )]
    vault_sol: Account<'info, VaultSol>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<InitVaultSolCtx>) -> Result<()> {
        
    require_keys_eq!(ctx.accounts.master.owner, ctx.accounts.signer.key(), Errors::NotOwner);

    if ctx.accounts.master.is_vault_sol_initialized {
        return Err(Errors::VaultSolAlreadyInitialized.into());
    }

    ctx.accounts.master.is_vault_sol_initialized = true;

    emit!(VaultSolInitialized {});

    Ok(())
}