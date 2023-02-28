#![allow(unused_imports)]
#![allow(dead_code)]

use core::panic;
use std::{
    any::{self, Any},
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, AtomicUsize, Ordering},
        Arc, Mutex, RwLock,
    },
    thread,
    time::{self, Duration, Instant},
};

use log::{debug, error, info};
use pixpox_utils::stats::Stats;
use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{
    component::{self},
    entity::{Entity, EntityManager},
    Label, Run, Storage, Texture, Update,
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

pub enum BucketAction {
    GET,
    PUT,
}

// TODO: Add a field for tick speed
pub struct World {
    id: WorldId,
    pub entities: EntityManager,
    pub component_vecs: Vec<Box<dyn ComponentVec + Send>>,
    pub storage: RwLock<Storage>,
    pub last_update: time::Instant,
    pub stats: Stats,
    pub input: WinitInputHelper,
    paused: bool,
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
            last_update: time::Instant::now(),
            storage: RwLock::new(Storage::new()),
            stats: Stats::new(),
            input: WinitInputHelper::new(),
            paused: false,
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.new_entity()
    }

    pub fn add_component_to_entity<
        ComponentType: 'static + Label + Run + Update + Clone + Send + Sync,
    >(
        &mut self,
        entity: Entity,
        mut component: ComponentType,
    ) {
        let now = Instant::now();
        // TODO: use a hashmap for this shit
        // Search for any existing ComponentVecs that match the type of the component being added.
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity.id as usize] = Some(component);

                debug!(
                    "World::add_component_to_entity() existing component_vec: {} micros",
                    now.elapsed().as_micros().to_string()
                );

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

        let label = component.label();
        // Give this Entity the Component.
        new_component_vec[entity.id as usize] = Some(component);
        self.component_vecs.push(Box::new(new_component_vec));

        debug!(
            "World::add_component_to_entity() - Added component: {} to entity: {} in {} micros",
            label,
            entity.id,
            now.elapsed().as_micros().to_string()
        );
    }

    pub fn query_components<T: 'static>(&mut self, entities: Vec<&Entity>) -> Option<Vec<&T>> {
        let now = Instant::now();

        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<Vec<Option<T>>>()
            {
                debug!(
                    "World::query_components() in {}",
                    now.elapsed().as_micros().to_string()
                );

                let res = entities
                    .iter()
                    .filter_map(|entity| {
                        component_vec
                            .get(entity.id)
                            .expect("Entity could not be found in vec")
                            .as_ref()
                    })
                    .collect::<Vec<&T>>();

                return Some(res);
            }
        }

        return None;
    }

    pub fn query_components_for_render<T: 'static + Texture>(&mut self) -> Option<Vec<&T>> {
        let now = Instant::now();

        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<Vec<Option<T>>>()
            {
                debug!(
                    "World::query_entities_for_render() in {}",
                    now.elapsed().as_micros().to_string()
                );

                let res = component_vec
                    .iter_mut()
                    .filter_map(|x| x.as_ref())
                    .collect::<Vec<&T>>();

                return Some(res);
            }
        }

        return None;
    }
    pub fn toggle_paused(&mut self) {
        self.paused = !self.paused;
    }

    pub fn handle_input(&mut self) {
        /*
        if self.input.key_pressed(VirtualKeyCode::P) {
            info!("Toggled world");
            self.toggle_paused();
        }
         */
    }

    pub fn run(&mut self) {
        self.stats.new_tick();

        if self.paused {
            return;
        }

        self.handle_input();

        let now = Instant::now();
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.run_all(&mut self.storage);
        }
        let elapsed = Instant::now() - now;
        self.stats.update_sector("run()", elapsed.as_secs_f32());

        let now = Instant::now();
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.update_all(&mut self.storage, &mut self.input);
        }
        let elapsed = Instant::now() - now;
        self.stats.update_sector("update()", elapsed.as_secs_f32());
    }

    fn new_entity(&mut self) -> Entity {
        let entity = self.entities.create();
        let now = Instant::now();

        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }

        debug!(
            "World::new_entity(): {} micros",
            now.elapsed().as_micros().to_string()
        );

        return entity;
    }

    fn spawn_random_terrain() {}

    fn serialize() {}
}

pub trait ComponentVec: Send + Sync {
    fn as_any(&self) -> &(dyn std::any::Any + Send + Sync);
    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + Send + Sync);
    fn push_none(&mut self);
    fn run_all(&mut self, storage: &RwLock<Storage>);
    fn update_all(&mut self, storage: &mut RwLock<Storage>, input: &mut WinitInputHelper);
}

impl<T: 'static + Run + Update + Send + Sync> ComponentVec for Vec<Option<T>> {
    fn as_any(&self) -> &(dyn std::any::Any + Send + Sync) {
        self as &(dyn std::any::Any + Send + Sync)
    }

    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + Send + Sync) {
        self as &mut (dyn std::any::Any + Send + Sync)
    }

    fn push_none(&mut self) {
        self.push(None)
    }

    fn run_all(&mut self, storage: &RwLock<Storage>) {
        self.par_iter_mut().for_each(|component| {
            if let Some(c) = component {
                c.run(&storage.read().unwrap());
            }
        })
    }

    fn update_all(&mut self, storage: &mut RwLock<Storage>, input: &mut WinitInputHelper) {
        self.par_iter_mut().for_each(|component| {
            if let Some(c) = component {
                c.update(&storage, input);
            }
        })
    }
}
