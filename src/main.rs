#[macro_use]
extern crate juniper;

use std::io;
use std::sync::Arc;
//use std::error::Error;
use std::fmt;

use std::future::Future;

use actix_files as fs;
use actix_cors::Cors;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Error, http::header};
use listenfd::ListenFd;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql_schema;

use crate::graphql_schema::{create_schema, Schema};


async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let schema = std::sync::Arc::new(create_schema());

    let address = "0.0.0.0:";
    let port = "8080";
    let target = format!("{}{}", address, port);

    let mut server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(
                Cors::new()
                .supports_credentials() 
                .allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)
                .finish(),
            )
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(
                 // static files
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
         
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(&target)?
    };


    println!("Server starting...");
    server.run().await
   
}



