use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse, punctuated::Punctuated, spanned::Spanned, token::Comma, Data, DeriveInput, Field, Fields};
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

fn gen_error() -> TokenStream2 {
    let enum_def = quote! {
        /// Enum for containing errors that might occur in parsing option lines.
        #[derive(Debug, PartialEq)]
        pub enum OptionParseError {
            /// Did not find `set ` at the beginning of the line.
            NoSetPlusSpace,
            /// Did not find an `=`.
            NoEquals,
            /// Did not find an option value after an `=`.
            NoValueAfterEquals,
            /// Did not find a matching option in the [`Options`] object.
            NoMatchingOption{
                /// The option that the user requested.
                option: String
            },
            /// Could not parse the value into the appropriate type.
            ValueParseError{
                /// The message from the parser.
                msg: String
            },
        }
    };

    let impl_def = quote! {
        impl std::fmt::Display for OptionParseError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    OptionParseError::NoSetPlusSpace => write!(f, "did not find `set ` at beginning of string"),
                    OptionParseError::NoEquals => write!(f, "did not find `=` in string"),
                    OptionParseError::NoValueAfterEquals => write!(f, "did not find a value after `=` in string"),
                    OptionParseError::NoMatchingOption{ option } => write!(f, "no matching option in `Options` for {}", option),
                    OptionParseError::ValueParseError{ msg } => write!(f, "failed to parse value: {}", msg)
                }
            }
        }
    };

    let from_str = quote! {
        impl ::std::convert::From<String> for OptionParseError {
            fn from(s: String) -> Self {
                Self::ValueParseError{ msg: s }
            }
        }
    };

    let from_bool_err = quote! {
        impl ::std::convert::From<::std::str::ParseBoolError> for OptionParseError {
            fn from(pbe: ::std::str::ParseBoolError) -> Self {
                Self::ValueParseError{ msg: format!("{}", pbe) }
            }
        }
    };

    let from_int_err = quote! {
        impl ::std::convert::From<::std::num::ParseIntError> for OptionParseError {
            fn from(pie: ::std::num::ParseIntError) -> Self {
                Self::ValueParseError{ msg: format!("{}", pie) }
            }
        }
    };

    let from_infallible = quote! {
        /// Panics.
        impl ::std::convert::From<::std::convert::Infallible> for OptionParseError {
            fn from(_: ::std::convert::Infallible) -> Self {
                panic!("cannot convert an Infallible into an OptionParseError");
            }
        }
    };

    let gen = quote! {
        #enum_def
        #impl_def
        #from_str
        #from_bool_err
        #from_int_err
        #from_infallible
    };
    gen
}

fn gen_extract_opt_val_func() -> TokenStream2 {
    quote! {
        fn extract_opt_val(s: &str) -> ::std::result::Result<(&str, &str), OptionParseError> {
            if let Some(s) = s.strip_prefix("set ") {
                let mut iter = s.splitn(2, '=');
                let opt = iter.next().unwrap(); // since n > 0, there will always be at least one item
                if let Some(value) = iter.next() {
                    let opt = opt.trim();
                    let value = value.trim();
                    if value.len() != 0 {
                        Ok((opt, value))
                    } else { Err(OptionParseError::NoValueAfterEquals) }
                } else { Err(OptionParseError::NoEquals) }
            } else { Err(OptionParseError::NoSetPlusSpace) }
        }
    }
}

fn gen_read_option_func(asserts: &Vec<TokenStream2>, matches: &Vec<TokenStream2>) -> TokenStream2 {
    quote! {
        impl OptionFactory {
            /// Read a single option/value pair from the passed string slice.
            ///
            /// See the [Config module-level documentation](crate::config) for details on the parsing.
            pub fn read_option(&mut self, s: &str) -> Result<(), OptionParseError> {
                #(#asserts)*
                let (opt, val) = extract_opt_val(s)?;
                match opt {
                    #(#matches)*
                    _ => Err(OptionParseError::NoMatchingOption{ option: opt.to_string() })
                }
            }
        }
    }
}

fn gen_set_option_func(asserts: &Vec<TokenStream2>, matches: &Vec<TokenStream2>) -> TokenStream2 {
    quote! {
        impl Options {
            /// Read a single option/value pair from the passed string slice, and set the associated
            /// value.
            ///
            /// See the [Config module-level documentation](crate::config) for details on the parsing.
            pub fn set_option(&mut self, s: &str) -> Result<(), OptionParseError> {
                #(#asserts)*
                let (opt_name, val) = extract_opt_val(s)?;
                match opt_name {
                    #(#matches)*
                    _ => Err(OptionParseError::NoMatchingOption{ option: opt_name.to_string() })
                }
            }
        }
    }
}

fn gen_read_option(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let mut matches_read = Vec::new();
    let mut matches_set = Vec::new();
    let mut asserts = Vec::new();
    for field in fields.iter() {
        let ident = field.ident.as_ref().expect(ERROR_MSG);
        let ty = &field.ty;

        let assert_from_str_name = format_ident!("_AssertFromStr_{}", ident);
        let assert_error_name = format_ident!("_AssertFromFromStrError_{}", ident);
        asserts.push(quote_spanned! {ty.span()=>
            #[allow(non_camel_case_types)] 
            struct #assert_from_str_name where #ty: ::std::str::FromStr; 
            #[allow(non_camel_case_types)]
            struct #assert_error_name where OptionParseError: ::std::convert::From<<#ty as ::std::str::FromStr>::Err>;
        });

        let func = format_ident!("set_{}", ident);
        matches_read.push(quote! {
            stringify!(#ident) => {
                let val_object = val.parse::<#ty>()?;
                self.#func(val_object);
                Ok(())
            }, 
        });

        matches_set.push(quote! {
            stringify!(#ident) => {
                let val_object = val.parse::<#ty>()?;
                self.#ident = val_object;
                Ok(())
            },
        });
    }

    let error_enum = gen_error();
    let extract_opt_val_func = gen_extract_opt_val_func();
    let read_option_func = gen_read_option_func(&asserts, &matches_read);
    let set_option_func = gen_set_option_func(&asserts, &matches_set);

    let gen = quote! {
        #error_enum
        #extract_opt_val_func
        #read_option_func
        #set_option_func
    };
    gen.into()
}
