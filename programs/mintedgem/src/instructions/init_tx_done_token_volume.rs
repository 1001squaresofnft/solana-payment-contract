use anchor_lang::prelude::*;

use crate::{
    constants::TRANSACTION_DONE_TOKEN_VOLUME, 
    events::InitTxDoneTokenVolume,
    state::TransactionDoneTokenVolume,
};

#[derive(Accounts)]
pub struct InitTransactionDoneTokenVolumeCtx<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [TRANSACTION_DONE_TOKEN_VOLUME, signer.key().as_ref()],
        bump,
        space = 8 + TransactionDoneTokenVolume::INIT_SPACE,
    )]
    transaction_done_token_volume: Account<'info, TransactionDoneTokenVolume>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn process(ctx: Context<InitTransactionDoneTokenVolumeCtx>) -> Result<()> {
    emit!(InitTxDoneTokenVolume {
        signer: ctx.accounts.signer.key(),
    });
    Ok(())
}
