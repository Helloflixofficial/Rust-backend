use actix_web::{web::Data, App, HttpServer};
mod database;
mod routes;
use database::*;
use routes::{create_new_user, get_all_todos, hello_user, home}; // Explicitly import routes

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish database connection
    let database = match database_connection().await {
        Ok(db) => {
            println!("Database connected successfully");
            db
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            std::process::exit(1); // Exit if database connection fails
        }
    };

    // Set up HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone())) // Inject the database connection
            .service(home) // Add routes
            .service(hello_user)
            .service(create_new_user)
            .service(get_all_todos)
    })
    .bind(("127.0.0.1", 8080))? // Bind to localhost on port 8080
    .run()
    .await // Start the server
}
