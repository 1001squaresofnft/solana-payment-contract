use anchor_lang::prelude::*;

#[event]
pub struct SetPercent {
    pub percent: u16,
}
