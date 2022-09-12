mod io;

use grim_macros::*;
use grim_traits::scene::*;
pub use io::*;

#[milo(Draw)]
pub struct DrawObject {}

impl Default for DrawObject {
    fn default() -> DrawObject {
        DrawObject {
            // Base object
            name: String::default(),
            type2: String::default(),
            note: String::default(),

            // Draw object
            showing: true,
            draw_objects: Vec::new(),
            sphere: Sphere::default(),
            draw_order: 0.0,
            override_include_in_depth_only_pass: OverrideIncludeInDepthOnlyPass::default(),
        }
    }
}
