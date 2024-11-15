use crate::routes::{logging, User};
use actix_web::http::StatusCode;
use actix_web::{post, web::Json, Responder};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct CreateUserResponse {
    user: User,
    id: u32,
}

#[post("/user/create")]
pub async fn create_new_user(user: Json<User>) -> impl Responder {
    logging("POST :/user/create");
    (
        Json(CreateUserResponse {
            id: 1,
            user: user.0,
        }),
        StatusCode::CREATED,
    )
}
