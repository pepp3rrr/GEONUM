use std::{fs::File, path::Path};

pub type CSVReader = csv::Reader<File>;

pub trait FromCSV: Sized {
    fn read(reader: CSVReader) -> Self;

    fn from_csv<P: AsRef<Path>>(path: P) -> Self {
        let file_reader = File::open(path).expect("Failed to read from CSV");
        let reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .delimiter(b',')
            .flexible(true)
            .from_reader(file_reader);

        Self::read(reader)
    }
}
