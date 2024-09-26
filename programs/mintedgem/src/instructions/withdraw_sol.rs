use anchor_lang::{prelude::*, system_program};

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

pub fn process(ctx: Context<WithdrawSolCtx>, _amount_sol: u64) -> Result<()> {
    let vault_sol = &ctx.accounts.vault_sol;

    require_keys_eq!(
        ctx.accounts.master.owner,
        ctx.accounts.signer.key(),
        CustomErrors::NotOwner
    );

    if _amount_sol == 0 {
        return Err(CustomErrors::InvalidAmount.into());
    }

    if vault_sol.to_account_info().lamports() < _amount_sol {
        return Err(CustomErrors::InsufficientAmount.into());
    }

    let signer_seeds: &[&[&[u8]]] = &[&[VAULT_SOL, &[ctx.bumps.vault_sol]]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.vault_sol.to_account_info().clone(),
            to: ctx.accounts.signer.to_account_info().clone(),
        },
        signer_seeds,
    );

    system_program::transfer(cpi_context, _amount_sol)?;

    emit!(WithdrawSolEvent {
        to: ctx.accounts.signer.key(),
        amount: _amount_sol,
    });

    Ok(())
}
