mod derive_input_helper;
pub use derive_input_helper::DeriveInputHelper;

use syn::{AttributeArgs, spanned::Spanned};

pub fn parse_attrs_to_metas(attrs: &Vec<syn::Attribute>) -> syn::Result<Vec<syn::Meta>> {
  attrs.iter().map(|attr| attr.parse_meta()).collect::<syn::Result<Vec<syn::Meta>>>()
}

fn get_macro_attr_value_from_meta(meta: &syn::Meta, attr_name: &str) -> syn::Result<Option<syn::Ident>> {
  if let syn::Meta::NameValue(kv) = meta {
      if let syn::Lit::Str(ref ident_str) = kv.lit {
          if kv.path.is_ident(attr_name) {
              return Ok(Some(syn::Ident::new(ident_str.value().as_str(), kv.span())));
          }
      }
  }
  Ok(None)
}

pub fn get_macro_nested_attr_value_ident(nested_metas: Vec<&syn::NestedMeta>, attr_name: &str, namespaces: Option<Vec<&str>>, namespace_allow_attrs: Option<Vec<&str>>) -> syn::Result<Option<syn::Ident>> {
  match namespaces {
      Some(namespaces) => {
          if let Some(nested_metas_vec) = get_namespace_nested_metas_vec_from_nested_metas(nested_metas, namespaces)? {
              // 多个同名，寻找第一个满足条件
              for nested_metas in nested_metas_vec.iter() {
                  // 校验每个属性是否有拼错或者不支持
                  for nested_meta in nested_metas.iter() {
                      if let syn::NestedMeta::Meta(meta) = nested_meta {
                          check_meta_available(meta, &namespace_allow_attrs)?;
                      }
                  }
                  for nested_meta in nested_metas.iter() {
                      if let syn::NestedMeta::Meta(meta) = nested_meta {
                          if let Some(ident) = get_macro_attr_value_from_meta(meta, attr_name)? {
                              return Ok(Some(ident))
                          }
                      }
                  }
              }
          }
      },
      None => {
          return get_macro_nested_attr_value_ident(nested_metas, attr_name, Some(vec![]), namespace_allow_attrs);
      },
  }
  Ok(None)
}

pub fn get_namespace_nested_metas_vec(namespaces: Vec<&str>, derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<Option<Vec<Vec<syn::NestedMeta>>>> {
  let mut total_vec = vec![];
    let metas = parse_attrs_to_metas(&derive_input_helper.value().attrs)?;
    if let Some(mut vec) = get_namespace_nested_metas_vec_from_metas(metas.iter().collect::<Vec<_>>(), namespaces.clone())? {
        total_vec.append(&mut vec)
    }
    if let Some(mut vec) = get_namespace_nested_metas_vec_from_nested_metas(args.iter().collect(), namespaces.clone())? {
        total_vec.append(&mut vec)
    }
    if total_vec.len() > 0 {
        Ok(Some(total_vec.into_iter().map(|i| i.into_iter().map(|i| i.clone()).collect::<Vec<_>>()).collect::<Vec<_>>()))
    } else {
        Ok(None)
    }
}

// 可能会存在多个同名属性，所以要用vec来存储
pub fn get_namespace_nested_metas_vec_from_metas<'a>(metas: Vec<&'a syn::Meta>, namespaces: Vec<&str>) -> syn::Result<Option<Vec<Vec<&'a syn::NestedMeta>>>> {
  let mut namespace_nested_metas_vec = vec![];
  if namespaces.len() != 0 {
      for meta in metas {
          if let syn::Meta::List(meta_list) = meta {
              for segment in meta_list.path.segments.iter() {
                  let mut namespaces: Vec<&str> = namespaces.iter().map(|&i| i).collect();
                  let first_namespace = namespaces.remove(0);
                  if segment.ident == first_namespace {
                      let nested_metas = meta_list.nested.iter().map(|i| i).collect();
                      if let Some(mut nested_metas_vec) = get_namespace_nested_metas_vec_from_nested_metas(nested_metas, namespaces)? {
                          namespace_nested_metas_vec.append(&mut nested_metas_vec);
                      }
                  }
              }
          }
      }
  }
  if namespace_nested_metas_vec.len() > 0 {
      Ok(Some(namespace_nested_metas_vec))
  } else {
      Ok(None)
  }
}

// 可能会存在多个同名属性，所以要用vec来存储
pub fn get_namespace_nested_metas_vec_from_nested_metas<'a>(nested_metas: Vec<&'a syn::NestedMeta>, namespaces: Vec<&str>) -> syn::Result<Option<Vec<Vec<&'a syn::NestedMeta>>>> {
  let mut namespace_nested_metas_vec = vec![];
  if namespaces.len() != 0 {
      let metas = nested_metas.iter().filter_map(|item| {
          if let syn::NestedMeta::Meta(meta) = item { Some(meta) } else { None }
      }).collect::<Vec<_>>();
      for meta in metas {
          if let  syn::Meta::List(meta_list) = meta {
              for segment in meta_list.path.segments.iter() {
                  let mut namespaces: Vec<&str> = namespaces.iter().map(|&i| i).collect();
                  let first_namespace = namespaces.remove(0);
                  if segment.ident == first_namespace {
                      let nested_metas = meta_list.nested.iter().map(|i| i).collect();
                      if let Some(mut nested_metas_vec) = get_namespace_nested_metas_vec_from_nested_metas(nested_metas, namespaces)? {
                          namespace_nested_metas_vec.append(&mut nested_metas_vec);
                      }
                  }
              }
          }
      }
  } else {
      namespace_nested_metas_vec.push(nested_metas);
      return Ok(Some(namespace_nested_metas_vec))
  }
  if namespace_nested_metas_vec.len() > 0 {
      Ok(Some(namespace_nested_metas_vec))
  } else {
      Ok(None)
  }
}

pub fn get_macro_attr_value_ident(metas: Vec<&syn::Meta>, attr_name: &str, namespaces: Option<Vec<&str>>, namespace_allow_attrs: Option<Vec<&str>>) -> syn::Result<Option<syn::Ident>> {
  match namespaces {
      Some(namespaces) => {
          // eg: #[orm="users"]
          if namespaces.len() == 0 {
              for meta in metas {
                  // 校验属性是否有拼错或者不支持
                  check_meta_available(meta, &namespace_allow_attrs)?;
                  if let Some(ident) = get_macro_attr_value_from_meta(&meta, attr_name)? {
                      return Ok(Some(ident))
                  }
              }
          } else { // eg: #[orm(name="uuid")], #[orm(column(name="uuid"))]
              if let Some(nested_metas_vec) = get_namespace_nested_metas_vec_from_metas(metas, namespaces)? {
                  for nested_metas in nested_metas_vec.iter() {
                      return get_macro_nested_attr_value_ident(nested_metas.clone(), attr_name, None, namespace_allow_attrs)
                  }
              }
          }
      },
      _ => {
          return get_macro_attr_value_ident(metas, attr_name, Some(vec![]), namespace_allow_attrs);
      },
  }
  Ok(None)
}

fn check_meta_available(meta: &syn::Meta, allow_attrs: &Option<Vec<&str>>) -> syn::Result<bool> {
  if let Some(allow_attrs) = allow_attrs {
      if let syn::Meta::NameValue(kv) = meta {
          let mut can_kv_path_allow = false;
          for attr_name in allow_attrs {
              if kv.path.is_ident(attr_name) {
                  can_kv_path_allow = true;
              }
          }
          if !can_kv_path_allow {
              return Err(syn::Error::new_spanned(&kv.path, format!("Support Macro Attr List: {:?}", &allow_attrs)));
          }
      }
  }
  Ok(true)
}