use crate::helper::{self, DeriveInputHelper};
use syn::{spanned::Spanned, AttributeArgs};

pub fn generate_has_one_associations_define(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs,) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_token_stream = proc_macro2::TokenStream::new();
    if let Some(has_one_args_vec) = helper::get_namespace_nested_metas_vec(vec!["has_one"], derive_input_helper, args)? {
        for has_one_args in has_one_args_vec.into_iter() {
            if let Some(has_one_through_token_stream) = generate_has_one_through_associations(has_one_args.iter().collect(), derive_input_helper, args,)? {
                final_token_stream.extend(has_one_through_token_stream);
            } else if let Some((association_ident, has_one_struct_ident, foreign_key_ident)) = handle_association_attributes(has_one_args.iter().collect(), derive_input_helper, args,)? {
                final_token_stream.extend(quote::quote! {
                    pub fn #association_ident(&self) -> sql_orm::sql_gen::manager::SelectManager<#has_one_struct_ident> {
                        let assocation_table_name = #has_one_struct_ident::orm().table_name();
                        let assocation_foregin_key = stringify!(#foreign_key_ident);
                        // // check foreign_key exists
                        let association_table_columns = #has_one_struct_ident::orm().table_column_names();
                        if !association_table_columns.contains(&assocation_foregin_key) {
                            panic!("Error: has_one foreign_key({}) Not In Table {} Columns: {:?}", assocation_foregin_key, assocation_table_name, association_table_columns);
                        }

                        let mut query = #has_one_struct_ident::orm().query();
                        let attr_primary_json_value = self.get_json_value_from_attr_name__(Self::orm().primary_key());
                        query.r#where(sql_orm::serde_json::json!([format!("{} = ?", assocation_foregin_key), attr_primary_json_value]));
                        query.limit(1);
                        query
                    }
                });
                let join_association_ident = syn::Ident::new(&format!("{}_join_string__", association_ident.to_string()), association_ident.span(),);
                final_token_stream.extend(quote::quote! {
                    pub fn #join_association_ident() -> String {
                    let assocation_table_name = #has_one_struct_ident::table_name();
                    let assocation_foregin_key = stringify!(#foreign_key_ident);
                    // check foreign_key exists
                    let association_table_columns = #has_one_struct_ident::table_column_names();
                    if !association_table_columns.contains(&assocation_foregin_key) {
                    panic!("Error: has_one foreign_key({}) Not In Table {} Columns: {:?}", assocation_foregin_key, assocation_table_name, association_table_columns);
                    }

                    let self_table_name = Self::table_name();
                    let self_primary_key = Self::primary_key();
                        format!("INNER JOIN {} ON {}.{} = {}.{}", self_table_name, assocation_table_name, assocation_foregin_key, self_table_name, self_primary_key)
                    }
                });
            }
        }
    }
    Ok(final_token_stream)
}

pub fn handle_association_attributes(has_one_args: Vec<&syn::NestedMeta>, derive_input_helper: &DeriveInputHelper, _args: &AttributeArgs,) -> syn::Result<Option<(syn::Ident, syn::Ident, syn::Ident)>> {
    let self_struct_ident = &derive_input_helper.value().ident;
    let self_struct_name = format!("{}", self_struct_ident.to_string());
    let self_struct_ident = &syn::Ident::new(&self_struct_name, derive_input_helper.value().span());

    if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = has_one_args.get(0).unwrap() {
        let association_name = name.value();
        let association_ident = syn::Ident::new(&association_name, name.span());
        if let Some(has_one_struct_ident) = helper::get_macro_nested_attr_value_ident(has_one_args.clone(), "struct", None, None)? {
            let foreign_key = format!("{}_id", inflector::cases::snakecase::to_snake_case(&self_struct_ident.to_string()));
            let mut foreign_key_ident = syn::Ident::new(&foreign_key, has_one_struct_ident.span());
            if let Some(custom_foreign_key_ident) = helper::get_macro_nested_attr_value_ident(has_one_args.clone(), "foreign_key", None, None,)? {
                // foreign_key = custom_foreign_key_ident.to_string();
                foreign_key_ident = custom_foreign_key_ident;
            }
            return Ok(Some((association_ident, has_one_struct_ident, foreign_key_ident,)));
        } else {
            return Err(syn::Error::new_spanned(name, "loss attr: struct".to_string(),));
        }
    }
    Ok(None)
}

pub fn handle_association_through_attributes(through_args: Vec<&syn::NestedMeta>, derive_input_helper: &DeriveInputHelper, _args: &AttributeArgs,) -> syn::Result<Option<(syn::Ident, syn::Ident, syn::Ident)>> {
    let self_struct_ident = &derive_input_helper.value().ident;
    let self_struct_name = format!("{}", self_struct_ident.to_string());
    let self_struct_ident = &syn::Ident::new(&self_struct_name, derive_input_helper.value().span());

    if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = through_args.get(0).unwrap() {
        let through_association_name = name.value();
        if let Some(through_struct_ident) =
            helper::get_macro_nested_attr_value_ident(through_args.clone(), "struct", None, None)?
        {
            // let through_struct_name = through_struct_ident.to_string();
            let through_foreign_key = format!(
                "{}_id",
                inflector::cases::snakecase::to_snake_case(&self_struct_ident.to_string())
            );
            let mut through_foreign_key_ident =
                syn::Ident::new(&through_foreign_key, through_struct_ident.span());
            if let Some(custom_foreign_key_ident) = helper::get_macro_nested_attr_value_ident(
                through_args.clone(),
                "foreign_key",
                None,
                None,
            )? {
                // foreign_key = custom_foreign_key_ident.to_string();
                through_foreign_key_ident = custom_foreign_key_ident;
            }
            let mut through_source_ident = syn::Ident::new(&through_association_name, name.span());
            if let Some(source_ident) = helper::get_macro_nested_attr_value_ident(through_args.clone(), "source", None, None,)? {
                through_source_ident = source_ident
            }
            return Ok(Some((through_struct_ident, through_foreign_key_ident, through_source_ident,)));
        } else {
            return Err(syn::Error::new_spanned(name, "Error: loss attr struct".to_string(),));
        }
    }
    Ok(None)
}

pub fn generate_has_one_through_associations(has_one_args: Vec<&syn::NestedMeta>, derive_input_helper: &DeriveInputHelper, args: &AttributeArgs,) -> syn::Result<Option<proc_macro2::TokenStream>> {
    if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = has_one_args.get(0).unwrap() {
        let association_name = name.value();
        let association_ident = syn::Ident::new(&association_name, name.span());
        if let Some(has_one_struct_ident) = helper::get_macro_nested_attr_value_ident(has_one_args.clone(), "struct", None, None)? {
            if let Some(through_ident) = helper::get_macro_nested_attr_value_ident(has_one_args.clone(), "through", None, None,)? {
                let through_args_vec = get_through_nested_metas_recursion(through_ident.to_string(), derive_input_helper, args,)?;
                let mut through_recursion_join_strings_token_stream = vec![];

                let mut last_through_struct_ident = None;
                let mut last_through_foreign_key_ident = None;
                // source字段指定内部的关联名称，默认直接使用自己的关联名称作为内部的关联名称
                let mut in_through_association_ident = association_ident.clone();
                if let Some(source_ident) = helper::get_macro_nested_attr_value_ident(has_one_args.clone(), "source", None, None,)? {
                    in_through_association_ident = source_ident
                }

                // let through_args_vec_length = through_args_vec.len();
                for through_args in through_args_vec.into_iter() {
                    if let Some((through_struct_ident, through_foreign_key_ident, through_source_ident,)) = handle_association_through_attributes(through_args.iter().collect(), derive_input_helper, args,)? {
                        last_through_struct_ident = Some(through_struct_ident.clone());
                        last_through_foreign_key_ident = Some(through_foreign_key_ident.clone());

                        let through_join_string_ident = syn::Ident::new(&format!("{}_join_string__", in_through_association_ident.to_string()), in_through_association_ident.span(),);
                        through_recursion_join_strings_token_stream.push(quote::quote! {
                            #through_struct_ident::#through_join_string_ident(),
                        });

                        in_through_association_ident = through_source_ident;
                    }
                }

                if let (Some(last_through_struct_ident), Some(last_through_foreign_key_ident)) = (last_through_struct_ident, last_through_foreign_key_ident) {
                    let mut final_token_stream = proc_macro2::TokenStream::new();
                    final_token_stream.extend(quote::quote! {
                        pub fn #association_ident(&self) -> sql_orm::sql_gen::manager::SelectManager<#has_one_struct_ident> {
                            let join_strings = vec![#(#through_recursion_join_strings_token_stream)*];
                            let full_join_string = join_strings.join(" ");

                            let mut query = #has_one_struct_ident::query();
                            let attr_primary_json_value = self.get_json_value_from_attr_name__(Self::orm().primary_key());
                            query.r#where(sql_orm::serde_json::json!([format!("{} = ?", assocation_foregin_key, attr_primary_json_value)]));
                            query.limit(1);
                            query
                        }
                    });
                    let join_association_ident = syn::Ident::new(
                        &format!("{}_join_string__", association_ident.to_string()),
                        association_ident.span(),
                    );
                    final_token_stream.extend(quote::quote! {
                        pub fn #join_association_ident() -> String {
                            let mut join_strings = vec![#(#through_recursion_join_strings_token_stream)*];

                            let assocation_table_name = #last_through_struct_ident::table_name();
                            let assocation_foregin_key = stringify!(#last_through_foreign_key_ident);
                            let self_table_name = Self::table_name();
                            let self_primary_key = Self::primary_key();

                            join_strings.push(format!("INNER JOIN {} ON {}.{} = {}.{}", self_table_name, assocation_table_name, assocation_foregin_key, self_table_name, self_primary_key));
                            join_strings.join(" ")
                        }
                    });
                    return Ok(Some(final_token_stream));
                }
            }
        } else {
            return Err(syn::Error::new_spanned(name, "Error: loss attr struct".to_string(),));
        }
    }
    Ok(None)
}

fn get_through_nested_metas_recursion(through_name: String, derive_input_helper: &DeriveInputHelper, args: &AttributeArgs,) -> syn::Result<Vec<Vec<syn::NestedMeta>>> {
    let mut through_recursion = vec![];
    let mut find_item_args = None;
    // if let Some(has_many_args_vec) = helper::get_namespace_nested_metas_vec_from_nested_metas(args.clone(), vec!["has_many"])? {
    //     for has_many_args in has_many_args_vec.into_iter() {
    //         if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = has_many_args.get(0).unwrap() {
    //             if name.value() == through_name {
    //                 find_item_args = Some(has_many_args);
    //                 break;
    //             }
    //         }
    //     }
    // }
    if let Some(has_one_args_vec) = helper::get_namespace_nested_metas_vec(vec!["has_one"], derive_input_helper, args)? {
        for has_one_args in has_one_args_vec.into_iter() {
            if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = has_one_args.get(0).unwrap() {
                if name.value() == through_name {
                    find_item_args = Some(has_one_args);
                    break;
                }
            }
        }
    }

    if let Some(find_item_args) = find_item_args {
        through_recursion.push(find_item_args.clone());
        if let Some(inner_through_ident) = helper::get_macro_nested_attr_value_ident(find_item_args.iter().collect(), "through", None, None,)? {
            through_recursion.append(&mut get_through_nested_metas_recursion(inner_through_ident.to_string(), derive_input_helper, args,)?);
        }
    }

    Ok(through_recursion)
}
