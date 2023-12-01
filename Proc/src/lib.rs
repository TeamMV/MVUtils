extern crate proc_macro;

use proc_macro::TokenStream;
use std::env::Args;
use std::fs;
use proc_macro2::Ident;
use quote::__private::ext::RepToTokensExt;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Data, Meta, Attribute, parse_quote};
use syn::token::{Struct, Token};
use crate::savable::{enumerator, named, unit, unnamed};

mod savable;

#[proc_macro_derive(Savable, attributes(unsaved))]
pub fn derive_savable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;

    match &input.data {
        Data::Struct(s) => {
            match &s.fields {
                Fields::Named(fields) => named(fields, name, generics),
                Fields::Unnamed(fields ) => unnamed(fields, name, generics),
                Fields::Unit => unit(name, generics)
            }
        }
        Data::Enum(e) => enumerator(e, name, generics),
        Data::Union(_) => panic!("Deriving Savable for unions is not supported!")
    }
}