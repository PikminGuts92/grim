use super::{Object, ObjectComponent, ObjectPython};

#[derive(Default)]
pub struct ObjectDirComponent {
    pub r#type: String,
    // TODO: How to handle entries lifetimes...
}

#[derive(Default)]
pub struct ObjectDirInstance {
    object: ObjectComponent,
    object_dir: ObjectDirComponent,
}

impl Object for ObjectDirInstance {
    fn get_object_component(&self) -> &ObjectComponent {
        &self.object
    }

    fn get_object_component_mut(&mut self) -> &mut ObjectComponent {
        &mut self.object
    }
}

pub trait ObjectDir : Object {
    fn get_object_dir_component(&self) -> &ObjectDirComponent;
    fn get_object_dir_component_mut(&mut self) -> &mut ObjectDirComponent;

    fn get_type(&self) -> &String {
        &self.get_object_dir_component().r#type
    }

    fn get_type_mut(&mut self) -> &mut String {
        &mut self.get_object_dir_component_mut().r#type
    }

    fn set_type(&mut self, value: String) {
        self.get_object_dir_component_mut().r#type = value;
    }
}

// Python bindings
use pyo3::{prelude::*, types::PyType};


#[derive(Default)]
#[pyclass(name="ObjectDir", extends=ObjectPython)]
pub struct ObjectDirPython {
    //pub model: Py<PyAny>,
    pub self_python: Option<Py<Self>>,
    pub value: ObjectDirComponent,
}

#[pymethods]
impl ObjectDirPython {
    #[new]
    fn new(/*src: Py<PyAny>*/) -> PyClassInitializer<Self> /*PyResult<Py<Self>>*/ {
        //let init = PyClassInitializer::from(Self::default());
        //init

        let subclass = Self::default();
        /*let subclass = Self {
            //model: initializer,
            value: ObjectDirComponent::default()
        };*/

        let init = PyClassInitializer::from(ObjectPython::default()).add_subclass(subclass);
        
        /*let init = Python::attach(|py| {
            /*let mut obj = Py::new(py, init);

            if let Ok(o) = obj.as_mut() {
                let b = o.bind(py).clone();
            }

            obj*/

            let obj = Py::new(py, init).unwrap();
            let clone_obj = Py::clone_ref(&obj, py);

            //bb.
        });*/
        
        init
    }

    fn __init__(/*mut slf: PyRefMut<Self>*/ slf: PyClassGuardMut<'_, Self>) {
        /*let py = slf.py();
        let self_python: Py<Self> = unsafe {
            Py::from_borrowed_ptr(py, slf.as_ptr())
        };

        slf.self_python = Some(self_python);*/
    }

    #[getter]
    fn get_type(&self) -> PyResult<String> {
        //Ok(self.value.test.to_owned())
        todo!()
    }

    #[setter]
    fn set_type(&mut self, value: String) {
        //self.value.test = value;
        todo!()
    }
}

impl Object for ObjectDirPython {
    fn get_object_component(&self) -> &ObjectComponent {
        todo!()
    }

    fn get_object_component_mut(&mut self) -> &mut ObjectComponent {
        todo!()
    }
}