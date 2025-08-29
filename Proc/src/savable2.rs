use proc_macro::TokenStream;
use quote::{quote};
use std::str::FromStr;
use syn::__private::Span;
use syn::{parse, Attribute, DataEnum, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, Generics, Ident, Meta, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

struct KeyValue {
    key: Ident,
    #[allow(dead_code)]
    eq_token: Token![=],
    value: Expr,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(KeyValue {
            key: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

fn get_custom(f: &Field) -> Option<(Expr, Expr)> {
    f.attrs.iter().filter_map(|attr| {
        if let Meta::List(ref l) = attr.meta {
            if l.path.segments.iter().any(|s| s.ident == "custom") {
                let tokens: TokenStream = l.tokens.clone().into();
                let values = parse::Parser::parse(Punctuated::<KeyValue, Token![,]>::parse_terminated, tokens).unwrap();
                if values.len() != 2 {
                    panic!("Expected 'save' and 'load' values for custom attribute")
                }
                if values[0].key.to_string() != "save" || values[1].key.to_string() != "load" {
                    panic!("Expected 'save' and 'load' values for custom attribute")
                }
                if !matches!(values[0].value, Expr::Path(_)) || !matches!(values[1].value, Expr::Path(_)) {
                    panic!("Expected 'save' and 'load' to be path attributes to saving and loading function")
                }
                return Some((values[0].value.clone(), values[1].value.clone()));
            }
        }
        None
    }).next()
}

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

    let fields = fields.into_iter().map(|f| (f, get_custom(f))).collect::<Vec<_>>();

    let save_fields = fields.iter().map(|(f, custom)| {
        let name = &f.ident;
        if let Some((save, _)) = custom {
            quote! {
                #save(saver, &self.#name);
            }
        } else {
            quote! {
                mvutils::save2::Savable2::save(&self.#name, saver);
            }
        }
    });

    let load_fields = fields.iter().map(|(f, custom)| {
        let name = &f.ident;
        let ty = &f.ty;
        if let Some((_, load)) = custom {
            panic!("Custom not supported in Savable2 get rekt");
        }
        quote! {
            self.#name.load_into(loader)?;
        }
    });

    let load_default_fields = unsaved_fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            *self.#name = <#ty as Default>::default();
        }
    });

    let init_struct = fields.iter().map(|(f, _)| {
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

    let t1 = if !fields.is_empty() { quote!{,} } else { quote!{} };

    let implementation = quote! {
        impl #impl_generics mvutils::save2::Savable2 for #name #ty_generics #where_clause {
            fn save(&self, saver: &mut bytebuffer::ByteBuffer) {
                #( #save_fields )*
            }

            fn load_into(&mut self, loader: &mut bytebuffer::ByteBuffer) -> Result<(), String> {
                #( #load_fields )*
                #( #load_default_fields )*

                Ok(())
            }
        }
    };

    TokenStream::from(implementation)
}

pub fn unnamed(fields: &FieldsUnnamed, name: Ident, generics: Generics) -> TokenStream {
    let fields: Vec<_> = fields.unnamed.iter().enumerate().collect();
    let amount = fields.len();

    let (fields, unsaved_fields): (Vec<_>, Vec<_>) = fields.iter().partition(|(_, f)| filter(f));

    let fields = fields.into_iter().map(|(i, f)| (i, (f, get_custom(f)))).collect::<Vec<_>>();

    let save_fields = fields.iter().map(|(i, (_, custom))| {
        let i = proc_macro2::TokenStream::from_str(&i.to_string()).unwrap();
        if let Some((save, _)) = custom {
            quote! {
                #save(saver, &self.#i);
            }
        } else {
            quote! {
                mvutils::save::Savable::save(&self.#i, saver);
            }
        }
    });

    let load_fields = fields.iter().map(|(i, (f, custom))| {
        let ty = &f.ty;
        let key = key(*i as u32);
        if let Some((_, load)) = custom {
            quote! {
                let #key = #load(loader)?;
            }
        } else {
            quote! {
                let #key = <#ty as mvutils::save::Savable>::load(loader)?;
            }
        }
    });

    let load_unsaved_fields = unsaved_fields.iter().map(|(i, f)| {
        let ty = &f.ty;
        let key = key(*i as u32);
        quote! {
            let #key = <#ty as Default>::default();
        }
    });

    let mut names = Vec::with_capacity(amount);
    for i in 0..amount {
        let key = key(i as u32);
        names.push(quote! {
            #key
        });
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let implementation = quote! {
        impl #impl_generics mvutils::save2::Savable2 for #name #ty_generics #where_clause {
            fn save(&self, saver: &mut bytebuffer::ByteBuffer) {
                #( #save_fields )*
            }

            fn load(loader: &mut bytebuffer::ByteBuffer) -> Result<(), String> {
                #( #load_fields )*
                #( #load_unsaved_fields )*
                Ok(())
            }
        }
    };

    TokenStream::from(implementation)
}

pub fn unit(name: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let implementation = quote! {
        impl #impl_generics mvutils::save::Savable for #name #ty_generics #where_clause {
            fn save(&self, saver: &mut impl mvutils::save::Saver) {}

            fn load(loader: &mut impl mvutils::save::Loader) -> Result<Self, String> {
                Ok(Self)
            }
        }
    };

    TokenStream::from(implementation)
}

pub fn enumerator(e: &DataEnum, name: Ident, generics: Generics, varint: bool) -> TokenStream {
    let len = e.variants.len();
    let id_ty = if varint {
        quote! { u64 }
    } else if len < 256 {
        quote! { u8 }
    } else if len < 65536 {
        quote! { u16 }
    } else {
        quote! { u32 }
    };

    let save = e.variants.iter().enumerate().map(|(i, v)| {
        let ident = &v.ident;
        let save_variant = if varint {
            quote! { mvutils::save::custom::varint_save(saver, &(#i as u64)) }
        } else {
            quote! { mvutils::save::Savable::save(&(#i as #id_ty), saver) }
        };
        match &v.fields {
            Fields::Named(fields) => {
                let names = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name
                    }
                });

                let (fields, _): (Vec<_>, Vec<_>) = fields.named.iter().partition(filter);

                let fields = fields.into_iter().map(|f| (f, get_custom(f))).collect::<Vec<_>>();

                let saves = fields.iter().map(|(f, custom)| {
                    let name = &f.ident;
                    if let Some((save, _)) = custom {
                        quote! {
                            #save(saver, #name);
                        }
                    } else {
                        quote! {
                            mvutils::save::Savable::save(#name, saver);
                        }
                    }
                });

                quote! {
                    #name::#ident { #( #names ),* } => {
                        #save_variant;
                        #( #saves )*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let fields: Vec<_> = fields.unnamed.iter().enumerate().collect();
                let amount = fields.len();

                let (fields, _): (Vec<_>, Vec<_>) = fields.iter().partition(|(_, f)| filter(f));

                let fields = fields.into_iter().map(|(i, f)| (i, (f, get_custom(f)))).collect::<Vec<_>>();

                let saves = fields.iter().map(|(i, (_, custom))| {
                    let name = key(*i as u32);
                    if let Some((save, _)) = custom {
                        quote! {
                            #save(saver, #name);
                        }
                    } else {
                        quote! {
                            mvutils::save::Savable::save(#name, saver);
                        }
                    }
                });

                let mut names = Vec::with_capacity(amount);
                for i in 0..amount {
                    let key = key(i as u32);
                    names.push(quote! {
                        #key
                    });
                }

                quote! {
                    #name::#ident( #( #names ),* ) => {
                        #save_variant;
                        #( #saves )*
                    },
                }
            }
            Fields::Unit => {
                quote! {
                    #name::#ident => #save_variant,
                }
            }
        }
    });

    let load = e.variants.iter().enumerate().map(|(i, v)| {
        let ident = &v.ident;
        let i = i as u32;
        match &v.fields {
            Fields::Named(fields) => {
                let (fields, unsaved_fields): (Vec<_>, Vec<_>) =
                    fields.named.iter().partition(filter);

                let fields = fields.into_iter().map(|f| (f, get_custom(f))).collect::<Vec<_>>();

                let names = fields.iter().map(|(f, _)| {
                    let name = &f.ident;
                    quote! {
                        #name
                    }
                });

                let load_fields = fields.iter().map(|(f, custom)| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    if let Some((_, load)) = custom {
                        quote! {
                            let #name = #load(loader)?;
                        }
                    } else {
                        quote! {
                            let #name = <#ty as mvutils::save::Savable>::load(loader)?;
                        }
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

                let t1 = if fields.is_empty() { quote!{} } else { quote!{,} };

                quote! {
                    #i => {
                        #( #load_fields )*
                        #( #load_default_fields )*

                        Ok(#name::#ident {
                            #( #names ),*#t1
                            #( #init_default_struct ),*
                        })
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let fields: Vec<_> = fields.unnamed.iter().enumerate().collect();
                let amount = fields.len();

                let (fields, unsaved_fields): (Vec<_>, Vec<_>) = fields.iter().partition(|(_, f)| filter(f));

                let fields = fields.into_iter().map(|(i, f)| (i, (f, get_custom(f)))).collect::<Vec<_>>();

                let loads = fields.iter().map(|(i, (f, custom))| {
                    let name = key(*i as u32);
                    let ty = &f.ty;
                    if let Some((_, load)) = custom {
                        quote! {
                            let #name = #load(loader)?;
                        }
                    } else {
                        quote! {
                            let #name = <#ty as mvutils::save::Savable>::load(loader)?;
                        }
                    }
                });

                let unsaved_loads = unsaved_fields.iter().map(|(i, f)| {
                    let ty = &f.ty;
                    let key = key(*i as u32);
                    quote! {
                        let #key = <#ty as Default>::default();
                    }
                });

                let mut names = Vec::with_capacity(amount);
                for i in 0..amount {
                    let key = key(i as u32);
                    names.push(quote! {
                        #key
                    });
                }

                quote! {
                    #i => {
                        #( #loads )*
                        #( #unsaved_loads )*
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

    let load_variant = if varint {
        quote! { mvutils::save::custom::varint_load(loader)? }
    } else {
        quote! { #id_ty::load(loader)? }
    };

    let implementation = quote! {
        impl #impl_generics mvutils::save::Savable for #name #ty_generics #where_clause {
            fn save(&self, saver: &mut impl mvutils::save::Saver) {
                match self {
                    #( #save )*
                }
            }

            fn load(loader: &mut impl mvutils::save::Loader) -> Result<Self, String> {
                match #load_variant as u32 {
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
