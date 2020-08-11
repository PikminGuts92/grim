use clap::{App, Arg, Clap};
use proc_macro;

#[proc_macro_derive(GameOptions)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output: proc_macro2::TokenStream = {
        proc_macro2::TokenStream::new()
    };

    //proc_macro::TokenStream::from(output)
    "pub milo_path: String,".parse().unwrap()
}