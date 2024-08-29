use anchor_lang::prelude::*;

#[event]
pub struct CreatePaymentByDoneEvent {
    pub item_id: u64,
    pub amount_done: u64,
}
