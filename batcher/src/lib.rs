#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val, Vec};

#[contract]
pub struct ContractBatcher;

pub trait BatcherInterface {
    fn batch(e: Env, auth_users: Vec<Address>, batch: Vec<(Address, Symbol, Vec<Val>)>)
        -> Vec<Val>;
}

#[contractimpl]
impl BatcherInterface for ContractBatcher {
    fn batch(
        e: Env,
        auth_users: Vec<Address>,
        batch: Vec<(Address, Symbol, Vec<Val>)>,
    ) -> Vec<Val> {
        for user in auth_users {
            user.require_auth();
        }
        let mut results = Vec::new(&e);
        for (contract, fn_name, args) in batch {
            results.push_back(e.invoke_contract(&contract, &fn_name, args));
        }
        results
    }
}

mod test;
