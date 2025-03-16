use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

// Platform configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub platform_wallet: Addr,
    pub oracle_authority: Addr,
    pub authority: Addr,
}

// Token metadata - modified to use String for mint instead of Addr
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenMetadata {
    pub name: String,
    pub music_uri: String,
    pub id: u64,
    pub mint: String, // Changed from Addr to String for Token Factory denom
}

// Price parameters
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceParameters {
    pub k: u64,
    pub m: u64,
}

// Viewership oracle - modified to use String for mint
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ViewershipOracle {
    pub mint: String, // Changed from Addr to String
    pub view_count: u64,
    pub last_updated: u64,
    pub price_params: PriceParameters,
}

// Token vault - modified to use String for mint
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenVault {
    pub mint: String, // Changed from Addr to String
    pub vault_account: Option<Addr>,
    pub inj_vault_wallet: Addr, 
    pub liquidity_threshold: Uint128,
    pub total_collected: Uint128,
    pub dex_pool: Option<Addr>,
}

// Define storage - using denom strings as keys
pub const CONFIG: Item<Config> = Item::new("config");
pub const TOKEN_METADATA: Map<&[u8], TokenMetadata> = Map::new("token_metadata");
pub const VIEWERSHIP_ORACLES: Map<&[u8], ViewershipOracle> = Map::new("viewership_oracles");
pub const TOKEN_VAULTS: Map<&[u8], TokenVault> = Map::new("token_vaults");