#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec};

pub mod interface;
use crate::interface::liquidity_pool::LiquidityPoolClient;
use crate::interface::token_custom::CustomTokenClient;

#[contracttype]
pub struct StakeDetail {
    owner: Address,
    total_staked: i128,
}

const LP_CA: Symbol = symbol_short!("NA");
const W_XLM_CA: Symbol = symbol_short!("NA");
const USD_X_CA: Symbol = symbol_short!("NA");
const ALL_STAKERS: Symbol = symbol_short!("STAKERS");

#[contract]
pub struct StakingYield;

#[contractimpl]
impl StakingYield {
    pub fn __constructor(e: &Env, lp_ca: Address, wxlm_ca: Address, usdx_ca: Address) {
        e.storage().instance().set(&LP_CA, &lp_ca);
        e.storage().instance().set(&W_XLM_CA, &wxlm_ca);
        e.storage().instance().set(&USD_X_CA, &usdx_ca);
    }

    fn add_staker_if_new(e: &Env, user: &Address) {
        let mut stakers = e
            .storage()
            .persistent()
            .get::<_, Vec<Address>>(&ALL_STAKERS)
            .unwrap_or(Vec::new(e));

        if !stakers.iter().any(|addr| &addr == user) {
            stakers.push_back(user.clone());
            e.storage().persistent().set(&ALL_STAKERS, &stakers);
        }
    }

    pub fn stake(e: &Env, user: Address, amount: i128) {
        user.require_auth();

        let wxlm = CustomTokenClient::new(
            e,
            &e.storage().instance().get::<_, Address>(&W_XLM_CA).unwrap(),
        );
        let usdx = CustomTokenClient::new(
            e,
            &e.storage().instance().get::<_, Address>(&USD_X_CA).unwrap(),
        );

        wxlm.transfer_from(
            &user.clone(),
            &user.clone(),
            &e.current_contract_address(),
            &amount,
        );
        usdx.mint(&user.clone(), &amount);

        let total_staked_key = symbol_short!("STAKED");
        let current = e
            .storage()
            .persistent()
            .get::<_, i128>(&total_staked_key)
            .unwrap_or(0);
        e.storage()
            .persistent()
            .set(&total_staked_key, &(current + amount));

        let user_key = (symbol_short!("STK"), user.clone());
        let user_stake = e
            .storage()
            .persistent()
            .get::<_, i128>(&user_key)
            .unwrap_or(0);
        e.storage()
            .persistent()
            .set(&user_key, &(user_stake + amount));

        Self::add_staker_if_new(e, &user);
    }

    pub fn unstake(e: &Env, user: Address, amount: i128) {
        user.require_auth();

        let wxlm = CustomTokenClient::new(
            e,
            &e.storage().instance().get::<_, Address>(&W_XLM_CA).unwrap(),
        );
        let usdx = CustomTokenClient::new(
            e,
            &e.storage().instance().get::<_, Address>(&USD_X_CA).unwrap(),
        );

        let user_key = (symbol_short!("STK"), user.clone());
        let user_stake = e
            .storage()
            .persistent()
            .get::<_, i128>(&user_key)
            .unwrap_or(0);
        assert!(user_stake >= amount, "Insufficient stake");

        usdx.burn(&user.clone(), &amount);
        wxlm.transfer(&e.current_contract_address(), &user.clone(), &amount);

        e.storage()
            .persistent()
            .set(&user_key, &(user_stake - amount));

        let total_staked_key = symbol_short!("STAKED");
        let current = e
            .storage()
            .persistent()
            .get::<_, i128>(&total_staked_key)
            .unwrap_or(0);
        e.storage()
            .persistent()
            .set(&total_staked_key, &(current - amount));
    }

    pub fn distribute_yield(e: &Env) {
        let total_staked_key = symbol_short!("STAKED");
        let total_staked = e
            .storage()
            .persistent()
            .get::<_, i128>(&total_staked_key)
            .unwrap_or(0);
        assert!(total_staked > 0, "No stakers");

        let simulated_yield = 1000;
        let stakers = e
            .storage()
            .persistent()
            .get::<_, Vec<Address>>(&ALL_STAKERS)
            .unwrap_or(Vec::new(e));

        for user in stakers.iter() {
            let user_key = (symbol_short!("STK"), user.clone());
            let user_stake = e
                .storage()
                .persistent()
                .get::<_, i128>(&user_key)
                .unwrap_or(0);

            if user_stake == 0 {
                continue;
            }

            let reward = simulated_yield * user_stake / total_staked;

            let reward_key = (symbol_short!("REWARD"), user.clone());
            let prev = e
                .storage()
                .persistent()
                .get::<_, i128>(&reward_key)
                .unwrap_or(0);
            e.storage().persistent().set(&reward_key, &(prev + reward));
        }
    }

    pub fn claim_rewards(e: &Env, user: Address) {
        user.require_auth();

        let usdx = CustomTokenClient::new(
            e,
            &e.storage().instance().get::<_, Address>(&USD_X_CA).unwrap(),
        );

        let reward_key = (symbol_short!("REWARD"), user.clone());
        let reward = e
            .storage()
            .persistent()
            .get::<_, i128>(&reward_key)
            .unwrap_or(0);
        assert!(reward > 0, "No rewards");

        e.storage().persistent().remove(&reward_key);
        usdx.mint(&user, &reward);
    }

    pub fn view_staked_balance(e: &Env, user: Address) -> i128 {
        e.storage()
            .persistent()
            .get::<_, i128>(&(symbol_short!("STK"), user))
            .unwrap_or(0)
    }

    pub fn view_total_staked(e: &Env) -> i128 {
        e.storage()
            .persistent()
            .get::<_, i128>(&symbol_short!("STAKED"))
            .unwrap_or(0)
    }

    pub fn view_accumulated_rewards(e: &Env) -> i128 {
        let mut total = 0;
        let stakers = e
            .storage()
            .persistent()
            .get::<_, Vec<Address>>(&ALL_STAKERS)
            .unwrap_or(Vec::new(e));

        for user in stakers.iter() {
            let reward_key = (symbol_short!("REWARD"), user.clone());
            let reward = e
                .storage()
                .persistent()
                .get::<_, i128>(&reward_key)
                .unwrap_or(0);
            total += reward;
        }

        total
    }
}
