use string_interner::{symbol::SymbolU32, StringInterner};

use std::{
    any::Any,
    borrow::BorrowMut,
    cell::RefCell,
    collections::{hash_map::RandomState, HashMap},
    fmt::Debug, sync::{Mutex, Arc}, ops::DerefMut,
};

use log::{debug, info};

use crate::Texture;


pub enum BucketAction {
    GET,
    PUT,
}

pub struct Storage {
    pub buckets: HashMap<&'static str, Box<dyn Any + Send + Sync>>,
    interner: StringInterner
}


impl Storage {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
            interner: StringInterner::new()
        }
    }

    pub fn query_storage<T: 'static>(
        &self,
        label: &'static str,
    ) -> Option<&T> {
        assert!(
            self.buckets.contains_key(&label),
            "World::query_storage() didn't find an item you were looking for."
        );

        if let Some(data) = self.buckets.get(&label) {
            // debug!("Storage::query_storage() - label found");
            if let Some(downcasted) = data.downcast_ref::<T>() {
                // debug!("Storage::query_storage() - value downcasted successfully");
                return Some(downcasted);
            }
        }

        None
    }

    pub fn query_storage_mut<T: 'static>(
        &mut self,
        label: &'static str,
    ) -> Option<&mut T> {
        assert!(
            self.buckets.contains_key(&label),
            "World::query_storage() didn't find an item you were looking for."
        );

        if let Some(data) = self.buckets.get_mut(&label) {
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
        assert!(
            self.buckets.contains_key(&label),
            "World::query_storage() didn't find an item you were looking for."
        );

        if let Some(data) = self.buckets.get_mut(&label) {
            // debug!("Storage::query_storage() - label found");
            if let Some(downcasted) = data.downcast_mut::<T>() {
                // debug!("Storage::query_storage() - value downcasted successfully");
                return Some(downcasted);
            }
        }

        None
    }

    pub fn new_bucket<T: 'static + Send + Sync>(
        &mut self,
        label: &'static str,
        data: T,
    ) {
        self.buckets.insert(label, Box::new(data));
    }

    pub fn new_hashmap_bucket<K: 'static + Send + Sync, V: 'static + Send + Sync>(
        &mut self,
        label: &'static str,
        default: Option<HashMap<K, V>>,
    ) {
        let datastorage = Box::new(HashMap::<K, V>::new());

        match default {
            Some(map) => {
                self.buckets.insert(label, Box::new(map));
            },
            None => {
                self.buckets.insert(label, datastorage);
            },
        }
    }
}
