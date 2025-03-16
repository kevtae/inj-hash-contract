use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg, to_binary};
use crate::{
    error::ContractError,
    state::{TOKEN_METADATA, TOKEN_VAULTS, CONFIG}
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

pub fn mint_token(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mint: String,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Validate parameters
    if amount.is_zero() {
        return Err(ContractError::InvalidAmount {});
    }
    
    // Validate mint is a proper token factory denom
    if !mint.starts_with("factory/") {
        return Err(ContractError::InvalidDenom {});
    }
    
    let mint_key = mint.as_bytes();
    
    // Verify token metadata and vault exist
    let metadata = TOKEN_METADATA.load(deps.storage, mint_key)?;
    let _vault = TOKEN_VAULTS.load(deps.storage, mint_key)?;
    let config = CONFIG.load(deps.storage)?;
    
    // Verify caller is authorized to mint
    if info.sender != config.authority {
        return Err(ContractError::Unauthorized {});
    }
    
    // Validate recipient
    let recipient_addr = deps.api.addr_validate(&recipient)?;
    
    // For Token Factory, we need to use the admin wallet to mint tokens
    // This is typically handled outside the contract via MsgMint
    // Here we're using a WasmMsg to call the tokenfactory module
    
    let mint_msg = WasmMsg::Execute {
        contract_addr: "inj1tokenfactory".to_string(), // This is a placeholder - replace with actual address
        msg: to_binary(&MintTokenMsg {
            mint: MintParams {
                sender: config.authority.to_string(), // Use the contract authority as sender
                amount: TokenAmount {
                    denom: mint.clone(),
                    amount: amount.to_string(),
                },
                recipient: recipient_addr.to_string(),
            }
        })?,
        funds: vec![],
    };
    
    // Note: In a real implementation, you might need to use a different approach
    // such as sending a message to a service that has permission to mint tokens
    
    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("action", "mint_token")
        .add_attribute("mint", mint)
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount.to_string()))
}