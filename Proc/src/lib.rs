extern crate proc_macro;

use crate::savable::{enumerator, named, unit, unnamed};
use proc_macro::{TokenStream};
use std::str::FromStr;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprClosure, Fields, LitStr, Meta, Path, Token};
use syn::parse::{ParseBuffer, Parser};
use syn::punctuated::Punctuated;

mod savable;
mod savable2;

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

#[proc_macro_derive(Savable2, attributes(unsaved, custom, varint))]
pub fn derive_savable2(input: TokenStream) -> TokenStream {
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
            Fields::Named(fields) => savable2::named(fields, name, generics),
            Fields::Unnamed(fields) => savable2::unnamed(fields, name, generics),
            Fields::Unit => savable2::unit(name, generics),
        },
        Data::Enum(e) => savable2::enumerator(e, name, generics, varint),
        Data::Union(_) => panic!("Deriving Savable2 for unions is not supported!"),
    }
}

#[proc_macro_derive(TryFromString, attributes(exclude, casing, pattern, custom, inner))]
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

    fn get_pattern(v: &syn::Variant) -> Option<String> {
        for attr in &v.attrs {
            if attr.path().is_ident("pattern") {
                if let Ok(list) = attr.meta.require_list() {
                    let l = list.parse_args::<LitStr>().ok()?;
                    return Some(l.value());
                }
            }
        }
        None
    }

    fn get_custom(v: &syn::Variant) -> Option<Vec<LitStr>> {
        for attr in &v.attrs {
            if attr.path().is_ident("custom") {
                if let Ok(list) = attr.meta.require_list() {
                    let parser = Punctuated::<LitStr, Token![,]>::parse_terminated;
                    if let Ok(punctuated) = parser.parse2(list.tokens.clone()) {
                        return Some(
                            punctuated
                                .into_iter()
                                .collect()
                        );
                    }
                }
            }
        }
        None
    }

    fn get_inner(v: &syn::Variant) -> Option<Expr> {
        for attr in &v.attrs {
            if attr.path().is_ident("inner") {
                if let Ok(list) = attr.meta.require_list() {
                    return list.parse_args::<Expr>().ok();
                }
            }
        }
        None
    }

    match &input.data {
        Data::Enum(e) => {
            let mut statics = quote! {};

            let values: Vec<proc_macro2::TokenStream> = e.variants.iter().filter(|v| !is_excluded(v)).flat_map(|v| {
                let ident = &v.ident;
                let name_str = ident.to_string();
                let casing = get_casing(v);
                let pattern = get_pattern(v);
                let custom = get_custom(v);
                let inner = get_inner(v);

                let constructor = if let Some(inner) = inner {
                    quote! {{
                        let e = #inner;
                        Ok(Self::#ident(e(value).ok_or(())?))
                    }}
                } else {
                    if !v.fields.is_empty() {
                        panic!("Attention! Inner fields must be provided a valid parse closure using the #[inner()] attribute! The closure takes an &String and returns a Option<T>")
                    }
                    quote! {
                        Ok(Self::#ident)
                    }
                };

                if let Some(custom) = custom {
                    vec![quote! {
                        s if [#(#custom),*].contains(s) => #constructor
                    }]
                } else if let Some(pattern) = pattern {
                    let regex_name_s = format!("{name}_{name_str}_regex");
                    let regex_name = Ident::new(&regex_name_s, Span::call_site());

                    statics.extend(quote! {
                        static #regex_name: Lazy<Regex> = Lazy::new(|| Regex::new(#pattern).unwrap());
                    });

                    vec![quote! {
                        _ if #regex_name.is_match() => #constructor
                    }]
                } else {
                    let mut arms = Vec::new();
                    match casing {
                        Casing::Lower => {
                            let lower = name_str.to_lowercase();
                            arms.push(quote! { #lower => #constructor });
                        }
                        Casing::Upper => {
                            let upper = name_str.to_uppercase();
                            arms.push(quote! { #upper => #constructor });
                        }
                        Casing::Both => {
                            let lower = name_str.to_lowercase();
                            let upper = name_str.to_uppercase();
                            arms.push(quote! { #lower => #constructor });
                            arms.push(quote! { #upper => #constructor });
                        }
                    }
                    arms
                }
            }).collect();

            let expanded = quote! {
                #statics

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