use crate::{Client, UpdateClientRequest};

#[utoipa::path(
  patch,
  path = "/clients/{id}",
  params(("id" = String, Path, description = "Client ID")),
  request_body = UpdateClientRequest,
  responses(
      (status = 200, description = "Client updated successfully", body = Client),
      (status = 404, description = "Client not found")
  )
)]
fn update_client() {}