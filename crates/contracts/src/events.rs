use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub fn initialized(e: &Env, admin: Address, fee_rate: u32) {
    let topics = (symbol_short!("init"), admin);
    e.events().publish(topics, fee_rate);
}

pub fn admin_changed(e: &Env, old_admin: Address, new_admin: Address) {
    let topics = (Symbol::new(e, "admin_changed"), old_admin);
    e.events().publish(topics, new_admin);
}

pub fn pool_registered(e: &Env, pool: Address) {
    e.events().publish((symbol_short!("reg_pool"),), pool);
}

pub fn paused(e: &Env) {
    e.events().publish((symbol_short!("paused"),), ());
}

pub fn unpaused(e: &Env) {
    e.events().publish((symbol_short!("unpaused"),), ());
}
