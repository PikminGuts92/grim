#[cfg(feature = "python")] use pyo3::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Default)]
#[cfg_attr(feature = "python", pyclass)]
pub struct Ark {
    #[cfg_attr(feature = "pyo3", pyo3(get, set))] pub version: i32,
    pub encryption: ArkEncryption,
    #[cfg_attr(feature = "pyo3", pyo3(get, set))] pub entries: Vec<ArkOffsetEntry>,
    pub path: PathBuf, // Hdr/ark path
}

#[derive(Debug)]
//#[cfg_attr(feature = "python", pyclass)]
pub enum ArkEncryption {
    None,
    ClassicEncryption(i32),
    NewEncryption(i32),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "python", pyclass)]
pub struct ArkOffsetEntry {
    #[cfg_attr(feature = "pyo3", pyo3(get, set))] pub id: u32,
    #[cfg_attr(feature = "pyo3", pyo3(get, set))] pub path: String,
    #[cfg_attr(feature = "pyo3", pyo3(get, set))] pub offset: u64,
    #[cfg_attr(feature = "pyo3", pyo3(get, set))] pub part: u32,
    #[cfg_attr(feature = "pyo3", pyo3(get, set))] pub size: usize,
    #[cfg_attr(feature = "pyo3", pyo3(get, set))] pub inflated_size: usize
}

impl Default for ArkEncryption {
    fn default() -> Self {
        ArkEncryption::None
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl Ark {
    #[staticmethod]
    fn from_file_path(path: &str) -> PyResult<Ark> {
        // TODO: Convert to PyResult
        let ark = Ark::from_path(path).unwrap();
        Ok(ark)
    }

    #[getter]
    fn get_encryption(&self) -> PyResult<Option<i32>> {
        let key = match self.encryption {
            ArkEncryption::ClassicEncryption(key) => Some(key),
            ArkEncryption::NewEncryption(key) => Some(key),
            _ => None
        };

        Ok(key)
    }
}