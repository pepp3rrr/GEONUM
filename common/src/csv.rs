use std::{fs::File, path::Path};

use crate::Point;

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

impl FromCSV for Vec<Point<2>> {
    fn read(mut reader: CSVReader) -> Self {
        reader
            .records()
            .map(|result| {
                let record = result.expect("Failed to read record");

                let x_str = record.get(0).unwrap();
                let y_str = record.get(1).unwrap();

                let x = x_str.parse().unwrap();
                let y = y_str.parse().unwrap();

                Point::<2>::new(x, y)
            })
            .collect()
    }
}

impl FromCSV for Vec<Point<3>> {
    fn read(mut reader: CSVReader) -> Self {
        reader
            .records()
            .map(|result| {
                let record = result.expect("Failed to read record");

                let x_str = record.get(0).unwrap();
                let y_str = record.get(1).unwrap();
                let z_str = record.get(2).unwrap();

                let x = x_str.parse().unwrap();
                let y = y_str.parse().unwrap();
                let z = z_str.parse().unwrap();

                Point::<3>::new(x, y, z)
            })
            .collect()
    }
}
