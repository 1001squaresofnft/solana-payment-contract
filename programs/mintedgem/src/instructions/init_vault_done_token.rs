use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::states::master::Master;
use crate::errors::Errors;
use crate::VaultDoneTokenInitialized;

#[derive(Accounts)]
pub struct InitVaultDoneTokenCtx<'info> {
    #[account(
        mut, 
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    /// CHECK
    #[account(
        init, 
        payer = signer, 
        seeds = [b"token_account_owner"],
        bump,
        space = 8
    )]
    token_account_owner_pda: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"vault_token", mint_of_token_being_sent.key().as_ref()],
        token::mint = mint_of_token_being_sent,
        token::authority = token_account_owner_pda,
        bump,
    )]
    vault_token: Account<'info, token::TokenAccount>,
    mint_of_token_being_sent: Account<'info, token::Mint>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
    rent: Sysvar<'info, Rent>,
}

    pub fn process(ctx: Context<InitVaultDoneTokenCtx>) -> Result<()> {

        require_keys_eq!(ctx.accounts.master.owner, ctx.accounts.signer.key(), Errors::NotOwner);

        if ctx.accounts.master.is_vault_done_token_initialized {
            return Err(Errors::VaultDoneTokenAlreadyInitialized.into());
        }

        ctx.accounts.master.is_vault_done_token_initialized = true;

        emit!(VaultDoneTokenInitialized {});

        Ok(())
    }