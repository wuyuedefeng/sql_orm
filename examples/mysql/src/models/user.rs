use sql_orm::prelude::*;
use super::{Wallet};

#[orm(
  has_one("wallet", struct = "Wallet", foreign_key = "user_id"),
)]
pub struct User {
  id: Option<usize>,
}