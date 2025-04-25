#![no_std]
use openzeppelin_fungible_token::{
    self as fungible,
    mintable::{mint, FungibleMintable},
    FungibleToken,
};
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn __constructor(_e: &Env) {}
}

#[contractimpl]
impl FungibleToken for Token {
    fn total_supply(e: &Env) -> i128 {
        fungible::total_supply(e)
    }

    fn balance(e: &Env, account: Address) -> i128 {
        fungible::balance(e, &account)
    }

    fn allowance(e: &Env, owner: Address, spender: Address) -> i128 {
        fungible::allowance(e, &owner, &spender)
    }

    fn transfer(e: &Env, from: Address, to: Address, amount: i128) {
        fungible::transfer(e, &from, &to, amount);
    }

    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        fungible::transfer_from(e, &spender, &from, &to, amount);
    }

    fn approve(e: &Env, owner: Address, spender: Address, amount: i128, live_until_ledger: u32) {
        fungible::approve(e, &owner, &spender, amount, live_until_ledger);
    }

    fn decimals(e: &Env) -> u32 {
        fungible::metadata::decimals(e)
    }

    fn name(e: &Env) -> String {
        fungible::metadata::name(e)
    }

    fn symbol(e: &Env) -> String {
        fungible::metadata::symbol(e)
    }
}

#[contractimpl]
impl FungibleMintable for Token {
    fn mint(e: &Env, account: Address, amount: i128) {
        mint(e, &account, amount);
    }
}

mod test;
