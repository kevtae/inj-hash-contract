use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response, Uint128, BankMsg, 
    coins, WasmMsg, to_json_binary, StdError
};
use crate::{
    error::ContractError, 
    state::{CONFIG, TOKEN_VAULTS, VIEWERSHIP_ORACLES},
    utils::calculate_price
};

// Custom message structure to be passed to a Token Factory handler contract
#[derive(serde::Serialize)]
struct MintMsg {
    denom: String,
    amount: String,
    recipient: String,
}

pub fn purchase_token(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mint: String,
    amount_tokens: Uint128,
) -> Result<Response, ContractError> {
    // Validate amount is not zero
    if amount_tokens.is_zero() {
        return Err(ContractError::InvalidAmount {});
    }
    
    // Validate mint is a proper token factory denom
    if !mint.starts_with("factory/") {
        return Err(ContractError::InvalidDenom {});
    }
    
    let mint_key = mint.as_bytes();
    
    // Load required data
    let oracle = VIEWERSHIP_ORACLES.load(deps.storage, mint_key)?;
    let mut vault = TOKEN_VAULTS.load(deps.storage, mint_key)?;
    let config = CONFIG.load(deps.storage)?;
    
    // Verify vault is properly set up
    if vault.vault_account.is_none() {
        return Err(ContractError::CustomError { 
            val: "Vault account not set up. Please call setup_vault_account first.".to_string() 
        });
    }
    
    // Calculate price based on viewership data
    let price_per_token = calculate_price(&oracle, Uint128::zero())?;
    
    // Calculate total cost
    let total_cost = price_per_token * amount_tokens;
    
    // Calculate platform fee (2.5%)
    let platform_fee = total_cost.multiply_ratio(25u128, 1000u128);
    let vault_amount = total_cost - platform_fee;
    
    // Check if sufficient INJ was sent
    let payment = info
        .funds
        .iter()
        .find(|c| c.denom == "inj")
        .map(|c| c.amount)
        .unwrap_or(Uint128::zero());
        
    if payment < total_cost {
        return Err(ContractError::InsufficientFunds {});
    }
    
    // Process payments
    let mut messages = vec![];
    
    // Validate platform wallet address before sending
    let platform_wallet_str = config.platform_wallet.to_string();
    if let Err(e) = deps.api.addr_validate(&platform_wallet_str) {
        return Err(ContractError::CustomError { 
            val: format!("Invalid platform wallet address: {}", e) 
        });
    }
    
    // Send platform fee
    let platform_msg = BankMsg::Send {
        to_address: platform_wallet_str.clone(),
        amount: coins(platform_fee.u128(), "inj"),
    };
    messages.push(cosmwasm_std::CosmosMsg::Bank(platform_msg));
    
    // Validate vault wallet address before sending
    let vault_wallet_str = vault.inj_vault_wallet.to_string();
    if let Err(e) = deps.api.addr_validate(&vault_wallet_str) {
        return Err(ContractError::CustomError { 
            val: format!("Invalid vault wallet address: {}", e) 
        });
    }
    
    // Send remainder to vault - use clone() to avoid moving the string
    let vault_msg = BankMsg::Send {
        to_address: vault_wallet_str.clone(),
        amount: coins(vault_amount.u128(), "inj"),
    };
    messages.push(cosmwasm_std::CosmosMsg::Bank(vault_msg));
    
    // Update vault total collected
    vault.total_collected += vault_amount;
    TOKEN_VAULTS.save(deps.storage, mint_key, &vault)?;
    
    // Create response with debugging information
    let mut response = Response::new()
        .add_messages(messages)
        .add_attribute("action", "purchase_token")
        .add_attribute("mint_denom", mint)
        .add_attribute("mint_amount", amount_tokens.to_string())
        .add_attribute("mint_to", info.sender)
        .add_attribute("total_cost", total_cost.to_string())
        .add_attribute("vault_addr", vault_wallet_str); // Now we can use it again
        
    if vault.total_collected >= vault.liquidity_threshold {
        if let Some(dex_pool) = &vault.dex_pool {
            response = response.add_attribute("notice", "Liquidity threshold reached - consider providing liquidity");
        }
    }
    
    Ok(response)
}