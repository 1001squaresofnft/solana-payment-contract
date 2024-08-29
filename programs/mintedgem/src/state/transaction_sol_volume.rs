use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TransctionSolVolume {
    pub creator: Pubkey,
    pub amount: u64,
}