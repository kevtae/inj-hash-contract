use cosmwasm_std::{Deps, StdResult};
use crate::{
    msg::TokenMetadataResponse,
    state::TOKEN_METADATA
};

pub fn get_token_metadata(deps: Deps, mint: String) -> StdResult<TokenMetadataResponse> {
    // Create an owned Vec<u8> for the key
    let key: Vec<u8>;
    
    if mint.starts_with("factory/") {
        // For factory denoms, use the string bytes directly
        key = mint.as_bytes().to_vec();
    } else {
        // For regular addresses, validate and convert to bytes
        let mint_addr = deps.api.addr_validate(&mint)?;
        key = mint_addr.as_bytes().to_vec();
    }
    
    let metadata = TOKEN_METADATA.load(deps.storage, &key)?;
    
    Ok(TokenMetadataResponse {
        name: metadata.name,
        music_uri: metadata.music_uri,
        id: metadata.id,
        mint: metadata.mint.to_string(),
    })
}