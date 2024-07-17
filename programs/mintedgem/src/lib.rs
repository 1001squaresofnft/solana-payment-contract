use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;

use crate::events::*;
use instructions::{
    create_payment::*, create_payment_by_done::*, init_vault_done_token::*, init_vault_sol::*,
    initialize::*, set_percent::*, transfer_done_token_in::*, transfer_sol_in::*,
    withdraw_done_token::*, withdraw_sol::*,
};

declare_id!("CdMkddBBv9zdB33UErvhzqH4XcZ3ZSxrBxY72Qo1vyUV");

#[program]
pub mod mintedgem {

    use super::*;

    pub fn initialize(ctx: Context<InitializeCtx>, percent: u64) -> Result<()> {
        instructions::initialize::process(ctx, percent)
    }

    pub fn init_vault_sol(ctx: Context<InitVaultSolCtx>) -> Result<()> {
        instructions::init_vault_sol::process(ctx)
    }

    pub fn init_vault_done_token(ctx: Context<InitVaultDoneTokenCtx>) -> Result<()> {
        instructions::init_vault_done_token::process(ctx)
    }

    pub fn transfer_sol_in(ctx: Context<TransferSolCtx>, amount: u64) -> Result<()> {
        instructions::transfer_sol_in::process(ctx, amount)
    }

    pub fn transfer_done_token_in(ctx: Context<TransferTokenCtx>, amount: u64) -> Result<()> {
        instructions::transfer_done_token_in::process(ctx, amount)
    }

    pub fn create_payment(
        ctx: Context<CreatePaymentContext>,
        item_id: u64,
        amount_sol: u64,
    ) -> Result<()> {
        instructions::create_payment::process(ctx, item_id, amount_sol)
    }

    pub fn create_payment_by_done(
        ctx: Context<CreatePaymentByDoneCtx>,
        item_id: u64,
        amount_done: u64,
    ) -> Result<()> {
        instructions::create_payment_by_done::process(ctx, item_id, amount_done)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSolCtx>, amount_sol: u64) -> Result<()> {
        instructions::withdraw_sol::process(ctx, amount_sol)
    }

    pub fn withdraw_done_token(ctx: Context<WithdrawDoneTokenCtx>, amount_done: u64) -> Result<()> {
        instructions::withdraw_done_token::process(ctx, amount_done)
    }

    pub fn set_percent(ctx: Context<SetPercentCtx>, percent: u64) -> Result<()> {
        instructions::set_percent::process(ctx, percent)
    }

    pub fn hello(_ctx: Context<HelloCtx>) -> Result<()> {
        emit!(Hello {
            msg: String::from("Hello, Mintedgem!"),
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct HelloCtx {}
