#[macro_use]
extern crate juniper;
use crate::graphql_schema::{create_schema, Schema};

use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
//use std::future::Future;
use futures::{future, Future, Stream, StreamExt, TryStreamExt};

//use log::debug;
use indexmap::IndexMap;

use actix_files as fs;
use actix_cors::Cors;

use actix_web::dev::Payload;
//use actix_web::error::MultipartError;
use actix_web::http::{self, StatusCode};
//use actix_web::multipart::MultipartItem;
use actix_multipart::Multipart;


use actix_web::{web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, Error, http::header};
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

#[derive(Template)]
#[template(path = "images.html")]
struct Images<'a> {
    images: &'a str,
}

// #[derive(Clone)]
// struct Record {
//     status: Status,
// }

#[derive(Clone)]
enum Status {
    InProgress,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::InProgress => write!(f, "in progress"),
        }
    }
}

//type SharedImages = Arc<Mutex<IndexMap<String, Record>>>;

// #[derive(Clone)]
// struct State {
//     state: SharedImages,
// }

extern crate tensorflow;

// pub fn handle_multipart_item(
//     item: MultipartItem<Payload>,
// ) -> Box<Stream<Item = Vec<u8>, Error = MultipartError>> {
//     match item {
//         MultipartItem::Field(field) => {
//             Box::new(field.concat2().map(|bytes| bytes.to_vec()).into_stream())
//         }
//         MultipartItem::Nested(mp) => Box::new(mp.map(handle_multipart_item).flatten()),
//     }
// }

// fn upload_handler(req: HttpRequest<State>) -> impl Future<Item = HttpResponse, Error = WebError> {
//     req.multipart()
//         .map(handle_multipart_item)
//         .flatten()
//         .into_future()
//         .and_then(|(bytes, stream)| {
//             if let Some(bytes) = bytes {
//                 Ok(bytes)
//             } else {
//                 Err((MultipartError::Incomplete, stream))
//             }
//         })
//         .map_err(|(err, _)| WebError::from(err))
//         .and_then(move |image| {
//             debug!("Image: {:?}", image);
//             let request = image;
//             req.state()
//                 .from_err()
//                 // .map(move |task_id| {
//                 //     let record = Record {
//                 //         task_id: task_id.clone(),
//                 //         timestamp: Utc::now(),
//                 //         status: Status::InProgress,
//                 //     };
//                 //     req.state().tasks.lock().unwrap().insert(task_id, record);
//                 //     req
//                 // })
//         })
//         .map(|req| {
//             HttpResponse::build_from(&req)
//                 .status(StatusCode::FOUND)
//                 .header(header::LOCATION, "/images")
//                 .finish()
//         })
// }

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    println!("saving...");
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", filename);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

// async fn images_handler(req: HttpRequest) -> Result<HttpResponse, Error> {
//     let images: Vec<_> = req
//         //.state()
//        // .images
//         //.lock()
//         //.unwrap()
//         //.values()
//         //.cloned()
//         .collect();
//     let tmpl = Images { images };
//     future::ok(HttpResponse::Ok().body(tmpl.render().unwrap())).await
// }

fn image_upload() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

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
    let html = graphiql_source("http://localhost:80/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let schema = std::sync::Arc::new(create_schema());
    //let images = Arc::new(Mutex::new(IndexMap::new()));
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    let address = "0.0.0.0:";
    let port = "80";
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
                web::resource("/images")
                .route(web::get().to(image_upload))
                .route(web::post().to(save_file))
            )
            // .resource("/image", |r| {
            //     //r.method(http::Method::GET).with_async(snd_msg);
            //     r.method(http::Method::POST).with_async(upload_handler);
            // })
            // .resource("/images", |r| r.method(http::Method::GET).with_async(tasks_handler))
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



