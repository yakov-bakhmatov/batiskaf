use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::column::{columns, Column};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let cs = columns(&input.data);
    let cs: Vec<Column> = cs.into_iter().filter(|c| !c.skip()).collect();
    let values: Vec<String> = cs
        .iter()
        .filter(|c| !c.primary_key())
        .map(|c| c.name())
        .map(|n| format!("{} = :{}", n, n))
        .collect();
    let keys: Vec<String> = cs
        .iter()
        .filter(|c| c.primary_key())
        .map(|c| c.name())
        .map(|n| format!("{} = :{}", n, n))
        .collect();
    if values.is_empty() {
        return syn::Error::new(
            name.span(),
            format!("struct {} must contain non `primary_key` field", name),
        )
        .to_compile_error();
    }
    if keys.is_empty() {
        return syn::Error::new(
            name.span(),
            format!("struct {} must contain `primary_key` field", name),
        )
        .to_compile_error();
    }
    let sql = format!(
        "UPDATE {{}} SET {} WHERE {}",
        values.join(", "),
        keys.join(" AND ")
    );
    quote! {
        impl #impl_generics ::batiskaf::SqlUpdate for #name #ty_generics #where_clause {
            fn update_statement(table: &str) -> String {
                format!(#sql, table)
            }
        }
    }
}
