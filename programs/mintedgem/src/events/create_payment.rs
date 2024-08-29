use anchor_lang::prelude::*;

#[event]
pub struct CreatePaymentEvent {
    pub signer: Pubkey,
    pub item_id: u64,
    pub amount: u64,
}
