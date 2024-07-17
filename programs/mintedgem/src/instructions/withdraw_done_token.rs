use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::states::master::Master;
use crate::errors::Errors;
use crate::WithdrawDoneTokenEvent;

#[derive(Accounts)]
pub struct WithdrawDoneTokenCtx<'info> {
    #[account(
        mut, 
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    /// CHECK
    #[account(mut,
        seeds=[b"token_account_owner"],
        bump
    )]
    token_account_owner_pda: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"vault_token", mint_of_token_being_sent.key().as_ref()],
        bump
    )]
    vault_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    sender_token_account: Account<'info, token::TokenAccount>,
    mint_of_token_being_sent: Account<'info, token::Mint>,

    #[account(mut)]
    signer: Signer<'info>,
    token_program: Program<'info, token::Token>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<WithdrawDoneTokenCtx>, amount_done: u64) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.master.owner,
        ctx.accounts.signer.key(),
        Errors::NotOwner
    );

    if ctx.accounts.vault_token.amount < amount_done {
        return Err(Errors::DeoDuSoDu.into());
    }

    let transfer_instruction = token::Transfer {
        from: ctx.accounts.vault_token.to_account_info(),
        to: ctx.accounts.sender_token_account.to_account_info(),
        authority: ctx.accounts.token_account_owner_pda.to_account_info(),
    };

    let bump = ctx.bumps.token_account_owner_pda;
    let seeds = &[b"token_account_owner".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        signer,
    );

    anchor_spl::token::transfer(cpi_ctx, amount_done)?;

    emit!(WithdrawDoneTokenEvent {
        to: ctx.accounts.signer.key(),
        amount_done: amount_done,
    });

    Ok(())
}
