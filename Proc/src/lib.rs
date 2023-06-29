use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Attribute, Data, Meta, Field};

#[proc_macro_derive(Savable, attributes(unsaved))]
pub fn derive_savable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (fields, unsaved_fields) = match &input.data {
        Data::Struct(s) => {
            let filter = |f: &&Field| !f.attrs.iter().any(is_unsaved);
            match &s.fields {
                Fields::Named(fields) => fields.named.iter().partition(filter),
                Fields::Unnamed(fields ) => fields.unnamed.iter().partition(filter),
                Fields::Unit => (vec![], vec![])
            }
        }
        Data::Enum(e) => todo!("Enum savable generation is planned and being worked on!"),
        Data::Union(_) => panic!("Deriving Savable for unions is not supported!")
    };

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

fn is_unsaved(attr: &Attribute) -> bool {
    if let Meta::Path(ref p) = attr.meta {
        p.segments.iter().any(|s| s.ident == "unsaved")
    } else {
        false
    }
}