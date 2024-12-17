use crate::Client;

#[utoipa::path(
  get,
  path = "/clients/{id}",
  params(("id" = String, Path, description = "Client ID")),
  responses(
      (status = 200, description = "Client fetched successfully", body = Client),
      (status = 404, description = "Client not found")
  )
)]
pub async fn get_client() {}