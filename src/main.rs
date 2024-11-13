use actix_web::{
    get,
    http::StatusCode,
    web::{Json, Path},
    App, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

#[get("/home")]
async fn home() -> impl Responder {
    let response = "Hello sire";
    response
}

#[get("/hello/{firstname}/{lastname}")]
async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    let response = User::new(params.0.clone(), params.1.clone());
    (Json(response), StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
struct User {
    first_name: String,
    last_name: String,
}

impl User {
    fn new(firstname: String, lastname: String) -> Self {
        Self {
            first_name: firstname,
            last_name: lastname,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(home).service(hello_user))
        .bind(("127.0.0.1", 8080))?
        .run();
    println!("service is Working here  http://127.0.0.1:8080/home");
    server.await
}
