use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, TokenAccount, Token
};

use crate::{
    constants::{MASTER, TOKEN_ACCOUNT_OWNER, VAULT_TOKEN},
    errors::CustomErrors,
    events::VaultDoneTokenInitialized,
    state::Master,
};

#[derive(Accounts)]
pub struct InitVaultDoneTokenCtx<'info> {
    #[account(
        mut, 
        seeds = [MASTER],
        bump,
    )]
    master: Account<'info, Master>,

    mint_of_token_being_sent: Account<'info, Mint>,
    /// CHECK
    #[account(
        init, 
        payer = signer, 
        seeds = [TOKEN_ACCOUNT_OWNER],
        bump,
        space = 8
    )]
    token_account_owner_pda: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [VAULT_TOKEN, mint_of_token_being_sent.key().as_ref()],
        token::mint = mint_of_token_being_sent,
        token::authority = token_account_owner_pda,
        bump,
    )]
    vault_token: Account<'info, TokenAccount>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

    pub fn process(ctx: Context<InitVaultDoneTokenCtx>) -> Result<()> {

        require_keys_eq!(ctx.accounts.master.owner, ctx.accounts.signer.key(), CustomErrors::NotOwner);

        if ctx.accounts.master.is_vault_done_token_initialized {
            return Err(CustomErrors::VaultDoneTokenAlreadyInitialized.into());
        }

        ctx.accounts.master.is_vault_done_token_initialized = true;

        emit!(VaultDoneTokenInitialized {});

        Ok(())
    }