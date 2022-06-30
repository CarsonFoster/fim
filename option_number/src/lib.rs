use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse, Data, DeriveInput, Fields, Ident, Type};
use quote::{format_ident, quote, quote_spanned};

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
                    Type::Path(type_path) if type_path.path.is_ident("i32") => return gen_impls(name),
                    _ => ()
                }
            }
        }
    }
    panic!("the OptionNumber derive macro only works on structs with one unnamed, i32 field");
}

fn gen_assert(name: &Ident) -> TokenStream2 {
    let span = name.span(); 
    let assert_name = format_ident!("_AssertVerifiable{}", name);
    quote_spanned! {span=>
        struct #assert_name where #name: Verifiable; 
    }
}

fn gen_impls(name: &Ident) -> TokenStream {
    let assert = gen_assert(name);
    let gen = quote! {
        #assert

        impl ::std::str::FromStr for #name {
            type Err = NumberParseError;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                let i: i32 = s.parse()?;
                Self::verify(i)?;
                Ok(#name(i))
            }
        }

        impl ::std::convert::From<#name> for i32 {
            fn from(t: #name) -> Self {
                t.0
            }
        }

        impl ::std::convert::From<i32> for #name {
            fn from(t: i32) -> Self {
                #name(t)
            }
        }
    };
    gen.into()
}
