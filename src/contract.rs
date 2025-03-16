#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
use crate::execute;
use crate::query;

// Contract name and version used for migration info
const CONTRACT_NAME: &str = "crates.io:hashmelody";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let platform_wallet = deps.api.addr_validate(&msg.platform_wallet)?;
    let oracle_authority = deps.api.addr_validate(&msg.oracle_authority)?;
    
    let config = Config {
        platform_wallet,
        oracle_authority,
        authority: info.sender.clone(),
    };
    
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdatePlatform { new_platform_wallet, new_oracle_authority } => {
            execute::update_platform(deps, env, info, new_platform_wallet, new_oracle_authority)
        },
        ExecuteMsg::InitializeTokenMetadata { id, name, music_uri, mint } => {
            execute::initialize_token_metadata(deps, env, info, id, name, music_uri, mint)
        },
        ExecuteMsg::InitializeTokenOracle { mint } => {
            execute::initialize_token_oracle(deps, env, info, mint)
        },
        ExecuteMsg::SetupVaultAccount { mint } => {
            execute::setup_vault_account(deps, env, info, mint)
        },
        ExecuteMsg::UpdateOracle { mint, new_view_count } => {
            execute::update_oracle(deps, env, info, mint, new_view_count)
        },
        ExecuteMsg::PurchaseToken { mint, amount_tokens } => {
            execute::purchase_token(deps, env, info, mint, amount_tokens)
        },
        ExecuteMsg::MintToken { mint, recipient, amount } => {
            execute::mint_token(deps, env, info, mint, recipient, amount)
        },
        ExecuteMsg::Receive(msg) => {
            execute::receive_cw20(deps, env, info, msg)
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPlatformConfig {} => to_binary(&query::get_platform_config(deps)?),
        QueryMsg::GetTokenMetadata { mint } => to_binary(&query::get_token_metadata(deps, mint)?),
        QueryMsg::GetTokenPrice { mint } => to_binary(&query::get_token_price(deps, mint)?),
        QueryMsg::GetVaultBalance { mint } => to_binary(&query::get_vault_balance(deps, mint)?),
    }
}