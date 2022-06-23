#[allow(unused_imports)]
pub(crate) mod helper;
pub(crate) mod orm;

use proc_macro::TokenStream;

/// use orm::prelude::*;
/// #[orm(table_name="users")]
/// struct User {
///     id: usize,
///     desc: String,
///     published: Option<bool>,
///     // ids: Vec<T>,
/// }
#[proc_macro_attribute]
pub fn orm(args: TokenStream, input: TokenStream) -> TokenStream {
    orm::create_orm(args, input)
}