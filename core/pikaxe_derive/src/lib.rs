use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemStruct, Fields, Type};

/*
milo_component(name="Group", super=Object,Draw,Trans)
- Creates trait object

milo(name="Group", components=Object,Draw,Trans)
- Implements given traits
*/

#[proc_macro_attribute]
pub fn autotrait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(item as ItemStruct);

    let struct_name = &input_struct.ident; 
    let struct_name_str = struct_name.to_string();

    let trait_name = if struct_name_str.ends_with("Component") {
        format_ident!("{}", &struct_name_str[..struct_name_str.len() - 9])
    } else {
        format_ident!("{}Object", struct_name_str)
    };

    let fields = match &input_struct.fields {
        Fields::Named(fields_named) => &fields_named.named,
        _ => panic!("#[autotrait] only supports structs with named fields."),
    };

    let component_get_fn = format_ident!("get_{}", snake_case(&struct_name_str));
    let component_get_mut_fn = format_ident!("get_{}_mut", snake_case(&struct_name_str));

    // Map over each field
    let methods = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let get_name = format_ident!("get_{}", field_name);
        let get_mut_name = format_ident!("get_{}_mut", field_name);
        let set_name = format_ident!("set_{}", field_name);

        // Check if the type is a primitive that implements Copy
        if is_copy_primitive(field_type) {
            // Primitive Copy-type logic: Return by value, skip `get_mut` method
            quote! {
                fn #get_name(&self) -> #field_type {
                    self.#component_get_fn().#field_name
                }

                fn #set_name(&mut self, value: #field_type) {
                    self.#component_get_mut_fn().#field_name = value;
                }
            }
        } else {
            // Reference type logic (e.g. String, custom structs): Keep original getters/setters
            quote! {
                fn #get_name(&self) -> &#field_type {
                    &self.#component_get_fn().#field_name
                }

                fn #get_mut_name(&mut self) -> &mut #field_type {
                    &mut self.#component_get_mut_fn().#field_name
                }

                fn #set_name(&mut self, value: #field_type) {
                    self.#component_get_mut_fn().#field_name = value;
                }
            }
        }
    });

    let expanded = quote! {
        #input_struct

        pub trait #trait_name: Default + Clone + Bundle {
            fn #component_get_fn(&self) -> &#struct_name;
            fn #component_get_mut_fn(&mut self) -> &mut #struct_name;

            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}

fn is_copy_primitive(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(ident) = type_path.path.get_ident() {
            let name = ident.to_string();
            return matches!(
                name.as_str(),
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
                "f32" | "f64" | "bool" | "char"
            );
        }
    }
    false
}

fn snake_case(s: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 { snake.push('_'); }
            snake.to_lowercase().extend(std::iter::once(ch));
        } else {
            snake.push(ch);
        }
    }
    snake
}