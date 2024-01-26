use std::fs::File;

use anyhow::Result;
use arrow::{ipc, json::ArrayWriter};

fn main() -> Result<()> {
    let reader = ipc::reader::FileReader::try_new(
        File::open("crates/clickhouse/fixtures/test.arrow")?,
        None,
    )?;

    // output as json
    let mut buf = Vec::with_capacity(10 * 1024);
    let mut writer = ArrayWriter::new(&mut buf);
    for batch in reader {
        writer.write(&batch?)?;
    }
    writer.finish()?;
    println!("{}", String::from_utf8(buf)?);
    Ok(())
}
