extern crate proc_macro;

use crate::savable::{enumerator, named, unit, unnamed};
use proc_macro::TokenStream;
use std::str::FromStr;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta};

mod savable;

#[proc_macro_derive(Savable, attributes(unsaved, custom, varint))]
pub fn derive_savable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;

    let varint = input.attrs.iter().any(|attr| {
        if let Meta::Path(ref p) = attr.meta {
            p.segments.iter().any(|s| s.ident == "varint")
        } else {
            false
        }
    });

    match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => named(fields, name, generics),
            Fields::Unnamed(fields) => unnamed(fields, name, generics),
            Fields::Unit => unit(name, generics),
        },
        Data::Enum(e) => enumerator(e, name, generics, varint),
        Data::Union(_) => panic!("Deriving Savable for unions is not supported!"),
    }
}

#[proc_macro_derive(TryFromString, attributes(exclude))]
pub fn try_from_string(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    fn is_excluded(v: &&syn::Variant) -> bool {
        v.attrs.iter().any(|attr| {
            if let Meta::Path(ref p) = attr.meta {
                p.segments.iter().any(|s| s.ident == "exclude")
            } else {
                false
            }
        })
    }

    match &input.data {
        Data::Enum(e) => {
            let values = e.variants.iter().filter(|v| !is_excluded(v)).map(|v| {
                let str = v.ident.to_string();
                let alt = str.chars().next().unwrap().to_lowercase().to_string() + &str.chars().skip(1).map(|c| {
                    if c.is_uppercase() {
                        "_".to_string() + &c.to_lowercase().to_string()
                    } else {
                        c.to_string()
                    }
                }).collect::<String>();
                format!("\"{}\" => Ok(Self::{}),\n\"{}\" => Ok(Self::{}),", str, str, alt, str)
            }).map(|s| {
                proc_macro2::TokenStream::from_str(&s).unwrap()
            });
            quote! {
                impl core::convert::TryFrom<String> for #name {
                    type Error = ();

                    fn try_from(value: String) -> Result<Self, Self::Error> {
                        match value.as_str() {
                            #( #values )*
                            _ => Err(())
                        }
                    }
                }
            }.into()
        },
        _ => panic!("`try_from_string` is only meant for enums")
    }
}
