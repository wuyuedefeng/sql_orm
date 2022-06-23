use sql_orm::prelude::*;
use super::{User};

#[orm(
  belongs_to("user", struct = "User", foreign_key = "user_id")
)]
pub struct Wallet {
  user_id: Option<usize>,
}