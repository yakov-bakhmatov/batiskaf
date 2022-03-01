use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::column::{columns, Column};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let cs = columns(&input.data);
    let cs: Vec<Column> = cs.into_iter().filter(|c| !c.skip()).collect();
    let names: Vec<String> = cs.iter().map(|c| c.name()).collect();
    let params: Vec<String> = names.iter().map(|c| format!(":{}", c)).collect();
    let keys: Vec<String> = cs
        .iter()
        .filter(|c| c.primary_key())
        .map(|c| c.name())
        .collect();
    let values: Vec<String> = cs
        .iter()
        .filter(|c| !c.primary_key())
        .map(|c| format!("{0} = excluded.{0}", c.name()))
        .collect();
    if keys.is_empty() {
        return syn::Error::new(
            name.span(),
            format!("struct {} must contain `primary_key` field", name),
        )
        .to_compile_error();
    }
    let sql = if values.is_empty() {
        format!(
            "INSERT INTO {{}} ({}) VALUES ({}) ON CONFLICT ({}) DO NOTHING",
            names.join(", "),
            params.join(", "),
            keys.join(", "),
        )
    } else {
        format!(
            "INSERT INTO {{}} ({}) VALUES ({}) ON CONFLICT ({}) DO UPDATE SET {}",
            names.join(", "),
            params.join(", "),
            keys.join(", "),
            values.join(", "),
        )
    };
    quote! {
        impl #impl_generics ::batiskaf::SqlUpsert for #name #ty_generics #where_clause {
            fn upsert_statement(table: &str) -> String {
                format!(#sql, table)
            }
        }
    }
}
