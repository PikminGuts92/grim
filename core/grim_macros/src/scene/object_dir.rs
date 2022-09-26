use crate::scene::ObjectTokens;
use proc_macro::TokenStream;
use quote::quote;

pub fn get_object_dir_tokens() -> ObjectTokens {
    let struct_fields = [
        quote! { pub entries_ids: Vec<grim_traits::scene::ObjectId> }.into(),
        quote! { pub subdirs_ids: Vec<grim_traits::scene::ObjectId> }.into(),
        quote! { pub viewports: [grim_traits::scene::Matrix; 7] }.into(),
        quote! { pub curr_viewport_index: u32 }.into(),
    ];

    let trait_impl = quote! {
        fn get_entries_ids(&self) -> &Vec<grim_traits::scene::ObjectId> {
            &self.entries_ids
        }
        fn get_entries_ids_mut(&mut self) -> &mut Vec<grim_traits::scene::ObjectId> {
            &mut self.entries_ids
        }
        fn set_entries_ids(&mut self, ids: Vec<grim_traits::scene::ObjectId>) {
            self.entries_ids = ids;
        }

        fn get_subdirs_ids(&self) -> &Vec<grim_traits::scene::ObjectId> {
            &self.subdirs_ids
        }
        fn get_subdirs_ids_mut(&mut self) -> &mut Vec<grim_traits::scene::ObjectId> {
            &mut self.subdirs_ids
        }
        fn set_subdirs_ids(&mut self, ids: Vec<grim_traits::scene::ObjectId>) {
            self.subdirs_ids = ids;
        }

        fn get_viewports(&self) -> &[grim_traits::scene::Matrix; 7] {
            &self.viewports
        }
        fn get_viewports_mut(&mut self) -> &mut [grim_traits::scene::Matrix; 7] {
            &mut self.viewports
        }
        fn set_viewports(&mut self, viewports: [grim_traits::scene::Matrix; 7]) {
            self.viewports = viewports;
        }

        fn get_curr_viewport_index(&self) -> u32 {
            self.curr_viewport_index
        }
        fn set_curr_viewport_index(&mut self, index: u32) {
            self.curr_viewport_index = index;
        }
    };

    ObjectTokens::from_tokens(
        Box::new(struct_fields),
        trait_impl
    )
}