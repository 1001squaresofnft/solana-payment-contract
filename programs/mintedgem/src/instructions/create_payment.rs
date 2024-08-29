use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token;

use crate::{
    errors::Errors,
    events::CreatePaymentEvent,
    state::{Master, ItemPayment, TransctionSolVolume, VaultSol},
};

#[derive(Accounts)]
#[instruction(item_id: u64)]
pub struct CreatePaymentContext<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"item_payment", item_id.to_le_bytes().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<ItemPayment>(),
    )]
    item_payment: Account<'info, ItemPayment>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"transaction_sol_volume", signer.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<TransctionSolVolume>(),
    )]
    transaction_sol_volume: Account<'info, TransctionSolVolume>,

    #[account(
        mut, 
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

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

    #[account(
        mut,
        seeds = [b"vault_sol"],
        bump
    )]
    vault_sol: Account<'info, VaultSol>,
}

pub fn process(ctx: Context<CreatePaymentContext>, item_id: u64, amount_sol: u64) -> Result<()> {
    let master = &ctx.accounts.master;
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
    system_program::transfer(cpi_context, amount_sol)?;

    // create item payment
    item_payment.amount = amount_sol;
    item_payment.creator = ctx.accounts.signer.key();
    item_payment.amount_done = 0;
    // update transaction sol volume
    transaction_sol_volume.amount += amount_sol;
    transaction_sol_volume.creator = ctx.accounts.signer.key();

    // check balance & transfer done token out
    let amount_done_token_out = (amount_sol * master.percent * 100) / 10000;

    if ctx.accounts.vault_token.amount < amount_done_token_out {
        return Err(Errors::DeoDuSoDu.into());
    }

    let transfer_instruction = token::Transfer {
        from: ctx.accounts.vault_token.to_account_info(),
        to: ctx.accounts.sender_token_account.to_account_info(),
        authority: ctx.accounts.token_account_owner_pda.to_account_info(),
    };

    let bump = ctx.bumps.token_account_owner_pda;
    let seeds = &[b"token_account_owner".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        signer,
    );

    anchor_spl::token::transfer(cpi_ctx, amount_done_token_out)?;

    emit!(CreatePaymentEvent {
        item_id: item_id,
        amount: amount_sol,
    });

    Ok(())
}