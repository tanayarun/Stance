#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
pub struct StakeDetail {
    owner: Address,
    total_staked: i128,
}

#[contract]
pub struct StakingYield;

#[contractimpl]
impl StakingYield {
    pub fn __constructor(e: &Env) {}

    pub fn stake(e: &Env, user: Address, amount: i128) {
        user.require_auth();
    }

    pub fn get_token_balance(e: &Env, token_address: Address, user: Address) -> i128 {
        let token = token::Client::new(e, &token_address);
        token.balance(&user)
    }
}
