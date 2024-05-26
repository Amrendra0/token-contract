use soroban_sdk::{contractimpl, symbol_short, Address, Env, Symbol};
use std::collections::HashMap;

pub struct TokenContract;

#[derive(Default)]
pub struct DataKey;

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: Env, admin: Address) {
        let data = DataKey::default();
        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.storage().instance().set(&symbol_short!("balances"), &HashMap::<Address, i128>::new());
        env.storage().instance().set(&symbol_short!("frozen"), &HashMap::<Address, bool>::new());
    }

    pub fn freeze_account(env: Env, account: Address) {
        let mut frozen: HashMap<Address, bool> = env.storage().instance().get(&symbol_short!("frozen")).unwrap();
        frozen.insert(account, true);
        env.storage().instance().set(&symbol_short!("frozen"), &frozen);
    }

    pub fn unfreeze_account(env: Env, account: Address) {
        let mut frozen: HashMap<Address, bool> = env.storage().instance().get(&symbol_short!("frozen")).unwrap();
        frozen.remove(&account);
        env.storage().instance().set(&symbol_short!("frozen"), &frozen);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        let mut balances: HashMap<Address, i128> = env.storage().instance().get(&symbol_short!("balances")).unwrap();
        let frozen: HashMap<Address, bool> = env.storage().instance().get(&symbol_short!("frozen")).unwrap();

        if *frozen.get(&from).unwrap_or(&false) {
            panic!("Account is frozen");
        }

        let from_balance = balances.get(&from).copied().unwrap_or(0);
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance = balances.get(&to).copied().unwrap_or(0);
        balances.insert(from.clone(), from_balance - amount);
        balances.insert(to.clone(), to_balance + amount);

        env.storage().instance().set(&symbol_short!("balances"), &balances);
    }
}
