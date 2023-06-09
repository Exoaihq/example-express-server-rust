extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", hello_world, "hello_world");

    let server_address = "localhost:3000";
    println!("Server is running on http://{}", server_address);

    Iron::new(router).http(server_address).unwrap();
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello, World!")))
}