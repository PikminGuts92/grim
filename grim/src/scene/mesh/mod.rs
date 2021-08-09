use grim_macros::*;
use grim_traits::scene::{Draw, Mesh, Trans};

#[milo(Mesh)]
#[milo_super(Draw, Trans)]
pub struct MeshObject {}
