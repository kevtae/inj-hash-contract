use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use crate::{
    error::ContractError,
    state::{VIEWERSHIP_ORACLES, TOKEN_VAULTS, ViewershipOracle, PriceParameters, TokenVault}
};

pub fn initialize_token_oracle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mint: String,
) -> Result<Response, ContractError> {
    // Validate mint is a proper token factory denom
    if !mint.starts_with("factory/") {
        return Err(ContractError::InvalidDenom {});
    }
    
    let mint_key = mint.as_bytes();
    
    // Check if oracle already exists
    if VIEWERSHIP_ORACLES.may_load(deps.storage, mint_key)?.is_some() {
        return Err(ContractError::OracleAlreadyExists {});
    }
    
    // Initialize oracle with default values
    let oracle = ViewershipOracle {
        mint: mint.clone(), // Store denom string
        view_count: 0,
        last_updated: env.block.time.seconds(),
        price_params: PriceParameters {
            k: 1,
            m: 100,
        },
    };
    
    VIEWERSHIP_ORACLES.save(deps.storage, mint_key, &oracle)?;
    
    // Also initialize a token vault
    let vault = TokenVault {
        mint: mint.clone(), // Store denom string
        vault_account: None,
        inj_vault_wallet: info.sender.clone(), // Temporarily set to creator, will be updated in setup_vault
        liquidity_threshold: Uint128::from(10_000_000_000u128), // 10 INJ in smallest units
        total_collected: Uint128::zero(),
        dex_pool: None,
    };
    
    TOKEN_VAULTS.save(deps.storage, mint_key, &vault)?;
    
    Ok(Response::new()
        .add_attribute("action", "initialize_token_oracle")
        .add_attribute("mint", mint))
}