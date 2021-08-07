use crate::graphql::{Schema};
use actix_web::{web, Error, HttpResponse};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};
use crate::database::Context;

pub async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphgl", None).await
}

pub async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphgl", None).await
}

pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Context::new();
    graphql_handler(&schema, &context, req, payload).await
}