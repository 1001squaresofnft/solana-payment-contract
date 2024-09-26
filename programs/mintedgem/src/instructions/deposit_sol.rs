use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
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

pub fn process(ctx: Context<DepositSolCtx>, amount_sol: u64) -> Result<()> {
    let vault_sol = &ctx.accounts.vault_sol;
    let signer = &ctx.accounts.signer;
    let system_program = &ctx.accounts.system_program;

    if amount_sol == 0 {
        return Err(CustomErrors::InvalidAmount.into());
    }

    let cpi_context = CpiContext::new(
        system_program.to_account_info(),
        Transfer {
            from: signer.to_account_info(),
            to: vault_sol.to_account_info(),
        },
    );
    transfer(cpi_context, amount_sol)?;

    emit!(DepositSolEvent {
        depositor: signer.key(),
        amount: amount_sol,
    });

    Ok(())
}