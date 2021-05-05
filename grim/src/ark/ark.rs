#[derive(Debug, Default)]
pub struct Ark {
    pub version: u32,
    pub encryption: ArkEncryption,
    pub entries: Vec<ArkOffsetEntry>,
    pub paths: Vec<String>,
}

#[derive(Debug)]
pub enum ArkEncryption {
    None,
    ClassicEncryption(u32),
    NewEncryption(u32),
}

#[derive(Debug)]
pub struct ArkOffsetEntry {
    pub id: u32,
    pub path: String,
    pub offset: u64,
    pub part: u32,
    pub size: usize,
    pub inflated_size: usize
}

impl Default for ArkEncryption {
    fn default() -> Self {
        ArkEncryption::None
    }
}