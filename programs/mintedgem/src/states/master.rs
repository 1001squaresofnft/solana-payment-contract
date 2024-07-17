use anchor_lang::prelude::*;

#[account]
pub struct Master {
    pub is_initialized: bool,
    pub is_vault_sol_initialized: bool,
    pub is_vault_done_token_initialized: bool,
    pub owner: Pubkey,
    pub percent: u64,
}