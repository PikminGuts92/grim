use crate::scene::ObjectTokens;
use proc_macro::TokenStream;
use quote::quote;

pub fn get_mesh_tokens() -> ObjectTokens {
    let struct_fields = [
        quote! { pub material: String }.into(),
        quote! { pub mesh_name: String }.into(),
        quote! { pub vertices: Vec<grim_traits::scene::Vertex> }.into(),
        quote! { pub faces: Vec<[u16; 3]> }.into(),
        quote! { pub face_groups: Vec<u8> }.into(),
        quote! { pub bones: Vec<grim_traits::scene::BoneTrans> }.into(),
    ];

    let trait_impl = quote! {
        fn get_material(&self) -> &String {
            &self.material
        }
        fn get_material_mut(&mut self) -> &mut String {
            &mut self.material
        }
        fn set_material(&mut self, material: String) {
            self.material = material;
        }

        fn get_mesh_name(&self) -> &String {
            &self.mesh_name
        }
        fn get_mesh_name_mut(&mut self) -> &mut String {
            &mut self.mesh_name
        }
        fn set_mesh_name(&mut self, mesh_name: String) {
            self.mesh_name = mesh_name;
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
    };

    ObjectTokens::from_tokens(
        Box::new(struct_fields),
        trait_impl
    )
}