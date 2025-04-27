#![cfg(test)]

extern crate std;

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{Token, TokenClient};

fn create_client<'a>(e: &Env) -> TokenClient<'a> {
    let address = e.register(Token, ());
    TokenClient::new(e, &address)
}

#[test]
fn mint() {
    let e = Env::default();
    let client = create_client(&e);
    let user = Address::generate(&e);

    client.mint(&user, &500);

    assert_eq!(client.balance(&user), 500);
    assert_eq!(client.total_supply(), 500);
}

#[test]
fn burn_token() {
    let e = Env::default();
    let client = create_client(&e);
    let user = Address::generate(&e);

    client.mint(&user, &500);
    assert_eq!(client.balance(&user), 500);

    user.require_auth();
    client.burn(&user, &300);
    assert_eq!(client.balance(&user), 200);
}
