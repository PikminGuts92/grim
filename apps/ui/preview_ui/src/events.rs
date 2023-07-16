use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Event)]
pub enum AppEvent {
    Exit,
    SelectMiloEntry(Option<String>),
    ToggleGridLines(bool),
    ToggleWireframes(bool),
}

#[derive(Event)]
pub enum AppFileEvent {
    Open(PathBuf),
}