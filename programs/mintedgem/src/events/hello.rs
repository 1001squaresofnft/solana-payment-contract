use anchor_lang::prelude::*;

#[event]
pub struct Hello {
    pub msg: String,
}
