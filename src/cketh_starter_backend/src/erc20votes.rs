use ic_cdk::api::call;
use ic_cdk::export::Principal;
use ic_cdk::storage;
use std::collections::HashMap;

#[derive(Default)]
struct Checkpoint {
    block_number: u64,
    votes: u128,
}

struct ERC20Votes {
    total_supply: u128,
    max_supply: u128,
    balances: HashMap<Principal, u128>,
    checkpoints: HashMap<Principal, Vec<Checkpoint>>,
}

impl ERC20Votes {
    fn new() -> Self {
        Self {
            total_supply: 0,
            max_supply: u128::MAX, // Equivalent to type(uint208).max
            balances: HashMap::new(),
            checkpoints: HashMap::new(),
        }
    }

    fn mint(&mut self, to: Principal, value: u128) {
        if self.total_supply + value > self.max_supply {
            panic!("ERC20ExceededSafeSupply");
        }
        self.total_supply += value;
        *self.balances.entry(to).or_insert(0) += value;
        self.update_checkpoints(to);
    }

    fn transfer(&mut self, from: Principal, to: Principal, value: u128) {
        let from_balance = self.balances.get(&from).cloned().unwrap_or(0);
        if from_balance < value {
            panic!("Insufficient balance");
        }
        
        self.balances.insert(from, from_balance - value);
        *self.balances.entry(to).or_insert(0) += value;
        
        self.update_checkpoints(from);
        self.update_checkpoints(to);
    }

    fn update_checkpoints(&mut self, account: Principal) {
        let current_block = ic_cdk::api::time();
        let votes = *self.balances.get(&account).unwrap_or(&0);
        
        let checkpoints = self.checkpoints.entry(account).or_insert(vec![]);
        checkpoints.push(Checkpoint { block_number: current_block, votes });
    }

    fn get_votes(&self, account: Principal) -> u128 {
        *self.balances.get(&account).unwrap_or(&0)
    }

    fn num_checkpoints(&self, account: Principal) -> usize {
        self.checkpoints.get(&account).map_or(0, |c| c.len())
    }

    fn checkpoints(&self, account: Principal, pos: usize) -> Option<&Checkpoint> {
        self.checkpoints.get(&account).and_then(|c| c.get(pos))
    }
}