//! src/main.rs
#![allow(non_snake_case)]
use dotenvy::dotenv;
use newsLetter::configuration::get_configuration;
use newsLetter::startup::run;
use newsLetter::telemetry;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let subscriber = telemetry::get_subscriber("newsLetter".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    println!("listening on: {}", address);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
