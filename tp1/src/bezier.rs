use geonum_common::*;

pub struct Bezier<const D: usize = 2> {
    pub control: Vec<Point<D>>,
}

impl<const D: usize> Bezier<D> {
    pub fn new(control: Vec<Point<D>>) -> Self {
        Self { control }
    }

    // Recursive De Casteljau method
    pub fn compute(mut self, t: f32) -> Point<D> {
        let n = self.control.len();

        if n == 1 {
            return self.control.pop().expect("Should not be empty");
        }

        let mut new = Vec::<Point<D>>::new();
        for k in 1..n {
            let a = self.control.get(k - 1).unwrap().clone();
            let b = self.control.get(k).unwrap().clone();

            let coord = ((1.0 - t) * a + t * b).into_point();
            new.push(coord);
        }

        Bezier::new(new).compute(t)
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
        Bezier::new(self.control.clone()).compute(t)
    }
}

impl BoundingBox for Bezier {
    fn bounding_box(&self) -> (Point, Point) {
        self.control.bounding_box()
    }
}
