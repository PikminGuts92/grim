use std::path::PathBuf;

pub enum AppEvent {
    Exit,
    SelectMiloEntry(Option<String>),
    ToggleGridLines(bool),
}

pub enum AppFileEvent {
    Open(PathBuf),
}