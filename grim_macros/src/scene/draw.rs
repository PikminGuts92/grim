use proc_macro::TokenStream;
use syn::{AttributeArgs, DeriveInput, NestedMeta, parse::Parser, parse_macro_input};
use quote::quote;

pub fn proc_trait_draw(input: TokenStream) -> TokenStream {
    let struct_fields = [
        quote! { pub showing: bool },
        quote! { pub bounding: grim_traits::Sphere },
        quote! { pub draw_order: f32 },
    ];

    let mut input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(s) = &mut input.data {
        if let syn::Fields::Named(fields) = &mut s.fields {
            for field in struct_fields {
                fields.named.push(syn::Field::parse_named.parse(field.into()).unwrap())
            }
        }
    }

    let ident = input.ident.to_owned();

    let output = quote! {
        #input

        impl grim_traits::scene::Draw for #ident {
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
        }
    };

    output.into()
}