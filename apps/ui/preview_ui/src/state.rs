use super::{AppSettings, AppEvent};
use bevy::prelude::*;
use grim::*;
use grim::ark::{Ark, ArkOffsetEntry};
use grim::scene::*;
use itertools::Itertools;
use std::{env::args, path::{Path, PathBuf}};

type ConsumeEventFn = fn(AppEvent);

#[derive(Default)]
pub struct MiloView {
    pub filter: String,
    pub class_filter: Option<String>,
    pub selected_entry: Option<String>,
}

#[derive(Default, Resource)]
pub struct AppState {
    pub ark: Option<Ark>,
    pub root: Option<ArkDirNode>,
    pub system_info: Option<SystemInfo>,
    pub milo: Option<ObjectDir>,
    pub settings_path: PathBuf,
    pub show_options: bool,
    pub pending_events: Vec<AppEvent>,
    pub side_bar_tab_index: usize,
    pub milo_view: MiloView,
    pub vert_count: usize,
    pub face_count: usize,
}

impl AppState {
    pub fn save_settings(&self, settings: &AppSettings) {
        settings.save_to_file(&self.settings_path);
        println!("Saved settings to \"{}\"", &self.settings_path.to_str().unwrap());
    }

    pub fn consume_events(&mut self, mut callback: impl FnMut(AppEvent)) {
        while !self.pending_events.is_empty() {
            callback(self.pending_events.remove(0));
        }
    }

    pub fn add_event(&mut self, ev: AppEvent) {
        self.pending_events.push(ev);
    }
}

#[derive(Debug)]
pub struct ArkDirNode {
    pub name: String,
    pub path: String,
    pub dirs: Vec<ArkDirNode>,
    pub files: Vec<usize>,
    pub loaded: bool,
}

impl ArkDirNode {
    pub fn expand(&mut self, ark: &Ark) {
        if self.loaded {
            return;
        }

        let (mut dirs, mut files) = get_dirs_and_files(&self.path, ark);
        self.dirs.append(&mut dirs);
        self.files.append(&mut files);
        self.loaded = true;

        // TODO: Rely on lazy load
        for c in &mut self.dirs {
            c.expand(ark);
        }
    }
}

fn get_dirs_and_files(dir: &str, ark: &Ark) -> (Vec<ArkDirNode>, Vec<usize>) {
    let is_root = match dir {
        "" | "." => true,
        _ => false,
    };

    if is_root {
        let files = ark.entries
            .iter()
            .enumerate()
            .filter(|(_i, e)| !e.path.contains('/')
                || (e.path.starts_with("./") && e.path.matches(|c: char | c.eq(&'/')).count() == 1))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let dirs = ark.entries
            .iter()
            .filter(|e| e.path.contains('/'))
            .map(|e| e.path.split('/').next().unwrap())
            .unique()
            .filter(|s| !s.eq(&"."))
            .map(|s| ArkDirNode {
                name: s.to_owned(),
                path: s.to_owned(),
                dirs: Vec::new(),
                files: Vec::new(),
                loaded: false,
            })
            .collect::<Vec<ArkDirNode>>();

        return (dirs, files);
    }

    let dir_path = format!["{}/", dir];
    let slash_count = dir_path.matches(|c: char| c.eq(&'/')).count();

    let files = ark.entries
        .iter()
        .enumerate()
        .filter(|(_i, e)| e.path.starts_with(&dir_path)
            && e.path.matches(|c: char| c.eq(&'/')).count() == slash_count)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let dirs = ark.entries
        .iter()
        .filter(|e| e.path.starts_with(&dir_path)
            && e.path.matches(|c: char| c.eq(&'/')).count() > slash_count)
        .map(|e| e.path.split('/').nth(slash_count).unwrap())
        .unique()
        .map(|s| ArkDirNode {
            name: s.to_owned(),
            path: format!("{}{}", dir_path, s),
            dirs: Vec::new(),
            files: Vec::new(),
            loaded: false,
        })
        .collect::<Vec<ArkDirNode>>();

    (dirs, files)
}