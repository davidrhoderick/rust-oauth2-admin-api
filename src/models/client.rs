
use utoipa::ToSchema;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Client {
  pub client_id: String,
  pub client_secret: String,
  pub name: String,
  pub redirect_uris: Vec<String>,
  #[serde(default)]
  pub access_token_validity: u64,
  #[serde(default)]
  pub refresh_token_validity: u64,
  #[serde(default)]
  pub disable_refresh_token: bool,
  #[serde(default)]
  pub refresh_refresh_token: bool,
}

impl Default for Client {
  fn default() -> Self {
    Self {
      client_id: String::new(),
      client_secret: String::new(),
      name: String::new(),
      redirect_uris: Vec::new(),
      access_token_validity: 3600,
      refresh_token_validity: 1209600,
      disable_refresh_token: false,
      refresh_refresh_token: true,
    }
  }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateClientRequest {
  pub name: String,
  pub redirect_uris: Vec<String>,
  pub access_token_validity: Option<u64>,
  pub refresh_token_validity: Option<u64>,
  pub disable_refresh_token: Option<bool>,
  pub refresh_refresh_token: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateClientRequest {
  pub name: Option<String>,
  pub redirect_uris: Option<Vec<String>>,
  pub access_token_validity: Option<u64>,
  pub refresh_token_validity: Option<u64>,
  pub disable_refresh_token: Option<bool>,
  pub refresh_refresh_token: Option<bool>,
}