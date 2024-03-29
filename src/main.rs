//! src/main.rs
#![allow(non_snake_case)]
use dotenvy::dotenv;
use newsLetter::configuration::get_configuration;
use newsLetter::startup::run;
use newsLetter::telemetry;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let subscriber = telemetry::get_subscriber("newsLetter".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(10))
        .max_connections(20)
        .connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    println!(" Server is up and running on : {}", address);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
