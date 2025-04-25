#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

mod token_interface;
use token_interface::TokenClient;

#[contract]
pub struct StakingYield;

#[contractimpl]
impl StakingYield {
    pub fn get_token_balance(e: &Env, token_address: Address, user: Address) -> i128 {
        let token = TokenClient::new(e, &token_address);
        token.balance(&user)
    }

    pub fn total_supply(e: &Env, token_address: Address) -> i128 {
        let token = TokenClient::new(e, &token_address);
        token.total_supply()
    }
}
