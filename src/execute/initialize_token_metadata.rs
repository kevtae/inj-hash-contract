use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::{error::ContractError, state::{TOKEN_METADATA, TokenMetadata}};

pub fn initialize_token_metadata(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
    name: String,
    music_uri: String,
    mint: String,
) -> Result<Response, ContractError> {
    // Validate inputs
    if name.len() > 32 {
        return Err(ContractError::NameTooLong {});
    }
    
    if music_uri.len() > 44 {
        return Err(ContractError::UriTooLong {});
    }
    
    // Validate mint is a proper token factory denom
    if !mint.starts_with("factory/") {
        return Err(ContractError::InvalidDenom {});
    }
    
    // Use the denom string as the key
    let key = mint.as_bytes();
    
    let metadata = TokenMetadata {
        name,
        music_uri,
        id,
        mint: mint.clone(), // Store the denom string
    };
    
    TOKEN_METADATA.save(deps.storage, key, &metadata)?;
    
    Ok(Response::new()
        .add_attribute("action", "initialize_token_metadata")
        .add_attribute("id", id.to_string())
        .add_attribute("mint", mint))
}