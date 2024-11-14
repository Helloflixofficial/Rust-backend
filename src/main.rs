use actix_web::{App, HttpServer};
mod routes;
use routes::*;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(home).service(hello_user))
        .bind(("localhost", 8080))?
        .run();
    println!("service is Working here  http://localhost:8080/home");
    server.await
}
