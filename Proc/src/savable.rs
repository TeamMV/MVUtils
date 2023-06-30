use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Meta, Field, FieldsNamed, FieldsUnnamed, Generics, Ident};

fn filter(f: &&Field) -> bool {
    !f.attrs.iter().any(is_unsaved)
}

fn is_unsaved(attr: &Attribute) -> bool {
    if let Meta::Path(ref p) = attr.meta {
        p.segments.iter().any(|s| s.ident == "unsaved")
    } else {
        false
    }
}

pub fn named(fields: &FieldsNamed, name: Ident, generics: Generics) -> TokenStream {
    let (fields, unsaved_fields): (Vec<_>, Vec<_>) = fields.named.iter().partition(filter);
    let save_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            Savable::save(&self.#name, saver);
        }
    });

    let load_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            let #name = <#ty as Savable>::load(loader)?;
        }
    });

    let load_default_fields = unsaved_fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            let #name = <#ty as Default>::default();
        }
    });

    let init_struct = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name
        }
    });

    let init_default_struct = unsaved_fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let implementation = quote! {
        impl #impl_generics Savable for #name #ty_generics #where_clause {
            fn save(&self, saver: &mut impl Saver) {
                #( #save_fields )*
            }

            fn load(loader: &mut impl Loader) -> Result<Self, String> {
                #( #load_fields )*
                #( #load_default_fields )*

                Ok(Self {
                    #( #init_struct ),*,
                    #( #init_default_struct ),*
                })
            }
        }
    };

    TokenStream::from(implementation)
}

pub fn unnamed(fields: &FieldsUnnamed, name: Ident, generics: Generics) -> TokenStream {
    let (fields, unsaved_fields): (Vec<_>, Vec<_>) = fields.unnamed.iter().partition(filter);
    if !unsaved_fields.is_empty() {
        panic!("Unnamed fields cannot be marked as unsaved!");
    }

    let save_fields = fields.iter().enumerate().map(|(i, _)| {
        quote! {
            Savable::save(&self.#i, saver);
        }
    });

    let load_fields = fields.iter().map(|f| {
        let ty = &f.ty;
        quote! {
            <#ty as Savable>::load(loader)?
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let implementation = quote! {
        impl #impl_generics Savable for #name #ty_generics #where_clause {
            fn save(&self, saver: &mut impl Saver) {
                #( #save_fields )*
            }

            fn load(loader: &mut impl Loader) -> Result<Self, String> {
                Ok(Self(#( #load_fields ),*))
            }
        }
    };

    TokenStream::from(implementation)
}

pub fn unit(name: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let implementation = quote! {
        impl #impl_generics Savable for #name #ty_generics #where_clause {
            fn save(&self, saver: &mut impl Saver) {}

            fn load(loader: &mut impl Loader) -> Result<Self, String> {
                Ok(Self)
            }
        }
    };

    TokenStream::from(implementation)
}