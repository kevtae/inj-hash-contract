use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::{
    error::ContractError,
    state::{CONFIG}
};

pub fn update_platform(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_platform_wallet: String,
    new_oracle_authority: Option<String>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    
    // Only the current authority can update platform settings
    if info.sender != config.authority {
        return Err(ContractError::Unauthorized {});
    }
    
    // Update platform wallet
    config.platform_wallet = deps.api.addr_validate(&new_platform_wallet)?;
    
    // Update oracle authority if provided
    if let Some(new_auth) = new_oracle_authority {
        config.oracle_authority = deps.api.addr_validate(&new_auth)?;
    }
    
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_platform")
        .add_attribute("new_platform_wallet", new_platform_wallet))
}
