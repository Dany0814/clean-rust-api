use actix_web::{dev::Server, middleware::Logger};
use actix_web::{App, HttpServer};
use std::{env, net::TcpListener};

use crate::adapters;

pub fn server(listener: TcpListener) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::init();

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || App::new().wrap(Logger::default())
        .configure(adapters::api::shared::routes::routes))
        .listen(listener)?
        .run();

    println!("Server running on port {}", port);

    Ok(server)
}
