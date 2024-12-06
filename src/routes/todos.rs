use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use sqlx::{mysql::MySqlQueryResult, Error, FromRow};
#[derive(Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct TypeDBError {
    pub error: String,
}

#[post("/todo/create")]
pub async fn create_new_todos(
    db: Data<sqlx::MySqlPool>,
    body: Json<CreateNewTodos>,
) -> impl Responder {
    let response: Result<MySqlQueryResult, Error> =
        sqlx::query("INSERT INTO TODOS (title, description) VALUES (?, ?)")
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

#[get("/todos/all")]
pub async fn get_all_todos(db: Data<MySqlPool>) -> impl Responder {
    let res: Result<Vec<TODO>, Error> =
        sqlx::query_as("SELECT id, title, description, status FROM todos")
            .fetch_all(&**db)
            .await;

    match res {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => HttpResponse::InternalServerError().json(TypeDBError {
            error: e.to_string(),
        }),
    }
}
