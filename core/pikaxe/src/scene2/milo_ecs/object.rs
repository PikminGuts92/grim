use bevy_ecs::prelude::*;
use pikaxe_derive::autotrait;
use pyo3::{prelude::*, types::PyType};

#[derive(Default, Clone, Component)]
//#[autotrait(extends=Default + Clone + Bundle)]
pub struct ObjectComponent {
    pub id: u32,
    pub note: String,
}

pub trait Object : Default + Clone + Bundle {
    fn get_class_name() -> &'static str;
    //fn get_super_classes() -> &'static [&'static str];

    fn get_object_component(&self) -> &ObjectComponent;
    fn get_object_component_mut(&mut self) -> &mut ObjectComponent;

    /*fn is_derived_from(&self, class_name: &str) -> bool {
        let self_class_name = Self::get_class_name();
        if self_class_name.eq(class_name) {
            return true;
        }

        // How to check supers of supers?
        //let super_class_names = Self::get_super_classes();

        false
    }*/

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
    fn get_class_name() -> &'static str {
        "Object"
    }

    /*fn get_super_classes() -> &'static [&'static str] {
        &[]
    }*/

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