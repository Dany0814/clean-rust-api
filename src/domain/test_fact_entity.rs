#[derive(Debug, Clone)]
pub struct TestFactEntity {
    pub fact_txt: String,
    pub fact_length: i32,
}

impl TestFactEntity {
    pub fn new(fact_txt: String, fact_length: i32) -> Self {
        TestFactEntity { fact_txt, fact_length }
    }
}