use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Meta, Field, FieldsNamed, FieldsUnnamed, Generics, Ident, DataEnum, Fields};
use syn::__private::Span;

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

pub fn enumerator(e: &DataEnum, name: Ident, generics: Generics) -> TokenStream {
    let len = e.variants.len();
    let id_ty = if len < 256 {
        quote! { u8 }
    } else if len < 65536 {
        quote! { u16 }
    } else {
        quote! { u32 }
    };

    let save = e.variants.iter().enumerate().map(|(i, v)| {
        let ident =  &v.ident;
        match &v.fields {
            Fields::Named(fields) => {
                let names = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name
                    }
                });

                let (fields, _): (Vec<_>, Vec<_>) = fields.named.iter().partition(filter);

                let saves = fields.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        Savable::save(#name, saver);
                    }
                });

                quote! {
                    #name::#ident { #( #names ),* } => {
                        Savable::save(&(#i as #id_ty), saver);
                        #( #saves )*
                    }
                }
            }
            Fields::Unnamed(fields ) => {
                let (fields, unsaved_fields): (Vec<_>, Vec<_>) = fields.unnamed.iter().partition(filter);

                if !unsaved_fields.is_empty() {
                    panic!("Unnamed fields cannot be marked as unsaved!");
                }

                let names = fields.iter().enumerate().map(|(i, _)| {
                    let key = key(i as u32);
                    quote! {
                        #key
                    }
                });

                let saves = fields.iter().enumerate().map(|(i, _)| {
                    let name = key(i as u32);
                    quote! {
                        Savable::save(#name, saver);
                    }
                });

                quote! {
                    #name::#ident( #( #names ),* ) => {
                        Savable::save(&(#i as #id_ty), saver);
                        #( #saves )*
                    },
                }
            }
            Fields::Unit => {
                quote! {
                    #name::#ident => Savable::save(&(#i as #id_ty), saver),
                }
            }
        }
    });

    let load = e.variants.iter().enumerate().map(|(i, v)| {
        let ident =  &v.ident;
        let i = i as u32;
        match &v.fields {
            Fields::Named(fields) => {
                let (fields, unsaved_fields): (Vec<_>, Vec<_>) = fields.named.iter().partition(filter);

                let names = fields.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name
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

                let init_default_struct = unsaved_fields.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name
                    }
                });

                quote! {
                    #i => {
                        #( #load_fields )*
                        #( #load_default_fields )*

                        Ok(#name::#ident {
                            #( #names ),*,
                            #( #init_default_struct ),*
                        })
                    }
                }
            }
            Fields::Unnamed(fields ) => {
                let (fields, unsaved_fields): (Vec<_>, Vec<_>) = fields.unnamed.iter().partition(filter);

                if !unsaved_fields.is_empty() {
                    panic!("Unnamed fields cannot be marked as unsaved!");
                }

                let names = fields.iter().enumerate().map(|(i, _)| {
                    let key = key(i as u32);
                    quote! {
                        #key
                    }
                });

                let loads = fields.iter().enumerate().map(|(i, f)| {
                    let name = key(i as u32);
                    let ty = &f.ty;
                    quote! {
                        let #name = <#ty as Savable>::load(loader)?;
                    }
                });

                quote! {
                    #i => {
                        #( #loads )*
                        Ok(#name::#ident( #( #names ),* ))
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    #i => Ok(#name::#ident),
                }
            }
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let implementation = quote! {
        impl #impl_generics Savable for #name #ty_generics #where_clause {
            fn save(&self, saver: &mut impl Saver) {
                match self {
                    #( #save )*
                }
            }

            fn load(loader: &mut impl Loader) -> Result<Self, String> {
                match #id_ty::load(loader)? as u32 {
                    #( #load )*
                    _ => Err(format!("Failed to load {} from loader!", stringify!(#name)))
                }
            }
        }
    };

    TokenStream::from(implementation)
}

fn key(mut n: u32) -> Ident {
    let mut result = String::new();
    loop {
        let remainder = (n % 26) as u8;
        result.push((b'a' + remainder) as char);
        n /= 26;
        if n == 0 {
            break;
        }
        n -= 1;
    }
    let s = result.chars().rev().collect::<String>();
    Ident::new_raw(&s, Span::call_site())
}
