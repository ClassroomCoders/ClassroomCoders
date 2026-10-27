use async_nats::Client;

pub async fn get_nats_client()
-> Result<Client, async_nats::error::Error<async_nats::ConnectErrorKind>> {
    let nats_addr = get_nats_addr();
    async_nats::connect(nats_addr).await
}

fn get_nats_addr() -> String {
    std::env::var("NATS_ADDR").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_owned())
}

#[cfg(feature = "db")]
pub async fn get_db_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&get_database_url())
        .await
}

#[cfg(feature = "db")]
fn get_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://classcoders:classcoders@127.0.0.1:5433/classcoders".to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
