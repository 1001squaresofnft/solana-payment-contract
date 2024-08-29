use anchor_lang::prelude::*;

#[event]
pub struct WithdrawDoneTokenEvent {
    pub to: Pubkey,
    pub amount_done: u64,
}
