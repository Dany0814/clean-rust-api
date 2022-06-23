use actix_web::dev::Server;
use std::net::TcpListener;

pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    infrastructure::server(listener)
}
