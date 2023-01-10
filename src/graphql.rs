// #![deny(warnings)]

use actix_web::{web, Error, HttpResponse, error::InternalError};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

pub(crate) mod schemas;
use schemas::Schema;

use crate::database::Database;

pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let db = Database::new().await.unwrap(); // must be unwrap due to actix error incompability with erroe stack ()
    // let db = match Database::new().await {
    //     Ok(db) => db,
    //     Err(err) => Err(InternalError(err))
    // }
    // let db = db.init_database().await.unwrap();
    return graphql_handler(&schema, &db, req, payload).await;
}

pub async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}
pub async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}
