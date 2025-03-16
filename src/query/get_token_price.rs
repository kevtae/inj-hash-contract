use cosmwasm_std::{Deps, StdResult, Uint128};
use crate::{
    msg::TokenPriceResponse,
    state::VIEWERSHIP_ORACLES,
    utils::calculate_price
};

pub fn get_token_price(deps: Deps, mint: String) -> StdResult<TokenPriceResponse> {
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
    
    let oracle = VIEWERSHIP_ORACLES.load(deps.storage, &key)?;
    
    // In a real implementation, you would query the CW20 token contract
    // to get the actual supply, but here we'll use a simplified approach
    let price = calculate_price(&oracle, Uint128::zero())?;
    
    Ok(TokenPriceResponse { price })
}