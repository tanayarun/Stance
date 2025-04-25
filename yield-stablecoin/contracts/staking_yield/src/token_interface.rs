use soroban_sdk::{contractclient, Address, Env};

#[contractclient(name = "TokenClient")]
pub trait TokenContract {
    fn balance(e: &Env, user: Address) -> i128;
    fn total_supply(e: &Env) -> i128;
}
