use std::env;
use std::net::TcpListener;

async fn main() -> std::io::Result<()> {
    let environment;
    if let Ok(e) = env::var("ENV"){
        environment = format!(".env.{}",e);
    } else {
        environment = String::from(".env");
    }

    dotenv::from_filename(environment).ok();

    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind random port");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    run(listener, &database_name)?.await
}
