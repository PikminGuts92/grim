use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

type ObjectId = u32;

/*
    1) trait Object
    2) struct ObjectSource
        This could be generic? Hash map of properties/methods?
        How to handle sharing mixed mutable/read-only references...
    3) struct ObjectProxy (points to obj source)
*/

pub trait Object {
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String);

    fn get_type(&self) -> &String;
    fn set_type(&mut self, r#type: String);

    // props (DataArray)

    fn get_note(&self) -> &String;
    fn set_note(&mut self, note: String);
}

pub struct ObjectSource {
    name: String,
    r#type: String,
    note: String,
}

pub struct ObjectProxy {
    source: Arc<Mutex<Box<dyn Object>>>,
}

/*
    Use static methods outside structs?
        ex: object_get_name<'a>(obj: &'a dyn Object) -> &'a String
*/

impl Object for ObjectSource {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_type(&self) -> &String {
        &self.r#type
    }

    fn set_type(&mut self, r#type: String) {
        self.r#type = r#type;
    }

    fn get_note(&self) -> &String {
        &self.note
    }

    fn set_note(&mut self, note: String) {
        self.note = note;
    }
}

impl ObjectProxy { // dyn Object
    fn get_name<'a>(&'a self) -> &'a String {
        let s = self.source.as_ref().lock().unwrap().get_name();

        s

        //todo!()
    }
}
/*
impl Object for ObjectProxy {
    fn get_name(&self) -> &String {
        let s = self.source.lock().unwrap().get_name();

        s
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_type(&self) -> &String {
        &self.r#type
    }

    fn set_type(&mut self, r#type: String) {
        self.r#type = r#type;
    }

    fn get_note(&self) -> &String {
        &self.note
    }

    fn set_note(&mut self, note: String) {
        self.note = note;
    }
}*/

pub struct MiloContext {
    objects: HashMap<ObjectId, Box<dyn Object>>,
}