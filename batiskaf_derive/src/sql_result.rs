use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote, DeriveInput, GenericParam, Generics};

use crate::column::{columns_with_fields, parse_attributes};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let attrs = parse_attributes(&input.attrs);
    let cs = columns_with_fields(&input.data, attrs.word.contains("default"));
    for (c, f) in cs.iter() {
        if c.skip() && !c.default() {
            return syn::Error::new(
                f.ident.span(),
                format!("field with `skip` attribute must have `default` attribute"),
            )
            .to_compile_error();
        }
    }
    let tokens = cs.iter().map(|cf| {
        let name = &cf.1.ident;
        let param = &format!("{}", cf.0.name());
        if cf.0.skip() {
            quote_spanned! { cf.1.span() =>
                #name: ::std::default::Default::default()
            }
        } else if cf.0.default() {
            quote_spanned! { cf.1.span() =>
                #name: {
                    let x = row.get_checked(#param);
                    if let Err(::rusqlite::Error::InvalidColumnName(_)) = x {
                        ::std::default::Default::default()
                    } else {
                        x?
                    }
                }
            }
        } else {
            quote_spanned! { cf.1.span() =>
                #name: row.get_checked(#param)?
            }
        }
    });
    quote! {
        impl #impl_generics ::batiskaf::SqlResult for #name #ty_generics #where_clause {
            fn from_row(row: &::rusqlite::Row) -> rusqlite::Result<Self> {
                Ok(#name {
                    #(#tokens),*
                })
            }
        }
    }
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::rusqlite::types::FromSql));
            type_param
                .bounds
                .push(parse_quote!(::std::default::Default));
        }
    }
    generics
}
