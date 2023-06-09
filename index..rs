use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;
use std::sync::mpsc;
use std::thread;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:3000")?
    .shutdown_timeout(60)
    .run();

    let server_handle = server.clone();
    thread::spawn(move || {
        let _ = rx.recv();
        server_handle.stop(true);
    });

    let _ = tx.send(());
    server.await
}