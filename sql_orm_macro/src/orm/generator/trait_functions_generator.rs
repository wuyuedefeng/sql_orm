use crate::helper::{self, DeriveInputHelper};
use syn::{spanned::Spanned, AttributeArgs};

pub fn generate_struct_orm_trait_functions_define(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
  let mut final_token_stream = proc_macro2::TokenStream::new();

  let fields = derive_input_helper.get_fields()?;
  let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();

  let arg_allow_attrs = vec!["table_name", "primary_key", "locking_column"];

  // table_name
  {
    if let Some(ident) = helper::get_macro_nested_attr_value_ident(args.iter().collect(), "table_name", None, Some(arg_allow_attrs.clone()))? {
      let token_stream = quote::quote! {
          fn table_name() -> String {
            stringify!(#ident).to_string()
          }
      };
      final_token_stream.extend(token_stream);
    }
  }

  // primary_key
  {
    if let Some(ident) = helper::get_macro_nested_attr_value_ident(args.iter().collect(), "primary_key", None, Some(arg_allow_attrs.clone()))? {
      // if let Some(table_name_ident) = get_struct_attr(args, "table_name")? {
      let token_stream = quote::quote! {
          fn primary_key() -> &'static str {
            stringify!(#ident)
          }
      };
      final_token_stream.extend(token_stream);
    }
  }

  // table_column_names
  {
    let mut idents: Vec<_> = vec![];
    for f in fields.iter() {
      if let Some(ident) = &f.ident {
        let metas = helper::parse_attrs_to_metas(&f.attrs)?;
        if let Some(rename_ident) = helper::get_macro_attr_value_ident(metas.iter().collect(), "table_column_name", Some(vec!["orm"]), None)? {
          idents.push(rename_ident);
        } else {
          idents.push(ident.clone());
        }
      }
    }
    final_token_stream.extend(quote::quote! {
      fn table_column_names() -> Vec<&'static str> {
        vec![
          #(stringify!(#idents),)*
        ]
      }
    })
  }

  // attr_names
  {
    final_token_stream.extend(quote::quote! {
      fn attr_names() -> Vec<&'static str> {
        vec![
          #(stringify!(#idents),)*
        ]
      }
    })
  }

  // attr_name_to_table_column_name
  {
    let mut match_tokens = vec![];
    for f in fields.iter() {
      if let Some(ident) = &f.ident {
        let metas = helper::parse_attrs_to_metas(&f.attrs)?;
        if let Some(rename_ident) = helper::get_macro_attr_value_ident(metas.iter().collect(), "table_column_name", Some(vec!["orm"]), None)? {
          match_tokens.push(quote::quote! {
            stringify!(#ident) => std::result::Result::Ok(stringify!(#rename_ident))
          });
        } else {
          match_tokens.push(quote::quote! {
            stringify!(#ident) => std::result::Result::Ok(stringify!(#ident))
          });
        }
      }
    }
    final_token_stream.extend(quote::quote! {
      fn attr_name_to_table_column_name<'a>(attr_name: &'a str) -> Result<&'a str, sql_orm::SqlError> {
        match attr_name {
          #(#match_tokens,)*
          _ => std::result::Result::Err(sql_orm::SqlError::Message(format!("Error: attr_name_to_table_column_name: {} Not Found", attr_name)))
        }
      }
    })
  }

  // fn table_column_name_to_attr_name<'a>(table_column_name: &'a str) -> Result<&'a str, sql_orm::SqlError>;
  {
    let mut match_tokens = vec![];
    for f in fields.iter() {
      if let Some(ident) = &f.ident {
        let metas = helper::parse_attrs_to_metas(&f.attrs)?;
        if let Some(rename_ident) = helper::get_macro_attr_value_ident(metas.iter().collect(), "table_column_name", Some(vec!["orm"]), None)? {
          match_tokens.push(quote::quote! {
              stringify!(#rename_ident) => std::result::Result::Ok(stringify!(#ident))
          });
        } else {
          match_tokens.push(quote::quote! {
              stringify!(#ident) => std::result::Result::Ok(stringify!(#ident))
          });
        }
      }
    }
    final_token_stream.extend(quote::quote! {
      fn table_column_name_to_attr_name<'a>(table_column_name: &'a str) -> Result<&'a str, sql_orm::SqlError> {
        match table_column_name {
          #(#match_tokens,)*
          _ => std::result::Result::Err(sql_orm::SqlError::Message(format!("Error: table_column_name_to_attr_name: {} Not Found", table_column_name)))
        }
      }
    })
  }

  // fn get_json_value_from_attr_name__(_attr_name: &str) -> String { panic!("Error: get_sql_value_string_from_attr_name Not Impl") }
  {
    final_token_stream.extend(quote::quote! {
      fn get_json_value_from_attr_name__(&self, attr_name: &str) -> Option<sql_orm::serde_json::Value> {
        match attr_name {
          #(stringify!(#idents) => {
            if let Some(ref value) = self.#idents {
              std::option::Option::Some(sql_orm::serde_json::json!(value))
            } else {
              std::option::Option::None
            }
          },)*
          _ => {
            eprintln!("Error: get_sql_value_string_from_attr_name Attr {} Not Found", attr_name);
            std::option::Option::None
          }
        }
      }
    })
  }

  Ok(quote::quote! {
    #final_token_stream
  })
}