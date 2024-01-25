#[allow(async_fn_in_trait)]
pub trait QueryRunner {
    type Response;
    type Error;
    async fn query(&self, sql: &str) -> Result<Self::Response, Self::Error>;
}
