use anyhow::Result;
use minerva_clickhouse::ClickHouseRunner;
use minerva_common::arrow2json;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let runner = ClickHouseRunner::new_s3("youtube.parquet");
    let rx = runner.run("SELECT * FROM youtube LIMIT 10", false, None)?;
    let data = rx.await?;
    println!("query finished: size: {} bytes", data.len());
    let json = arrow2json(data)?;
    println!("{}", json);
    Ok(())
}
