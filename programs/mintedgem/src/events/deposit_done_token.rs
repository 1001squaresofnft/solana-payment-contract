use anchor_lang::prelude::*;

#[event]
pub struct DepositDoneTokenEvent {
    pub depositor: Pubkey,
    pub amount_done: u64,
}
