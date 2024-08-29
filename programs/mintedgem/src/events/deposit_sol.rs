use anchor_lang::prelude::*;

#[event]
pub struct DepositSolEvent {
    pub depositor: Pubkey,
    pub amount: u64,
}
