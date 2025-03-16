use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::{
    error::ContractError,
    state::{CONFIG, VIEWERSHIP_ORACLES}
};

pub fn update_oracle(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mint: String,
    new_view_count: u64,
) -> Result<Response, ContractError> {
    // Get the oracle authority from config
    let config = CONFIG.load(deps.storage)?;
    
    // Only the oracle authority can update view counts
    if info.sender != config.oracle_authority {
        return Err(ContractError::Unauthorized {});
    }
    
    // Validate mint is a proper token factory denom
    if !mint.starts_with("factory/") {
        return Err(ContractError::InvalidDenom {});
    }
    
    let key = mint.as_bytes();
    
    let mut oracle = VIEWERSHIP_ORACLES.load(deps.storage, key)?;
    
    // Validate the new view count (should be increasing)
    if new_view_count < oracle.view_count {
        return Err(ContractError::InvalidViewCount {});
    }
    
    // Update the oracle data
    oracle.view_count = new_view_count;
    oracle.last_updated = _env.block.time.seconds();
    
    VIEWERSHIP_ORACLES.save(deps.storage, key, &oracle)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_oracle")
        .add_attribute("mint", mint)
        .add_attribute("new_view_count", new_view_count.to_string()))
}