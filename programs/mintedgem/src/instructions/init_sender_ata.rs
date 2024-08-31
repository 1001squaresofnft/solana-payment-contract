use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::events::InitSenderAta;

#[derive(Accounts)]
pub struct InitSenderAtaCtx<'info> {
    mint_of_token_being_sent: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_of_token_being_sent,
        associated_token::authority = signer,
    )]
    sender_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn process(ctx: Context<InitSenderAtaCtx>) -> Result<()> {
    emit!(InitSenderAta {
        signer: ctx.accounts.signer.key(),
    });
    Ok(())
}
