use proc_macro::TokenStream;
use syn::{AttributeArgs, DeriveInput, Meta, MetaList, NestedMeta, parse::Parser, parse_macro_input};
use quote::quote;

mod common;
mod scene;

use common::*;

// TODO: Implment custom macro for getting type
/*#[proc_macro_attribute]
pub fn milo_object(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    for arg in &args {
        match arg {
            NestedMeta::Lit(lit) => {

            },
            NestedMeta::Meta(meta) => {

            },
            _ => {

            }
        }
    }

    input
}*/

#[proc_macro_attribute]
pub fn version(_args: TokenStream, input: TokenStream) -> TokenStream {
    let struct_fields = quote! {
        pub version: Option<u32>
    };

    let mut input = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(s) = &mut input.data {
        if let syn::Fields::Named(fields) = &mut s.fields {
            fields.named.push(syn::Field::parse_named.parse2(struct_fields).unwrap())
        }
    }

    //let DeriveInput { ident, data, .. }

    let ident = input.ident.to_owned();

    let output = quote! {
        #input

        impl #ident {
            fn get_version(&self) -> Option<u32> {
                self.version
            }

            fn set_version(&mut self, version: Option<u32>) {
                self.version = version
            }
        }
    };

    output.into()

    /*(quote! {
        #input
    }).into()*/
}

#[proc_macro_derive(Draw)]
pub fn draw(input: TokenStream) -> TokenStream {
    scene::proc_trait_draw(input)
}

#[proc_macro_attribute]
pub fn milo(args: TokenStream, input: TokenStream) -> TokenStream {
    //return scene::proc_trait_draw(input);

    let args = parse_macro_input!(args as AttributeArgs);
    let tags = get_meta_list(&args);

    if let Some(tag) = tags.first() {
        match tag.as_str() {
            "Draw" => {
                return scene::proc_trait_draw(input);
            },
            _ => {

            }
        }
    }

    input
}

#[proc_macro_attribute]
pub fn milo_super(args: TokenStream, input: TokenStream) -> TokenStream {
    //scene::proc_trait_draw(input)

    let args = parse_macro_input!(args as AttributeArgs);
    for arg in &args {
        match arg {
            NestedMeta::Lit(lit) => {

            },
            NestedMeta::Meta(meta) => {

            },
            _ => {

            }
        }
    }


    input
}

#[proc_macro]
pub fn milo2(input: TokenStream) -> TokenStream {

    input
}