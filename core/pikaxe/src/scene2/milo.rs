mod object;
mod object_dir;

use object::*;
use object_dir::*;
use pyo3::{prelude::*, types::PyType};

pub fn add_milo_classes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ObjectPython>()?;
    m.add_class::<ObjectDirPython>()?;

    Ok(())
}