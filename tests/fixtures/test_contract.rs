#[contract]
pub struct VulnerableVault;

#[contractimpl]
impl VulnerableVault {
    // 🚨 Vulnerable: Public function modifies storage but lacks `require_auth()`
    pub fn deposit(env: Env, amount: i128) {
        let mut balance: i128 = env.storage().instance().get(&Symbol::new(&env, "bal")).unwrap_or(0);
        balance += amount;
        env.storage().instance().set(&Symbol::new(&env, "bal"), &balance);
    }

    // ✅ Secure: Includes `require_auth()` check
    pub fn withdraw(env: Env, amount: i128) {
        let user = env.storage().instance().get(&Symbol::new(&env, "owner")).unwrap();
        user.require_auth();
        
        let mut balance: i128 = env.storage().instance().get(&Symbol::new(&env, "bal")).unwrap_or(0);
        balance -= amount;
        env.storage().instance().set(&Symbol::new(&env, "bal"), &balance);
    }
}