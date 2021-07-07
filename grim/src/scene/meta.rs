
pub trait Version {
    fn get_version(&self) -> Option<u32>;
    fn set_version(&mut self, version: Option<u32>);
}

pub struct Metadata {
    pub version: Option<u32>,
    pub revision: Option<u32>,
    pub r#type: String,
    // pub props: DataArray,
    pub note: String,
}