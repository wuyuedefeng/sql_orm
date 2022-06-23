use crate::helper::{self, DeriveInputHelper};
use syn::{spanned::Spanned, AttributeArgs};

pub fn generate_belongs_to_associations_define(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs,) -> syn::Result<proc_macro2::TokenStream> {
  let mut final_token_stream = proc_macro2::TokenStream::new();
  if let Some(belongs_to_args_vec) = helper::get_namespace_nested_metas_vec(vec!["belongs_to"], derive_input_helper, args)? {
    for belongs_to_args in belongs_to_args_vec.into_iter() {
        if let Some((association_ident, belongs_to_struct_ident, foreign_key_ident)) = handle_association_attributes(belongs_to_args.iter().collect(), derive_input_helper, args,)? {
          final_token_stream.extend(quote::quote! {
            pub fn #association_ident(&self) -> sql_orm::sql_gen::manager::SelectManager<#belongs_to_struct_ident> {
              let mut query = #belongs_to_struct_ident::orm().query();
              if let Some(ref foregin_key) = self.#foreign_key_ident {
                query.r#where(sql_orm::serde_json::json!([format!("{} = ?", #belongs_to_struct_ident::orm().primary_key()), foregin_key]));
              } else {
                query.r#where(sql_orm::serde_json::json!([format!("{} = ?", #belongs_to_struct_ident::orm().primary_key()), null]));
              }
              query.limit(1);
              query
            }
          });
          let join_association_ident = syn::Ident::new(&format!("{}_join_string__", association_ident.to_string()), association_ident.span(),);
          final_token_stream.extend(quote::quote! {
            pub fn #join_association_ident() -> String {
              let assocation_table_name = #belongs_to_struct_ident::orm().table_name();
              let assocation_primary_key = #belongs_to_struct_ident::orm().primary_key();
              let self_table_name = Self::orm().table_name();
              let self_foreign_key = stringify!(#foreign_key_ident);
              // check foreign_key exists
              let self_table_columns = Self::orm().table_column_names();
              if !self_table_columns.contains(&self_foreign_key) {
                panic!("belongs_to foreign_key({}) Not In Table {} Columns: {:?}", self_foreign_key, self_table_name, self_table_columns);
              }

              format!("INNER JOIN {} ON {}.{} = {}.{}", self_table_name, assocation_table_name, assocation_primary_key, self_table_name, self_foreign_key)
            }
          });
        }
    }
  }
  Ok(final_token_stream)
}

pub fn handle_association_attributes(belongs_to_args: Vec<&syn::NestedMeta>, _derive_input_helper: &DeriveInputHelper, _args: &AttributeArgs) -> syn::Result<Option<(syn::Ident, syn::Ident, syn::Ident)>> {
  if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = belongs_to_args.get(0).unwrap() {
      let association_name = name.value();
      let association_ident = syn::Ident::new(&association_name, name.span());
      if let Some(belongs_to_struct_ident) = helper::get_macro_nested_attr_value_ident(belongs_to_args.clone(), "struct", None, None)? {
          let belongs_to_struct_name = belongs_to_struct_ident.to_string();
          let foreign_key = format!("{}_id", inflector::cases::snakecase::to_snake_case(&belongs_to_struct_name));
          let mut foreign_key_ident = syn::Ident::new(&foreign_key, belongs_to_struct_ident.span());
          if let Some(custom_foreign_key_ident) = helper::get_macro_nested_attr_value_ident(belongs_to_args.clone(), "foreign_key", None, None)? {
              // foreign_key = custom_foreign_key_ident.to_string();
              foreign_key_ident = custom_foreign_key_ident;
          }
          return Ok(Some((association_ident, belongs_to_struct_ident, foreign_key_ident)))
      } else {
          return Err(syn::Error::new_spanned(name, "Error: Loss Attr struct".to_string()))
      }
  }
  Ok(None)
}