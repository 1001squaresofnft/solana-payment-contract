use anchor_lang::prelude::*;

#[account]
pub struct ItemPayment {
    pub creator: Pubkey,
    pub amount: u64,
    pub amount_done: u64,
}