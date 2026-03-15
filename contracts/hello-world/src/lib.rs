#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol};

const OWNER: Symbol = symbol_short!("OWNER");
const BENEFICIARY: Symbol = symbol_short!("BENEF");
const MESSAGE: Symbol = symbol_short!("MSG");

#[contract]
pub struct OrchainWill;

#[contractimpl]
impl OrchainWill {
    // Initialize the will
    pub fn create_will(env: Env, owner: Address, beneficiary: Address, message: String) {
        owner.require_auth();

        env.storage().instance().set(&OWNER, &owner);
        env.storage().instance().set(&BENEFICIARY, &beneficiary);
        env.storage().instance().set(&MESSAGE, &message);
    }

    // Beneficiary retrieves the message
    pub fn claim_will(env: Env, caller: Address) -> String {
        caller.require_auth();

        let beneficiary: Address = env.storage().instance().get(&BENEFICIARY).unwrap();

        if caller != beneficiary {
            panic!("Not authorized");
        }

        let message: String = env.storage().instance().get(&MESSAGE).unwrap();

        message
    }

    // View beneficiary
    pub fn get_beneficiary(env: Env) -> Address {
        env.storage().instance().get(&BENEFICIARY).unwrap()
    }
}
