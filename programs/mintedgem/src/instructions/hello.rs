use anchor_lang::prelude::*;

use crate::events::Hello;

#[derive(Accounts)]
pub struct HelloCtx {}

pub fn hello(_ctx: Context<HelloCtx>) -> Result<()> {
    msg!("Hello, Mintedgem!");
    emit!(Hello {
        msg: String::from("Hello, Mintedgem!"),
    });
    Ok(())
}
