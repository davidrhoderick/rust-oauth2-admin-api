use crate::Client;

#[utoipa::path(
  get,
  path = "/clients",
  responses(
      (status = 200, description = "List all clients", body = [Client])
  )
)]
fn list_clients() {}