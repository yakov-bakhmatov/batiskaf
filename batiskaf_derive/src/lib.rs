/*

атрибуты:
- column = "" - переименование столбца
- primary_key - поле является первичным ключом (используется в SqlUpdate и SqlDelete)
- autogenerated - поле является автогенерируемым и пропускается в SqlInsert
- skip - поле пропускается
- default - если поле пропущено или его нет в строке (SqlResult), использовать значение по-умолчанию


SqlParam
применяется только к именованным структурам
атрибуты полей:
- column
- skip

SqlResult
применяется только к именованным структурам
все generic-типы в объявлении структуры получают дополнительные ограничения: Default + FromSql
атрибут структуры:
- default - все поля получают атрибут default
атрибуты полей:
- column
- skip
- default

SqlInsert
атрибуты полей:
- column
- autogenerated
- skip

SqlUpdate
атрибуты полей:
- column
- primary_key
- skip

SqlDelete
атрибуты полей:
- column
- primary_key
- skip

*/

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{self, parse_macro_input, DeriveInput};

mod column;
mod sql_delete;
mod sql_insert;
mod sql_param;
mod sql_result;
mod sql_update;

#[proc_macro_derive(SqlParam, attributes(batiskaf))]
pub fn derive_sql_param(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    sql_param::derive(input).into()
}

#[proc_macro_derive(SqlResult, attributes(batiskaf))]
pub fn derive_sql_result(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    sql_result::derive(input).into()
}

#[proc_macro_derive(SqlInsert, attributes(batiskaf))]
pub fn derive_sql_insert(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    sql_insert::derive(input).into()
}

#[proc_macro_derive(SqlUpdate, attributes(batiskaf))]
pub fn derive_sql_update(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    sql_update::derive(input).into()
}

#[proc_macro_derive(SqlDelete, attributes(batiskaf))]
pub fn derive_sql_delete(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    sql_delete::derive(input).into()
}
