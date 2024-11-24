use actix_web::{web::Data, App, HttpServer};
mod database;
mod routes;
use database::*;
use routes::*;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = database_connection().await.expect("Faild To Connect");
    println!("Database connected");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            .service(home)
            .service(hello_user)
            .service(create_new_user)
    })
    .bind(("localhost", 8080))?
    .run();
    println!("Service is working at localhost:8080");
    server.await
}
