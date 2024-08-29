use anchor_lang::prelude::*;

#[event]
pub struct WithdrawSolEvent {
    pub to: Pubkey,
    pub amount: u64,
}
