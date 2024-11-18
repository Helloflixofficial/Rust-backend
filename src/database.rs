use sqlx::mysql::MySqlPool;
pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    let connection_string = "mysql://root:root@127.0.0.1:3307/actix-web";
    MySqlPool::connect(connection_string).await
}
