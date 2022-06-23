use crate::adapters::api::test::payloads::TestPayload;
use crate::adapters::api::test::presenters::TestFactPresenter;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::test_fact_entity::TestFactEntity;

pub struct TestFactPresenterMapper {}

impl ApiMapper<TestFactEntity, TestFactPresenter, TestPayload> for TestFactPresenterMapper {
    fn to_api(entity: TestFactEntity) -> TestFactPresenter {
        TestFactPresenter {
            fact: entity.fact_txt,
            nb_chars: entity.fact_length,
        }
    }

    fn to_entity(_payload: TestPayload) -> TestFactEntity {
        panic!("not implemented");
    }
}