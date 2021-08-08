use crate::scene::ObjectTokens;
use proc_macro::TokenStream;
use quote::quote;

pub fn get_trans_tokens() -> ObjectTokens {
    let struct_fields = [
        quote! { pub local_transform: grim_traits::scene::Matrix }.into(),
        quote! { pub world_transform: grim_traits::scene::Matrix }.into(),
        quote! { pub constraint: grim_traits::scene::TransConstraint }.into(),
        quote! { pub target: String }.into(),
        quote! { pub preserve_scale: bool }.into(),
        quote! { pub parent: String }.into(),
    ];

    let trait_impl = quote! {
        fn get_local_transform(&self) -> &grim_traits::scene::Matrix {
            &self.local_transform
        }
        fn get_local_transform_mut(&mut self) -> &mut grim_traits::scene::Matrix {
            &mut self.local_transform
        }
        fn set_local_transform(&mut self, transform: grim_traits::scene::Matrix) {
            self.local_transform = transform;
        }

        fn get_world_transform(&self) -> &grim_traits::scene::Matrix {
            &self.world_transform
        }
        fn get_world_transform_mut(&mut self) -> &mut grim_traits::scene::Matrix {
            &mut self.world_transform
        }
        fn set_world_transform(&mut self, transform: grim_traits::scene::Matrix) {
            self.world_transform = transform;
        }

        fn get_constraint(&self) -> &grim_traits::scene::TransConstraint {
            &self.constraint
        }
        fn get_constraint_mut(&mut self) -> &mut grim_traits::scene::TransConstraint {
            &mut self.constraint
        }
        fn set_constraint(&mut self, constraint: grim_traits::scene::TransConstraint) {
            self.constraint = constraint;
        }

        fn get_target(&self) -> &String {
            &self.target
        }
        fn get_target_mut(&mut self) -> &mut String {
            &mut self.target
        }
        fn set_target(&mut self, target: String) {
            self.target = target;
        }

        fn get_preserve_scale(&self) -> bool {
            self.preserve_scale
        }
        fn set_preserve_scale(&mut self, preserve_scale: bool) {
            self.preserve_scale = preserve_scale;
        }

        fn get_parent(&self) -> &String {
            &self.parent
        }
        fn get_parent_mut(&mut self) -> &mut String {
            &mut self.parent
        }
        fn set_parent(&mut self, parent: String) {
            self.parent = parent;
        }
    };

    ObjectTokens::from_tokens(
        Box::new(struct_fields),
        trait_impl
    )
}