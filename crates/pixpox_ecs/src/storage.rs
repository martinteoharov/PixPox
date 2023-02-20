use lasso::{ThreadedRodeo, Spur};
use string_interner::{symbol::SymbolU32, StringInterner};

use std::{
    any::Any,
    borrow::BorrowMut,
    cell::RefCell,
    collections::{hash_map::RandomState, HashMap},
    fmt::Debug,
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use log::{debug, info};

use crate::Texture;

pub enum BucketAction {
    GET,
    PUT,
}

pub struct Storage {
    pub buckets: HashMap<Spur, Box<dyn Any + Send + Sync>>,
    interner: ThreadedRodeo
}

impl Storage {
    pub fn new() -> Self {

        Self {
            buckets: HashMap::new(),
            interner: ThreadedRodeo::new()
        }
    }

    pub fn query_storage<T: 'static>(&self, label: &'static str) -> Option<&T> {
        let key = &self.interner.get(label).unwrap();

        assert!(
            self.buckets.contains_key(key),
            "World::query_storage() didn't find an item you were looking for."
        );

        if let Some(data) = self.buckets.get(key) {
            // debug!("Storage::query_storage() - label found");
            if let Some(downcasted) = data.downcast_ref::<T>() {
                // debug!("Storage::query_storage() - value downcasted successfully");
                return Some(downcasted);
            }
        }

        None
    }

    pub fn query_storage_mut<T: 'static>(&mut self, label: &'static str) -> Option<&mut T> {
        let key = &self.interner.get(label).unwrap();

        assert!(
            self.buckets.contains_key(key),
            "World::query_storage() didn't find an item you were looking for."
        );

        if let Some(data) = self.buckets.get_mut(key) {
            // debug!("Storage::query_storage() - label found");
            if let Some(downcasted) = data.downcast_mut::<T>() {
                // debug!("Storage::query_storage() - value downcasted successfully");
                return Some(downcasted);
            }
        }

        None
    }

    pub fn query_global_pixel_map<T: 'static + Texture>(
        &mut self,
        label: &'static str,
    ) -> Option<&mut T> {
        let key = &self.interner.get(label).unwrap();

        assert!(
            self.buckets.contains_key(key),
            "World::query_storage() didn't find an item you were looking for."
        );

        if let Some(data) = self.buckets.get_mut(key) {
            // debug!("Storage::query_storage() - label found");
            if let Some(downcasted) = data.downcast_mut::<T>() {
                // debug!("Storage::query_storage() - value downcasted successfully");
                return Some(downcasted);
            }
        }

        None
    }

    pub fn new_bucket<T: 'static + Send + Sync>(&mut self, label: &'static str, data: T) {
        let key = self.interner.get_or_intern(label);

        self.buckets.insert(key, Box::new(data));
    }

    pub fn new_hashmap_bucket<K: 'static + Send + Sync, V: 'static + Send + Sync>(
        &mut self,
        label: &'static str,
        default: Option<HashMap<K, V>>,
    ) {
        let key = self.interner.get_or_intern(label);
        let datastorage = Box::new(HashMap::<K, V>::new());

        match default {
            Some(map) => {
                self.buckets.insert(key, Box::new(map));
            },
            None => {
                self.buckets.insert(key, datastorage);
            },
        }
    }
}
