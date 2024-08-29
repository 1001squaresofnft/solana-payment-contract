use anchor_lang::prelude::*;

#[event]
pub struct CreatePaymentByDoneEvent {
    pub signer: Pubkey,
    pub item_id: u64,
    pub amount_done: u64,
}
