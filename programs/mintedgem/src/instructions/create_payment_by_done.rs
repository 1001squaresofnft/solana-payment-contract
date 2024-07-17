use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::errors::Errors;
use crate::states::{
    item_payment::ItemPayment, transaction_done_token_volume::TransactionDoneTokenVolume,
};
use crate::CreatePaymentByDoneEvent;

#[derive(Accounts)]
#[instruction(item_id: u64)]
pub struct CreatePaymentByDoneCtx<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"item_payment_by_done", item_id.to_le_bytes().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<ItemPayment>(),
    )]
    item_payment: Account<'info, ItemPayment>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"transaction_done_token_volume", signer.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<TransactionDoneTokenVolume>(),
    )]
    transaction_done_token_volume: Account<'info, TransactionDoneTokenVolume>,

    /// CHECK
    #[account(mut,
        seeds=[b"token_account_owner"],
        bump
    )]
    token_account_owner_pda: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"vault_token", mint_of_token_being_sent.key().as_ref()],
        bump
    )]
    vault_token: Account<'info, token::TokenAccount>,
    #[account(mut)]
    sender_token_account: Account<'info, token::TokenAccount>,
    mint_of_token_being_sent: Account<'info, token::Mint>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
    rent: Sysvar<'info, Rent>,
}

pub fn process(ctx: Context<CreatePaymentByDoneCtx>, item_id: u64, amount_done: u64) -> Result<()> {
    // let master = &ctx.accounts.master;
    let item_payment = &mut ctx.accounts.item_payment;
    let transaction_done_token_volume = &mut ctx.accounts.transaction_done_token_volume;

    // check balance & transfer DONE token IN
    if ctx.accounts.sender_token_account.amount < amount_done {
        return Err(Errors::DeoDuSoDu.into());
    }
    // transfer DONE token in
    let transfer_instruction = token::Transfer {
        from: ctx.accounts.sender_token_account.to_account_info(),
        to: ctx.accounts.vault_token.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
    );

    anchor_spl::token::transfer(cpi_ctx, amount_done)?;

    // create item payment
    item_payment.amount = 0;
    item_payment.creator = ctx.accounts.signer.key();
    item_payment.amount_done = amount_done;
    // update transaction done token volume
    transaction_done_token_volume.amount += amount_done;
    transaction_done_token_volume.creator = ctx.accounts.signer.key();

    emit!(CreatePaymentByDoneEvent {
        item_id: item_id,
        amount_done: amount_done,
    });

    Ok(())
}
