#![allow(unused_imports)]
#![allow(dead_code)]

use std::{
    any::{self, Any},
    cell::{RefCell, RefMut},
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
    time::{self, Duration, Instant},
};

use log::{info, debug};

use crate::{
    components::{self, BaseComponent},
    entity::{Entity, EntityManager},
    Label, Run,
};

use ticktock::{Clock, Timer};

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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub struct World {
    id: WorldId,
    pub entities: EntityManager,
    pub component_vecs: Vec<Box<dyn ComponentVec>>,
    pub change_tick: time::Instant,
    pub last_change_tick: time::Instant,
}

impl World {
    pub fn new() -> Self {
        let entities = EntityManager::new();
        let component_vecs = Vec::new();

        // show some fps measurements every 5 seconds
        let mut fps = Timer::apply(|delta_t, prev_tick| (delta_t, *prev_tick), 0)
            .every(time::Duration::from_secs(5))
            .start(time::Instant::now());
        
        print_type_of(&fps);

        Self {
            id: WorldId::new()
                .expect("More PixPox worlds have been created than currently supported."),
            entities,
            component_vecs,
            change_tick: time::Instant::now(),
            last_change_tick: time::Instant::now(),
        }
    }

    pub fn add_component_to_entity<ComponentType: 'static + Label + Run + Copy>(
        &mut self,
        entity: Entity,
        mut component: ComponentType,
    ) {
        // Search for any existing ComponentVecs that match the type of the component being added.
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity.id as usize] = Some(component);
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
            "World::add_component_to_entity() - Added component: {} to entity: {}",
            component.label(),
            entity.id
        );
    }

    // pub fn borrow_component_vec_mut<ComponentType: 'static + Label + Run>(
    //     &self,
    // ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
    //     for component_vec in self.component_vecs.iter() {
    //         if let Some(component_vec) = component_vec
    //             .as_any()
    //             .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
    //         {
    //             return Some(component_vec.borrow_mut());
    //         }
    //     }
    //     None
    // }

    pub fn run(&mut self) {
        let now = Instant::now();

        for component_vec in self.component_vecs.iter_mut() {
            component_vec.run_all();
        }

        info!("Run all components: {} micros", now.elapsed().as_micros().to_string());
    }

    pub fn spawn_random_terrain() {}

    pub fn serialize() {}
}

pub trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
    fn run_all(&mut self);
}

impl<T: 'static + Run> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }

    fn run_all(&mut self) {
        for component in self.borrow_mut().iter_mut() {
            if let Some(c) = component {
                c.run();
            }
        }
    }
}
