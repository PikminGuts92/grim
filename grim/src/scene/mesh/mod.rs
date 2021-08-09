use grim_macros::*;
use grim_traits::scene::{Draw, Mesh, Trans, Group};

#[milo(Mesh)]
#[milo_super(Draw, Trans)]
pub struct MeshObject {}
