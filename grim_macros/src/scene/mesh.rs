use crate::scene::ObjectTokens;
use proc_macro::TokenStream;
use quote::quote;

pub fn get_mesh_tokens() -> ObjectTokens {
    let struct_fields = [
        quote! { pub mat: String }.into(),
        quote! { pub geom_owner: String }.into(),
        quote! { pub mutable: u32 }.into(),
        quote! { pub volume: grim_traits::scene::Volume }.into(),
        quote! { pub vertices: Vec<grim_traits::scene::Vertex> }.into(),
        quote! { pub faces: Vec<[u16; 3]> }.into(),
        quote! { pub face_groups: Vec<u8> }.into(),
        quote! { pub bones: Vec<grim_traits::scene::BoneTrans> }.into(),
        quote! { pub keep_mesh_data: bool }.into(),
    ];

    let trait_impl = quote! {
        fn get_mat(&self) -> &String {
            &self.mat
        }
        fn get_mat_mut(&mut self) -> &mut String {
            &mut self.mat
        }
        fn set_mat(&mut self, mat: String) {
            self.mat = mat;
        }

        fn get_geom_owner(&self) -> &String {
            &self.geom_owner
        }
        fn get_geom_owner_mut(&mut self) -> &mut String {
            &mut self.geom_owner
        }
        fn set_geom_owner(&mut self, geom_owner: String) {
            self.geom_owner = geom_owner;
        }

        fn get_mutable(&self) -> u32 {
            self.mutable
        }
        fn set_mutable(&mut self, mutable: u32) {
            self.mutable = mutable;
        }

        fn get_volume(&self) -> &grim_traits::scene::Volume {
            &self.volume
        }
        fn get_volume_mut(&mut self) -> &mut grim_traits::scene::Volume {
            &mut self.volume
        }
        fn set_volume(&mut self, volume: grim_traits::scene::Volume) {
            self.volume = volume;
        }

        fn get_vertices(&self) -> &Vec<grim_traits::scene::Vertex> {
            &self.vertices
        }
        fn get_vertices_mut(&mut self) -> &mut Vec<grim_traits::scene::Vertex> {
            &mut self.vertices
        }
        fn set_vertices(&mut self, vertices: Vec<grim_traits::scene::Vertex>) {
            self.vertices = vertices;
        }

        fn get_faces(&self) -> &Vec<[u16; 3]> {
            &self.faces
        }
        fn get_faces_mut(&mut self) -> &mut Vec<[u16; 3]> {
            &mut self.faces
        }
        fn set_faces(&mut self, faces: Vec<[u16; 3]>) {
            self.faces = faces;
        }

        fn get_face_groups(&self) -> &Vec<u8> {
            &self.face_groups
        }
        fn get_face_groups_mut(&mut self) -> &mut Vec<u8> {
            &mut self.face_groups
        }
        fn set_face_groups(&mut self, face_groups: Vec<u8>) {
            self.face_groups = face_groups;
        }

        fn get_bones(&self) -> &Vec<grim_traits::scene::BoneTrans> {
            &self.bones
        }
        fn get_bones_mut(&mut self) -> &mut Vec<grim_traits::scene::BoneTrans> {
            &mut self.bones
        }
        fn set_bones(&mut self, bones: Vec<grim_traits::scene::BoneTrans>) {
            self.bones = bones;
        }

        fn get_keep_mesh_data(&self) -> bool {
            self.keep_mesh_data
        }
        fn set_keep_mesh_data(&mut self, keep_mesh_data: bool) {
            self.keep_mesh_data = keep_mesh_data;
        }
    };

    ObjectTokens::from_tokens(
        Box::new(struct_fields),
        trait_impl
    )
}