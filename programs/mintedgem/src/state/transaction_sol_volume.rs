use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TransactionSolVolume {
    pub creator: Pubkey,
    pub amount: u64,
}
