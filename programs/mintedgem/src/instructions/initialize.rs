use anchor_lang::prelude::*;

use crate::{
    constants::MASTER, errors::CustomErrors, events::OwnerInitialized, program::Mintedgem, state::Master
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
    #[account(
        constraint = mintedgem_program.programdata_address()? == Some(mintedgem_program_data.key()) @ CustomErrors::InvalidMintedgemProgram
    )]
    mintedgem_program: Program<'info, Mintedgem>,
    #[account(
        constraint = mintedgem_program_data.upgrade_authority_address == Some(signer.key()) @ CustomErrors::InvalidAdmin
    )]
    mintedgem_program_data: Account<'info, ProgramData>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<InitializeCtx>, percent_pay_w_sol: u16, percent_pay_w_done_token: u16) -> Result<()> {
    let master = &mut ctx.accounts.master;
    let signer = &ctx.accounts.signer;

    require!(percent_pay_w_sol <= 10000, CustomErrors::InvalidPercent);
    require!(percent_pay_w_done_token<= 10000, CustomErrors::InvalidPercent);

    if master.is_initialized {
        return Err(CustomErrors::MasterAccountAlreadyInitialized.into());
    }

    master.is_initialized = true;
    master.owner = signer.key();
    master.percent_pay_w_sol = percent_pay_w_sol;
    master.percent_pay_w_done_token = percent_pay_w_done_token;

    emit!(OwnerInitialized {});

    Ok(())
}