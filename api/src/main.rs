// #![deny(warnings)]

mod models;
mod database;
mod graphql;
mod endpoints;
mod data_types;

use std::env;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware,
    web::{self, Data},
    App, HttpServer,
};

use graphql::schema::{create_schema};
use endpoints::{graphql_route, playground_route, graphiql_route};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(create_schema()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphgl")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_route)))
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}
// now go to http://127.0.0.1:8080/playground or graphiql and execute
//{  apiVersion,  user(id: 2){id, name}}
