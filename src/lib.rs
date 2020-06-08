use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;
use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use serde_bytes::ByteBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordA<'a> {
    field_a: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordB<'a> {
    field_b: &'a str,
}

pub fn read_and_write<'de: 'a, 'a, W: Write + ?Sized, T: Deserialize<'de> + Serialize>(
    bytes: &'a [u8],
    transformed: &'a mut W,
) -> Result<(), Box<dyn std::error::Error>> 
where
  T: Deserialize<'de>,
{
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_reader(&bytes[0..]);

    let mut writer = WriterBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_writer(transformed);

    let mut raw_record = csv::StringRecord::new();

    while reader.read_record(&mut raw_record)? {
        let record: T = raw_record.deserialize(None)?;
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let read_buffer = std::fs::read_to_string("sample.csv")?;
    let mut write_buffer = Vec::new();
    let csv_errors = read_and_write::<_, RecordA>(&read_buffer.as_bytes(), &mut write_buffer);
    // do something with csv_errors

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        run().expect("error");
    }
}
