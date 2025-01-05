use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::MySqlPool;

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

#[derive(Deserialize)]
pub struct UpdateTitleStruct {
    pub id: i32,
    pub title: String,
}

#[post("/todo/title/update")]
pub async fn update_todo_title(
    body: Json<UpdateTitleStruct>,
    db: Data<MySqlPool>,
) -> impl Responder {
    let res = sqlx::query("UPDATE todos SET title = ? WHERE id = ?")
        .bind(&body.title)
        .bind(body.id)
        .execute(&**db)
        .await;

    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("/todo/create")]
pub async fn create_new_todos(db: Data<MySqlPool>, body: Json<CreateNewTodos>) -> impl Responder {
    let response = sqlx::query("INSERT INTO todos (title, description) VALUES (?, ?)")
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
            error: format!("Database error: {}", e),
        }),
    }
}

#[get("/todos/all")]
pub async fn get_all_todos(db: Data<MySqlPool>) -> impl Responder {
    let response = sqlx::query_as::<_, TODO>("SELECT id, title, description, status FROM todos")
        .fetch_all(&**db)
        .await;

    match response {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => HttpResponse::InternalServerError().json(TypeDBError {
            error: format!("Database error: {}", e),
        }),
    }
}
