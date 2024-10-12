use crate::{
    constants::{ITEM_PAYMENT, MASTER, TRANSACTION_SOL_VOLUME, VAULT_SOL},
    errors::CustomErrors,
    events::CreatePaymentEvent,
    state::{ItemPayment, Master, TransactionSolVolume, VaultSol},
};
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::{
    token::Token,
    token_interface::{Mint, TokenAccount},
};
use raydium_cp_swap::{
    cpi::{accounts::Swap, swap_base_input},
    program::RaydiumCpSwap,
    states::{AmmConfig, ObservationState, PoolState},
};

#[derive(Accounts)]
#[instruction(item_id: u64)]
pub struct CreatePaymentContext<'info> {
    #[account(
        mut,
        seeds = [MASTER],
        bump,
    )]
    master: Account<'info, Master>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [ITEM_PAYMENT, item_id.to_le_bytes().as_ref()],
        bump,
        space = 8 + ItemPayment::INIT_SPACE,
    )]
    item_payment: Account<'info, ItemPayment>,

    #[account(
        mut,
        seeds = [TRANSACTION_SOL_VOLUME, signer.key().as_ref()],
        bump,
    )]
    transaction_sol_volume: Box<Account<'info, TransactionSolVolume>>,

    #[account(
        mut,
        seeds = [VAULT_SOL],
        bump
    )]
    vault_sol: Box<Account<'info, VaultSol>>,

    wsol_mint: InterfaceAccount<'info, Mint>,
    done_token_mint: InterfaceAccount<'info, Mint>,

    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(
        mut,
        associated_token::mint = wsol_mint,
        associated_token::authority = signer
    )]
    sender_wsol_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = done_token_mint,
        associated_token::authority = signer,
    )]
    sender_done_token_ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: pool vault and lp mint authority
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,

    /// The factory state to read protocol fees
    #[account(address = pool_state.load()?.amm_config)]
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// The vault token account for input token
    #[account(
        mut,
        constraint = input_vault.key() == pool_state.load()?.token_0_vault || input_vault.key() == pool_state.load()?.token_1_vault
    )]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The vault token account for output token
    #[account(
        mut,
        constraint = output_vault.key() == pool_state.load()?.token_0_vault || output_vault.key() == pool_state.load()?.token_1_vault
    )]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    /// The program account for the most recent oracle observation
    #[account(mut, address = pool_state.load()?.observation_key)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    pub cp_swap_program: Program<'info, RaydiumCpSwap>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,
}

pub fn process(ctx: Context<CreatePaymentContext>, item_id: u64, amount_sol: u64) -> Result<()> {
    let master = &ctx.accounts.master;
    let item_payment = &mut ctx.accounts.item_payment;
    let transaction_sol_volume = &mut ctx.accounts.transaction_sol_volume;
    let vault_sol = &ctx.accounts.vault_sol;

    let wsol_mint = &ctx.accounts.wsol_mint;
    let done_token_mint = &ctx.accounts.done_token_mint;
    let pool_state = &ctx.accounts.pool_state;
    let sender_wsol_ata = &ctx.accounts.sender_wsol_ata;
    let sender_done_token_ata = &ctx.accounts.sender_done_token_ata;
    let authority = &ctx.accounts.authority;
    let amm_config = &ctx.accounts.amm_config;
    let input_vault = &ctx.accounts.input_vault;
    let output_vault = &ctx.accounts.output_vault;
    let observation_state = &ctx.accounts.observation_state;
    let cp_swap_program = &ctx.accounts.cp_swap_program;

    let signer = &ctx.accounts.signer;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;

    // ====== 1. Check the amount_sol (in lamports)
    // mus be greater than 0
    if amount_sol == 0 {
        return Err(CustomErrors::InvalidAmount.into());
    }

    let amount_sol_swap;
    let amount_sol_to_treasury;

    if master.percent_pay_w_sol > 0 {
        amount_sol_swap = (amount_sol * u64::from(master.percent_pay_w_sol)) / 10000;
        amount_sol_to_treasury = amount_sol - amount_sol_swap;
    } else {
        amount_sol_swap = 0;
        amount_sol_to_treasury = amount_sol;
    }

    // ===== 2. Make transaction
    // 2.1 Transfer Sol to treasury
    let cpi_ctx = CpiContext::new(
        system_program.to_account_info(),
        system_program::Transfer {
            from: signer.to_account_info(),
            to: vault_sol.to_account_info(),
        },
    );
    let result = system_program::transfer(cpi_ctx, amount_sol_to_treasury);
    if result.is_err() {
        return Err(CustomErrors::TransferFailed.into());
    }
    //
    // 2.2 Swap SOL to DONE token -> Send back DONE token to the sender (if percent_pay_w_sol > 0)
    if amount_sol_swap > 0 {
        let cpi_ctx = CpiContext::new(
            cp_swap_program.to_account_info(),
            Swap {
                payer: signer.to_account_info(),
                authority: authority.to_account_info(),
                amm_config: amm_config.to_account_info(),
                pool_state: pool_state.to_account_info(),
                input_token_account: sender_wsol_ata.to_account_info(),
                output_token_account: sender_done_token_ata.to_account_info(),
                input_vault: input_vault.to_account_info(),
                output_vault: output_vault.to_account_info(),
                input_token_program: token_program.to_account_info(),
                output_token_program: token_program.to_account_info(),
                input_token_mint: wsol_mint.to_account_info(),
                output_token_mint: done_token_mint.to_account_info(),
                observation_state: observation_state.to_account_info(),
            },
        );
        let result = swap_base_input(cpi_ctx, amount_sol_swap, 0);
        if result.is_err() {
            return Err(CustomErrors::CPISwapFailed.into());
        }
        ctx.accounts.sender_wsol_ata.reload()?;
    }

    // ===== 3. Update states
    // check if item payment already created
    if item_payment.creator != Pubkey::default() {
        require_keys_eq!(
            item_payment.creator,
            signer.key(),
            CustomErrors::InvalidCreator
        );
    }
    // create item payment
    item_payment.creator = signer.key();
    item_payment.amount = amount_sol;
    // update transaction sol volume
    transaction_sol_volume.creator = signer.key();
    transaction_sol_volume.amount += amount_sol;

    emit!(CreatePaymentEvent {
        signer: signer.key(),
        item_id,
        amount: amount_sol,
    });

    Ok(())
}
