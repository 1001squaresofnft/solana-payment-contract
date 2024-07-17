use anchor_lang::prelude::*;

use crate::states::{master::Master, vault_sol::VaultSol};
use crate::errors::Errors;
use crate::WithdrawSolEvent;

#[derive(Accounts)]
pub struct WithdrawSolCtx<'info> {
    #[account(
        mut,
        seeds = [b"master"],
        bump
    )]
    master: Account<'info, Master>,

    #[account(
        mut,
        seeds = [b"vault_sol"],
        bump
    )]
    vault_sol: Account<'info, VaultSol>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn process(ctx: Context<WithdrawSolCtx>, amount_sol: u64) -> Result<()> {
    let vault_sol = &ctx.accounts.vault_sol;

    require_keys_eq!(
        ctx.accounts.master.owner,
        ctx.accounts.signer.key(),
        Errors::NotOwner
    );

    if vault_sol.to_account_info().lamports() < amount_sol {
        return Err(Errors::DeoDuSoDu.into());
    }

    ctx.accounts.vault_sol.sub_lamports(amount_sol)?;
    ctx.accounts.signer.add_lamports(amount_sol)?;

    emit!(WithdrawSolEvent {
        to: ctx.accounts.signer.key(),
        amount: amount_sol,
    });

    Ok(())
}
