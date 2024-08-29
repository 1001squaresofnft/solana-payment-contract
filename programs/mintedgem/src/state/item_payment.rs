use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ItemPayment {
    pub creator: Pubkey,
    pub amount: u64,
    pub amount_done: u64,
}