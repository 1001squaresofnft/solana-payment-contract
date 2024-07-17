use anchor_lang::prelude::*;

#[account]
pub struct TransactionDoneTokenVolume {
    pub creator: Pubkey,
    pub amount: u64,
}
