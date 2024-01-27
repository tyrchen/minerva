use anyhow::Result;
use minerva_common::QueryRunner;
use minerva_df::df::queryrunner::DFQueryRunner;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let runner = DFQueryRunner::new_s3("flights.parquet");
    let data = runner.query("SELECT * FROM flights LIMIT 10").await?;
    println!("query finished: size: {} bytes", data.len());
    Ok(())
}
