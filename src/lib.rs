use rand::distributions::{Alphanumeric, DistString};
use url::Url;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;
use worker::*;

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

#[derive(OpenApi)]
#[openapi(
  paths(),
  components(schemas(Client, CreateClientRequest, UpdateClientRequest)),
  tags(
    (name = "Clients", description = "API endpoints for managing oauth2 clients")
  )
)]
pub struct ApiDoc;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  console_error_panic_hook::set_once();

  let router = Router::new();

  return router
    .get_async("/clients/:id", |_req, ctx| async move {
      if let Some(id) = ctx.param("id") {
        let clients = ctx.kv("CLIENTS")?;
        return match clients.get(id).json::<Client>().await? {
          Some(account) => Response::from_json(&account),
          None => Response::error("Not found", 404),
        };
      }

      return Response::error("Bad Request", 400);
    })
    .get_async("/clients", |_req, ctx| async move {
      let clients = ctx.kv("CLIENTS")?;
      let keys = clients.list().execute().await?;
      let mut client_list = Vec::new();

      for key in keys.keys {
        if let Some(client) = clients.get(&key.name).json::<Client>().await? {
          client_list.push(client);
        }
      }

      return Response::from_json(&client_list);
    })
    .post_async("/clients", |mut req, ctx| async move {
      let clients = ctx.kv("CLIENTS")?;

      // Parse the request body into the `CreateClientRequest` struct
      let body: CreateClientRequest = match req.json().await {
        Ok(parsed) => parsed,
        Err(_) => return Response::error("Invalid request body", 400),
      };

      // Validate required fields
      if body.name.is_empty() {
        return Response::error("Missing 'name' field", 400);
      }

      if body.redirect_uris.is_empty() {
        return Response::error("At least one redirect URI is required", 400);
      }

      for uri in &body.redirect_uris {
        if uri.parse::<Url>().is_err() {
          return Response::error("Invalid redirect URI provided", 400);
        }
      }

      let client_id = Uuid::new_v4();
      let client_secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

      let client = Client {
        client_id: client_id.clone().to_string(),
        client_secret: client_secret.clone(),
        name: body.name,
        redirect_uris: body.redirect_uris,
        access_token_validity: body.access_token_validity.unwrap_or_default(),
        refresh_token_validity: body.refresh_token_validity.unwrap_or_default(),
        disable_refresh_token: body.disable_refresh_token.unwrap_or_default(),
        refresh_refresh_token: body.refresh_refresh_token.unwrap_or_default(),
      };

      clients
        .put(
          &client_id.to_string(),
          serde_json::to_string(&client)?.as_str(),
        )?
        .execute()
        .await?;

      return Response::error("Bad Request", 400);
    })
    .patch_async("/clients/:id", |mut req, ctx| async move {
      if let Some(id) = ctx.param("id") {
        let clients = ctx.kv("CLIENTS")?;

        if let Some(mut existing_client) = clients.get(id).json::<Client>().await? {
          let updates: UpdateClientRequest = match req.json().await {
            Ok(parsed) => parsed,
            Err(_) => return Response::error("Invalid request body", 400),
          };

          if let Some(name) = updates.name {
            existing_client.name = name;
          }

          if let Some(redirect_uris) = updates.redirect_uris {
            for uri in &redirect_uris {
              if uri.parse::<url::Url>().is_err() {
                return Response::error("Invalid redirect URI provided", 400);
              }
            }
            existing_client.redirect_uris = redirect_uris;
          }

          if let Some(validity) = updates.access_token_validity {
            existing_client.access_token_validity = validity;
          }

          if let Some(validity) = updates.refresh_token_validity {
            existing_client.refresh_token_validity = validity;
          }

          if let Some(disable) = updates.disable_refresh_token {
            existing_client.disable_refresh_token = disable;
          }

          if let Some(refresh) = updates.refresh_refresh_token {
            existing_client.refresh_refresh_token = refresh;
          }

          clients
            .put(id, serde_json::to_string(&existing_client)?.as_str())?
            .execute()
            .await?;

          return Response::from_json(&existing_client);
        }

        return Response::error("Client not found", 404);
      }

      return Response::error("Bad Request", 400);
    })
    .delete_async("/clients/:id", |_req, ctx| async move {
      if let Some(id) = ctx.param("id") {
        let clients = ctx.kv("CLIENTS")?;
        clients.delete(id).await?;
        return Response::ok("Client deleted successfully");
      }

      return Response::error("Bad Request", 400);
    })
    .get_async("/clients-openapi.json", |_req, _ctx| async move {
      let openapi_json = ApiDoc::openapi().to_json().unwrap();
      return Response::from_json(&openapi_json);
    })
    .run(req, env)
    .await;
}