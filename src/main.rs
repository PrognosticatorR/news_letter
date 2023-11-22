//! src/main.rs
#![allow(non_snake_case)]
use newsLetter::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{}", port);
    run(listener)?.await
}
