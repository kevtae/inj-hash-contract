use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Addr};
use crate::{
    error::ContractError,
    state::{TOKEN_VAULTS}
};

pub fn setup_vault_account(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mint: String,
) -> Result<Response, ContractError> {
    // Validate mint is a proper token factory denom
    if !mint.starts_with("factory/") {
        return Err(ContractError::InvalidDenom {});
    }
    
    let mint_key = mint.as_bytes();
    
    let mut vault = TOKEN_VAULTS.load(deps.storage, mint_key)?;
    
    // Check if vault is already fully set up
    if vault.vault_account.is_some() {
        return Err(ContractError::VaultAlreadySetup {});
    }
    
    // Create a deterministic vault address (would be a sub-account in practice)
    // In a real implementation, you would derive this address in a secure way
    // For Token Factory, we can use a derived address based on the denom
    
    // Extract creator address from the denom (assuming format is factory/{creator}/{subdenom})
    let parts: Vec<&str> = mint.split('/').collect();
    if parts.len() < 3 {
        return Err(ContractError::InvalidDenom {});
    }
    
    let creator = parts[1];
    let subdenom = parts[2];
    
    // Create deterministic vault addresses
    let vault_addr = Addr::unchecked(format!("vault_{}_{}", creator, subdenom));
    let inj_vault_wallet = Addr::unchecked(format!("inj_vault_{}_{}", creator, subdenom));
    
    // Update the vault with proper addresses
    vault.vault_account = Some(vault_addr.clone());
    vault.inj_vault_wallet = inj_vault_wallet.clone();
    
    TOKEN_VAULTS.save(deps.storage, mint_key, &vault)?;
    
    Ok(Response::new()
        .add_attribute("action", "setup_vault_account")
        .add_attribute("mint", mint)
        .add_attribute("vault_account", vault_addr.to_string())
        .add_attribute("inj_vault_wallet", inj_vault_wallet.to_string()))
}