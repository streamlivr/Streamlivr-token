use ic_cdk::api;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{update, query};
use std::collections::HashMap;

#[derive(CandidType, Deserialize)]
struct Token {
    name: String,
    symbol: String,
    total_supply: u64,
    balances: HashMap<Principal, u64>,
    owner: Principal,
}

#[update]
fn mint(to: Principal, amount: u64) {
    let caller = api::caller();
    let mut token = get_token();

    if caller != token.owner {
        panic("Only the owner can mint tokens.");
    }

    if amount == 0 {
        panic("Amount must be more than zero.");
    }

    let balance = token.balances.entry(to).or_insert(0);
    *balance += amount;
    token.total_supply += amount;

    save_token(token);
}

#[update]
fn burn(amount: u64) {
    let caller = api::caller();
    let mut token = get_token();

    if caller != token.owner {
        panic("Only the owner can burn tokens.");
    }

    let balance = token.balances.get_mut(&caller).unwrap_or_else(|| panic("Burn amount exceeds balance."));
    
    if *balance < amount {
        panic("Burn amount exceeds balance.");
    }

    *balance -= amount;
    token.total_supply -= amount;

    save_token(token);
}

#[query]
fn balance_of(owner: Principal) -> u64 {
    let token = get_token();
    *token.balances.get(&owner).unwrap_or(&0)
}

fn get_token() -> Token {
    // This function retrieves the token state from storage.
}

fn save_token(token: Token) {
    // This function saves the updated token state to storage.
}

#[update]
fn initialize(initial_supply: u64, to: Principal) {
    let caller = api::caller();

    if to == Principal::anonymous() {
        panic("Cannot mint to zero address.");
    }

    if initial_supply == 0 {
        panic("Initial supply must be more than zero.");
    }

    let mut token = Token {
        name: "Streamlivr".to_string(),
        symbol: "LIVR".to_string(),
        total_supply: initial_supply,
        balances: HashMap::new(),
        owner: caller,
    };

    token.balances.insert(to, initial_supply);
    
    save_token(token);
}