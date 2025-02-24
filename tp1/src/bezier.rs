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
    fn read(reader: CSVReader) -> Self {
        Self {
            control: Vec::<Point>::read(reader),
        }
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
