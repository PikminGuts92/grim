#[derive(Default)]
pub struct ObjectComponent {
    pub name: String,
}

#[derive(Default)]
pub struct ObjectInstance {
    object: ObjectComponent,
}

// TODO: Use AsRef and AsMut instead?
pub struct ObjectInstanceRef<'a> {
    pub object: &'a mut ObjectComponent,
}

impl ObjectInstance {
    pub fn as_instance_ref(&mut self) -> ObjectInstanceRef<'_> {
        let ObjectInstance { object } = self;

        ObjectInstanceRef {
            object,
        }
    }
}

/*impl<'a> From<ObjectInstance> for ObjectInstanceRef<'a> {
    fn from(value: ObjectInstance) -> Self {
        ObjectInstanceRef {
            object: &mut value.object,
        }
    }
}*/

pub trait Object {
    fn get_object_component(&self) -> &ObjectComponent;
    fn get_object_component_mut(&mut self) -> &mut ObjectComponent;

    fn get_name(&self) -> &String {
        &self.get_object_component().name
    }

    fn get_name_mut(&mut self) -> &mut String {
        &mut self.get_object_component_mut().name
    }

    fn set_name(&mut self, value: String) {
        self.get_object_component_mut().name = value;
    }
}

impl Object for ObjectInstance {
    fn get_object_component(&self) -> &ObjectComponent {
        &self.object
    }

    fn get_object_component_mut(&mut self) -> &mut ObjectComponent {
        &mut self.object
    }
}

impl<'a> Object for ObjectInstanceRef<'a> {
    fn get_object_component(&self) -> &ObjectComponent {
        &self.object
    }

    fn get_object_component_mut(&mut self) -> &mut ObjectComponent {
        &mut self.object
    }
}

/*
#[derive(Default)]
pub struct ObjectInstanceGeneric<T>
where
    T: AsRef<ObjectComponent> + AsMut<ObjectComponent>,
{
    pub object: T,
}

impl<T> Object for ObjectInstanceGeneric<T>
where
    T: AsRef<ObjectComponent> + AsMut<ObjectComponent>,
{
    fn get_object_component(&self) -> &ObjectComponent {
        self.object.as_ref()
    }

    fn get_object_component_mut(&mut self) -> &mut ObjectComponent {
        self.object.as_mut()
    }
}*/

// Python bindings
use pyo3::{prelude::*, types::PyType};

#[derive(Default)]
#[pyclass(name="Object", subclass)]
pub struct ObjectPython {
    //pub python_self: Option<Py<Self>>,
    pub value: ObjectComponent,
}

#[pymethods]
impl ObjectPython {
    #[new]
    /*fn new() -> PyClassInitializer<Self> {
        let init = PyClassInitializer::from(ObjectPython::default());

        init
    }*/
    fn new(/*initializer: Py<Self>*/) -> Self {
        let init = PyClassInitializer::from(Self::default());
        //init

        Self::default()
    }

    fn __init__(slf: PyRef<'_, Self>) {
        //let mut slf_mut = slf.into_ptr();

        let refcnt = unsafe { pyo3::ffi::Py_REFCNT(slf.as_ptr()) };
        //let v = &slf;

        //let c = slf.clone();

        //let p = slf.as_ptr();
        //let a = p.cast::<PyRef<Self>>();

        //slf_mut
    }

    #[getter]
    fn get_name(&self) -> PyResult<String> {
        Ok(Object::get_name(self).to_owned())
    }

    #[setter]
    fn set_name(&mut self, value: String) {
        Object::set_name(self, value);
    }
}

impl Object for ObjectPython {
    fn get_object_component(&self) -> &ObjectComponent {
        &self.value
    }

    fn get_object_component_mut(&mut self) -> &mut ObjectComponent {
        &mut self.value
    }
}

fn test() {
    let mut obj = ObjectInstance::default();
    let name = obj.get_name();
    obj.set_name(String::from("lolz"));

    let mut obj = ObjectInstance::default();
    let obj_ref = obj.as_instance_ref();

    let name = obj_ref.get_name();
    obj.set_name(String::from("lolz"));
}