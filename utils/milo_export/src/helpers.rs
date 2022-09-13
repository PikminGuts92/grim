use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{Object, ObjectDir, ObjectDirBase, PackedObject, PropAnim, PropKeysEvents, Tex, AnimRate};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::{PathBuf, Path};

#[derive(Default)]
pub struct GameAnalyzer {
    pub game_dir: PathBuf,
    pub cams: Cams,
    pub post_procs: Vec<String>
}

impl GameAnalyzer {
    pub fn new(game_path: PathBuf) -> GameAnalyzer {
        GameAnalyzer {
            game_dir: game_path,
            ..Default::default()
         }
    }

    pub fn process(&mut self) {
        let songs = self
            .game_dir
            .join("songs")
            .read_dir()
            .unwrap()
            .map(|d| d.unwrap().path())
            .filter(|d| d.is_dir())
            .map(|p| p.file_name().and_then(|f| f.to_str()).map(|s| s.to_string()).unwrap())
            .filter(|s| s.ne("gen"))
            .collect::<Vec<_>>();

        println!("Found {} songs", songs.len());

        let venue_names = [
            "americanidiot",
            "dookie",
            "twentyfirst"
        ];

        for venue_name in venue_names.iter() {
            let milo_file_name = format!("{venue_name}.milo");

            let venue_path = self.game_dir.join("world").join(venue_name).join(&milo_file_name);
            let venue_milo_dir = try_open_milo(venue_path.as_path());

            if venue_milo_dir.is_err() {
                continue;
            }

            let (_, venue_milo_dir) = venue_milo_dir.unwrap();
            //venue_milo_dir.unpack_entries(&system_info).unwrap();

            let entry_count = venue_milo_dir.get_entries().len();
            println!("Found {entry_count} entries");

            let cams = get_names_for_type_from_dir(&venue_milo_dir, "BandCamShot");

            self.cams.venues.push(StringValues {
                id: venue_name.to_string(),
                values: cams
            })
        }

        for song_name in songs.iter() {
            let song_dir = self.game_dir
                .join("songs")
                .join(song_name);

            // Get cams
            let milo_cams_file_name = format!("{song_name}_cams.milo");
            let cams_path = song_dir.join(&milo_cams_file_name);
            if let Ok((_, cams_milo_dir)) = try_open_milo(cams_path.as_path()) {
                let cams = get_names_for_type_from_dir(&cams_milo_dir, "BandCamShot");

                if cams.is_empty() {
                    continue;
                }

                self.cams.songs.push(StringValues {
                    id: song_name.to_string(),
                    values: cams
                })
            }
        }

        // Read post procs
        let post_procs_path = self.game_dir
            .join("world")
            .join("shared")
            .join("camera.milo");

        if let Ok((_, post_procs_dir)) = try_open_milo(post_procs_path.as_path()) {
            let mut post_procs = get_names_for_type_from_dir(&post_procs_dir, "PostProc");

            post_procs.sort();

            self.post_procs = post_procs;
        }
    }

    pub fn export<T: AsRef<Path>>(&self, output_dir: T) {
        let output_dir = output_dir.as_ref();

        // Create dir
        if !output_dir.exists() {
            std::fs::create_dir(output_dir).unwrap();
        }

        // Write cams
        let cams_json = serde_json::to_string_pretty(&self.cams).unwrap();
        std::fs::write(output_dir.join("cams.json"), cams_json)
            .expect("Error \"cams.json\" to file");

        // Write post procs
        let post_procs_json = serde_json::to_string_pretty(&self.post_procs).unwrap();
        std::fs::write(output_dir.join("post_procs.json"), post_procs_json)
            .expect("Error \"post_procs.json\" to file");
    }
}

fn try_open_milo(milo_path: &Path) -> Result<(SystemInfo, ObjectDir), Box<dyn Error>> {
    const PLATFORMS: [&str; 3] = ["ps3", "wii", "xbox"];

    let dir_path = milo_path
        .parent()
        .unwrap();

    let file_name = milo_path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap();

    for ext in PLATFORMS.iter() {
        let milo_file_name = PathBuf::from(format!("{file_name}_{ext}"));

        // First try regular path
        let mut result = open_milo(dir_path.join(&milo_file_name).as_path());
        if result.is_ok() {
            return result;
        }

        // Then try w/ gen path
        result = open_milo(dir_path.join("gen").join(&milo_file_name).as_path());
        if result.is_ok() {
            return result;
        }
    }

    Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")))
}

fn open_milo(milo_path: &Path) -> Result<(SystemInfo, ObjectDir), Box<dyn Error>> {
    // Open milo
    let mut stream: Box<dyn Stream> = Box::new(FileStream::from_path_as_read_open(&milo_path)?);
    let milo = MiloArchive::from_stream(&mut stream)?;

    // Unpack dir and entries
    let system_info = SystemInfo::guess_system_info(&milo, &milo_path);
    let obj_dir = milo.unpack_directory(&system_info)?;

    Ok((system_info, obj_dir))
}

fn get_names_for_type_from_dir(obj_dir: &ObjectDir, entry_type: &str) -> Vec<String> {
    let mut entries = obj_dir
        .get_entries()
        .iter()
        .filter(|e| e.get_type().eq(entry_type))
        .map(|e| e.get_name().to_string())
        .collect::<Vec<_>>();

    entries.sort();
    entries
}

#[derive(Default, Deserialize, Serialize)]
pub struct StringValues {
    pub id: String,
    pub values: Vec<String>
}

#[derive(Default, Deserialize, Serialize)]
pub struct Cams {
    pub venues: Vec<StringValues>,
    pub songs: Vec<StringValues>
}

/*#[derive(Default)]
pub struct Character {

}*/