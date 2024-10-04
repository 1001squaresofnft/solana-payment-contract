use anchor_lang::prelude::*;

use crate::{
    constants::{MASTER, VAULT_SOL},
    errors::CustomErrors,
    events::WithdrawSolEvent,
    state::{Master, VaultSol},
};

#[derive(Accounts)]
pub struct WithdrawSolCtx<'info> {
    #[account(
        mut,
        seeds = [MASTER],
        bump
    )]
    master: Account<'info, Master>,

    #[account(
        mut,
        seeds = [VAULT_SOL],
        bump
    )]
    vault_sol: Account<'info, VaultSol>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn process(ctx: Context<WithdrawSolCtx>, amount_sol: u64) -> Result<()> {
    let master = &ctx.accounts.master;
    let signer = &mut ctx.accounts.signer;
    let vault_sol = &mut ctx.accounts.vault_sol;

    require_keys_eq!(
        master.owner,
        signer.key(),
        CustomErrors::NotOwner
    );

    if amount_sol == 0 {
        return Err(CustomErrors::InvalidAmount.into());
    }

    if vault_sol.to_account_info().lamports() < amount_sol {
        return Err(CustomErrors::InsufficientAmount.into());
    }

    vault_sol.sub_lamports(amount_sol)?;
    signer.add_lamports(amount_sol)?;

    emit!(WithdrawSolEvent {
        to: signer.key(),
        amount: amount_sol,
    });

    Ok(())
}
