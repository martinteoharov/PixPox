#![allow(unused_imports)]
#![allow(dead_code)]

use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

use crate::{component::Components};

static MAX_WORLD_ID: AtomicUsize = AtomicUsize::new(0);

struct WorldId(usize);

impl WorldId {
    pub fn new() -> Option<Self> {
        MAX_WORLD_ID
            // We use `Relaxed` here since this atomic only needs to be consistent with itself
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |val| {
                val.checked_add(1)
            })
            .map(WorldId)
            .ok()
    }
}

pub struct World {
    id: WorldId,
    // pub(crate) entities: Entities,
    // pub(crate) components: Components,
    // pub(crate) archetypes: Archetypes,
    // pub(crate) change_tick: AtomicU32,
    // pub(crate) last_change_tick: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            id: WorldId::new()
                .expect("More PixPox worlds have been created than currently supported."),
        }
    }

    fn spawn_random_terrain() {}

    fn serialize() {}
}
