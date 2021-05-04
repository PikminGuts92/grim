use crate::ark::*;
use crate::io::*;
use std::path::Path;

impl Ark {
    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Ark, ArkReadError> {
        let path = path.as_ref();

        let stream = FileStream::from_path_as_read_open(path)
            .map_err(|e| ArkReadError::CantOpenArk)?;

        Ok(Ark::default())
    }
}