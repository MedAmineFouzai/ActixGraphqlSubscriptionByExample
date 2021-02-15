use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig,MultipartOptions};
use async_graphql::Schema;
use async_graphql_actix_web::{Request, Response, WSSubscription};
mod controllers;
use controllers::{MessageSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot,MyToken};
use actix_cors::Cors;
use actix_files as fs;

async fn index(schema: web::Data<MessageSchema>, req: HttpRequest, gql_request: Request) -> Response {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| MyToken(s.to_string())).ok());
   
   
    let mut request = gql_request.into_inner();
    if let Some(token) = token {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}
async fn index_ws(
    schema: web::Data<MessageSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    WSSubscription::start(Schema::clone(&*schema), &req, payload)
}

async fn index_playground() -> Result<HttpResponse> {

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
        .wrap(Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .supports_credentials())
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index)
            .app_data(MultipartOptions::default().max_num_files(3)),
        )
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),)
            .service(fs::Files::new("/media", "/static/uploads/.").show_files_listing())

            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
