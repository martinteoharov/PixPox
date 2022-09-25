use bit_set::BitSet;
use queues::*;

const MAX_ENTITES: usize = std::u32::MAX as usize;

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub(crate) id: u32,
}

pub struct EntityManager {
    signatures: [BitSet; MAX_ENTITES], // array of signatures where index corresponds to entity id
    available_ids: Queue<usize>, // queue of available ids
    living_entity_count: usize, // number of living entities
}

impl EntityManager {
    pub fn new() -> Self {
        // TODO: read about zeroed & unsafe
        let mut signatures: [BitSet; MAX_ENTITES] = unsafe { std::mem::zeroed() };
        let mut available_ids = Queue::new();

        for i in 0..MAX_ENTITES {
            signatures[i] = BitSet::new();
            available_ids.add(i);
        }

        Self {
            signatures,
            available_ids,
            living_entity_count: 0,
        }
    }

    pub fn create(mut self) -> Entity {
        let id: u32 = self.available_ids.remove().unwrap() as u32;
        self.living_entity_count += 1;

        Entity { id }
    }

    pub fn destroy(mut self, entity: Entity) {
        self.signatures[entity.id as usize].clear();
        self.available_ids.add(entity.id as usize).unwrap();
        self.living_entity_count -= 1;
        
    }

    pub fn set_signature(mut self, entity: Entity, signature: BitSet) {
        self.signatures[entity.id as usize] = signature;

    }

    pub fn get_signature(self, entity: Entity) -> BitSet {
        self.signatures[entity.id as usize].clone()
    }
}