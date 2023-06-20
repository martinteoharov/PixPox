use lasso::{Spur, ThreadedRodeo};
use log::debug;

use std::{any::Any, collections::HashMap};

use crate::GlobalPixelMap as GlobalPixelMapTrait;
use crate::Texture;
pub use pixpox_utils::InputHandler;

pub enum BucketAction {
    GET,
    PUT,
}

/// # Storage
///
/// Storage is a data structure designed to store global data accessible to all components.
///
/// It introduces a query system to create and retrieve buckets through a simple-to-use API.
///
/// To optimize bucket lookup, Storage implements a multi-threaded interner that efficiently stores bucket
/// labels and associates them with Spurs. This interner facilitates fast and efficient hashmap lookup.
///
/// ## Example
///
/// ```
/// let mut storage = app.world.storage.write().unwrap();
///
/// let (width, height) = (cfg.window_width, cfg.window_height);
/// storage.new_bucket::<(u32, u32)>("grid-size", (width, height));
///
/// let (width, height) = storage
///     .query_storage::<HashMap<LogicalPosition<u32>, bool>>("grid-size")
///     .expect("Could not query storage: grid-size");

/// let mut (width, height) = storage
///     .query_storage_mut::<HashMap<LogicalPosition<u32>, bool>>("grid-size")
///     .expect("Could not query storage: grid-size");
///
/// ```
///
/// ## Panics
///
/// - If the bucket associated with the given label is not found, `query_storage()` and `query_storage_mut()`
/// will panic with an error message.
///
/// - If the multi-threaded interner fails to recognize a string, `query_storage()` and `query_storage_mut()`
/// will panic with an error message.
///
/// ## Safety
///
/// - The data stored in the `Storage` is not guaranteed to be thread-safe by default.
/// If you need thread-safety, you can use a `RwLock` or a `Mutex` to synchronize access to the `Storage`.

pub struct Storage {
    pub buckets: HashMap<Spur, Box<dyn Any + Send + Sync>>,
    interner: ThreadedRodeo,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
            interner: ThreadedRodeo::new(),
        }
    }

    pub fn query_storage<T: 'static>(&self, label: &'static str) -> Option<&T> {
        let key = &self
            .interner
            .get(label)
            .expect("String interner could not find a label");

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
        let key = &self
            .interner
            .get(label)
            .expect("String interner could not find a label");

        assert!(
            self.buckets.contains_key(key),
            "World::query_storage_mut() didn't find an item you were looking for."
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

    pub fn query_global_pixel_map<T: 'static + GlobalPixelMapTrait>(&mut self) -> Option<&mut dyn GlobalPixelMapTrait> {
        let key = &self
            .interner
            .get("pixelmap")
            .expect("String interner could not find a label");

        assert!(
            self.buckets.contains_key(key),
            "World::query_global_pixel_map() didn't find an item you were looking for."
        );

        if let Some(data) = self.buckets.get_mut(key) {
            if let Some(downcasted) = data.downcast_mut::<T>() {
                return Some(downcasted);
            }
        }

        None
    }

    pub fn update_global_pixel_map<T: 'static + GlobalPixelMapTrait>(&mut self, input: &InputHandler) {
        let key = &self
            .interner
            .get("pixelmap")
            .expect("String interner could not find a label");

        assert!(
            self.buckets.contains_key(key),
            "World::update_global_pixel_map() didn't find an item you were looking for."
        );

        if let Some(data) = self.buckets.get_mut(key) {
            if let Some(downcasted) = data.downcast_mut::<T>() {
                debug!("Updating global pixel map");
                debug!("Updating global pixel map");
                debug!("Updating global pixel map");
                debug!("Updating global pixel map");
                debug!("Updating global pixel map");
                debug!("Updating global pixel map");
                downcasted.update(input);
            }
        }
    }

    pub fn new_global_pixel_map<T: 'static + GlobalPixelMapTrait + Send + Sync>(
        &mut self,
        pixelmap: T,
    ) {
        let key = self.interner.get_or_intern("pixelmap");

        self.buckets.insert(key, Box::new(pixelmap));
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
