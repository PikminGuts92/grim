use std::path::PathBuf;

pub enum AppEvent {
    Exit,
    SelectMiloEntry(Option<String>),
}

pub enum AppFileEvent {
    Open(PathBuf),
}