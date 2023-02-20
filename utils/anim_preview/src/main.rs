#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use keyframe::{CanTween, keyframes, Keyframe, AnimationSequence, functions::Linear, functions::EaseInOut};

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{Anim, Object, ObjectDir, PackedObject, MeshAnim, MiloObject, Trans, Vector3};

use nalgebra as na;

use rerun::external::glam;
use rerun::{
    components::{ColorRGBA, LineStrip3D, MeshId, Point3D, Radius, RawMesh3D},
    MsgSender, Session,
    time::Timeline
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

            /*let strip: LineStrip3D = points
                .iter()
                .map(|Vector3 { x, y, z }| [ *x, *y, *z ])
                .collect::<Vec<_>>()
                .into();*/

            /*let mesh = RawMesh3D {
                mesh_id: MeshId::random(),
                positions: points
                    .iter()
                    .map(|Vector3 { x, y, z }| [ *x, *y, *z ])
                    .collect::<Vec<_>>()
            };*/

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

            // Send line strip to rerun
            /*MsgSender::new(mesh_anim.get_name().as_str())
                .with_component(&[strip])?
                .with_time(Timeline::new_sequence("frame"), i as i64)
                .send(&mut session)
                .unwrap();*/
        }
    }

    // Get bones
    let root_bone = BoneNode::new(&obj_dir);

    if let Some(root_bone) = root_bone {
        let points = generate_bone_points(&root_bone);

        MsgSender::new(root_bone.name)
                .with_component(&points)?
                .send(&mut session)
                .unwrap();
    }

    session.show().unwrap();

    Ok(())
}

fn generate_bone_points(bone: &BoneNode) -> Vec<Point3D> {
    let mut points = Vec::new();

    //let v: na::Vector3<f32> = bone.transform.transform_vector(&na::Vector3::zeros());
    let v = bone.transform.column(3).xyz();
    points.push(Point3D::from([v[0], v[1], v[2]]));

    for child in bone.children.iter() {
        let mut child_points = generate_bone_points(child);
        points.append(&mut child_points);
    }

    points
}

pub struct BoneNode<'a> {
    pub name: &'a str,
    pub object: Option<&'a dyn Trans>,
    pub children: Vec<BoneNode<'a>>,
    pub transform: na::Matrix4<f32>,
}

impl<'a> BoneNode<'a> {
    fn new(obj_dir: &'a ObjectDir) -> Option<Self> {
        let dir_name = match obj_dir {
            ObjectDir::ObjectDir(base) => base.name.as_str(),
        };

        // Get bones
        let bones = obj_dir
            .get_entries()
            .iter()
            .filter_map(|o| match o {
                Object::Mesh(m) if m.faces.is_empty() // GH1 bones
                    => Some(m as &dyn Trans),
                Object::Trans(t) => Some(t as &dyn Trans),
                _ => None
            })
            .map(|b| (b.get_name().as_str(), b))
            .collect::<HashMap<_, _>>();

        // Map children
        let child_map = bones
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<&str, Vec<&'a dyn Trans>>, (_, b)| {
                if b.get_parent().eq(b.get_name()) {
                    // If bone references self, ignore
                    return acc;
                }

                acc
                    .entry(b.get_parent().as_str())
                    .and_modify(|e| e.push(*b))
                    .or_insert(vec![*b]);

                acc
            });

        // Create root node
        let mut root = BoneNode {
            name: dir_name,
            object: None,
            children: Vec::new(),
            transform: na::Matrix4::identity(),
        };

        // Find bones that belong to object dir
        root.children = root.find_child_nodes(&bones, &child_map);

        if root.children.is_empty() {
            return None;
        }

        Some(root)
    }

    fn find_child_nodes(&self, bone_map: &HashMap<&str, &'a dyn Trans>, child_map: &HashMap<&str, Vec<&'a dyn Trans>>) -> Vec<BoneNode<'a>> {
        let parent_name = self.name;

        let Some(children) = child_map.get(parent_name) else {
            return Vec::new();
        };

        children
            .iter()
            .map(|c| {
                let trans_obj = bone_map.get(c.get_name().as_str()).map(|o| *o);
                let local_transform = trans_obj
                    .map(|o| {
                        let m = o.get_local_xfm();

                        na::Matrix4::new(
                            // Column-major order...
                            m.m11, m.m21, m.m31, m.m41,
                            m.m12, m.m22, m.m32, m.m42,
                            m.m13, m.m23, m.m33, m.m43,
                            m.m14, m.m24, m.m34, m.m44
                        )

                        /*na::Matrix4::new(
                            m.m11, m.m12, m.m13, m.m14,
                            m.m21, m.m22, m.m23, m.m24,
                            m.m31, m.m32, m.m33, m.m34,
                            m.m41, m.m42, m.m43, m.m44
                        )*/
                    })
                    .unwrap_or(na::Matrix4::identity());

                let mut bone = BoneNode {
                    name: c.get_name().as_str(),
                    object: trans_obj,
                    children: Vec::new(),
                    transform: self.transform * local_transform
                };

                bone.children = bone.find_child_nodes(bone_map, child_map);
                bone
            })
            .collect()
    }
}