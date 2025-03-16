use cosmwasm_std::{Deps, StdResult};
use crate::{
    msg::PlatformConfigResponse,
    state::CONFIG
};

pub fn get_platform_config(deps: Deps) -> StdResult<PlatformConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    
    Ok(PlatformConfigResponse {
        platform_wallet: config.platform_wallet.to_string(),
        oracle_authority: config.oracle_authority.to_string(),
        authority: config.authority.to_string(),
    })
}
