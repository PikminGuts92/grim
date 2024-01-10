mod anim;
mod draw;
mod group;
mod mesh;
mod object;
mod object_dir;
mod poll;
mod trans;

use crate::*;
pub use object::*;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Path, parse_macro_input};

pub fn get_object_tokens(obj_type: &str) -> Option<ObjectTokens> {
    let get_tokens: fn() -> ObjectTokens = match obj_type {
        "Anim" => anim::get_anim_tokens,
        "Draw" => draw::get_draw_tokens,
        "Group" => group::get_group_tokens,
        "Mesh" => mesh::get_mesh_tokens,
        "ObjectDir" => object_dir::get_object_dir_tokens,
        "Poll" => poll::get_poll_tokens,
        "Trans" => trans::get_trans_tokens,
        _ => {
            return None
        },
    };

    Some(get_tokens())
}

pub struct ObjectTokens {
    pub struct_fields: Box<[TokenStream]>,
    pub trait_impl: proc_macro2::TokenStream,
}

impl ObjectTokens {
    pub fn from_tokens(struct_fields: Box<[TokenStream]>, trait_impl: proc_macro2::TokenStream) -> Self {
        ObjectTokens {
            struct_fields,
            trait_impl,
        }
    }

    pub fn apply(self, input: TokenStream, trait_path: &Path) -> TokenStream {
        // TODO: Fix for regular milo attribute
        let mut input = parse_macro_input!(input as DeriveInput);

        insert_as_struct_fields(&mut input, self.struct_fields);
        with_trait_implementation(&input, trait_path, self.trait_impl)
    }
}