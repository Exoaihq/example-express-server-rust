```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;
use std::sync::mpsc;
use std::thread;

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    let server = HttpServer::new(|| App::new().route("/", web::get().to(hello_world)))
        .bind("127.0.0.1:3000")?
        .shutdown_timeout(60)
        .run();

    let _ = thread::spawn(move || {
        let _ = rx.recv();
        server.stop(true);
    });

    let _ = tx.clone();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })
    .expect("Error setting Ctrl-C handler");

    println!("Server is running on http://localhost:3000");
    server.await
}
```
