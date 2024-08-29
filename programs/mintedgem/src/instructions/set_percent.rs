use anchor_lang::prelude::*;

use crate::{errors::Errors, events::SetPercent, state::Master};

#[derive(Accounts)]
pub struct SetPercentCtx<'info> {
    #[account(
        mut,
        seeds = [b"master"],
        bump
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn process(ctx: Context<SetPercentCtx>, percent: u64) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.master.owner,
        ctx.accounts.signer.key(),
        Errors::NotOwner
    );

    ctx.accounts.master.percent = percent;

    emit!(SetPercent { percent: percent });

    Ok(())
}
