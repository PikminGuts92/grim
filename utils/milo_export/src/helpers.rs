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
    pub venues: Vec<Venue>
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

        self.venues.clear();
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

            let mut cams = venue_milo_dir
                .get_entries()
                .iter()
                .filter(|e| e.get_type().eq("BandCamShot"))
                .map(|e| e.get_name().to_string())
                .collect::<Vec<_>>();

            cams.sort();

            self.venues.push(Venue {
                id: venue_name.to_string(),
                cams
            })
        }
    }

    pub fn export<T: AsRef<Path>>(&self, output_dir: T) {
        let output_dir = output_dir.as_ref();
        let json_venues = serde_json::to_string_pretty(&self.venues).unwrap();

        // Create dir
        if !output_dir.exists() {
            std::fs::create_dir(output_dir).unwrap();
        }

        std::fs::write(output_dir.join("venues.json"), json_venues)
            .expect("Error \"venues.json\" to file");
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

#[derive(Default, Deserialize, Serialize)]
pub struct Venue {
    pub id: String,
    pub cams: Vec<String>
}

/*#[derive(Default)]
pub struct Character {

}*/