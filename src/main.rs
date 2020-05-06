#[macro_use]
extern crate juniper;
use crate::graphql_schema::{create_schema, Schema};

use std::io;
use std::sync::Arc;
use std::collections::HashMap;

use std::fmt;

use std::future::Future;

use actix_files as fs;
use actix_cors::Cors;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Error, http::header};
use listenfd::ListenFd;
use askama::Template;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql_schema;


#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    text: &'a str,
    pokemon: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

<<<<<<< HEAD
extern crate tensorflow;


=======
async fn index(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    let s = if let Some(pokemon) = query.get("pokemon") {
        UserTemplate {
            text: "Cool!",
            pokemon,
          
        }
        .render()
        .unwrap()
    } else {
        Index.render().unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

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
    let html = graphiql_source("http://localhost:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
>>>>>>> master


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
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)
                .finish(),
            )
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/form").route(web::get().to(index)))
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



