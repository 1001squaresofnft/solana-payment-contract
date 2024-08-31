use anchor_lang::prelude::*;

#[event]
pub struct InitSenderAta {
    pub signer: Pubkey,
}
