use axum::{response::IntoResponse, routing::get, Router};
use hyper::server::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));

    println!("Server running at: http://{}", address);
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn root() -> impl IntoResponse {
    "Axum server is running!"
}

// use actix_web::{web::Data, App, HttpServer};
// mod database;
// mod routes;
// use database::*;
// use routes::{create_new_todos, create_new_user, delete_a_todo, get_all_todos, hello_user, home};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // Establish database connection
//     let database = match database_connection().await {
//         Ok(db) => {
//             println!("Database connected successfully");
//             db
//         }
//         Err(e) => {
//             eprintln!("Failed to connect to the database: {}", e);
//             std::process::exit(1);
//         }
//     };

//     HttpServer::new(move || {
//         App::new()
//             .app_data(Data::new(database.clone()))
//             .service(home)
//             .service(hello_user)
//             .service(create_new_user)
//             .service(create_new_todos)
//             .service(get_all_todos)
//             .service(delete_a_todo)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
