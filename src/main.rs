use actix_files as fs;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

use std::{env, error::Error};
use twilio_async::{MsgResp, Twilio, TwilioJson, TwilioRequest};

mod twilio;

// async fn index(_req: HttpRequest) -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .service(
                 // static files
                 fs::Files::new("/", "./static/").index_file("index.html"),
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
   
}


