use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: i64,
  pub email: String,
  pub name: String,
  pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
  pub email: String,
  pub password: String,
  #[serde(default)]
  pub remember_me: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub exp: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
  pub name: String,
  pub email: String,
  pub password: String
}
