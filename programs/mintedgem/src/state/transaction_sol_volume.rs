use anchor_lang::prelude::*;

#[account]
pub struct TransctionSolVolume {
    pub creator: Pubkey,
    pub amount: u64,
}