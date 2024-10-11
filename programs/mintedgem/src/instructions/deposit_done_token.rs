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
pub struct DepositDoneCtx<'info> {
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

pub fn process(ctx: Context<DepositDoneCtx>, amount_done_token: u64) -> Result<()> {
        let sender_token_account = &mut ctx.accounts.sender_token_account;
        let vault_token = &mut ctx.accounts.vault_token;
        let signer = &ctx.accounts.signer;
        let token_program = &ctx.accounts.token_program;

        if amount_done_token == 0 {
            return Err(CustomErrors::InvalidAmount.into());
        }

        let transfer_instruction = Transfer {
            from: sender_token_account.to_account_info(),
            to: vault_token.to_account_info(),
            authority: signer.to_account_info(),
        };
    
        let cpi_ctx = CpiContext::new(
            token_program.to_account_info(),
            transfer_instruction,
        );
    
        let result = transfer(cpi_ctx, amount_done_token);

        if result.is_err() {
            return Err(CustomErrors::TransferFailed.into());
        }

        emit!(DepositDoneTokenEvent {
            depositor: signer.key(),
            amount_done: amount_done_token,
        });

        Ok(())
    }