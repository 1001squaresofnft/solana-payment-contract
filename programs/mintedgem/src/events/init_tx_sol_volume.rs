use anchor_lang::prelude::*;

#[event]
pub struct InitTxSolVolume {
    pub signer: Pubkey,
}
