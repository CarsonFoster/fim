use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput, Data, Fields};

#[proc_macro_derive(TryFromString)]
pub fn try_from_string(enum_ts: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(enum_ts).expect("#[try_from_string] can only be applied to enums");
    impl_try_from_string(&ast)
}

fn impl_try_from_string(ast: &DeriveInput) -> TokenStream {
    let enum_name = &ast.ident;
    if let Data::Enum(enum_data) = &ast.data {
        let mut matches = Vec::new();
        for variant in enum_data.variants.iter() {
            if let Fields::Unit = variant.fields {} else {
                panic!("#[try_from_string] can only be applied to enums with unit variants");
            }
            let ident = &variant.ident;
            matches.push(quote! {
                stringify!(#ident) => ::std::result::Result::Ok(<#enum_name>::#variant), 
            });
        }

        let gen = quote! {
            impl ::std::convert::TryFrom<&str> for #enum_name {
                type Error = ::std::string::String;

                fn try_from(s: &str) -> ::std::result::Result<Self, Self::Error> {
                    match s {
                        #(#matches)*
                        _ => ::std::result::Result::Err(format!("could not parse {} into {}", s, stringify!(#enum_name)))
                    }
                }
            }
        };

        gen.into()
    } else {
        panic!("#[try_from_string] can only be applied to enums");
    }
}
