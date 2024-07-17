use anchor_lang::prelude::*;

use crate::states::master::Master;
use crate::errors::Errors;
use crate::OwnerInitialized;

#[derive(Accounts)]
pub struct InitializeCtx<'info> {
    #[account(
        init, 
        payer = signer,
        seeds = [b"master"],
        bump,
        space = 8 + std::mem::size_of::<Master>(),
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<InitializeCtx>, percent: u64) -> Result<()> {
    let master = &mut ctx.accounts.master;

    if master.is_initialized {
        return Err(Errors::MasterAccountAlreadyInitialized.into());
    }

    master.is_initialized = true;
    master.owner = ctx.accounts.signer.key();
    master.percent = percent;

    emit!(OwnerInitialized {});

    Ok(())
}