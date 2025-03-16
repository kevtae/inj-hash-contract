use cosmwasm_std::{Deps, StdResult, Uint128};
use crate::{
    msg::VaultBalanceResponse,
    state::TOKEN_VAULTS
};

pub fn get_vault_balance(deps: Deps, mint: String) -> StdResult<VaultBalanceResponse> {
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
    
    let vault = TOKEN_VAULTS.load(deps.storage, &key)?;
    
    Ok(VaultBalanceResponse {
        balance: vault.total_collected,
    })
}