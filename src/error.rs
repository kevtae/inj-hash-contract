use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Name must be between 1 and 32 characters")]
    NameTooLong {},

    #[error("URI must be between 1 and 44 characters")]
    UriTooLong {},

    #[error("Invalid amount provided")]
    InvalidAmount {},

    #[error("Insufficient funds for purchase")]
    InsufficientFunds {},

    #[error("Supply calculation overflow")]
    SupplyOverflow {},

    #[error("Invalid view count provided")]
    InvalidViewCount {},

    #[error("Oracle already exists for this token")]
    OracleAlreadyExists {},

    #[error("Vault already setup for this token")]
    VaultAlreadySetup {},

    #[error("Invalid denom format, must start with 'factory/'")]
    InvalidDenom {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}