use std::net::TcpListener;

use sqlx::postgres::PgPool;

use actix_web_sqlx_test::config::get_config;
use actix_web_sqlx_test::run;
use actix_web_sqlx_test::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("actix-web-sqlx-test".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // stop application if get config or db connect fail
    let config = get_config().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&config.database.get_connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let addr = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(addr)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
