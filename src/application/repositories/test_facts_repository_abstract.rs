
use async_trait::async_trait;

use crate::domain::test_fact_entity::TestFactEntity;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait TestFactsRepositoryAbstract {
    async fn get_random_test_fact(&self) -> Result<TestFactEntity, Box<dyn Error>>;
}