use std::path::{Path, PathBuf};
use syn::{AttributeArgs, DeriveInput, Meta, MetaList, NestedMeta, parse::Parser, parse_macro_input};

pub fn get_meta_list(args: &Vec<NestedMeta>) -> Vec<String> {
    let mut metas = Vec::new();

    for arg in args {
        if let NestedMeta::Meta(meta) = arg {
            if let Meta::Path(path) = meta {
                let segment = path.segments.last().unwrap();
                metas.push(segment.ident.to_string());
            } else {
                panic!()
            }
        }
    }

    metas
}