#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> soroban_sdk::Vec<Symbol> {
        soroban_sdk::vec![&env, symbol_short!("Hello"), to]
    }
}

#[cfg(test)]
mod test;