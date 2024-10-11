use anchor_lang::prelude::*;

#[event]
pub struct SetOwner {
    pub new_owner: Pubkey,
}