use async_trait::async_trait;

use crate::{
    application::{
        repositories::test_facts_repository_abstract::TestFactsRepositoryAbstract,
        usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::{error::ApiError, test_fact_entity::TestFactEntity},
};

pub struct GetTestFactsUseCase<'a> {
    repository: &'a dyn TestFactsRepositoryAbstract,
}

impl<'a> GetTestFactsUseCase<'a> {
    pub fn new(repository: &'a dyn TestFactsRepositoryAbstract) -> Self {
        GetTestFactsUseCase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<TestFactEntity> for GetTestFactsUseCase<'a> {
    async fn execute(&self) -> Result<TestFactEntity, ApiError> {
        let cat_fact = self.repository.get_random_test_fact().await;

        match cat_fact {
            Ok(fact) => Ok(fact),
            Err(e) => Err(ErrorHandlingUtils::application_error(
                "Cannot get all cat facts",
                Some(e),
            )),
        }
    }
}
