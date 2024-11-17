use sqlx::postgres::PgPool;
use std::env;
pub async fn database_connection() -> Result<PgPool, sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    PgPool::connect(&database_url).await
}
