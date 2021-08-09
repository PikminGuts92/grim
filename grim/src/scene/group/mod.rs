use grim_macros::*;
use grim_traits::scene::{Anim, Draw, Group, Trans};

#[milo(Group)]
#[milo_super(Anim, Draw, Trans)]
pub struct GroupObject {}
