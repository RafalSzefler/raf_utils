use std::{collections::HashMap, hash::Hash};

pub(super) struct TwoWayMap<TKey, TValue>
    where TKey: Eq + Hash + Clone,
        TValue: Eq + Hash + Clone,
{
    forward: HashMap<TKey, TValue>,
    backward: HashMap<TValue, TKey>,
}

impl<TKey, TValue> TwoWayMap<TKey, TValue>
    where TKey: Eq + Hash + Clone,
        TValue: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            forward: HashMap::new(),
            backward: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: TKey, value: TValue) {
        self.forward.insert(key.clone(), value.clone());
        self.backward.insert(value, key);
    }

    pub fn get_by_key(&self, key: &TKey) -> Option<&TValue> {
        self.forward.get(key)
    }

    pub fn get_by_value(&self, value: &TValue) -> Option<&TKey> {
        self.backward.get(value)
    }

    pub fn remove_by_key(&mut self, key: &TKey) -> Option<TValue> {
        if let Some(value) = self.forward.remove(key) {
            self.backward.remove(&value);
            Some(value)
        } else {
            None
        }
    }

    pub fn remove_by_value(&mut self, value: &TValue) -> Option<TKey> {
        if let Some(key) = self.backward.remove(value) {
            self.forward.remove(&key);
            Some(key)
        } else {
            None
        }
    }
}
