use anchor_lang::prelude::*;

mod constants;
mod errors;
mod events;
mod instructions;
mod state;

use instructions::*;

declare_id!("p4oawzVcmB9BSTKtDjRNhgbR4qFdHufnz5Zft2wNBF5");

#[program]
pub mod mintedgem {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCtx>, percent: u64) -> Result<()> {
        initialize::process(ctx, percent)
    }

    pub fn init_vault_sol(ctx: Context<InitVaultSolCtx>) -> Result<()> {
        init_vault_sol::process(ctx)
    }

    pub fn init_vault_done_token(ctx: Context<InitVaultDoneTokenCtx>) -> Result<()> {
        init_vault_done_token::process(ctx)
    }

    pub fn deposit_sol(ctx: Context<TransferSolCtx>, amount: u64) -> Result<()> {
        deposit_sol::process(ctx, amount)
    }

    pub fn deposit_done_token(ctx: Context<TransferTokenCtx>, amount: u64) -> Result<()> {
        deposit_done_token::process(ctx, amount)
    }

    pub fn create_payment(
        ctx: Context<CreatePaymentContext>,
        item_id: u64,
        amount_sol: u64,
    ) -> Result<()> {
        create_payment::process(ctx, item_id, amount_sol)
    }

    pub fn create_payment_by_done(
        ctx: Context<CreatePaymentByDoneCtx>,
        item_id: u64,
        amount_done: u64,
    ) -> Result<()> {
        create_payment_by_done::process(ctx, item_id, amount_done)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSolCtx>, amount_sol: u64) -> Result<()> {
        withdraw_sol::process(ctx, amount_sol)
    }

    pub fn withdraw_done_token(ctx: Context<WithdrawDoneTokenCtx>, amount_done: u64) -> Result<()> {
        withdraw_done_token::process(ctx, amount_done)
    }

    pub fn set_percent(ctx: Context<SetPercentCtx>, percent: u64) -> Result<()> {
        set_percent::process(ctx, percent)
    }

    pub fn hello(ctx: Context<HelloCtx>) -> Result<()> {
        hello::hello(ctx)
    }
}
