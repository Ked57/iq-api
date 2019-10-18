#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate juniper;

use std::io;
use std::sync::Arc;
use std::env;


use actix_cors::Cors;
use actix_web::{web, App, Error, HttpResponse, HttpServer, middleware::Logger, http::header};
use dotenv::dotenv;
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod db;
mod graphql_schema;
mod resolvers;
mod schema;
mod typedefs;

use crate::db::establish_connection;
use crate::graphql_schema::{create_schema, Context, Schema};

fn graphiql() -> HttpResponse {
    let graphql_url = env::var("GRAPHQL_URL").expect("GRAPHQL_URL must be set");
    let html = graphiql_source(&graphql_url);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    ctx: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("Received a request for /graphql");
    web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn main() -> io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let port = env::var("PORT").expect("PORT must be set");
    let origin = env::var("ORIGIN").expect("ORIGIN must be set");
    let pool = establish_connection();
    let schema_context = Context { db: pool.clone() };
    let schema = std::sync::Arc::new(create_schema());
    // println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&origin)
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .data(schema.clone())
            .data(schema_context.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
}
