use crate::helper::{self, DeriveInputHelper};
use syn::{spanned::Spanned, AttributeArgs};

pub fn generate_has_many_associations_define(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
  let mut final_token_stream = proc_macro2::TokenStream::new();
  Ok(final_token_stream)
}