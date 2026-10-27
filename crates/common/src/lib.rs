use async_nats::Client;

pub async fn get_nats_client()
-> Result<Client, async_nats::error::Error<async_nats::ConnectErrorKind>> {
    let nats_addr = get_nats_addr();
    async_nats::connect(nats_addr).await
}

fn get_nats_addr() -> String {
    std::env::var("NATS_ADDR").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_owned())
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
