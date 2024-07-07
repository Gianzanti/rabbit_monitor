use csv::{Terminator, Writer, WriterBuilder};
use std::fs::OpenOptions;
use std::fs::{self, File};

pub struct RabbitCSV {
    pub csv_writer: Writer<File>,
}

impl RabbitCSV {
    pub fn new(file_name: &str, headers: &Vec<&str>) -> Self {
        let size = fs::metadata(&file_name);
        let size = match size {
            Ok(size) => size.len(),
            Err(_) => 0,
        };

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&file_name)
            .unwrap();

        let mut csv_writer = WriterBuilder::new()
            .has_headers(false)
            .delimiter(b';')
            .terminator(Terminator::CRLF)
            .from_writer(file);

        if size == 0 {
            csv_writer.write_record(headers).unwrap();
        }

        RabbitCSV { csv_writer }
    }
}
