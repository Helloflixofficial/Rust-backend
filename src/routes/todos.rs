use actix_web::web::{Data, Json};
use actix_web::{delete, get, post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, MySqlPool, Pool};

#[derive(Deserialize)]
pub struct CreateNewTodos {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct Id {
    pub id: i32,
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
        .execute(db.get_ref()) // Use get_ref() for clarity
        .await;

    match res {
        Ok(_) => HttpResponse::Ok().json("Title updated successfully"),
        Err(e) => HttpResponse::InternalServerError().json(TypeDBError {
            error: e.to_string(),
        }),
    }
}

#[post("/todo/create")]
pub async fn create_new_todos(db: Data<MySqlPool>, body: Json<CreateNewTodos>) -> impl Responder {
    let response = sqlx::query("INSERT INTO todos (title, description) VALUES (?, ?)")
        .bind(&body.title)
        .bind(&body.description)
        .execute(db.get_ref())
        .await;

    match response {
        Ok(result) => HttpResponse::Created().json(TODO {
            id: result.last_insert_id() as i32,
            title: body.title.clone(),
            description: body.description.clone(),
            status: "New".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(TypeDBError {
            error: e.to_string(),
        }),
    }
}

#[get("/todos/all")]
pub async fn get_all_todos(db: Data<MySqlPool>) -> impl Responder {
    let response: Result<Vec<TODO>, sqlx::Error> =
        sqlx::query_as("SELECT id, title, description, status FROM todos")
            .fetch_all(db.get_ref())
            .await;

    match response {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => HttpResponse::InternalServerError().json(TypeDBError {
            error: e.to_string(),
        }),
    }
}

#[delete("/todo/delete")]
pub async fn delete_a_todo(db: Data<Pool<MySql>>, id: Json<Id>) -> impl Responder {
    let res = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id.id)
        .execute(&**db)
        .await;

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
