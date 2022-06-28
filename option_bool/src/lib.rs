use proc_macro::TokenStream;
use syn::{parse, Data, DeriveInput, Fields, Ident, Type};
use quote::quote;

#[proc_macro_derive(OptionBool)]
pub fn option_bool(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(item).expect("the OptionBool derive macro only works on structs with one unnamed, bool field");
    impl_option_bool(&ast)
}

fn impl_option_bool(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    if let Data::Struct(struct_data) = data {
        if let Fields::Unnamed(fields) = &struct_data.fields {
            if fields.unnamed.len() == 1 {
                let field = fields.unnamed.first().unwrap(); 
                match &field.ty {
                    Type::Path(type_path) if type_path.path.is_ident("bool") => return gen_impls(name),
                    _ => ()
                }
            }
        }
    }
    panic!("the OptionBool derive macro only works on structs with one unnamed, bool field");
}

fn gen_impls(name: &Ident) -> TokenStream {
    let gen = quote! {
        impl ::std::str::FromStr for #name {
            type Err = ::std::str::ParseBoolError;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                let b: bool = s.parse()?;
                Ok(#name(b))
            }
        }

        impl ::std::convert::From<#name> for bool {
            fn from(t: #name) -> Self {
                t.0
            }
        }

        impl ::std::convert::From<bool> for #name {
            fn from(t: bool) -> Self {
                #name(t)
            }
        }
    };
    gen.into()
}
