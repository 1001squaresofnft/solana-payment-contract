use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Master {
    pub is_initialized: bool,
    pub is_vault_sol_initialized: bool,
    pub is_vault_done_token_initialized: bool,
    pub owner: Pubkey,
    pub percent: u64, // 10_000 = 100%, 1_000 = 10%, 100 = 1%, 10 = 0.1%, 1 = 0.01%
}
