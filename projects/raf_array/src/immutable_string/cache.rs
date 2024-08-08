use std::{collections::HashMap, sync::{LazyLock, RwLock}};

use core::cell::UnsafeCell;

use super::{
    temporary_string::TemporaryString,
    weak_string::WeakString};

pub(super) struct Cache {
    inner_map: UnsafeCell<RwLock<HashMap<TemporaryString, WeakString>>>,
}

unsafe impl Sync for Cache { }

impl Default for Cache {
    fn default() -> Self {
        Self { inner_map: UnsafeCell::new(RwLock::new(HashMap::new())) }
    }
}

impl Cache {
    #[inline(always)]
    pub fn get(&self, key: &TemporaryString) -> Option<WeakString> {
        let read = self.map().read().unwrap();
        read.get(key).cloned()
    }

    #[inline(always)]
    pub fn set(&self, key: &TemporaryString, value: WeakString) {
        let mut write = self.map().write().unwrap();
        write.insert(key.clone(), value);
    }

    #[inline(always)]
    pub fn remove(&self, key: &TemporaryString) {
        let mut write = self.map().write().unwrap();
        write.remove(key);
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    fn map(&self) -> &mut RwLock<HashMap<TemporaryString, WeakString>> {
        unsafe { &mut *self.inner_map.get() }
    }
}

pub(super) static CACHE: LazyLock<Cache> = LazyLock::new(Cache::default);
