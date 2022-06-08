use proc_macro::TokenStream;
use syn::{ parse, punctuated::Punctuated, spanned::Spanned, token::Comma, Data, DeriveInput, Field, Fields };
use quote::{format_ident, quote, quote_spanned};

static ERROR_MSG: &str = "the ReadOption derive macro only works with structs named Options with named fields that implement FromStr";

#[proc_macro_derive(ReadOption)]
pub fn read_option(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(item).expect(ERROR_MSG);
    impl_read_option(&ast)
}

fn impl_read_option(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if name == "Options" {
        if let Data::Struct(struct_data) = &ast.data {
            if let Fields::Named(fields) = &struct_data.fields {
                let fields = &fields.named;
                return gen_read_option(fields);
            }
        }
    }
    panic!("{}", ERROR_MSG);
}

fn gen_read_option(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let mut matches = Vec::new();
    let mut matches_set = Vec::new();
    let mut asserts = Vec::new();
    for field in fields.iter() {
        let ident = field.ident.as_ref().expect(ERROR_MSG);
        let ty = &field.ty;

        let assert_name = format_ident!("_AssertFromStr_{}", ident);
        asserts.push(quote_spanned! {ty.span()=>
            #[allow(non_camel_case_types)] 
            struct #assert_name where #ty: ::std::str::FromStr; 
        });

        let func = format_ident!("set_{}", ident);
        matches.push(quote! {
            stringify!(#ident) => {
                let result = val.parse::<#ty>();
                if let Ok(val_object) = result {
                    factory.#func(val_object);
                    return true;
                }
                return false;
            }, 
        });

        matches_set.push(quote! {
            stringify!(#ident) => {
                let result = val.parse::<#ty>();
                if let Ok(val_object) = result {
                    opt.#ident = val_object; 
                    return true;
                }
                return false;
            },
        });
    }

    let extract_opt_val_func = quote! {
        fn extract_opt_val(s: &str) -> ::std::option::Option<(&str, &str)> {
            if let Some(s) = s.strip_prefix("set ") {
                let mut iter = s.splitn(2, '=');
                if let Some(opt) = iter.next() {
                    if let Some(value) = iter.next() {
                        let opt = opt.trim();
                        let value = value.trim();
                        return Some((opt, value));
                    }
                }
            }
            None
        }
    };

    let read_option_func = quote! {
        /// Read a single option/value pair from the passed string slice.
        ///
        /// Returns `true` if a pair was read and inputted into the `OptionFactory`, and `false`
        /// otherwise.
        pub fn read_option(factory: &mut OptionFactory, s: &str) -> bool {
            #(#asserts)*
            if let Some((opt, val)) = extract_opt_val(s) {
                match opt {
                    #(#matches)*
                    _ => ()
                }
            }
            false
        }
    };

    let set_option_func = quote! {
        /// Read a single option/value pair from the passed string slice, and set the associated
        /// value in the passed `Options` object.
        ///
        /// Returns `true` if a pair was read and set in the `Options`, and `false` otherwise.
        pub fn set_option(opt: &mut Options, s: &str) -> bool {
            #(#asserts)*
            if let Some((opt_name, val)) = extract_opt_val(s) {
                match opt_name {
                    #(#matches_set)*
                    _ => ()
                }
            }
            false
        }
    };

    let gen = quote! {
        #extract_opt_val_func
        #read_option_func
        #set_option_func
    };
    gen.into()
}
