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

            // (1-t)a + tb <=> a -ta + tb <=> a + t(b - a)
            // This form respects point and vec operation rules
            let coord = a + (b - a) * t;
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

    fn bounding_box(&self) -> (Point, Point) {
        let right = self
            .control
            .iter()
            .map(|p| p.x)
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap();
        let top = self
            .control
            .iter()
            .map(|p| p.y)
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap();
        let left = self
            .control
            .iter()
            .map(|p| p.x)
            .max_by(|a, b| b.total_cmp(&a))
            .unwrap();
        let bottom = self
            .control
            .iter()
            .map(|p| p.y)
            .max_by(|a, b| b.total_cmp(&a))
            .unwrap();

        (Point::new(left, bottom), Point::new(right, top))
    }
}
