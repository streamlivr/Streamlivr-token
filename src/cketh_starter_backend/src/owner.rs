use ic_cdk::api::caller;
use ic_cdk::export::Principal;
use ic_cdk_macros::{update, query};
use std::cell::RefCell;

thread_local! {
    static OWNER: RefCell<Principal> = RefCell::new(Principal::anonymous());
}

#[derive(Debug)]
pub struct OwnableError;

impl OwnableError {
    pub fn unauthorized_account(account: Principal) -> String {
        format!("Unauthorized account: {:?}", account)
    }

    pub fn invalid_owner(owner: Principal) -> String {
        format!("Invalid owner: {:?}", owner)
    }
}

#[update]
pub fn initialize(initial_owner: Principal) {
    if initial_owner == Principal::anonymous() {
        panic!("{}", OwnableError::invalid_owner(Principal::anonymous()));
    }
    OWNER.with(|owner| {
        *owner.borrow_mut() = initial_owner;
    });
}

#[query]
pub fn owner() -> Principal {
    OWNER.with(|owner| owner.borrow().clone())
}

fn check_owner() {
    let caller = caller();
    OWNER.with(|owner| {
        if *owner.borrow() != caller {
            panic!("{}", OwnableError::unauthorized_account(caller));
        }
    });
}

#[update]
pub fn transfer_ownership(new_owner: Principal) {
    check_owner();
    if new_owner == Principal::anonymous() {
        panic!("{}", OwnableError::invalid_owner(Principal::anonymous()));
    }
    OWNER.with(|owner| {
        *owner.borrow_mut() = new_owner;
    });
}

#[update]
pub fn renounce_ownership() {
    check_owner();
    OWNER.with(|owner| {
        *owner.borrow_mut() = Principal::anonymous();
    });
}