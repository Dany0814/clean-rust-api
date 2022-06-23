use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::usecases::interfaces::AbstractUseCase;

use crate::domain::test_fact_entity::TestFactEntity;
use crate::{
    adapters::api::{
        shared::error_presenter::ErrorReponse, test::mappers::TestFactPresenterMapper,
    },
    application::usecases::test_facts_usecase::GetTestFactsUseCase,
};
use actix_web::{get, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_test);
}

#[get("/")]
async fn get_test() -> Result<HttpResponse, ErrorReponse> {
    let get_one_random_cat_fact_usecase = GetTestFactsUseCase::new();
    let test_fact = get_one_random_cat_fact_usecase.execute().await;

    let test_fact = TestFactEntity {
        fact_txt: "String".to_string(),
        fact_length: 32,
    };

    test_fact
        .map_err(ErrorReponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(TestFactPresenterMapper::to_api(fact)))
}
