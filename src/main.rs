use std::net::TcpListener;

use sqlx::postgres::PgPool;

use actix_web_sqlx_test::config::get_config;
use actix_web_sqlx_test::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // stop application if get config or db connect fail
    let config = get_config().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&config.database.get_connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let addr = format!("127.0.0.1:{}", config.port);
    println!("Starting Actix Web server at: {}", addr);

    let listener = TcpListener::bind(addr)?;
    startup(listener, connection_pool)?.await?;
    Ok(())
}
