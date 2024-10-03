use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{
    constants::{
        ITEM_PAYMENT, MASTER, TOKEN_ACCOUNT_OWNER, TRANSACTION_SOL_VOLUME, VAULT_SOL, VAULT_TOKEN,
    },
    errors::CustomErrors,
    events::CreatePaymentEvent,
    state::{ItemPayment, Master, TransactionSolVolume, VaultSol},
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
        init_if_needed,
        payer = signer,
        seeds = [TRANSACTION_SOL_VOLUME, signer.key().as_ref()],
        bump,
        space = 8 + TransactionSolVolume::INIT_SPACE,
    )]
    transaction_sol_volume: Account<'info, TransactionSolVolume>,

    #[account(
        mut,
        seeds = [VAULT_SOL],
        bump
    )]
    vault_sol: Account<'info, VaultSol>,

    mint_of_token_being_sent: Account<'info, Mint>,
    /// CHECK
    #[account(
        mut,
        seeds=[TOKEN_ACCOUNT_OWNER],
        bump
    )]
    token_account_owner_pda: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [VAULT_TOKEN, mint_of_token_being_sent.key().as_ref()],
        bump,
        token::mint = mint_of_token_being_sent,
        token::authority = token_account_owner_pda,
    )]
    vault_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_of_token_being_sent,
        associated_token::authority = signer,
    )]
    sender_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<CreatePaymentContext>, item_id: u64, amount_sol: u64) -> Result<()> {
    let master = &ctx.accounts.master;
    let mint_of_token_being_sent = &ctx.accounts.mint_of_token_being_sent;
    let vault_sol = &mut ctx.accounts.vault_sol;
    let vault_token = &mut ctx.accounts.vault_token;
    let sender_token_account = &mut ctx.accounts.sender_token_account;
    let token_account_owner_pda = &ctx.accounts.token_account_owner_pda;
    let item_payment = &mut ctx.accounts.item_payment;
    let transaction_sol_volume = &mut ctx.accounts.transaction_sol_volume;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let signer = &ctx.accounts.signer;

    // 1. Check the amount_sol (in lamports)
    // mus be greater than 0
    if amount_sol == 0 {
        return Err(CustomErrors::InvalidAmount.into());
    }
    // must be less than or equal to the amount user has
    if amount_sol > signer.lamports() {
        return Err(CustomErrors::UserInsufficientAmount.into());
    }

    // ===== 2. Make transaction
    // 2.1 Transfer Sol in
    let cpi_ctx = CpiContext::new(
        system_program.to_account_info(),
        system_program::Transfer {
            from: signer.to_account_info(),
            to: vault_sol.to_account_info(),
        },
    );
    let result = system_program::transfer(cpi_ctx, amount_sol);

    if result.is_err() {
        return Err(CustomErrors::TransferFailed.into());
    }
    // 2.2 send back DONE token to the sender
    // Calculate the amount of DONE token user will get back
    let amount_done_token_out = (amount_sol * u64::from(master.percent_pay_w_sol)) / 10000;
    let amount_done_token_out = amount_done_token_out
        / (10u64.pow(9) / 10u64.pow(mint_of_token_being_sent.decimals.into()));
    // Send back
    if vault_token.amount < amount_done_token_out {
        return Err(CustomErrors::InsufficientAmount.into());
    }

    let transfer_instruction = Transfer {
        from: vault_token.to_account_info(),
        to: sender_token_account.to_account_info(),
        authority: token_account_owner_pda.to_account_info(),
    };

    let bump = ctx.bumps.token_account_owner_pda;
    let seeds = &[TOKEN_ACCOUNT_OWNER, &[bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        transfer_instruction,
        signer_seeds,
    );

    let result = transfer(cpi_ctx, amount_done_token_out);

    if result.is_err() {
        return Err(CustomErrors::TransferBackFailed.into());
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
