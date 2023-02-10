use std::{any::Any, borrow::BorrowMut, cell::RefCell, collections::HashMap};

use log::info;

pub enum BucketAction {
    GET,
    PUT,
}
pub struct Storage {
    pub buckets: HashMap<&'static str, RefCell<Box<dyn Any>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    pub fn query_storage<T: 'static>(&mut self, label: &str) -> Option<&mut Box<T>> {
        assert!(
            self.buckets.contains_key(label),
            "World::query_storage() didn't find an item you were looking for."
        );

        match self.buckets.get_mut(label) {
            Some(data) => { 
                let mut kur = data.borrow_mut();
                kur.borrow().downcast_mut::<Box<T>>()
            },
            None => None,
        }
    }

    pub fn query_bucket<T: 'static>(
        &mut self,
        label: &str,
        action: BucketAction,
        index: Option<u64>,
    ) {
    }

    pub fn new_hashmap_bucket<K: 'static, V: 'static>(
        &mut self,
        label: &'static str,
        default: Option<HashMap<K, V>>,
    ) {
        let datastorage = RefCell::new(Box::new(HashMap::<K, V>::new()));

        match default {
            Some(map) => {
                self.buckets.insert(label, RefCell::new(Box::new(map)));
            },
            None => {
                self.buckets.insert(label, datastorage);
            },
        }
    }

    pub fn new_bucket<T: 'static>(&mut self, label: &'static str, data: T) {
        self.buckets.insert(label, RefCell::new(Box::new(data)));
    }
}
