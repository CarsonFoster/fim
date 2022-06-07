use proc_macro::TokenStream;
use syn::{parse, Data, DeriveInput, Fields, Ident, Type};
use quote::quote;

#[proc_macro_derive(OptionNumber)]
pub fn option_number(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(item).expect("the OptionNumber derive macro only works on structs with one unnamed, i32 field");
    impl_option_number(&ast)
}

fn impl_option_number(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    if let Data::Struct(struct_data) = data {
        if let Fields::Unnamed(fields) = &struct_data.fields {
            if fields.unnamed.len() == 1 {
                let field = fields.unnamed.first().unwrap(); 
                match &field.ty {
                    Type::Path(type_path) if type_path.path.is_ident("i32") => return gen_from_str(name),
                    _ => ()
                }
            }
        }
    }
    panic!("the OptionNumber derive macro only works on structs with one unnamed, i32 field");
}

fn gen_from_str(name: &Ident) -> TokenStream {
    let gen = quote! {
        impl ::std::str::FromStr for #name {
            type Err = ::std::num::ParseIntError;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                let i: i32 = s.parse()?;
                Ok(#name(i))
            }
        }
    };
    gen.into()
}
