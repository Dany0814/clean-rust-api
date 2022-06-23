use std::env;
use std::net::TcpListener;

use clean_rust_api::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // init env vars
    let environment = match env::var("ENV_NAME") {
        Ok(val) => format!(".env.{}", val),
        Err(_e) => String::from(".env"),
    };

    // building address and ip
    let host = std::env::var("HOST_API").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT_API").unwrap_or("8080".to_string());
    let address = format!("{}:{}", host, port);

    dotenv::from_filename(environment).ok();

    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    run(listener)?.await
}
