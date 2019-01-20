use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote, DeriveInput, GenericParam, Generics, Field};

use crate::column::{columns_with_fields, Column};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let cs = columns_with_fields(&input.data, false);
    let cs: Vec<(Column, &Field)> = cs.into_iter().filter(|cf| !cf.0.skip()).collect();
    let tokens = cs.iter().map(|cf| {
        let name = &cf.1.ident;
        let param = &format!(":{}", cf.0.name());
        quote_spanned! { cf.1.span() =>
            if let Ok(Some(_)) = stmt.parameter_index(#param) {
                params.push((#param, &self.#name as &::rusqlite::types::ToSql));
            }
        }
    });
    quote! {
        impl #impl_generics ::batiskaf::SqlParam for #name #ty_generics #where_clause {
            fn to_named_params(&self, stmt: &::rusqlite::Statement) -> ::std::vec::Vec<(&str, &dyn ::rusqlite::types::ToSql)> {
                let mut params = ::std::vec::Vec::new();
                #(#tokens)*
                params
            }
        }
    }
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(::rusqlite::types::ToSql));
            }
    }
    generics
}
