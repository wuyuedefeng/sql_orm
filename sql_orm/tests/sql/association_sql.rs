use super::models;

#[test]
fn test_sql() {
  let mut user = models::User::default();
  user.id = Some(1);

  let mut wallet = models::Wallet::default();
  wallet.user_id = Some(2);

  let sql: String = wallet.user().to_sql().unwrap().try_into().unwrap();
  assert_eq!(sql, "SELECT * FROM users WHERE id = 2 LIMIT 1".to_string());

  let sql: String = user.wallet().to_sql().unwrap().try_into().unwrap();
  assert_eq!(sql, "SELECT * FROM wallets WHERE user_id = 1 LIMIT 1".to_string());
}