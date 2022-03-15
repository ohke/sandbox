use sqlx::SqlitePool;
// use sqlx::sqlite::SqlitePoolOptions;

#[derive(sqlx::FromRow, Debug)]
struct User {
    id: i64,
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;
    let mut conn = pool.acquire().await?;

    let rowid = sqlx::query!(
        "INSERT INTO users (id, name) VALUES (?1, ?2), (?3, ?4)",
        1,
        "tanaka",
        2,
        "suzuki"
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();
    println!("rowid: {}", rowid);

    let rows = sqlx::query_as!(User, "SELECT id, name FROM users")
        .fetch_all(&pool)
        .await?;
    println!("{:?}", rows);

    Ok(())
}
