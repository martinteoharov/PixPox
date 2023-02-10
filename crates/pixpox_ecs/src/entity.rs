use std::time::Instant;

// max entities should be the size of the largest possible u32 value
// const MAX_ENTITIES: usize = std::usize::MAX;

use log::{debug, error, info, warn};

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub id: usize,
}

pub struct EntityManager {
    pub id_counter: usize,
    pub living_entity_count: usize, // number of living entities
}

impl EntityManager {
    pub fn new() -> Self {

        Self {
            id_counter: 0,
            living_entity_count: 0,
        }
    }

    pub fn create(&mut self) -> Entity {
        let now = Instant::now();

        let id = self.id_counter;
        self.id_counter += 1;
        self.living_entity_count += 1;

        debug!(
            "EntityManager::create() - id: {}, living_entity_count: {}, in {} micros",
            id, self.living_entity_count,
            now.elapsed().as_micros().to_string()
        );

        Entity { id }
    }

    pub fn destroy(&mut self, entity: Entity) {
        self.living_entity_count -= 1;

        debug!(
            "EntityManager::destroy() - id: {}, living_entity_count: {}",
            entity.id, self.living_entity_count
        );
    }
}
