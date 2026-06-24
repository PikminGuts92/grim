mod object;
mod object_dir;
mod trans;

use bevy_ecs::{
    query::{QueryData, QueryFilter},
    prelude::*,
};
use object::*;
use object_dir::*;
use std::marker::PhantomData;
use trans::*;

use pyo3::{prelude::*, types::PyType};

#[derive(Default, Clone)]
pub struct ObjectNamedPointer { // TODO: Add generic type constraint?
    pub name: String,
    pub id: Option<u32>,
}

#[derive(Default, Clone)]
pub struct ObjectNamedPointerTyped<T: Object> {
    _marker: PhantomData<T>,
    pub name: String,
    pub id: Option<u32>,
}

impl<T: Object> ObjectNamedPointerTyped<T> {
    fn get_object(&self) -> T {
        todo!()
    }
}

pub fn add_milo_classes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //m.add_class::<ObjectPython>()?;
    //m.add_class::<ObjectDirPython>()?;

    Ok(())
}

#[derive(QueryData)]
struct ObjectQuery {
    entity: Entity,
    object: &'static ObjectComponent,
}

#[derive(QueryData)]
struct ObjectDirQuery {
    entity: Entity,
    object: &'static ObjectComponent,
    object_dir: &'static ObjectDirComponent,
}

#[derive(Default)]
pub struct MiloEngine {
    world: World,
}

impl MiloEngine {
    // TODO: Extract query behaviors to another struct
    fn create_object<T: Object>(&mut self) -> T {
        let obj_entity = self
            .world
            .spawn_empty()
            .id();

        let mut obj = T::default();
        obj.set_id(obj_entity.index_u32());

        self
            .world
            .commands()
            .entity(obj_entity)
            .insert(obj.clone());

        obj
    }

    fn get_object_by_id(&mut self, id: u32) -> Option<ObjectInstance> {
        let entity = Entity::from_raw_u32(id).expect("Id is valid");
        //let entity = self.world.commands().entity(entity);

        let mut query = self.world.query::<ObjectQuery>();
        let Ok(obj_data) = query.get(&self.world, entity) else {
            return None;
        };

        let obj_instance = ObjectInstance {
            object: obj_data.object.to_owned()
        };

        //self.world.query()
        /*for obj_data in self.world.query::<ObjectQuery>().iter(&self.world) {
            let obj_instance = ObjectInstance {
                object: obj_data.object.to_owned()
            };
        }*/

        Some(obj_instance)
    }
}