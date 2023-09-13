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

#[proc_macro_attribute]
pub fn hello_world(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(s) = &input.data {
        match &s.fields {
            Fields::Named(fields) => {
                if fields.named.iter().filter(|f| f.ident.as_ref().unwrap() == "msg").next().is_some() {
                    let name = input.ident;
                    let vis = input.vis;
                    let fields = &fields.named;
                    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
                    let i = quote! {
                        #vis struct #name #ty_generics #where_clause {
                            #fields
                        }

                        impl #impl_generics #name #ty_generics #where_clause {
                            pub fn hello_world(&self) {
                                println!("{}", self.msg);
                            }
                        }
                    };
                    return TokenStream::from(i);
                } else {
                    panic!("Struct must contain `msg` field");
                }
            }
            _ => panic!("Needs a named struct")
        }
    }
    else {
        panic!("Hello world made for structs!");
    }
}

#[proc_macro_attribute]
pub fn R(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let asset_dir = args.to_string().replace("\"", "");
    println!("{}", asset_dir);
    if let Data::Struct(s) = &input.data {
        match &s.fields {
            Fields::Unit => {
                let name = input.ident;
                let vis = input.vis;
                let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
                let fs = fs::read_dir(asset_dir);
                if let Ok(entries) = fs {
                    let mut vec: Vec<String> = vec![];
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let filename = entry.file_name();
                            let filename_str = filename.to_string_lossy().split(".").next().unwrap_or(&filename.to_string_lossy()).to_string().replace("\"", "").replace(" ", "_");
                            println!("{}", filename_str);
                            vec.push(filename_str);
                        }
                    }
                    let field_idents: Vec<Ident> = vec.iter().map(|name| Ident::new(&name, proc_macro2::Span::call_site())).collect();
                    let ts = quote!(
                        #vis struct #name #ty_generics #where_clause {
                            #(#field_idents: i32,)*
                        }
                    );

                    return TokenStream::from(ts);
                } else {
                    println!("{}", fs.err().unwrap());
                    return TokenStream::new();
                }
            }
            _ => panic!("Needs a named struct")
        }
    }
    else {
        panic!("R macro is made for structs!");
    }
}