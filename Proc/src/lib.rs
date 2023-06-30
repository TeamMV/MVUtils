use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Fields, Data};
use crate::savable::{named, unit, unnamed};

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
        Data::Enum(_) => todo!("Enum savable generation is not supported yet!"),
        Data::Union(_) => panic!("Deriving Savable for unions is not supported!")
    }
}