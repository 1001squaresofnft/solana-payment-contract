use anchor_lang::prelude::*;

#[event]
pub struct SetPercentPayWSol {
    pub percent_pay_w_sol: u16,
}

#[event]
pub struct SetPercentPayWDoneToken {
    pub percent_pay_w_done_token: u16,
}