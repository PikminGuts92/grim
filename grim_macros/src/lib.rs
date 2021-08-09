use crate::scene::get_object_tokens;
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

#[proc_macro_attribute]
pub fn milo(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let paths = get_meta_paths(&args);

    // TODO: Inherit base fields like type/field/metadata

    if let Some(path) = paths.first() {
        let trait_name = path.segments.last().unwrap().ident.to_string();

        return match get_object_tokens(&trait_name) {
            Some(tokens) => tokens.apply(input, path),
            _ => panic!("Unsupported trait!"),
        };
    }

    input
}

#[proc_macro_attribute]
pub fn milo_super(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let paths = get_meta_paths(&args);

    let mut input = parse_macro_input!(input as DeriveInput);
    let mut transformed_input = proc_macro2::TokenStream::new();

    for path in &paths {
        let trait_name = path.segments.last().unwrap().ident.to_string();

        transformed_input = match get_object_tokens(&trait_name) {
            Some(tokens) => {
                insert_as_struct_fields(&mut input, tokens.struct_fields);
                extend_token_stream_with_trait_implementation(transformed_input, &input.ident, path, tokens.trait_impl)
            },
            _ => panic!("Unsupported \"{}\" trait!", &trait_name),
        };
    }

    (quote! {
        #input
        #transformed_input
    }).into()
}