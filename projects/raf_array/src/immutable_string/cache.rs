use std::{collections::HashMap, sync::{LazyLock, RwLock}};

use crate::atomic_array::WeakArray;

use super::temporary_string::TemporaryString;

pub(super) struct Cache {
    inner_map: RwLock<HashMap<TemporaryString, WeakArray<u8>>>,
}

unsafe impl Sync for Cache { }

impl Default for Cache {
    fn default() -> Self {
        Self { inner_map: RwLock::new(HashMap::new()) }
    }
}

impl Cache {
    #[inline(always)]
    pub fn get(&self, key: &TemporaryString) -> Option<WeakArray<u8>> {
        let read = self.inner_map.read().unwrap();
        read.get(key).cloned()
    }

    #[inline(always)]
    pub fn set(&self, key: &TemporaryString, value: WeakArray<u8>) {
        let mut write = self.inner_map.write().unwrap();
        write.insert(key.clone(), value);
    }

    #[inline(always)]
    pub fn remove(&self, key: &TemporaryString) {
        let mut write = self.inner_map.write().unwrap();
        write.remove(key);
    }
}

pub(super) static CACHE: LazyLock<Cache> = LazyLock::new(Cache::default);
