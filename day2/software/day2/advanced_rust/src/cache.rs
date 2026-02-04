use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;


pub struct CacheEntry<V> {
    pub value : V,
    pub expire_at : Instant,

}

pub struct Cache<K, V> {
    pub entries : Arc<Mutex<HashMap<K, CacheEntry<V>>>>,
    pub default_ttl: Duration,

}

impl<K, V> Cache<K, V> where 
    K: std::hash::Hash + Eq + Clone,
    V: Clone
 {
    pub fn new(default_ttl: Duration) -> Self {
        Cache {
            entries: Arc::new(Mutex::new(HashMap::new())),
            default_ttl,
        }
    }
    
    pub fn insert(&self, key: K, value: V) {
        let entry = CacheEntry {
            value,
            expire_at: Instant::now() + self.default_ttl,
        };
        let mut entries = self.entries.lock().unwrap();
        entries.insert(key, entry);
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut entries = self.entries.lock().unwrap();
        if let Some(entry) = entries.get(key) {
            if Instant::now() > entry.expire_at {
                entries.remove(key);
                None
            } else {
                Some(entry.value.clone())
            }
        } else {
            None
        }
    }

    pub fn cleanup_expired(&self) {
        let mut entries = self.entries.lock().unwrap();
        let now = Instant::now();
        entries.retain(|_, entry| entry.expire_at > now);
    }

    pub fn start_cleanup_thread(&self, interval: Duration) {
        let entries = Arc::clone(&self.entries);
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(interval);
                let mut entries = entries.lock().unwrap();
                let now = Instant::now();
                entries.retain(|_, entry| entry.expire_at > now);
            }
        });

        
    }
}


