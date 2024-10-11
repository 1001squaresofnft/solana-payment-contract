use anchor_lang::prelude::*;

mod constants;
mod errors;
mod events;
mod instructions;
mod state;

use instructions::*;

declare_id!("6nJtHYi1D6Vcxq5PXdusDWpB8M9GACHosMdch3yJgh6d");

#[program]
pub mod mintedgem {
    use super::*;

    // ADMIN FUNCTIONS
    pub fn initialize(ctx: Context<InitializeCtx>, percent_pay_w_sol: u16, percent_pay_w_done_token: u16) -> Result<()> {
        initialize::process(ctx, percent_pay_w_sol, percent_pay_w_done_token)
    }

    pub fn init_vault_sol(ctx: Context<InitVaultSolCtx>) -> Result<()> {
        init_vault_sol::process(ctx)
    }

    pub fn init_vault_done_token(ctx: Context<InitVaultDoneTokenCtx>) -> Result<()> {
        init_vault_done_token::process(ctx)
    }

    pub fn deposit_sol(ctx: Context<DepositSolCtx>, amount: u64) -> Result<()> {
        deposit_sol::process(ctx, amount)
    }

    pub fn deposit_done_token(ctx: Context<DepositDoneCtx>, amount: u64) -> Result<()> {
        deposit_done_token::process(ctx, amount)
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSolCtx>, amount_sol: u64) -> Result<()> {
        withdraw_sol::process(ctx, amount_sol)
    }

    pub fn withdraw_done_token(ctx: Context<WithdrawDoneTokenCtx>, amount_done: u64) -> Result<()> {
        withdraw_done_token::process(ctx, amount_done)
    }

    pub fn set_percent_pay_w_sol(ctx: Context<UpdateMasterCtx>, percent_pay_w_sol: u16) -> Result<()> {
        update_master::set_percent_pay_w_sol(ctx, percent_pay_w_sol)
    }

    pub fn set_percent_pay_w_done_token(ctx: Context<UpdateMasterCtx>, percent_pay_w_done_token: u16) -> Result<()> {
        update_master::set_percent_pay_w_done_token(ctx, percent_pay_w_done_token)
    }

    pub fn set_owner(ctx: Context<UpdateMasterCtx>, new_owner: Pubkey) -> Result<()> {
        update_master::set_owner(ctx, new_owner)
    }

    // USER FUNCTIONS   
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

    pub fn init_tx_sol_volume(ctx: Context<InitTransactionSolVolumeCtx>) -> Result<()> {
        init_tx_sol_volume::process(ctx)
    }

    pub fn init_tx_done_token_volume(
        ctx: Context<InitTransactionDoneTokenVolumeCtx>,
    ) -> Result<()> {
        init_tx_done_token_volume::process(ctx)
    }

    pub fn init_sender_ata(ctx: Context<InitSenderAtaCtx>) -> Result<()> {
        init_sender_ata::process(ctx)
    }

}