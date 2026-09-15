# Soroban Guard: Security Rule Catalog

This document details the security checks enforced by `soroban-guard` (`SG001` through `SG007`) for Soroban smart contracts.

---

## `SG001`: Missing Authentication Check

* **Severity:** Critical
* **Target:** Public `#[contractimpl]` functions modifying storage or executing privileged logic.
* **Description:** Public contract functions that mutate storage or transfer assets without invoking `require_auth()` allow unauthenticated accounts to invoke state changes.
* **Remediation:** Enforce address authentication using `address.require_auth()`.

```rust
// ❌ Vulnerable
pub fn deposit(env: Env, user: Address, amount: i128) {
    let mut balance: i128 = env.storage().instance().get(&user).unwrap_or(0);
    balance += amount;
    env.storage().instance().set(&user, &balance);
}

// ✅ Remediation
pub fn deposit(env: Env, user: Address, amount: i128) {
    user.require_auth();
    let mut balance: i128 = env.storage().instance().get(&user).unwrap_or(0);
    balance = balance.checked_add(amount).expect("overflow");
    env.storage().instance().set(&user, &balance);
}