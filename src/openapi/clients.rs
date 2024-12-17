use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  paths(
    get_client,
    list_clients,
    create_client,
    update_client,
    delete_client),
  components(schemas(Client, CreateClientRequest, UpdateClientRequest)),
  tags(
    (name = "Clients", description = "API endpoints for managing oauth2 clients")
  )
)]
pub struct ApiDoc;


    // .get_async("/clients-openapi.json", |_req, _ctx| async move {
    //   let openapi_json = ApiDoc::openapi().to_json().unwrap();
    //   Response::from_json(&openapi_json)
    // })
    // .get_async("/clients-docs", |_req, _ctx| async move {
    //   let html = utoipa_swagger_ui::SwaggerUi::new("/clients-openapi.json").to_html();
    //   Response::from_body(html).map(|resp| resp.with_content_type("text/html; charset=utf-8"))
    // })