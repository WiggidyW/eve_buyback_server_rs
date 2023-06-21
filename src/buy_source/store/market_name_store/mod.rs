use super::env_store::EnvStore;

pub trait MarketNameStore: Send + Sync + 'static {
    fn get_market_name(&self, location: &u64) -> String;
}

pub fn new_market_name_store() -> impl MarketNameStore {
    return EnvStore::new(|i: &u64| format!("MARKET_{i}"));
}
