use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response, Uint128, BankMsg, CosmosMsg, 
    coins, SubMsg, WasmMsg, to_binary
};
use crate::{
    error::ContractError, 
    state::{CONFIG, TOKEN_VAULTS, VIEWERSHIP_ORACLES},
    utils::calculate_price
};

// Custom token factory mint message structure (for Injective)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
struct MintTokenMsg {
    mint: MintParams,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
struct MintParams {
    sender: String,
    amount: TokenAmount,
    recipient: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
struct TokenAmount {
    denom: String,
    amount: String,
}

pub fn purchase_token(
    deps: DepsMut,
    _env: Env,
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
    let mut messages: Vec<CosmosMsg> = vec![];
    
    // Send platform fee
    let platform_msg = BankMsg::Send {
        to_address: config.platform_wallet.to_string(),
        amount: coins(platform_fee.u128(), "inj"),
    };
    messages.push(CosmosMsg::Bank(platform_msg));
    
    // Send remainder to vault
    let vault_msg = BankMsg::Send {
        to_address: vault.inj_vault_wallet.to_string(),
        amount: coins(vault_amount.u128(), "inj"),
    };
    messages.push(CosmosMsg::Bank(vault_msg));
    
    // Update vault total collected
    vault.total_collected += vault_amount;
    TOKEN_VAULTS.save(deps.storage, mint_key, &vault)?;
    
    // For Token Factory, we need to use the admin wallet to mint tokens
    // This is typically handled outside the contract via MsgMint
    // Here we're using a SubMsg to call the tokenfactory module
    
    let mint_msg = WasmMsg::Execute {
        contract_addr: "inj1tokenfactory".to_string(), // This is a placeholder - replace with actual address
        msg: to_binary(&MintTokenMsg {
            mint: MintParams {
                sender: config.authority.to_string(), // Use the contract authority as sender
                amount: TokenAmount {
                    denom: mint.clone(),
                    amount: amount_tokens.to_string(),
                },
                recipient: info.sender.to_string(),
            }
        })?,
        funds: vec![],
    };
    
    // Note: In a real implementation, you might need to use a different approach
    // such as sending a message to a service that has permission to mint tokens
    messages.push(CosmosMsg::Wasm(mint_msg));
    
    // Check liquidity threshold
    let mut response = Response::new()
        .add_messages(messages)
        .add_attribute("action", "purchase_token")
        .add_attribute("amount", amount_tokens.to_string())
        .add_attribute("total_cost", total_cost.to_string());
        
    if vault.total_collected >= vault.liquidity_threshold {
        if let Some(dex_pool) = &vault.dex_pool {
            response = response.add_attribute("notice", "Liquidity threshold reached - consider providing liquidity");
        }
    }
    
    Ok(response)
}