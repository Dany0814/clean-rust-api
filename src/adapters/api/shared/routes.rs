
use actix_web::web;

use crate::adapters::api::test;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/v1/test").configure(test::controllers::routes));
}