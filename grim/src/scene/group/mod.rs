mod io;

use grim_macros::*;
use grim_traits::scene::*;
pub use io::*;

#[milo(Group)]
#[milo_super(Anim, Draw, Trans)]
pub struct GroupObject {}

impl Default for GroupObject {
    fn default() -> GroupObject {
        GroupObject {
            // Base object
            name: String::default(),
            type2: String::default(),
            note: String::default(),

            // Anim object
            frame: 0.0,
            rate: AnimRate::default(),

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
            override_include_in_depth_only_pass: OverrideIncludeInDepthOnlyPass::default(),

            // Group object
            environ: String::default(),
            objects: Vec::new(),
            lod_width: 0.0,
            lod_height: 0.0,
        }
    }
}