use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::{
    constants::{MASTER, VAULT_SOL},
    errors::CustomErrors,
    events::DepositSolEvent,
    state::Master,
    state::VaultSol,
};

#[derive(Accounts)]
pub struct DepositSolCtx<'info> {
    #[account(
        mut, 
        seeds = [MASTER],
        bump,
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

pub fn process(ctx: Context<DepositSolCtx>, _amount: u64) -> Result<()> {
    if _amount == 0 {
        return Err(CustomErrors::InvalidAmount.into());
    }

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.signer.to_account_info().clone(),
            to: ctx.accounts.vault_sol.to_account_info().clone(),
        },
    );
    system_program::transfer(cpi_context, _amount)?;

    emit!(DepositSolEvent {
        depositor: ctx.accounts.signer.key(),
        amount: _amount,
    });

    Ok(())
}