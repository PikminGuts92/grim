use bevy_ecs::prelude::*;
use pikaxe_derive::autotrait;
use pyo3::{prelude::*, types::PyType};
use super::{Object, ObjectComponent, ObjectPython};

#[derive(Default, Clone)]
pub struct ObjectEntry {
    pub name: String,
    pub id: Option<u32>,
}

#[derive(Default, Clone, Component)]
#[require(ObjectComponent)]
#[autotrait(extends=Object)]
pub struct ObjectDirComponent {
    pub entries: Vec<ObjectEntry>,
}

#[derive(Default, Clone, Bundle)]
pub struct ObjectDirInstance {
    pub(crate) object: ObjectComponent,
    pub(crate) object_dir: ObjectDirComponent,
}

impl Object for ObjectDirInstance {
    fn get_object_component(&self) -> &ObjectComponent {
        &self.object
    }

    fn get_object_component_mut(&mut self) -> &mut ObjectComponent {
        &mut self.object
    }
}

impl ObjectDir for ObjectDirInstance {
    fn get_object_dir_component(&self) -> &ObjectDirComponent {
        &self.object_dir
    }

    fn get_object_dir_component_mut(&mut self) -> &mut ObjectDirComponent {
        &mut self.object_dir
    }
}

// Python bindings

#[derive(Default)]
#[pyclass(name="ObjectDir", subclass, extends=ObjectPython)]
pub struct ObjectDirPython {
    pub value: Option<ObjectDirInstance>,
}

#[pymethods]
impl ObjectDirPython {
    #[new]
    fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(ObjectPython::default())
            .add_subclass(Self {
                value: Some(Default::default())
            })
    }

    #[getter]
    fn get_note(&self) -> String {
        self.value.as_ref().unwrap().object.note.to_owned()
    }

    #[setter]
    fn set_note(&mut self, value: String) {
        self.value.as_mut().unwrap().object.note = value;
    }

    /*#[getter]
    fn get_entries(&self) -> String {
        //self.value.as_ref().unwrap().object.note.to_owned()
    }

    #[setter]
    fn set_entries(&mut self, value: String) {
        //self.value.as_mut().unwrap().object.note = value;
    }*/
}