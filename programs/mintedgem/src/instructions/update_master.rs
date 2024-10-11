use anchor_lang::prelude::*;

use crate::{
    constants::MASTER,
    errors::CustomErrors,
    events::{SetOwner, SetPercentPayWDoneToken, SetPercentPayWSol},
    state::Master,
};

#[derive(Accounts)]
pub struct UpdateMasterCtx<'info> {
    #[account(
        mut,
        seeds = [MASTER],
        bump
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn set_percent_pay_w_sol(ctx: Context<UpdateMasterCtx>, percent_pay_w_sol: u16) -> Result<()> {
    require!(percent_pay_w_sol <= 10000, CustomErrors::InvalidPercent);

    require_keys_eq!(
        ctx.accounts.master.owner,
        ctx.accounts.signer.key(),
        CustomErrors::NotOwner
    );

    ctx.accounts.master.percent_pay_w_sol = percent_pay_w_sol;

    emit!(SetPercentPayWSol { percent_pay_w_sol });

    Ok(())
}

pub fn set_percent_pay_w_done_token(
    ctx: Context<UpdateMasterCtx>,
    percent_pay_w_done_token: u16,
) -> Result<()> {
    require!(
        percent_pay_w_done_token <= 10000,
        CustomErrors::InvalidPercent
    );

    require_keys_eq!(
        ctx.accounts.master.owner,
        ctx.accounts.signer.key(),
        CustomErrors::NotOwner
    );

    ctx.accounts.master.percent_pay_w_done_token = percent_pay_w_done_token;

    emit!(SetPercentPayWDoneToken {
        percent_pay_w_done_token
    });

    Ok(())
}

pub fn set_owner(ctx: Context<UpdateMasterCtx>, new_owner: Pubkey) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.master.owner,
        ctx.accounts.signer.key(),
        CustomErrors::NotOwner
    );

    ctx.accounts.master.owner = new_owner;

    emit!(SetOwner { new_owner });

    Ok(())
}
