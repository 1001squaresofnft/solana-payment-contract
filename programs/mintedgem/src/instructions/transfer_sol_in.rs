use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::{
    events::DepositSolEvent,
    state::Master,
    state::VaultSol,
};

#[derive(Accounts)]
pub struct TransferSolCtx<'info> {
    #[account(
        mut, 
        seeds = [b"master"],
        bump,
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

pub fn process(ctx: Context<TransferSolCtx>, amount: u64) -> Result<()> {
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.signer.to_account_info().clone(),
            to: ctx.accounts.vault_sol.to_account_info().clone(),
        },
    );
    system_program::transfer(cpi_context, amount)?;

    emit!(DepositSolEvent {
        depositor: ctx.accounts.signer.key(),
        amount: amount,
    });

    Ok(())
}