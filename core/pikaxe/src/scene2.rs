pub(crate) mod milo;

use pyo3::{prelude::*, types::PyType};
use std::sync::{Arc, Mutex};

#[derive(Default)]
#[pyclass(subclass, dict, get_all, set_all, name="MiloObject")]
pub struct Object {
    pub name: String,
    pub r#type: String,
    pub note: String,
    //pub width: u32,
}

#[pymethods]
impl Object {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}

#[pyclass]
pub struct ObjectDir {
    //pub entries: Vec<>
}

// How to inherit from Object??

#[derive(Default)]
#[pyclass(extends=Object, dict, get_all, set_all)]
pub struct RndTex {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,

    pub index_f: f32,
    pub index: i32,

    pub ext_path: String,
    pub use_ext_path: bool,

    //pub bitmap: Option<Bitmap>
}

#[pymethods]
impl RndTex {
    #[new]
    fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(Object::default()).add_subclass(Self::default())
    }
    /*fn new() -> PyResult<Self> {
        /*
        Python::attach(|py| {
            let dict: Py<PyDict> = PyDict::new(py).unbind();
            Foo { inner: dict }
        })
        */

        //Self::default()

        Python::attach(|py| {
            //let dict: Py<PyDict> = PyDict::new(py).unbind();
            //Self::default()

            let tex: Py<Self> = Py::new(py, Self::default());
            //Ok(tex)

            todo!()
        })
    }*/

    fn compute_image_size(&self) -> PyResult<u32> {
        let size = (self.width * self.height * self.bpp) / 8;
        Ok(size)
    }

    /*#[staticmethod]
    fn create_new() -> PyResult<RndTex> {
        Ok(RndTex::default())
    }*/
}

pub trait ObjectTrait {
    fn get_name(&self) -> &str;
    fn set_name(&mut self, value: String);

    fn get_type(&self) -> &str;
    fn set_type(&mut self, value: String);

    fn get_note(&self) -> &str;
    fn set_note(&mut self, value: String);

    fn get_class_name(&self) -> &'static str;

    fn as_ref<T: ObjectTrait>(&self) -> Option<&T>;
}

pub trait RndTexTrait: ObjectTrait {
    fn get_width(&self) -> u32;
    fn set_width(&mut self, value: u32);

    fn get_height(&self) -> u32;
    fn set_height(&mut self, value: u32);

    fn get_bpp(&self) -> u32;
    fn set_bpp(&mut self, value: u32);
}

impl ObjectTrait for RndTex {
    fn get_name(&self) -> &str {
        todo!()
    }

    fn set_name(&mut self, value: String) {
        todo!()
    }

    fn get_type(&self) -> &str {
        todo!()
    }

    fn set_type(&mut self, value: String) {
        todo!()
    }

    fn get_note(&self) -> &str {
        todo!()
    }

    fn set_note(&mut self, value: String) {
        todo!()
    }

    fn get_class_name(&self) -> &'static str {
        "Tex"
    }

    fn as_ref<T: ObjectTrait>(&self) -> Option<&T> {

        todo!()
    }
}

impl RndTexTrait for RndTex {
    fn get_width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, value: u32) {
        self.width = value;
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn set_height(&mut self, value: u32) {
        self.height = value;
    }

    fn get_bpp(&self) -> u32 {
        self.bpp
    }

    fn set_bpp(&mut self, value: u32) {
        self.bpp = value;
    }
}


fn convert_object<T: ObjectTrait, S: ObjectTrait>(obj: &T) {


}

trait Object2 {
    type Items: std::fmt::Debug;

    /*fn get_item(&self) -> Option<Self::Items> {
        //Self::Items
        //Self::Items
        
        None
    }*/

    fn test(&self) {
        //let t = Self::Items::is_sorted(self);
    }
}

#[derive(Debug, Default)]
struct Object2Struct {
    name: String,
}

impl Object2 for Object2Struct {
    type Items = Self;
}

fn test() {
    let obj = Object2Struct::default();
    obj.test();
}

pub struct ObjectComponent {
    pub name: String,
}

pub struct TexComponent {
    pub width: u32,
}

pub trait Object2Trait {
    fn get_name(&self) -> &str;
    fn set_name(&mut self, value: String);
}

pub trait TexTrait: Object2Trait {
    fn get_width(&self) -> u32;
    fn set_width(&mut self, value: u32);
}

/*pub struct ObjectInstance {
    pub object: ObjectComponent,
}

impl Object2Trait for ObjectInstance {
    fn get_name(&self) -> &str {
        &self.object.name
    }

    fn set_name(&mut self, value: String) {
        self.object.name = value;
    }
}*/

//pub struct ObjectInstance(Arc<Mutex<dyn Object2Trait>>);

pub struct ObjectInstance {
    //pub object: Arc<Mutex<ObjectComponent>>,
    pub object: ObjectComponent,
}

impl Object2Trait for ObjectInstance {
    fn get_name(&self) -> &str {
        //&self.object.lock().unwrap().name
        &self.object.name
    }

    fn set_name(&mut self, value: String) {
        self.object.name = value;
    }
}