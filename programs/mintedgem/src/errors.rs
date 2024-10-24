use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("Mintedgem: The master account is already initialized")]
    MasterAccountAlreadyInitialized,

    #[msg("Mintedgem: Insufficient amount")]
    InsufficientAmount,

    #[msg("Mintedgem: User Insufficient amount")]
    UserInsufficientAmount,

    #[msg("Mintedgem: Invalid admin")]
    InvalidAdmin,

    #[msg("Mintedgem: Invalid mindgem program")]
    InvalidMintedgemProgram,

    #[msg("Mintedgem: Invalid WSOL address")]
    InvalidWSOLAddress,

    #[msg("Mintedgem: Invalid DONE token address")]
    InvalidDoneTokenAddress,

    #[msg("Mintedgem: Only owner can call this function!")]
    NotOwner,

    #[msg("Mintedgem: Vault SOL is already initialized")]
    VaultSolAlreadyInitialized,

    #[msg("Mintedgem: Vault DONE token is already initialized")]
    VaultDoneTokenAlreadyInitialized,

    #[msg("Mintedgem: Amount must be greater than 0")]
    InvalidAmount,

    #[msg("Mintedgem: Percent must be greater thean or equal 0 and less than or equal 100")]
    InvalidPercent,

    #[msg("Mintedgem: Invalid creator")]
    InvalidCreator,

    #[msg("Mintedgem: Transaction failed")]
    TransferFailed,

    #[msg("Mintedgem: Transfer back failed")]
    TransferBackFailed,

    #[msg("Mintedgem: CPI Swap failed")]
    CPISwapFailed
}
