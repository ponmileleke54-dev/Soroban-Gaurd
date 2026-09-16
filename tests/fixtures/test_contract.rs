use soroban_sdk::{contract, contractimpl, Address, Env, Vec, Symbol};

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

    // ⚠️ Triggers SG007 (Unused try_get return value)
    pub fn check_status(env: Env) {
        env.storage().instance().try_get::<Symbol, i128>(&Symbol::short("BAL"));
    }

    // 🚨 Triggers SG008 (Reentrancy: state mutation after external call)
    pub fn execute_external_transfer(env: Env, target_contract: Address, amount: i128) {
        target_contract.require_auth();
        let _ = env.invoke_contract::<i128>(&target_contract, &Symbol::short("payout"), Vec::new(&env));
        env.storage().instance().set(&Symbol::short("BAL"), &0i128);
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

    // soroban-guard:disable-next-line SG001
    pub fn deposit_with_suppression(env: Env, amount: i128) {
        let balance: i128 = env.storage().instance().get(&Symbol::short("BAL")).unwrap_or(0);
        let new_balance = balance + amount;
        env.storage().instance().set(&Symbol::short("BAL"), &new_balance);
    }
}