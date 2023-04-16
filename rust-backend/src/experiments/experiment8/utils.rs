use std::{fs::File, io::Error};

pub trait HumanReadable {
    fn human_readable(&self) -> String;
}

pub fn write_csv_file<'a>(
    file_path: &str,
    columns: &[&str],
    rows: impl Iterator<Item = Vec<String>>,
) -> Result<(), Error> {
    let file = File::create(file_path)?;
    let mut csv_writer = csv::Writer::from_writer(file);
    csv_writer.write_record(columns)?;
    for row in rows {
        csv_writer.write_record(row)?;
    }
    csv_writer.flush()?;
    Ok(())
}
