use grim_macros::*;
use grim_traits::scene::{Draw, Mesh, Trans};

//#[derive(Version)]
#[milo(Draw)]
#[milo_super(Mesh, Trans)]
pub struct DrawObject {
    //pub version: Option<u32>,
}


impl DrawObject {
    pub fn test(&self) {
        //self.
        milo2! {
            
        }
    }
}