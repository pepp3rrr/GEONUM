use geonum_common::{FromCSV, Point};

#[derive(Debug)]
pub struct BSpline {
    pub control: Vec<Point>,
    pub knots: Vec<f32>,
}

impl FromCSV for BSpline {
    fn read(mut reader: geonum_common::CSVReader) -> Self {
        assert!(reader.has_headers());
        let headers = reader.headers().unwrap();
        let control_n: usize = headers.get(0).unwrap().parse().unwrap();

        let mut control = Vec::with_capacity(control_n);

        let mut records = reader.records();
        for _ in 0..control_n {
            let record = records.next().unwrap().unwrap();

            let x = record.get(0).unwrap().parse().unwrap();
            let y = record.get(1).unwrap().parse().unwrap();

            control.push(Point::<2>::new(x, y));
        }

        let knots_n: usize = records
            .next()
            .unwrap()
            .unwrap()
            .get(0)
            .unwrap()
            .parse()
            .unwrap();

        let mut knots = Vec::with_capacity(knots_n);

        for _ in 0..knots_n {
            let record = records.next().unwrap().unwrap();

            let knot = record.get(0).unwrap().parse().unwrap();

            knots.push(knot);
        }

        Self { control, knots }
    }
}
