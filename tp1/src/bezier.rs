use geonum_common::*;

pub struct Bezier {
    pub control: Vec<Point>,
}

impl Bezier {
    pub fn new(control: Vec<Point>) -> Self {
        Self { control }
    }

    fn compute(t: f32, mut control: Vec<Point>) -> Point {
        let n = control.len();

        if n == 1 {
            return control.pop().expect("Should not be empty");
        }

        let mut new = Vec::<Point>::new();
        for k in 1..n {
            let a = control.get(k - 1).unwrap().clone();
            let b = control.get(k).unwrap().clone();

            let coord = ((1.0 - t) * a + t * b).into_point();
            new.push(coord);
        }

        Self::compute(t, new)
    }
}

impl FromCSV for Bezier {
    fn read(mut reader: CSVReader) -> Self {
        let control = reader
            .records()
            .map(|result| {
                let record = result.expect("Failed to read record");

                let x_str = record.get(0).unwrap();
                let y_str = record.get(1).unwrap();

                let x = x_str.parse().unwrap();
                let y = y_str.parse().unwrap();

                Point::new(x, y)
            })
            .collect();

        Self { control }
    }
}

impl Plot for Bezier {
    fn sample(&self, t: f32) -> Point {
        Self::compute(t, self.control.clone())
    }
}

impl BoundingBox for Bezier {
    fn bounding_box(&self) -> (Point, Point) {
        self.control.bounding_box()
    }
}
