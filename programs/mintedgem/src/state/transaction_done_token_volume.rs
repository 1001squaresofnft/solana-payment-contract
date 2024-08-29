use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TransactionDoneTokenVolume {
    pub creator: Pubkey,
    pub amount: u64,
}
