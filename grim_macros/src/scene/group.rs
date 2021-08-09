use crate::scene::ObjectTokens;
use proc_macro::TokenStream;
use quote::quote;

pub fn get_group_tokens() -> ObjectTokens {
    let struct_fields = [
        quote! { pub environ: String }.into(),
        quote! { pub objects: Vec<String> }.into(),
        quote! { pub lod_width: f32 }.into(),
        quote! { pub lod_height: f32 }.into(),
    ];

    let trait_impl = quote! {
        fn get_environ(&self) -> &String {
            &self.environ
        }
        fn get_environ_mut(&mut self) -> &mut String {
            &mut self.environ
        }
        fn set_environ(&mut self, environ: String) {
            self.environ = environ;
        }

        fn get_objects(&self) -> &Vec<String> {
            &self.objects
        }
        fn get_objects_mut(&mut self) -> &mut Vec<String> {
            &mut self.objects
        }
        fn set_objects(&mut self, objects: Vec<String>) {
            self.objects = objects;
        }

        fn get_lod_width(&self) -> f32 {
            self.lod_width
        }
        fn set_lod_width(&mut self, lod_width: f32) {
            self.lod_width = lod_width;
        }

        fn get_lod_height(&self) -> f32 {
            self.lod_height
        }
        fn set_lod_height(&mut self, lod_height: f32) {
            self.lod_height = lod_height;
        }
    };

    ObjectTokens::from_tokens(
        Box::new(struct_fields),
        trait_impl
    )
}