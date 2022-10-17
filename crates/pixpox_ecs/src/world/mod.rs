#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    cell::{RefCell, RefMut},
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
};

use log::info;

use crate::entity::{Entity, EntityManager};

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
    pub entities: EntityManager,
    pub component_vecs: Vec<Box<dyn ComponentVec>>,
    pub change_tick: AtomicU32,
    pub last_change_tick: u32,
}

impl World {
    pub fn new() -> Self {
        let entities = EntityManager::new();
        let component_vecs = Vec::new();

        Self {
            id: WorldId::new()
                .expect("More PixPox worlds have been created than currently supported."),
            entities,
            component_vecs,
            change_tick: AtomicU32::new(0),
            last_change_tick: 0,
        }
    }

    pub fn add_component_to_entity<ComponentType: 'static + Label + Run>(
        &mut self,
        entity: Entity,
        component: ComponentType,
    ) {
        // Search for any existing ComponentVecs that match the type of the component being added.
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                component_vec.borrow_mut()[entity.id as usize] = Some(component);
                return;
            }
        }

        // No matching component storage exists yet, so we have to make one.
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities.living_entity_count);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.entities.living_entity_count {
            new_component_vec.push(None);
        }

        // Give this Entity the Component.
        new_component_vec[entity.id as usize] = Some(component);
        self.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));

        info!(
            "World::add_component_to_entity() - Added component to entity: {}",
            entity.id
        );
    }

    pub fn borrow_component_vec_mut<ComponentType: 'static + Label + Run>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    pub fn run(&mut self) {
    }

    pub fn spawn_random_terrain() {}

    pub fn serialize() {}
}

pub trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}

// Base traits that all components must have
pub trait Label {
    fn label() -> &'static str;
}

pub trait Run {
    fn run();
}
