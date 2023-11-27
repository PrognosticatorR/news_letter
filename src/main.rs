//! src/main.rs
#![allow(non_snake_case)]
use dotenv::dotenv;
use newsLetter::configuration::get_configuration;
use newsLetter::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connenction = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)?;
    println!("Running on: {:?}", address);
    run(listener, connenction)?.await
}
