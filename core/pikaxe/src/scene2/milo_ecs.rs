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

/*
MiloFile -> ObjectDirId
MiloFile -> ObjectEntry[]
ObjectDir -> MiloFile[] (inline subdirs)
ObjectEntry -> ObjectId

MiloFile
- revision
- entries
- object_dir_entry
- file_path

Possibly give MiloFile its own "name" property because it doesn't always match object dir
*/

#[derive(Default, Clone)]
pub struct ObjectNamedPointer { // TODO: Add generic type constraint?
    pub name: String,
    pub id: Option<u32>,
}

#[derive(Default, Clone)]
pub struct ObjectPointer<T: Object> {
    _marker: PhantomData<T>,
    pub id: Option<u32>,
}

#[derive(Default, Clone)]
pub struct ObjectNamedPointerTyped<T: Sized> {
    _marker: PhantomData<T>,
    pub name: String,
    pub id: Option<u32>,
}

impl<T: Object> ObjectNamedPointerTyped<T> {
    fn get_object(&self) -> T {
        todo!()
    }
}

pub enum ObjectTyped {
    Object(Box<dyn Object>),
    Trans(Box<dyn Trans>),
    ObjectDir(Box<dyn ObjectDir>),
}

pub enum ObjectDirTyped {
    ObjectDir(Box<dyn ObjectDir>),
}

#[derive(Component, Clone)] // TODO: Implement default?
pub enum ObjectTypeDefinition {
    Object,
    Trans,
    ObjectDir
}

impl ObjectTypeDefinition {
    /*pub(crate) fn get_query(&self) {
        let query = match self {
            ObjectTypeDefinition::Object => QueryState::new(world)
        };
    }*/

    pub(crate) fn get_object_from_query(&self, world: &mut World, entity: Entity) {
        match self {
            ObjectTypeDefinition::Object => {
                let query = world.query::<ObjectDirQuery>();

            },
            _ => {
                todo!()
            }
        }
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
    type_definition: &'static ObjectTypeDefinition,
}

#[derive(QueryData)]
struct TransQuery {
    entity: Entity,
    object: &'static ObjectComponent,
    trans: &'static TransComponent,
}

#[derive(QueryData)]
struct ObjectDirQuery {
    entity: Entity,
    object: &'static ObjectComponent,
    object_dir: &'static ObjectDirComponent,
}

#[derive(Default, Clone)]
struct MiloFile {
    pub entries: Vec<ObjectNamedPointer>,
    //pub object: ObjectNamedPointerTyped<Box<dyn Object>>,
    pub dir_object: ObjectNamedPointer,
}

impl MiloFile {
    pub fn get_directory(&self, milo_engine: &mut MiloEngine) -> ObjectDirTyped {
        todo!()
    }
}

#[derive(Default)]
pub struct MiloEngine {
    world: World,
}

impl MiloEngine {
    // TODO: Extract query behaviors to another struct
    /*fn create_object<T: Object>(&mut self) -> T {
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
    }*/

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

    fn get_object_typed_by_id(&mut self, id: u32) -> Option<ObjectTyped> {
        let entity = Entity::from_raw_u32(id).expect("Id is valid");

        let mut obj_type_query = self.world.query::<&ObjectTypeDefinition>();
        let obj_type_definition = obj_type_query.get(&self.world, entity).ok()?;

        let obj_typed: ObjectTyped = match obj_type_definition {
            &ObjectTypeDefinition::Object => self
                .world.query::<ObjectQuery>()
                .get(&self.world, entity)
                .map(|obj| ObjectInstance {
                    object: obj.object.clone(),
                })
                .map(|t| t.into())
                .ok()?
            ,
            &ObjectTypeDefinition::Trans => self
                .world.query::<TransQuery>()
                .get(&self.world, entity)
                .map(|obj| TransInstance {
                    object: obj.object.clone(),
                    trans: obj.trans.clone(),
                })
                .map(|t| t.into())
                .ok()?
            ,
            _ => todo!()
        };

        Some(obj_typed)
    }

    fn get_object_dir_typed_by_id(&mut self, id: u32) -> Option<ObjectDirTyped> {
        todo!()
    }
}