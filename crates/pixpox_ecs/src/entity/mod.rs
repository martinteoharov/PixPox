use bit_set::BitSet;
use pixpox_utils::box_array;
use queues::*;

// max entities should be the size of the largest possible u32 value
const MAX_ENTITIES: usize = 10_000_000;

use log::{debug, error, info, warn};

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub(crate) id: u32,
}

pub struct EntityManager {
    available_ids: Queue<usize>,    // queue of available ids
    pub living_entity_count: usize, // number of living entities
}

impl EntityManager {
    pub fn new() -> Self {
        let mut available_ids = Queue::new();

        for i in 0..MAX_ENTITIES {
            available_ids.add(i).unwrap();
        }

        Self {
            available_ids,
            living_entity_count: 0,
        }
    }

    pub fn create(&mut self) -> Entity {
        let id: u32 = self.available_ids.remove().unwrap() as u32;
        self.living_entity_count += 1;

        info!(
            "EntityManager::create() - id: {}, living_entity_count: {}",
            id, self.living_entity_count
        );

        Entity { id }
    }

    pub fn destroy(&mut self, entity: Entity) {
        self.available_ids.add(entity.id as usize).unwrap();
        self.living_entity_count -= 1;

        info!(
            "EntityManager::destroy() - id: {}, living_entity_count: {}",
            entity.id, self.living_entity_count
        );
    }
}
