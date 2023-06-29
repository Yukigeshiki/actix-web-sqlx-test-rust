use actix_web_diesel_test::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;
    println!("Starting Actix Web server at: {address}");
    run(listener)?.await
}
