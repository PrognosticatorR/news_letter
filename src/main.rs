//! src/main.rs
#![allow(non_snake_case)]
use newsLetter::configuration::get_configuration;
use newsLetter::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration."); // We have removed the hard-coded `8000` - it's now coming from our settings!
    println!("{:?}", configuration);
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
