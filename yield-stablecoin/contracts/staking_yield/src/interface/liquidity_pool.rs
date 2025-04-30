use soroban_sdk::{contractclient, Address, Env};

#[contractclient(name = "LiquidityPoolClient")]
pub trait LiquidityPoolTrait {
    fn balance_shares(e: Env, user: Address) -> i128;

    fn deposit(e: Env, to: Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128);

    fn swap(e: Env, to: Address, buy_a: bool, out: i128, in_max: i128);

    fn withdraw(e: Env, to: Address, share_amount: i128, min_a: i128, min_b: i128) -> (i128, i128);

    fn get_rsrvs(e: Env) -> (i128, i128);
}
