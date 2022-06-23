mod models;

use sql_orm::prelude::*;

fn main() {
  let sql: String = models::User::query().to_sql().unwrap().try_into().unwrap();
  println!("sql: {}", sql);

  let mut user = models::User::default();
  user.id = Some(1);
  let sql: String = user.wallet().to_sql().unwrap().try_into().unwrap();
  println!("sql: {}", sql);

  let wallet = models::Wallet::default();
  println!("user_id1: {:?}", wallet.get_json_value_from_attr_name__("user_id"));
  let sql: String = wallet.user().to_sql().unwrap().try_into().unwrap();
  println!("sql: {}", sql);
}