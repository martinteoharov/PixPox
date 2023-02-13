use interner::{global::GlobalPool, Pooled};

use std::{
    any::Any,
    borrow::BorrowMut,
    cell::RefCell,
    collections::{hash_map::RandomState, HashMap},
    fmt::Debug,
};

use log::{debug, info};
pub static STRINGS: GlobalPool<String> = GlobalPool::new();

use crate::Texture;

pub enum BucketAction {
    GET,
    PUT,
}
pub struct Storage {
    pub buckets: HashMap<Pooled<&'static GlobalPool<String>, RandomState>, Box<dyn Any>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    pub fn query_storage<T: 'static>(
        &mut self,
        label: &'static str,
    ) -> Option<&mut T> {
        assert!(
            self.buckets.contains_key(&STRINGS.get(label)),
            "World::query_storage() didn't find an item you were looking for."
        );

        if let Some(data) = self.buckets.get_mut(&STRINGS.get(label)) {
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
            self.buckets.contains_key(&STRINGS.get(label)),
            "World::query_storage() didn't find an item you were looking for."
        );

        if let Some(data) = self.buckets.get_mut(&STRINGS.get(label)) {
            // debug!("Storage::query_storage() - label found");
            if let Some(downcasted) = data.downcast_mut::<T>() {
                // debug!("Storage::query_storage() - value downcasted successfully");
                return Some(downcasted);
            }
        }

        None
    }

    pub fn new_hashmap_bucket<K: 'static, V: 'static>(
        &mut self,
        label: &'static str,
        default: Option<HashMap<K, V>>,
    ) {
        let datastorage = Box::new(HashMap::<K, V>::new());

        match default {
            Some(map) => {
                self.buckets.insert(STRINGS.get(label), Box::new(map));
            },
            None => {
                self.buckets.insert(STRINGS.get(label), datastorage);
            },
        }
    }

    pub fn new_bucket<T: 'static>(
        &mut self,
        label: &'static str,
        data: T,
    ) {
        self.buckets.insert(STRINGS.get(label), Box::new(data));
    }
}
