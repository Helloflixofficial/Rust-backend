use actix_web::{web, App, HttpServer};
mod database;
mod routes;
use database::*;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = database_connection()
        .await
        .expect("Failed to connect to the database");
    println!("Database connected");
    let db_data = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(home)
            .service(hello_user)
            .service(create_new_user)
    })
    .bind(("localhost", 8080))?
    .run();

    println!("Service is working at http://localhost:8080/home");
    server.await
}
