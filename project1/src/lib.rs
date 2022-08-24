use std::collections::HashMap;

pub struct KvStore {
    store: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> Self {
        KvStore{
            store: HashMap::new()
        }
    }

    pub fn set(&mut self, k: String, v: String) {
        self.store.insert(k, v);
    }

    pub fn get(&self, k: String) -> Option<String> {
        self.store.get(&k).cloned()
    }

    pub fn remove(&mut self, k: String) {
        self.store.remove(&k);
    }
}