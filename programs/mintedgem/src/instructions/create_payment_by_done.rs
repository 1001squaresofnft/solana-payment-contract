use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{
    constants::{
        ITEM_PAYMENT_BY_DONE, TOKEN_ACCOUNT_OWNER, TRANSACTION_DONE_TOKEN_VOLUME, VAULT_TOKEN,
    },
    errors::CustomErrors,
    events::CreatePaymentByDoneEvent,
    state::{ItemPayment, TransactionDoneTokenVolume},
};

#[derive(Accounts)]
#[instruction(item_id: u64)]
pub struct CreatePaymentByDoneCtx<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [ITEM_PAYMENT_BY_DONE, item_id.to_le_bytes().as_ref()],
        bump,
        space = 8 + ItemPayment::INIT_SPACE,
    )]
    item_payment: Account<'info, ItemPayment>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [TRANSACTION_DONE_TOKEN_VOLUME, signer.key().as_ref()],
        bump,
        space = 8 + TransactionDoneTokenVolume::INIT_SPACE,
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

pub fn process(
    ctx: Context<CreatePaymentByDoneCtx>,
    item_id: u64,
    _amount_done: u64,
) -> Result<()> {
    let item_payment = &mut ctx.accounts.item_payment;
    let transaction_done_token_volume = &mut ctx.accounts.transaction_done_token_volume;

    // check balance & transfer DONE token IN
    if _amount_done == 0 {
        return Err(CustomErrors::InvalidAmount.into());
    }

    if ctx.accounts.sender_token_account.amount < _amount_done {
        return Err(CustomErrors::InsufficientAmount.into());
    }
    // transfer DONE token in
    let transfer_instruction = Transfer {
        from: ctx.accounts.sender_token_account.to_account_info(),
        to: ctx.accounts.vault_token.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
    );

    let result = transfer(cpi_ctx, _amount_done);

    if result.is_err() {
        return Err(CustomErrors::TransferFailed.into());
    }

    if item_payment.creator != Pubkey::default() {
        require_keys_eq!(
            item_payment.creator,
            ctx.accounts.signer.key(),
            CustomErrors::InvalidCreator
        );
    }

    // create item payment
    item_payment.creator = ctx.accounts.signer.key();
    item_payment.amount_done = _amount_done;
    // update transaction done token volume
    transaction_done_token_volume.creator = ctx.accounts.signer.key();
    transaction_done_token_volume.amount += _amount_done;

    emit!(CreatePaymentByDoneEvent {
        signer: ctx.accounts.signer.key(),
        item_id,
        amount_done: _amount_done,
    });

    Ok(())
}
