use anchor_lang::prelude::*;

#[event]
pub struct CreatePaymentEvent {
    pub item_id: u64,
    pub amount: u64,
}
