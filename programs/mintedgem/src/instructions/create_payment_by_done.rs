use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{
    constants::{
        ITEM_PAYMENT, MASTER, TOKEN_ACCOUNT_OWNER, TRANSACTION_DONE_TOKEN_VOLUME, VAULT_TOKEN,
    },
    errors::CustomErrors,
    events::CreatePaymentByDoneEvent,
    state::{ItemPayment, Master, TransactionDoneTokenVolume},
};

#[derive(Accounts)]
#[instruction(item_id: u64)]
pub struct CreatePaymentByDoneCtx<'info> {
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
        seeds = [TRANSACTION_DONE_TOKEN_VOLUME, signer.key().as_ref()],
        bump,
    )]
    transaction_done_token_volume: Account<'info, TransactionDoneTokenVolume>,

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

pub fn process(ctx: Context<CreatePaymentByDoneCtx>, item_id: u64, amount_done: u64) -> Result<()> {
    let master = &ctx.accounts.master;
    let item_payment = &mut ctx.accounts.item_payment;
    let token_account_owner_pda = &ctx.accounts.token_account_owner_pda;
    let vault_token = &mut ctx.accounts.vault_token;
    let sender_token_account = &mut ctx.accounts.sender_token_account;
    let transaction_done_token_volume = &mut ctx.accounts.transaction_done_token_volume;
    let token_program = &ctx.accounts.token_program;
    let signer = &ctx.accounts.signer;

    // 1. Check the amount of DONE token user wants to send valid or not
    // mus be greater than 0
    if amount_done == 0 {
        return Err(CustomErrors::InvalidAmount.into());
    }
    // must be less than or equal to the amount user has
    if amount_done > sender_token_account.amount {
        return Err(CustomErrors::UserInsufficientAmount.into());
    }

    // ===== 2. Make transaction
    // 2.1 Transfer DONE token in
    let transfer_instruction = Transfer {
        from: sender_token_account.to_account_info(),
        to: vault_token.to_account_info(),
        authority: signer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
    );

    let result = transfer(cpi_ctx, amount_done);

    if result.is_err() {
        return Err(CustomErrors::TransferFailed.into());
    }
    // 2.2 send back DONE token to the sender (if percnet_pay_w_done_token > 0)
    // Calculate the amount of DONE token user will get back
    if master.percent_pay_w_done_token > 0 {
        let amount_done_token_out =
            (amount_done * u64::from(master.percent_pay_w_done_token)) / 10000;
        // Send back
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
    }

    // ===== 3. Update states
    // check if the item payment is already created
    if item_payment.creator != Pubkey::default() {
        require_keys_eq!(
            item_payment.creator,
            signer.key(),
            CustomErrors::InvalidCreator
        );
    }
    // create item payment
    item_payment.creator = signer.key();
    item_payment.amount_done = amount_done;
    // update transaction done token volume
    transaction_done_token_volume.creator = signer.key();
    transaction_done_token_volume.amount += amount_done;

    emit!(CreatePaymentByDoneEvent {
        signer: signer.key(),
        item_id,
        amount_done,
    });

    Ok(())
}
