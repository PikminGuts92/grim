#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::error::Error;
use std::path::PathBuf;

use keyframe::{CanTween, keyframes, Keyframe, AnimationSequence, functions::Linear, functions::EaseInOut};

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{Anim, Object, ObjectDir, PackedObject, MeshAnim, MiloObject, Vector3};

use rerun::external::glam;
use rerun::{
    components::{ColorRGBA, Point3D, Radius},
    MsgSender, Session,
    time::{Timeline}
};

#[derive(Clone, Default)]
struct Vec3Collection(Vec<Vector3>);

impl CanTween for Vec3Collection {
    fn ease(from: Self, to: Self, time: impl keyframe::num_traits::Float) -> Self {
        let (Self(from), Self(to)) = (from, to);

        let mut points = Vec::new();

        for (from, to) in from.into_iter().zip(to.into_iter()) {
            let Vector3 { x: x1, y: y1, z: z1 } = from;
            let Vector3 { x: x2, y: y2, z: z2 } = to;

            points.push(Vector3 {
                x: f32::ease(x1, x2, time),
                y: f32::ease(y1, y2, time),
                z: f32::ease(z1, z2, time)
            });
        }

        Self(points)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() < 1 {
        println!("anim_preview.exe [input_milo_path]");
        return Ok(());
    }

    let milo_path = PathBuf::from(&args[0]);

    if let Some(file_name) = milo_path.file_name() {
        let file_name = file_name.to_str().unwrap_or("file");

        println!("Opening {}", file_name);
    }

    // Open milo
    let mut stream: Box<dyn Stream> = Box::new(FileStream::from_path_as_read_open(&milo_path)?);
    let milo = MiloArchive::from_stream(&mut stream)?;

    // Unpack milo
    let system_info = SystemInfo::guess_system_info(&milo, &milo_path);
    let mut obj_dir = milo.unpack_directory(&system_info)?;
    obj_dir.unpack_entries(&system_info)?;

    // Get mesh anims
    let mesh_anims = obj_dir
        .get_entries()
        .iter()
        .filter_map(|e| match e {
            Object::MeshAnim(ma) => Some(ma),
            _ => None
        })
        .collect::<Vec<_>>();

    let mut session = Session::new();

    for mesh_anim in mesh_anims {
        println!("{}", mesh_anim.get_name());
        println!("{} point keys", mesh_anim.vert_point_keys.len());

        // Collect mesh anim frames
        let frames = mesh_anim
            .vert_point_keys
            .iter()
            .map(|v| Keyframe::new(Vec3Collection(v.value.clone()), v.pos, Linear))
            .collect::<Vec<_>>();

        // Generate missing frames with linear interpolation
        let mut sequence = AnimationSequence::from(frames);
        sequence.advance_to(0.0);

        println!("Sequence length is {}", sequence.duration());

        let mut interp_frames = Vec::new();

        while !sequence.finished() {
            let new_frame = sequence.now();
            interp_frames.push(new_frame);

            sequence.advance_by(1.0);
        }

        println!("{} interpolated point keys", interp_frames.len());

        for (i, frame) in interp_frames.into_iter().enumerate() {
            let Vec3Collection(points) = frame;

            let glam_points = points
                .into_iter()
                .map(|Vector3 { x, y, z }| Point3D::new(x, y, z))
                .collect::<Vec<_>>();

            // Send points to rerun
            MsgSender::new(mesh_anim.get_name().as_str())
                .with_component(&glam_points)?
                .with_time(Timeline::new_sequence("frame"), i as i64)
                .send(&mut session)
                .unwrap();
        }
    }

    session.show().unwrap();

    Ok(())
}
