use anchor_lang::prelude::*;
use anchor_spl::token::{
    TokenAccount, Mint, Transfer, Token, transfer
};

use crate::{
    constants::{MASTER, TOKEN_ACCOUNT_OWNER, VAULT_TOKEN},
    errors::CustomErrors,
    events::DepositDoneTokenEvent,
    state::Master,
};

#[derive(Accounts)]
pub struct TransferTokenCtx<'info> {
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
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<TransferTokenCtx>, _amount: u64) -> Result<()> {
        if _amount <= 0 {
            return Err(CustomErrors::InvalidAmount.into());
        }

        let transfer_instruction = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.vault_token.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
    
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );
    
        transfer(cpi_ctx, _amount)?;

        emit!(DepositDoneTokenEvent {
            depositor: ctx.accounts.signer.key(),
            amount_done: _amount,
        });

        Ok(())
    }