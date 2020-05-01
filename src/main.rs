use actix_files as fs;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;


// async fn index(_req: HttpRequest) -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let address = "0.0.0.0:";
    let port = "8088";
    let target = format!("{}{}", address, port);


    let mut server = HttpServer::new(|| {
        App::new()
            .service(
                 // static files
                 fs::Files::new("/", "./static/").index_file("index.html"),
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        println!("Listening on: {}", &target);
        server.listen(l)?
    } else {
        server.bind(&target)?
    };

    println!("Started http server: {}", &target);
    server.run().await
   
}


