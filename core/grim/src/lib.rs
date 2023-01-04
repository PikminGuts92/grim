#![allow(dead_code)]
#![allow(unused_imports)]

pub mod ark;
pub mod dta;
pub mod io;
#[cfg(feature = "midi")] pub mod midi;
#[cfg(feature = "model")] pub mod model;
pub mod scene;
mod system;
pub mod texture;

#[cfg(feature = "python")] use pyo3::prelude::*;
pub use grim_traits::*;
pub use system::*;

#[cfg_attr(feature = "python", pymodule)]
fn grim(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[cfg(feature = "model")] m.add_function(wrap_pyfunction!(model::print_test, m)?)?;
    Ok(())
}