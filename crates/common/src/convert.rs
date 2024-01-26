use std::sync::Arc;

use anyhow::Result;
use arrow::{
    buffer::Buffer,
    ipc::{
        convert::fb_to_schema,
        reader::{read_footer_length, FileDecoder},
        root_as_footer,
    },
    json::ArrayWriter,
};

pub fn arrow2json(data: Vec<u8>) -> Result<String> {
    let buffer = Buffer::from_vec(data);
    let trailer_start = buffer.len() - 10;
    let footer_len = read_footer_length(buffer[trailer_start..].try_into()?)?;
    let footer = root_as_footer(&buffer[trailer_start - footer_len..trailer_start]).unwrap();

    let schema = fb_to_schema(footer.schema().unwrap());

    let mut decoder = FileDecoder::new(Arc::new(schema), footer.version());

    // Read dictionaries
    for block in footer.dictionaries().iter().flatten() {
        let block_len = block.bodyLength() as usize + block.metaDataLength() as usize;
        let data = buffer.slice_with_length(block.offset() as _, block_len);
        decoder.read_dictionary(block, &data)?;
    }

    // Read record batch
    let batches = footer.recordBatches().unwrap();

    let block = batches.get(0);
    let block_len = block.bodyLength() as usize + block.metaDataLength() as usize;
    let data = buffer.slice_with_length(block.offset() as _, block_len);
    let batch = decoder.read_record_batch(block, &data)?.unwrap();
    // output as json
    let mut buf = Vec::new();
    let mut writer = ArrayWriter::new(&mut buf);
    writer.write(&batch)?;
    writer.finish()?;
    Ok(String::from_utf8(buf)?)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn arrow2json_should_work() -> Result<()> {
        let data = include_bytes!("../../clickhouse/fixtures/test.arrow");
        let json = arrow2json(data.to_vec())?;
        let data: Vec<serde_json::Value> = serde_json::from_str(&json)?;
        assert_eq!(
            data[0],
            json!({
                "DEPARTURE_DELAY": -11.0,
                "ARRIVAL_DELAY": -22.0,
                "DISTANCE": 1448,
                "SCHEDULED_DEPARTURE": 0.08333333333333333
            })
        );
        Ok(())
    }
}
