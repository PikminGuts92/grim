use crate::scene::ObjectTokens;
use proc_macro::TokenStream;
use quote::quote;
use syn::{AttributeArgs, DeriveInput, NestedMeta, Path, parse::Parser, parse_macro_input};

pub fn get_draw_tokens() -> ObjectTokens {
    let struct_fields: [TokenStream; 3] = [
        quote! { pub showing: bool }.into(),
        quote! { pub bounding: grim_traits::Sphere }.into(),
        quote! { pub draw_order: f32 }.into(),
    ];

    let trait_impl = quote! {
        fn get_showing(&self) -> bool {
            self.showing
        }

        fn set_showing(&mut self, showing: bool) {
            self.showing = showing;
        }

        fn get_bounding(&self) -> &grim_traits::Sphere {
            &self.bounding
        }

        fn get_bounding_mut(&mut self) -> &mut grim_traits::Sphere {
            &mut self.bounding
        }

        fn set_bounding(&mut self, bounding: grim_traits::Sphere) {
            self.bounding = bounding;
        }

        fn get_draw_order(&self) -> f32 {
            self.draw_order
        }

        fn set_draw_order(&mut self, draw_order: f32) {
            self.draw_order = draw_order;
        }
    };

    ObjectTokens::from_tokens(Box::new(struct_fields), trait_impl)
}