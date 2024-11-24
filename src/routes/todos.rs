use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySql, Error, FromRow};

pub struct CreateNewTodos {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct TODO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Serialize)]
pub struct TypeDBError {
    pub error: String,
}

#[post("/todo/create")]
pub async fn create_new_todos(
    db: Data<sqlx::MySqlPool>,
    body: Json<CreateNewTodos>,
) -> impl Responder {
    let response = sqlx::query("INSERT INTO TODOS (title, description) VALUES (?, ?)")
        .bind(&body.title)
        .bind(&body.description)
        .execute(&**db)
        .await;

    match response {
        Ok(result) => HttpResponse::Created().json(TODO {
            id: result.last_insert_id() as i32,
            title: body.title.clone(),
            description: body.description.clone(),
            status: "new".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(TypeDBError {
            error: e.to_string(),
        }),
    }
}
