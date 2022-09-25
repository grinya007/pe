use serde::Serialize;
use std::error::Error;
use std::io;

pub enum Output<'a> {
    File(&'a str),
    STDOUT,
}

/// # Errors
pub fn write_csv<T: Serialize, D: Iterator<Item = T>>(
    output: &Output<'_>,
    data: D,
) -> Result<(), Box<dyn Error>> {
    if let Output::File(path) = output {
        let mut writer = csv::Writer::from_path(path)?;
        for record in data {
            writer.serialize(record)?;
        }
    } else {
        let mut writer = csv::Writer::from_writer(io::stdout());
        for record in data {
            writer.serialize(record)?;
        }
    }

    Ok(())
}
