use crate::routes::logging;
use actix_web::{get, Responder};
#[get("/home")]
pub async fn home() -> impl Responder {
    logging("Get: /home");
    let response = "Hello sire";
    response
}
