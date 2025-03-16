use cosmwasm_std::{StdResult, StdError, Uint128};
use crate::state::ViewershipOracle;

pub fn calculate_price(oracle: &ViewershipOracle, _supply: Uint128) -> StdResult<Uint128> {
    // Base price calculation using view count
    let base_price = Uint128::from(oracle.price_params.k) * Uint128::from(1_000_000_000u64);
    let view_factor = Uint128::from(oracle.view_count) * Uint128::from(oracle.price_params.m);
    
    // Simple bonding curve calculation - can be made more sophisticated
    let price = base_price + view_factor;
    
    Ok(price)
}

pub fn get_inj_amount(funds: &[cosmwasm_std::Coin]) -> Uint128 {
    funds
        .iter()
        .find(|c| c.denom == "inj")
        .map(|c| c.amount)
        .unwrap_or(Uint128::zero())
}

