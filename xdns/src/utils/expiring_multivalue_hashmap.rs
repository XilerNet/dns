use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

pub struct ExpiringMultiValueHashMap<K, V> {
    map: HashMap<K, Vec<(V, Instant)>>,
}

impl<K, V> ExpiringMultiValueHashMap<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    pub fn new() -> Self {
        ExpiringMultiValueHashMap {
            map: HashMap::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.map.iter().map(|(_, v)| v.len()).sum()
    }

    pub fn insert(&mut self, key: K, value: V, ttl: Duration) {
        let expiration_time = Instant::now() + ttl;
        let entry = self.map.entry(key).or_insert_with(Vec::new);
        entry.push((value, expiration_time));
    }

    pub fn contains_key<T>(&self, key: T) -> bool
    where
        T: Into<K> + Clone,
    {
        self.map.contains_key(&key.clone().into()) && self.get(key).is_some()
    }

    pub fn get<T>(&self, key: T) -> Option<Vec<(V, Instant)>>
    where
        T: Into<K>,
    {
        let entries = self.map.get(&key.into())?;
        let now = Instant::now();

        let values: Vec<(V, Instant)> = entries
            .iter()
            .filter(|(_, expiration_time)| expiration_time > &now)
            .cloned()
            .collect();

        if values.is_empty() {
            return None;
        }

        Some(values)
    }

    pub async fn cleanup(&mut self) {
        let now = Instant::now();
        self.map.retain(|_, entries| {
            entries.retain(|(_, expiration_time)| expiration_time > &now);
            !entries.is_empty()
        });
    }
}
