use super::{MarketNameStore, TokenStore};
use std::{
    collections::HashMap,
    env::{self, VarError},
    fmt::Debug,
    hash::Hash,
    sync::RwLock,
};

pub struct EnvStore<K> {
    store: RwLock<HashMap<K, String>>,
    env_key: fn(&K) -> String,
}

impl<K> EnvStore<K>
where
    K: Eq + Hash + Debug + Clone,
{
    pub fn new(env_key: fn(&K) -> String) -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
            env_key,
        }
    }

    // If self.store has key, return it.
    // Otherwise, get it from the environment using self.env_key and store it,
    // then return it.
    pub fn get(&self, key: &K) -> String {
        let store = self.store.read().unwrap();
        if let Some(value) = store.get(key) {
            value.clone()
        } else {
            drop(store);
            let mut store = self.store.write().unwrap();
            let env_key = (self.env_key)(key);
            let value = match env::var(&env_key) {
                Ok(value) => value,
                Err(VarError::NotPresent) => "".to_string(),
                Err(VarError::NotUnicode(e)) => panic!("{}: {:?}", env_key, e),
            };
            store.insert(key.clone(), value.clone());
            value
        }
    }
}

impl MarketNameStore for EnvStore<u64> {
    fn get_market_name(&self, location: &u64) -> String {
        self.get(location)
    }
}

impl TokenStore for EnvStore<u64> {
    fn get_token(&self, location: &u64) -> String {
        self.get(location)
    }
}
