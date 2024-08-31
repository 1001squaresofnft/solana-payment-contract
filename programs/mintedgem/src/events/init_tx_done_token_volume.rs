use anchor_lang::prelude::*;

#[event]
pub struct InitTxDoneTokenVolume {
    pub signer: Pubkey,
}
