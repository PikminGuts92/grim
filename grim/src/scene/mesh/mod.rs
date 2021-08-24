mod io;

use grim_macros::*;
use grim_traits::scene::*;
pub use io::*;

#[milo(RndMesh)]
#[milo_super(Draw, Trans)]
pub struct MeshObject {}

impl Default for MeshObject {
    fn default() -> MeshObject {
        MeshObject {
            // Base object
            name: String::default(),
            type2: String::default(),
            note: String::default(),

            // Trans object
            local_xfm: Matrix::default(),
            world_xfm: Matrix::default(),

            constraint: TransConstraint::default(),
            target: String::default(),

            preserve_scale: false,
            parent: String::default(),

            // Draw object
            showing: true,
            sphere: Sphere::default(),
            draw_order: 0.0,

            // Mesh object
            mat: String::default(),
            geom_owner: String::default(),

            mutable: Mutable::kMutableNone,
            volume: Volume::kVolumeTriangles,

            vertices: Vec::new(),
            faces: Vec::new(),

            face_groups: Vec::new(),
            bones: Vec::new(),
            keep_mesh_data: false,
        }
    }
}