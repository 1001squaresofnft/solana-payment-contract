use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::states::master::Master;

use crate::DepositDoneTokenEvent;

#[derive(Accounts)]
pub struct TransferTokenCtx<'info> {
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
    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<TransferTokenCtx>, amount: u64) -> Result<()> {
        let transfer_instruction = token::Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.vault_token.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
    
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );
    
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        emit!(DepositDoneTokenEvent {
            depositor: ctx.accounts.signer.key(),
            amount_done: amount,
        });

        Ok(())
    }