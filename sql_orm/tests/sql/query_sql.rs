use sql_orm::prelude::*;

#[orm]
struct User {}

#[test]
fn test_sql() {
  let sql: String = User::query().to_sql().unwrap().try_into().unwrap();
  assert_eq!(sql, "SELECT * FROM users".to_string());
  let sql: String = User::ORM.query().to_sql().unwrap().try_into().unwrap();
  assert_eq!(sql, "SELECT * FROM users".to_string());
  let sql: String = User::orm().query().to_sql().unwrap().try_into().unwrap();
  assert_eq!(sql, "SELECT * FROM users".to_string());
}