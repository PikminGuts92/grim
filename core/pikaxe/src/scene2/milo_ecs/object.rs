use bevy_ecs::prelude::*;
use pyo3::{prelude::*, types::PyType};

#[derive(Default, Clone, Component)]
pub struct ObjectComponent {
    pub id: u32,
    pub note: String,
}

pub trait Object : Default + Clone + Bundle {
    fn get_object_component(&self) -> &ObjectComponent;
    fn get_object_component_mut(&mut self) -> &mut ObjectComponent;

    fn get_id(&self) -> u32 {
        self.get_object_component().id
    }

    fn set_id(&mut self, value: u32) {
        self.get_object_component_mut().id = value;
    }

    fn get_note(&self) -> &String {
        &self.get_object_component().note
    }

    fn get_note_mut(&mut self) -> &mut String {
        &mut self.get_object_component_mut().note
    }

    fn set_note(&mut self, value: String) {
        self.get_object_component_mut().note = value;
    }
}

#[derive(Default, Clone, Bundle)]
//#[pyclass(name="Object", subclass)]
pub struct ObjectInstance {
    pub(crate) object: ObjectComponent,
}

impl Object for ObjectInstance {
    fn get_object_component(&self) -> &ObjectComponent {
        &self.object
    }

    fn get_object_component_mut(&mut self) -> &mut ObjectComponent {
        &mut self.object
    }
}

// Python bindings

#[derive(Default)]
#[pyclass(name="Object", subclass)]
pub struct ObjectPython {
    pub value: Option<ObjectInstance>,
}

#[pymethods]
impl ObjectPython {
    // TODO: Possibly only initialize from MiloEngine/ObjectDir create_object() method...
    #[new]
    fn new() -> Self {
        Self {
            value: Some(Default::default())
        }
    }

    #[getter]
    fn get_note(&self) -> String {
        self.value.as_ref().unwrap().object.note.to_owned()
    }

    #[setter]
    fn set_note(&mut self, value: String) {
        self.value.as_mut().unwrap().object.note = value;
    }
}