use anchor_lang::prelude::*;

// ERRORS
#[error_code]
pub enum Errors {
    #[msg("minted-gem: Transfer failed")]
    TransferFailed,
    #[msg("minted-gem: You are not authorized to perform this action")]
    Unauthorized,
    #[msg("minted-gem: The master account is already initialized")]
    MasterAccountAlreadyInitialized,
    #[msg("minted-gem: Deo du so du")]
    DeoDuSoDu,
    #[msg("Only owner can call this function!")]
    NotOwner,
    #[msg("Vault SOL is already initialized")]
    VaultSolAlreadyInitialized,
    #[msg("Vault DONE token is already initialized")]
    VaultDoneTokenAlreadyInitialized,
}