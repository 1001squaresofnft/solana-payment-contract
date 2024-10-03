use anchor_lang::prelude::*;

use crate::{
    constants::MASTER,
    errors::CustomErrors,
    events::OwnerInitialized,
    state::Master,
};

#[derive(Accounts)]
pub struct InitializeCtx<'info> {
    #[account(
        init, 
        payer = signer,
        seeds = [MASTER],
        bump,
        space = 8 + Master::INIT_SPACE,
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<InitializeCtx>, percent_pay_w_sol: u16, percent_pay_w_done_token: u16) -> Result<()> {
    require!(percent_pay_w_sol <= 10000, CustomErrors::InvalidPercent);
    require!(percent_pay_w_done_token<= 10000, CustomErrors::InvalidPercent);

    let master = &mut ctx.accounts.master;

    if master.is_initialized {
        return Err(CustomErrors::MasterAccountAlreadyInitialized.into());
    }

    master.is_initialized = true;
    master.owner = ctx.accounts.signer.key();
    master.percent_pay_w_sol = percent_pay_w_sol;
    master.percent_pay_w_done_token = percent_pay_w_done_token;

    emit!(OwnerInitialized {});

    Ok(())
}