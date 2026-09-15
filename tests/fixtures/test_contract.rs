use soroban_sdk::{contract, contractimpl, Env, Vec, Symbol};

#[contract]
pub struct VulnerableVault;

#[contractimpl]
impl VulnerableVault {
    // 🚨 Triggers SG001 (Missing auth), SG002 (Missing TTL), & SG006 (Raw addition +)
    pub fn deposit(env: Env, amount: i128) {
        let balance: i128 = env.storage().instance().get(&Symbol::short("BAL")).unwrap_or(0);
        let new_balance = balance + amount;
        env.storage().instance().set(&Symbol::short("BAL"), &new_balance);
    }

    // ⚠️ Triggers SG003 (Unbounded loop)
    pub fn batch_process(env: Env, recipients: Vec<Symbol>) {
        for recipient in recipients.iter() {
            // Process recipient
        }
    }

    // ⚠️ Triggers SG004 (Bare panic)
    pub fn emergency_halt(env: Env) {
        panic!("Emergency halt triggered!");
    }

    // ✅ Safe function
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