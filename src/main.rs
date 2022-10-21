#![deny(elided_lifetimes_in_paths)]

use actix_cors::Cors;
use actix_web::{
    get,
    http::header,
    web::{self, Data},
    App, HttpServer, Responder,
};
use serde::Serialize;

mod database;
mod errors;
mod graphql;

#[derive(Serialize)]
struct ExData {
    value: u32,
}

#[get("/")]
async fn home() -> std::io::Result<impl Responder> {
    let obj = ExData { value: 1 };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(move || {
        App::new()
            .service(home)
            .app_data(Data::new(graphql::schemas::schema()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(
                web::resource("/graphql")
                    .route(web::post().to(move |req, payl, data| {
                        // let db = &*db.clone();
                        graphql::graphql_route(req, payl, data)
                    }))
                    .route(
                        web::get()
                            .to(move |req, payl, data| graphql::graphql_route(req, payl, data)),
                    ),
            )
            .service(web::resource("/playground").route(web::get().to(graphql::playground_route)))
            .service(web::resource("/graphiql").route(web::get().to(graphql::graphiql_route)))
    })
    .bind("127.0.0.1:9999")
    .expect("fucked up serwer")
    .run()
    .await
}
