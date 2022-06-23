use crate::helper::{self, DeriveInputHelper};
use super::generator;
use syn::{spanned::Spanned, AttributeArgs};
// use syn::{parse_quote};

pub fn generate(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs,) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_token_stream = proc_macro2::TokenStream::new();
    final_token_stream.extend(generate_struct(derive_input_helper, args)?);
    Ok(final_token_stream)
}

pub fn generate_struct(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs,) -> syn::Result<proc_macro2::TokenStream> {
    let (impl_generics, type_generics, where_clause) = derive_input_helper.value().generics.split_for_impl();

    // let fields = derive_input_helper.get_fields()?;
    // let field_idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    // let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let struct_ident = &derive_input_helper.value().ident;
    //  User
    let struct_name_literal = struct_ident.to_string();
    let struct_ident = &syn::Ident::new(&struct_name_literal, derive_input_helper.value().span());

    let struct_fields_quote = generator::fields_generator::generate_struct_fields_define(derive_input_helper)?;
    let struct_fields_init_default_quote = generator::fields_generator::generate_struct_fields_init_default_clauses(derive_input_helper)?;
    let orm_trait_functions_quote = generator::trait_functions_generator::generate_struct_orm_trait_functions_define(derive_input_helper, args)?;
    let struct_associations_quote = generator::association_generator::generate_associations_define(derive_input_helper, args)?;

    // model
    Ok(quote::quote! {
        // pub struct User {}
        #[derive(std::clone::Clone, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
        pub struct #struct_ident #impl_generics {
            #[serde(skip_serializing, skip_deserializing)]
            pub _persisted: std::option::Option<Box<#struct_ident #type_generics>>,
            #struct_fields_quote
        }
         // impl std::default::Default for User {}
        impl #impl_generics std::default::Default for #struct_ident #type_generics #where_clause {
            fn default() -> Self {
                Self {
                    _persisted: std::option::Option::None,
                    #struct_fields_init_default_quote
                }
            }
        }
        // impl Ormable for User {}
        impl #impl_generics sql_orm::Ormable for #struct_ident #type_generics #where_clause {
            #orm_trait_functions_quote
        }
        // impl User
        impl #impl_generics #struct_ident #type_generics #where_clause {
          const ORM: sql_orm::Orm::<Self> = sql_orm::Orm::<Self>::default();
          pub fn orm() -> sql_orm::Orm::<Self> { Self::ORM }
          #struct_associations_quote
        }
    })
}
