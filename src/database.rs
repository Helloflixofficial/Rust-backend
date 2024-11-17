use sqlx::mysql::MySqlPool;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect("mysql://root:yourpassword@localhost:3307/actix-web").await
}
