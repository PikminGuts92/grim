use crate::apps::{GameOptions, SubApp};
use clap::Parser;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{AnimEvent, CharLipSync, Object, ObjectDir, ObjectDirBase, MiloObject, Morph, MorphPose, PackedObject, Quat, Tex, TransAnim};

#[derive(Parser, Debug)]
pub struct Milo2KrApp {
    #[arg(help = "Path to input milo file w/ lipsync", required = true)]
    pub input_path: String,
    #[arg(help = "Path to output rnd file", required = true)]
    pub output_path: String,
    #[arg(short, long, help = "Enable to leave output milo archive uncompressed", required = false)]
    pub uncompressed: bool,
}

const LIPSYNC_FPS: u32 = 30;

impl SubApp for Milo2KrApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let milo_path = Path::new(&self.input_path);
        let rnd_path = Path::new(&self.output_path);

        if let Some(file_name) = milo_path.file_name() {
            let file_name = file_name.to_str().unwrap_or("file");

            println!("Opening {}", file_name);
        }

        let mut stream: Box<dyn Stream> = Box::new(FileStream::from_path_as_read_open(milo_path)?);
        let milo = MiloArchive::from_stream(&mut stream)?;

        // TODO: First get system info from args then guess if not supplied
        //let system_info = self.get_system_info();
        let system_info = SystemInfo::guess_system_info(&milo, &milo_path);

        let obj_dir = milo.unpack_directory(&system_info)?;
        //obj_dir.unpack_entries(&SYSTEM_INFO);

        let lipsyncs = get_lipsync_files(&obj_dir, &system_info);

        if lipsyncs.is_empty() {
            println!("No lipsync files found in milo");
            return Ok(());
        }

        let Some(default_lipsync) = lipsyncs.iter().find(|l| l.get_name().eq("song.lipsync")) else {
            println!("Can't find song.lipsync in milo");
            return Ok(());
        };

        let song_length = (default_lipsync.frames_count as f32 / LIPSYNC_FPS as f32) * 1000.;

        let morphs = convert_lipsync_to_morphs(default_lipsync);

        let kr_info = SystemInfo {
            platform: Platform::PS2,
            version: 10,
            endian: IOEndian::Little
        };

        let kr_block_type = if self.uncompressed { BlockType::TypeA } else { BlockType::TypeB };

        let kr_obj_dir = ObjectDir::ObjectDir(ObjectDirBase {
            entries: morphs
                .into_iter()
                .map(|m| Object::Morph(m))
                .chain(vec![
                    Object::TransAnim(TransAnim {
                        name: String::from("song_head.tnm"),
                        rot_keys: vec![
                            AnimEvent {
                                value: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
                                pos: if song_length > 1000. { 1000. } else { 0. }
                            },
                            AnimEvent {
                                value: Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
                                pos: (song_length - 1000.).max(0.0)
                            }
                        ],
                        trans_anim_owner: String::from("song_head.tnm"),
                        trans_spline: true,
                        ..Default::default()
                    })
                ])
                .collect(),
            ..ObjectDirBase::new()
        });

        let rnd = MiloArchive::from_object_dir(&kr_obj_dir, &kr_info, Some(kr_block_type)).unwrap();

        let mut stream = FileStream::from_path_as_read_write_create(&rnd_path)?;
        rnd.write_to_stream(&mut stream)?;

        if let Some(file_name) = rnd_path.file_name() {
            let file_name = file_name.to_str().unwrap();
            println!("Successfully wrote {}", file_name);
        }

        Ok(())
    }
}

fn get_lipsync_files(obj_dir: &ObjectDir, info: &SystemInfo) -> Vec<CharLipSync> {
    obj_dir
        .get_entries()
        .iter()
        .filter_map(|e| match e.get_type() {
            "CharLipSync" => match e.unpack(info) {
                Some(Object::CharLipSync(cls)) => Some(cls),
                _ => panic!("Unable to open {}", e.get_name())
            },
            _ => None
        })
        .collect()
}

fn convert_lipsync_to_morphs(lipsync: &CharLipSync) -> Vec<Morph> {
    // Group changes by pose/viseme name and calculate ms positions
    let viseme_weights = lipsync
        .get_frames()
        .into_iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (idx, frame)| {
            //let pos = (idx as f32 / LIPSYNC_FPS as f32) * 1000.0; // Convert to ms

            for (viseme, weight) in frame.visemes {
                //let weight = weight as f32 / 255.0;

                acc
                    .entry(viseme)
                    .and_modify(|e: &mut Vec<(usize, u8)>| e.push((idx, weight)))
                    .or_insert_with(|| vec![(idx, weight)]);
            }

            acc
        });

    let mouth_pose_map: [(&str, Vec<&str>); 15] = [
        ("EAT", vec![ "Eat_hi", "Eat_lo" ]),
        ("ERTH", vec![ "Earth_hi", "Earth_lo" ]),
        ("IF", vec![ "If_hi", "If_lo" ]),
        ("OX", vec![ "Ox_hi", "Ox_lo" ]),
        ("OAT", vec![ "Oat_hi", "Oat_lo" ]),

        ("WET", vec![ "Wet_hi", "Wet_lo" ]),
        ("SIZE", vec![ "Size_hi", "Size_lo" ]),
        ("CHUR", vec![ "Church_hi", "Church_lo" ]),
        ("FAVE", vec![ "Fave_hi", "Fave_lo" ]),
        ("THOU", vec![ "Though_hi", "Though_lo" ]),

        ("TOLD", vec![ "Told_hi", "Told_lo" ]),
        ("BUMP", vec![ "Bump_hi", "Bump_lo" ]),
        ("NEW", vec![ "New_hi", "New_lo" ]),
        ("ROAR", vec![ "Roar_hi", "Roar_lo" ]),
        ("CAGE", vec![ "Cage_hi", "Cage_lo" ]),
    ];

    /*let brow_map: [(&str, Vec<&str>); 1] = [
        ("EBRR EBRL", vec![ ]),
    ];*/

    vec![
        Morph {
            name: String::from("song_brow.mrf"),
            poses: vec![
                MorphPose::default(),
                MorphPose::default()
            ],
            ..Morph::default()
        },
        Morph {
            name: String::from("song_lid.mrf"),
            poses: vec![
                MorphPose::default(),
                MorphPose::default()
            ],
            ..Morph::default()
        },
        Morph {
            name: String::from("song_mouth.mrf"),
            poses: convert_visemes_to_poses(&viseme_weights, &mouth_pose_map),
            ..Morph::default()
        },
    ]
}

fn convert_visemes_to_poses(viseme_weights: &HashMap<&str, Vec<(usize, u8)>>, pose_map:  &[(&str, Vec<&str>)]) -> Vec<MorphPose> {
    let mut poses = vec![ MorphPose::default() ]; // First one is always empty

    for (_, viseme_names) in pose_map {
        // TODO: Average weights between hi + lo values
        let weights = viseme_names
            .iter()
            .filter_map(|v| viseme_weights.get(v))
            .map(|v| v
                .iter()
                .map(|(i, w)| AnimEvent {
                    value: *w as f32 / 255.0,
                    pos: (*i as f32 / LIPSYNC_FPS as f32) * 1000.0
                }) //((*i as f32 / LIPSYNC_FPS as f32) * 1000.0, *w as f32 / 255.0))
                .collect()
            )
            .next()
            .unwrap_or_else(|| Vec::new());

        poses.push(MorphPose {
            events: weights
        });
    }

    poses
}