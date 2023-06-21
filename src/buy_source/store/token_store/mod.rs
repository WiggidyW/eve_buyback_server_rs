use super::env_store::EnvStore;

pub trait TokenStore: Send + Sync + 'static {
    fn get_token(&self, location: &u64) -> String;
}

pub fn new_token_store() -> impl TokenStore {
    return EnvStore::new(|i: &u64| format!("TOKEN_{}", i));
}
