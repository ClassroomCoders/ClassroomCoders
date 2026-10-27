use common::get_db_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_db_pool().await?;

    // Migrations are embedded in the binary, so deploys don't need the SQL files.
    // sqlx takes a Postgres advisory lock, so concurrent runs are safe.
    sqlx::migrate!("./migrations").run(&pool).await?;

    println!("migrations applied");
    Ok(())
}
