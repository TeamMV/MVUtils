extern crate proc_macro;

use crate::savable::{enumerator, named, unit, unnamed};
use proc_macro::TokenStream;
use std::str::FromStr;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta, Path};

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

#[proc_macro_derive(TryFromString, attributes(exclude, casing))]
pub fn try_from_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    #[derive(Clone, Copy)]
    enum Casing {
        Lower,
        Upper,
        Both,
    }

    // helper: check #[exclude]
    fn is_excluded(v: &syn::Variant) -> bool {
        v.attrs.iter().any(|attr| attr.path().is_ident("exclude"))
    }

    // helper: check #[casing(...)]
    fn get_casing(v: &syn::Variant) -> Casing {
        for attr in &v.attrs {
            if attr.path().is_ident("casing") {
                if let Ok(list) = attr.meta.require_list() {
                    if let Ok(path) = list.parse_args::<Path>() {
                        let ident = path.get_ident().unwrap().to_string();
                        return match ident.as_str() {
                            "Lower" => Casing::Lower,
                            "Upper" => Casing::Upper,
                            "Both" => Casing::Both,
                            other => panic!("Invalid casing: {}", other),
                        };
                    }
                }
            }
        }
        Casing::Both
    }

    match &input.data {
        Data::Enum(e) => {
            let values = e.variants.iter().filter(|v| !is_excluded(v)).flat_map(|v| {
                let ident = &v.ident;
                let name_str = ident.to_string();
                let casing = get_casing(v);

                let mut arms = Vec::new();
                match casing {
                    Casing::Lower => {
                        let lower = name_str.to_lowercase();
                        arms.push(quote! { #lower => Ok(Self::#ident) });
                    }
                    Casing::Upper => {
                        let upper = name_str.to_uppercase();
                        arms.push(quote! { #upper => Ok(Self::#ident) });
                    }
                    Casing::Both => {
                        let lower = name_str.to_lowercase();
                        let upper = name_str.to_uppercase();
                        arms.push(quote! { #lower => Ok(Self::#ident) });
                        arms.push(quote! { #upper => Ok(Self::#ident) });
                    }
                }
                arms
            });

            let expanded = quote! {
                impl core::str::FromStr for #name {
                    type Err = ();

                    fn from_str(value: &str) -> Result<Self, Self::Err> {
                        match value {
                            #(#values,)*
                            _ => Err(()),
                        }
                    }
                }
            };

            expanded.into()
        }
        _ => panic!("`TryFromString` can only be derived for enums"),
    }
}

enum Casing {
    Lower,
    Upper,
    Both,
}