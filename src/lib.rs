use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

/// Creates comsuming builder methods for every field.
#[proc_macro_derive(Builder)]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let mut builders = quote!();

    let syn::Data::Struct(data) =  input.data else {panic!()};
    let syn::Fields::Named(fields) = data.fields else {panic!();};

    for field in fields.named.into_iter() {
        let ident = field.ident.unwrap();
        let f_type = field.ty;

        let builder = if quote!(#f_type).to_string() == "String" {
            quote! {
                pub fn #ident(self, #ident: &str) -> Self {
                    Self {
                        #[allow(clippy::needless_update)]
                        #ident: #ident.to_string(), ..self
                    }
                }
            }
        } else {
            quote! {
                pub fn #ident(self, #ident: #f_type) -> Self {
                    Self {
                        #[allow(clippy::needless_update)]
                        #ident, ..self
                    }
                }
            }
        };

        builders.extend(builder);
    }

    let expanded = quote! {
        impl #name {
            #builders
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(GetSet)]
pub fn getset(input: TokenStream) -> TokenStream {
    let dinput = parse_macro_input!(input as DeriveInput);

    let fields = struct_fields(&dinput);
    let name = dinput.ident;
    let mut methods = quote!();

    for field in fields.named.into_iter() {
        let ident = field.ident.unwrap();
        let f_type = field.ty;
        let get_name = syn::Ident::new(&format!("get_{ident}"), ident.span());
        let set_name = syn::Ident::new(&format!("set_{ident}"), ident.span());

        let builder = quote! {
            pub fn #get_name(&self) -> &#f_type {
                &self.#ident
            }
            pub fn #set_name(&mut self, input: #f_type) {
                self.#ident = input;
            }
        };

        methods.extend(builder);
    }

    TokenStream::from(quote! {impl #name { #methods}})
}

fn struct_fields(input: &DeriveInput) -> FieldsNamed {
    let syn::Data::Struct(data) =  input.clone().data else {panic!()};
    let syn::Fields::Named(fields) = data.fields else {panic!();};
    fields
}
