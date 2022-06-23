pub mod prelude;
pub use prelude::{orm};
pub use prelude::{sql_gen, SqlGen, SqlError, Ormable, thiserror, regex, once_cell, inflector, chrono};
pub use serde;
pub use serde_json;

use std::{marker::PhantomData};
pub use sql_gen::{manager};

pub struct Orm<T: Ormable> {
  _marker: PhantomData<T>,
}
impl<T: Ormable> Orm<T> {
  pub const fn default() -> Self {
    Orm {
      _marker: PhantomData,
    }
  }
  pub fn table_name(&self) -> String { T::table_name() }
  pub fn table_column_names(&self) -> Vec<&'static str> { T::table_column_names() }
  pub fn attr_names(&self) -> Vec<&'static str> { T::attr_names() }
  pub fn attr_name_to_table_column_name<'a>(&self, attr_name: &'a str) -> Result<&'a str, crate::SqlError> { T::attr_name_to_table_column_name(attr_name) }
  pub fn table_column_name_to_attr_name<'a>(&self, table_column_name: &'a str) -> Result<&'a str, crate::SqlError> { T::table_column_name_to_attr_name(table_column_name) }

  pub fn primary_key(&self) -> &'static str { T::primary_key() }
  pub fn id(&self) -> &'static str { T::id() }

  pub fn query(&self) -> manager::SelectManager<T> { T::query() }
  pub fn create<S: serde::Serialize>(&self, insert_condition: S) -> manager::InsertManager<T> { T::create(insert_condition) }
  pub fn update_all<S: serde::Serialize>(&self, update_condition: S) -> manager::UpdateManager<T> { T::update_all(update_condition) }
  pub fn delete_all<S: serde::Serialize>(&self, where_condition: S) -> manager::DeleteManager<T> { T::delete_all(where_condition) }
}