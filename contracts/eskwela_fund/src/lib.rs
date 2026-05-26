#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token};

#[contracttype]
pub enum DataKey {
    Admin,
    Token,
}

#[contract]
pub struct EskwelaFund;

#[contractimpl]
impl EskwelaFund {
    /// Initializes the contract with an admin and the USDC token address.
    /// Maps to the setup phase before MVP disbursements can occur.
    pub fn init(env: Env, admin: Address, token: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    /// MVP Feature: Disburses tokens to a student.
    /// Requires the authorized admin to sign the transaction, pulling funds from the admin's balance.
    pub fn disburse(env: Env, admin: Address, student: Address, amount: i128) {
        // Ensure the transaction is signed by the caller
        admin.require_auth();
        
        // Verify caller matches the authorized admin in storage
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        if admin != stored_admin {
            panic!("Unauthorized admin");
        }
        
        // Execute the transfer using the Stellar asset token client
        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        
        token_client.transfer(&admin, &student, &amount);
    }

    /// Verifies current state: Returns the stored admin address.
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).expect("Not initialized")
    }
}

mod test;