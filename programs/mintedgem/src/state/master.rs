use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Master {
    pub is_initialized: bool,
    pub is_vault_sol_initialized: bool,
    pub is_vault_done_token_initialized: bool,
    pub owner: Pubkey,
    pub percent_pay_w_sol: u16, // 10_000 = 100%, 1_000 = 10%, 100 = 1%, 10 = 0.1%, 1 = 0.01%
    pub percent_pay_w_done_token: u16, // 10_000 = 100%, 1_000 = 10%, 100 = 1%, 10 = 0.1%, 1 = 0.01%
}
