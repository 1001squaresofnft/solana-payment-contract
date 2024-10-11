use anchor_lang::prelude::*;
use anchor_spl::token::{
    Transfer, Token, Mint, TokenAccount, transfer
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

pub fn process(ctx: Context<WithdrawDoneTokenCtx>, amount_done: u64) -> Result<()> {
    let master = &mut ctx.accounts.master;
    let signer = &mut ctx.accounts.signer;
    let vault_token = &mut ctx.accounts.vault_token;
    let sender_token_account = &mut ctx.accounts.sender_token_account;
    let token_account_owner_pda = &mut ctx.accounts.token_account_owner_pda;
    let token_program = &mut ctx.accounts.token_program;

    require_keys_eq!(
        master.owner,
        signer.key(),
        CustomErrors::NotOwner
    );

    if amount_done == 0 {
        return Err(CustomErrors::InvalidAmount.into());
    }

    if vault_token.amount < amount_done {
        return Err(CustomErrors::InsufficientAmount.into());
    }

    let transfer_instruction = Transfer {
        from: vault_token.to_account_info(),
        to: sender_token_account.to_account_info(),
        authority: token_account_owner_pda.to_account_info(),
    };

    let bump = ctx.bumps.token_account_owner_pda;
    let seeds = &[TOKEN_ACCOUNT_OWNER, &[bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        transfer_instruction,
        signer_seeds,
    );

    let result = transfer(cpi_ctx, amount_done);

    if result.is_err() {
        return Err(CustomErrors::TransferFailed.into());
    }

    emit!(WithdrawDoneTokenEvent {
        to: signer.key(),
        amount_done: amount_done,
    });

    Ok(())
}
