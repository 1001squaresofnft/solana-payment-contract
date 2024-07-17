use anchor_lang::prelude::*;
// EVENTS
#[event]
pub struct CreatePaymentEvent {
    pub item_id: u64,
    pub amount: u64,
}

#[event]
pub struct CreatePaymentByDoneEvent {
    pub item_id: u64,
    pub amount_done: u64,
}

#[event]
pub struct DepositSolEvent {
    pub depositor: Pubkey,
    pub amount: u64,
}

#[event]
pub struct DepositDoneTokenEvent {
    pub depositor: Pubkey,
    pub amount_done: u64,
}

#[event]
pub struct WithdrawSolEvent {
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WithdrawDoneTokenEvent {
    pub to: Pubkey,
    pub amount_done: u64,
}

#[event]
pub struct SetPercent {
    pub percent: u64,
}

#[event]
pub struct VaultSolInitialized {}

#[event]
pub struct VaultDoneTokenInitialized {}

#[event]
pub struct OwnerInitialized {}

#[event]
pub struct Hello {
    pub msg: String,
}
