use crate::helper::{self, DeriveInputHelper};

pub fn generate_struct_fields_define(derive_input_helper: &DeriveInputHelper) -> syn::Result<proc_macro2::TokenStream> {
  let fields = derive_input_helper.get_fields()?;
  let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
  let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

  Ok(quote::quote! {
    #(pub #idents: #types),*
  })
}

pub fn generate_struct_fields_init_default_clauses(derive_input_helper: &DeriveInputHelper) -> syn::Result<proc_macro2::TokenStream> {
    let fields = derive_input_helper.get_fields()?;
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    Ok(quote::quote! {
        #(#idents: std::option::Option::None),*
    })
}