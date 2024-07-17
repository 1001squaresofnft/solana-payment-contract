use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_lang::system_program;


declare_id!("CdMkddBBv9zdB33UErvhzqH4XcZ3ZSxrBxY72Qo1vyUV");

#[program]
pub mod mintedgem {
    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>, percent: u64) -> Result<()> {
        let master = &mut ctx.accounts.master;

        if master.is_initialized {
            return err!(Errors::MasterAccountAlreadyInitialized);
        }

        master.is_initialized = true;
        master.owner = ctx.accounts.signer.key();
        master.percent = percent;

        emit!(OwnerInitialized {});

        Ok(())
    }

    pub fn init_vault_sol(ctx: Context<InitVaultSolCtx>) -> Result<()> {
        
        require_keys_eq!(ctx.accounts.master.owner, ctx.accounts.signer.key(), Errors::NotOwner);

        if ctx.accounts.master.is_vault_sol_initialized {
            return Err(Errors::VaultSolAlreadyInitialized.into());
        }

        ctx.accounts.master.is_vault_sol_initialized = true;

        emit!(VaultSolInitialized {});

        Ok(())
    }

    pub fn init_vault_done_token(ctx: Context<InitVaultDoneTokenCtx>) -> Result<()> {

        require_keys_eq!(ctx.accounts.master.owner, ctx.accounts.signer.key(), Errors::NotOwner);

        if ctx.accounts.master.is_vault_done_token_initialized {
            return Err(Errors::VaultDoneTokenAlreadyInitialized.into());
        }

        ctx.accounts.master.is_vault_done_token_initialized = true;

        emit!(VaultDoneTokenInitialized {});

        Ok(())
    }

    pub fn transfer_sol_in(ctx: Context<TransferSolCtx>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info().clone(),
                to: ctx.accounts.vault_sol.to_account_info().clone(),
            },
        );
        system_program::transfer(cpi_context, amount)?;

        emit!(DepositSolEvent {
            depositor: ctx.accounts.signer.key(),
            amount: amount,
        });

        Ok(())
    }

    pub fn transfer_done_token_in(ctx: Context<TransferTokenCtx>, amount: u64) -> Result<()> {
        let transfer_instruction = token::Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.vault_token.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
    
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );
    
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        emit!(DepositDoneTokenEvent {
            depositor: ctx.accounts.signer.key(),
            amount_done: amount,
        });

        Ok(())
    }

    pub fn create_payment(ctx: Context<CreatePaymentContext>, item_id: u64, amount_sol: u64) -> Result<()> {
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

    pub fn create_payment_by_done(ctx: Context<CreatePaymentByDoneContext>, item_id: u64, amount_done: u64) -> Result<()> {
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

    pub fn withdraw_sol(ctx: Context<WithdrawSolContext>, amount_sol: u64) -> Result<()> {
        let vault_sol = &ctx.accounts.vault_sol;

        require_keys_eq!(ctx.accounts.master.owner, ctx.accounts.signer.key(), Errors::NotOwner);

        if vault_sol.to_account_info().lamports() < amount_sol {
            return Err(Errors::DeoDuSoDu.into());
        }

        ctx.accounts.vault_sol.sub_lamports(amount_sol)?;
        ctx.accounts.signer.add_lamports(amount_sol)?;

        emit!(WithdrawSolEvent {
            to: ctx.accounts.signer.key(),
            amount: amount_sol,
        });
    
        Ok(())
    }

    pub fn withdraw_done_token(ctx: Context<WithdrawDoneTokenContext>, amount_done: u64) -> Result<()> {
        require_keys_eq!(ctx.accounts.master.owner, ctx.accounts.signer.key(), Errors::NotOwner);

        if ctx.accounts.vault_token.amount < amount_done {
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

        anchor_spl::token::transfer(cpi_ctx, amount_done)?;

        emit!(WithdrawDoneTokenEvent {
            to: ctx.accounts.signer.key(),
            amount_done: amount_done,
        });

        Ok(())
    }

    pub fn set_percent(ctx: Context<SetPercentCtx>, percent: u64) -> Result<()> {
        require_keys_eq!(ctx.accounts.master.owner, ctx.accounts.signer.key(), Errors::NotOwner);

        ctx.accounts.master.percent = percent;

        emit!(SetPercent {
            percent: percent,
        });

        Ok(())
    }
    
    pub fn hello(ctx: Context<HelloCtx>) -> Result<()> {
        emit!(Hello {
            msg: String::from("Hello, Mintedgem!"),
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct HelloCtx {}

// CONTEXT
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(
        init, 
        payer = signer,
        seeds = [b"master"],
        bump,
        space = 8 + std::mem::size_of::<Master>(),
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitVaultSolCtx<'info> {
    #[account(
        mut, 
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    #[account(
        init, 
        payer = signer,
        seeds = [b"vault_sol"],
        bump,
        space = 8 + std::mem::size_of::<VaultSol>(),
    )]
    vault_sol: Account<'info, VaultSol>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitVaultDoneTokenCtx<'info> {
    #[account(
        mut, 
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    /// CHECK
    #[account(
        init, 
        payer = signer, 
        seeds = [b"token_account_owner"],
        bump,
        space = 8
    )]
    token_account_owner_pda: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"vault_token", mint_of_token_being_sent.key().as_ref()],
        token::mint = mint_of_token_being_sent,
        token::authority = token_account_owner_pda,
        bump,
    )]
    vault_token: Account<'info, token::TokenAccount>,
    mint_of_token_being_sent: Account<'info, token::Mint>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct TransferTokenCtx<'info> {
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
}

#[derive(Accounts)]
pub struct TransferSolCtx<'info> {
    #[account(
        mut, 
        seeds = [b"master"],
        bump,
    )]
    master: Account<'info, Master>,

    #[account(
        mut,
        seeds = [b"vault_sol"],
        bump
    )]
    vault_sol: Account<'info, VaultSol>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

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

#[derive(Accounts)]
#[instruction(item_id: u64)]
pub struct CreatePaymentByDoneContext<'info> {
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

#[derive(Accounts)]
pub struct WithdrawSolContext<'info> {
    #[account(
        mut,
        seeds = [b"master"],
        bump
    )]
    master: Account<'info, Master>,

    #[account(
        mut,
        seeds = [b"vault_sol"],
        bump
    )]
    vault_sol: Account<'info, VaultSol>,
    
    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawDoneTokenContext<'info> {
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
    token_program: Program<'info, token::Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SetPercentCtx<'info> {
    #[account(
        mut,
        seeds = [b"master"],
        bump
    )]
    master: Account<'info, Master>,

    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

// ACCOUNTS
#[account]
pub struct Master {
    pub is_initialized: bool,
    pub is_vault_sol_initialized: bool,
    pub is_vault_done_token_initialized: bool,
    pub owner: Pubkey,
    pub percent: u64,
}

#[account]
pub struct VaultSol {}

#[account]
pub struct ItemPayment {
    pub creator: Pubkey,
    pub amount: u64,
    pub amount_done: u64,
}

#[account]
pub struct TransctionSolVolume {
    pub creator: Pubkey,
    pub amount: u64,
}

#[account]
pub struct TransactionDoneTokenVolume {
    pub creator: Pubkey,
    pub amount: u64,
}

// ERRORS
#[error_code]
pub enum Errors {
    #[msg("minted-gem: Transfer failed")]
    TransferFailed,
    #[msg("minted-gem: You are not authorized to perform this action")]
    Unauthorized,
    #[msg("minted-gem: The master account is already initialized")]
    MasterAccountAlreadyInitialized,
    #[msg("minted-gem: Deo du so du")]
    DeoDuSoDu,
    #[msg("Only owner can call this function!")]
    NotOwner,
    #[msg("Vault SOL is already initialized")]
    VaultSolAlreadyInitialized,
    #[msg("Vault DONE token is already initialized")]
    VaultDoneTokenAlreadyInitialized,
}

// EVENTS
#[event]
pub struct CreatePaymentEvent {
    pub item_id: u64,
    pub amount: u64,
}

#[event]
pub struct CreatePaymentByDoneEvent {
    pub item_id: u64,
    pub amount_done: u64,
}

#[event]
pub struct DepositSolEvent {
    pub depositor: Pubkey,
    pub amount: u64,
}

#[event]
pub struct DepositDoneTokenEvent {
    pub depositor: Pubkey,
    pub amount_done: u64,
}

#[event]
pub struct WithdrawSolEvent {
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WithdrawDoneTokenEvent {
    pub to: Pubkey,
    pub amount_done: u64,
}

#[event]
pub struct SetPercent {
    pub percent: u64,
}

#[event]
pub struct VaultSolInitialized {}

#[event]
pub struct VaultDoneTokenInitialized {}

#[event] 
pub struct OwnerInitialized {}

#[event]
pub struct Hello {
    pub msg: String,
}