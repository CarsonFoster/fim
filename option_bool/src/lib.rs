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
                    Type::Path(type_path) if type_path.path.is_ident("bool") => return gen_from_str(name),
                    _ => ()
                }
            }
        }
    }
    panic!("the OptionBool derive macro only works on structs with one unnamed, bool field");
}

fn gen_from_str(name: &Ident) -> TokenStream {
    let gen = quote! {
        impl ::std::str::FromStr for #name {
            type Err = ::std::str::ParseBoolError;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                let b: bool = s.parse()?;
                Ok(#name(b))
            }
        }
    };
    gen.into()
}
