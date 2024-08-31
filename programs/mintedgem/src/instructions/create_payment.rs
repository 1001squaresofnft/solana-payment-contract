use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{
    Token, Mint,  TokenAccount, Transfer, transfer 
};

use crate::{
    constants::{ITEM_PAYMENT, MASTER, TOKEN_ACCOUNT_OWNER, TRANSACTION_SOL_VOLUME, VAULT_SOL, VAULT_TOKEN},
    errors::CustomErrors,
    events::CreatePaymentEvent,
    state::{Master, ItemPayment, TransctionSolVolume, VaultSol},
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
        init,
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
    transaction_sol_volume: Account<'info, TransctionSolVolume>,

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

pub fn process(ctx: Context<CreatePaymentContext>, item_id: u64, _amount_sol: u64) -> Result<()> {
    let master = &ctx.accounts.master;
    let mint_of_token_being_sent = &ctx.accounts.mint_of_token_being_sent;
    let item_payment = &mut ctx.accounts.item_payment;
    let transaction_sol_volume = &mut ctx.accounts.transaction_sol_volume;

    // transfer sol in
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.signer.to_account_info().clone(),
            to: ctx.accounts.vault_sol.to_account_info().clone(),
        },
    );
    system_program::transfer(cpi_context, _amount_sol)?;

    // create item payment
    item_payment.creator = ctx.accounts.signer.key();
    item_payment.amount = _amount_sol;
    item_payment.amount_done = 0;
    // update transaction sol volume
    transaction_sol_volume.creator = ctx.accounts.signer.key();
    transaction_sol_volume.amount += _amount_sol;

    // check balance & transfer done token out
    let amount_done_token_out = (_amount_sol * master.percent * 100) / 10000;
    let amount_done_token_out = amount_done_token_out / (10u64.pow(9 as u32) / 10u64.pow(mint_of_token_being_sent.decimals as u32));

    if ctx.accounts.vault_token.amount < amount_done_token_out {
        return Err(CustomErrors::InsufficientAmount.into());
    }

    let transfer_instruction = Transfer {
        from: ctx.accounts.vault_token.to_account_info(),
        to: ctx.accounts.sender_token_account.to_account_info(),
        authority: ctx.accounts.token_account_owner_pda.to_account_info(),
    };

    let bump = ctx.bumps.token_account_owner_pda;
    let seeds = &[TOKEN_ACCOUNT_OWNER.as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        signer,
    );

    transfer(cpi_ctx, amount_done_token_out)?;

    emit!(CreatePaymentEvent {
        signer: ctx.accounts.signer.key(),
        item_id: item_id,
        amount: _amount_sol,
    });

    Ok(())
}