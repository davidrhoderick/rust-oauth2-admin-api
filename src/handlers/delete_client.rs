#[utoipa::path(
  delete,
  path = "/clients/{id}",
  params(("id" = String, Path, description = "Client ID")),
  responses(
      (status = 200, description = "Client deleted successfully"),
      (status = 404, description = "Client not found")
  )
)]
fn delete_client() {}