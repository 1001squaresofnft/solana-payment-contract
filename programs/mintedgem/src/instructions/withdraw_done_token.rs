use anchor_lang::prelude::*;
use anchor_spl::token::{
    Transfer, Token, Mint, TokenAccount
};

use crate::{
    constants::{MASTER, TOKEN_ACCOUNT_OWNER, VAULT_TOKEN},
    errors::CustomErrors,
    events::WithdrawDoneTokenEvent,
    state::Master,
};

#[derive(Accounts)]
pub struct WithdrawDoneTokenCtx<'info> {
    #[account(
        mut, 
        seeds = [MASTER],
        bump,
    )]
    master: Account<'info, Master>,

    mint_of_token_being_sent: Account<'info, Mint>,
    /// CHECK
    #[account(mut,
        seeds=[TOKEN_ACCOUNT_OWNER],
        bump
    )]
    token_account_owner_pda: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [VAULT_TOKEN, mint_of_token_being_sent.key().as_ref()],
        bump,
        token::mint = mint_of_token_being_sent,
        token::authority = token_account_owner_pda,
    )]
    vault_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_of_token_being_sent,
        associated_token::authority = signer,
    )]
    sender_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    signer: Signer<'info>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<WithdrawDoneTokenCtx>, _amount_done: u64) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.master.owner,
        ctx.accounts.signer.key(),
        CustomErrors::NotOwner
    );

    if _amount_done == 0 {
        return Err(CustomErrors::InvalidAmount.into());
    }

    if ctx.accounts.vault_token.amount < _amount_done {
        return Err(CustomErrors::InsufficientAmount.into());
    }

    let transfer_instruction = Transfer {
        from: ctx.accounts.vault_token.to_account_info(),
        to: ctx.accounts.sender_token_account.to_account_info(),
        authority: ctx.accounts.token_account_owner_pda.to_account_info(),
    };

    let bump = ctx.bumps.token_account_owner_pda;
    let seeds = &[TOKEN_ACCOUNT_OWNER, &[bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        signer,
    );

    anchor_spl::token::transfer(cpi_ctx, _amount_done)?;

    emit!(WithdrawDoneTokenEvent {
        to: ctx.accounts.signer.key(),
        amount_done: _amount_done,
    });

    Ok(())
}
