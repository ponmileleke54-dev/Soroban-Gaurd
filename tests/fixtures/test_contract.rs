use soroban_sdk::{contract, contractimpl, Env, Vec, Symbol};

#[contract]
pub struct VulnerableVault;

#[contractimpl]
impl VulnerableVault {
    // 🚨 Triggers SG001 (Missing auth check) & SG002 (Missing TTL extension)
    pub fn deposit(env: Env, amount: i128) {
        let mut balance: i128 = env.storage().instance().get(&Symbol::short("BAL")).unwrap_or(0);
        balance += amount;
        env.storage().instance().set(&Symbol::short("BAL"), &balance);
    }

    // ⚠️ Triggers SG003 (Unbounded loop over Vec without .len() check)
    pub fn batch_process(env: Env, recipients: Vec<Symbol>) {
        for recipient in recipients.iter() {
            // Process each recipient without validating recipients.len()
        }
    }

    // ✅ Safe function: Has require_auth and extend_ttl
    pub fn withdraw(env: Env, user: Symbol, amount: i128) {
        user.require_auth();
        env.storage().instance().extend_ttl(100, 100);
        let mut balance: i128 = env.storage().instance().get(&Symbol::short("BAL")).unwrap_or(0);
        if balance >= amount {
            balance -= amount;
            env.storage().instance().set(&Symbol::short("BAL"), &balance);
        }
    }
}