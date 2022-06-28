use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse, punctuated::Punctuated,  token::Comma, Data, DeriveInput, Field, Fields};
use quote::{format_ident, quote};

static ERROR: &str = "this macro only works on structs named `Options` with all public, named fields.";

#[proc_macro_derive(OptionFactory)]
pub fn option_factory(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(item).expect(ERROR);
    impl_option_factory(&ast)
}

fn impl_option_factory(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if name == "Options" {
        if let Data::Struct(struct_data) = &ast.data {
            if let Fields::Named(fields) = &struct_data.fields {
                let fields = &fields.named;
                return gen_option_factory(fields);
            }
        }
    }
    panic!("{}", ERROR);
}

fn gen_option_factory(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let def = gen_def();
    let util_funcs = gen_util_funcs();
    let set_funcs = gen_set_funcs(fields);

    quote! {
        #def
        impl OptionFactory {
            #util_funcs
            #(#set_funcs)* 
        }
    }.into()
}

fn gen_def() -> TokenStream2 {
    quote! {
        /// Struct that creates an [`Options`](crate::config::options::Options) object.
        pub struct OptionFactory {
            #[doc(hidden)]
            opt: Options,
        }
    }
}

fn gen_util_funcs() -> TokenStream2 {
    quote! {
        /// Create a new [`OptionFactory`](crate::config::options::OptionFactory).
        ///
        /// The `Options` object begins with its default value.
        pub fn new() -> Self {
            OptionFactory{ opt: Options::default() }
        }

        /// Consume the `OptionFactory` and return the created `Options` object.
        pub fn options(self) -> Options {
            self.opt
        }

        /// Return a reference to the `Options` object in the process of being created.
        pub fn peek(&self) -> &Options {
            &self.opt
        }
    }
}

fn gen_set_funcs(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream2> {
    fields.into_iter()
        .map(|field| {
            let field_name = field.ident.as_ref().expect(ERROR);
            let field_type = &field.ty;
            let func = format_ident!("set_{}", field_name);
            let msg = format!("Set the `{}` field of the `Options` object.", field_name);
            quote! {
                #[doc = #msg]
                pub fn #func(&mut self, #field_name: #field_type) -> &mut Self {
                    self.opt.#field_name = #field_name;
                    self
                }
            }
        })
        .collect::<Vec<TokenStream2>>()
}
