use std::path::PathBuf;

pub enum AppEvent {
    Exit,
    SelectMiloEntry(Option<String>),
    ToggleGridLines(bool),
    ToggleWireframes(bool),
}

pub enum AppFileEvent {
    Open(PathBuf),
}