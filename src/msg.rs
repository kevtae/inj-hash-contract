use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint128};
use cw20::Cw20ReceiveMsg;
use cosmwasm_schema::{QueryResponses};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub platform_wallet: String,
    pub oracle_authority: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Platform management
    UpdatePlatform {
        new_platform_wallet: String,
        new_oracle_authority: Option<String>,
    },
    
    // Token setup
    InitializeTokenMetadata {
        id: u64,
        name: String,
        music_uri: String,
        mint: String, // CW20 token address
    },
    InitializeTokenOracle {
        mint: String,
    },
    SetupVaultAccount {
        mint: String,
    },
    
    // Token operations
    UpdateOracle {
        mint: String,
        new_view_count: u64,
    },
    PurchaseToken {
        mint: String,
        amount_tokens: Uint128,
    },
    
    // New mint function
    MintToken {
        mint: String,
        recipient: String,
        amount: Uint128,
    },
    
    // CW20 receive function for token transfers
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[returns(PlatformConfigResponse)]
    GetPlatformConfig {},
    
    #[returns(TokenMetadataResponse)]
    GetTokenMetadata { mint: String },
    
    #[returns(TokenPriceResponse)]
    GetTokenPrice { mint: String },
    
    #[returns(VaultBalanceResponse)]
    GetVaultBalance { mint: String },
}

// Response types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PlatformConfigResponse {
    pub platform_wallet: String,
    pub oracle_authority: String,
    pub authority: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenMetadataResponse {
    pub name: String,
    pub music_uri: String,
    pub id: u64,
    pub mint: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenPriceResponse {
    pub price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultBalanceResponse {
    pub balance: Uint128,
}
