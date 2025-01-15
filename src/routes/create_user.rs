use crate::routes::{logging, User};
use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    user: User,
    id: u32,
}

#[post("/user/create")]
pub async fn create_new_user(user: Json<User>) -> impl Responder {
    logging("POST :/user/create");

    HttpResponse::Created().json(CreateUserResponse {
        id: 1,
        user: user.0,
    })
}
