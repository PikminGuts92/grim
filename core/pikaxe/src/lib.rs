pub mod ark;
#[cfg(feature = "audio")] pub mod audio;
pub mod dta;
pub mod io;
pub(crate) mod math;
#[cfg(feature = "midi")] pub mod midi {
    pub use pikaxe_midi::*;
}
#[cfg(feature = "model")] pub mod model;
pub mod scene;
mod system;
pub mod texture;

#[cfg(feature = "python")] use pyo3::prelude::*;
pub use pikaxe_traits::*;
pub use system::*;

#[cfg(feature = "python")]
#[pymodule]
fn pikaxe(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ark::Ark>()?;
    m.add_class::<ark::ArkOffsetEntry>()?;
    m.add_class::<texture::Bitmap>()?;

    Ok(())
}