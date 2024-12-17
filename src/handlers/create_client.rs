use crate::{Client, CreateClientRequest};

#[utoipa::path(
  post,
  path = "/clients",
  request_body = CreateClientRequest,
  responses(
      (status = 201, description = "Client created successfully", body = Client),
      (status = 400, description = "Invalid request body")
  )
)]
fn create_client() {}