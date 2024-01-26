use anyhow::Result;
use minerva_clickhouse::ClickHouseRunner;
use minerva_common::arrow2json;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let runner = ClickHouseRunner::new_s3("2015_flights.parquet");
    let data = runner.run("SELECT * FROM 2015_flights LIMIT 10", None)?;
    println!("query finished: size: {} bytes", data.len());
    let json = arrow2json(data)?;
    println!("{}", json);
    Ok(())
}
