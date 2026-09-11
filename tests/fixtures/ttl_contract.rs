use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct TtlFixture;

#[contractimpl]
impl TtlFixture {
    pub fn missing_instance_ttl(env: Env) {
        env.storage().instance().set(&Symbol::short("KEY"), &1i128);
    }

    pub fn extended_persistent_ttl(env: Env) {
        env.storage().persistent().extend_ttl(100, 100);
        env.storage().persistent().set(&Symbol::short("KEY"), &1i128);
    }
}