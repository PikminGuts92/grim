mod object;
mod object_dir;

use bevy_ecs::prelude::*;
use object::*;
use object_dir::*;

use pyo3::{prelude::*, types::PyType};

pub fn add_milo_classes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //m.add_class::<ObjectPython>()?;
    //m.add_class::<ObjectDirPython>()?;

    Ok(())
}

#[derive(Default)]
pub struct MiloEngine {
    world: World,
}

impl MiloEngine {
    fn create_object<T: Object>(&mut self) -> T {
        let mut obj = T::default();

        let id = self
            .world
            .spawn(obj.clone())
            .id();

        obj.set_id(id.index().index());

        todo!()
    }
}