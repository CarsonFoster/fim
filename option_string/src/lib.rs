use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput, Data, Fields};

#[proc_macro_derive(OptionString)]
pub fn option_string(enum_ts: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(enum_ts).expect("#[option_string] can only be applied to enums");
    impl_option_string(&ast)
}

fn impl_option_string(ast: &DeriveInput) -> TokenStream {
    let enum_name = &ast.ident;
    if let Data::Enum(enum_data) = &ast.data {
        let mut matches = Vec::new();
        for variant in enum_data.variants.iter() {
            if let Fields::Unit = variant.fields {} else {
                panic!("#[option_string] can only be applied to enums with unit variants");
            }
            let ident = &variant.ident;
            matches.push(quote! {
                stringify!(#ident) => ::std::result::Result::Ok(<#enum_name>::#ident), 
            });
        }

        let gen = quote! {
            impl ::std::str::FromStr for #enum_name {
                type Err = ::std::string::String;

                fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                    match s {
                        #(#matches)*
                        _ => ::std::result::Result::Err(format!("could not parse {} into {}", s, stringify!(#enum_name)))
                    }
                }
            }
        };

        gen.into()
    } else {
        panic!("#[option_string] can only be applied to enums");
    }
}
