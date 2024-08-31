use anchor_lang::prelude::*;

use crate::{
    constants::TRANSACTION_SOL_VOLUME,
    state::TransctionSolVolume,
    events::InitTxSolVolume,
};

#[derive(Accounts)]
pub struct InitTransactionSolVolumeCtx<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [TRANSACTION_SOL_VOLUME, signer.key().as_ref()],
        bump,
        space = 8 + TransctionSolVolume::INIT_SPACE,
    )]
    transaction_sol_volume: Account<'info, TransctionSolVolume>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn process(ctx: Context<InitTransactionSolVolumeCtx>) -> Result<()> {
    emit!(InitTxSolVolume {
        signer: ctx.accounts.signer.key(),
    });
    Ok(())
}

