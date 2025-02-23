use geonum_common::{BoundingBox, FromCSV, Point};

#[derive(Clone)]
pub struct SubdivisionCurve {
    pub control: Vec<Point>,
}

pub enum ComputeMethod {
    Chaikin,
    CornerCutting { a: f32, b: f32 },
    FourPoint { w: f32 },
}

impl SubdivisionCurve {
    pub fn compute(&self, method: ComputeMethod, steps: u16) -> Vec<Point> {
        let worker = self.clone();

        match method {
            ComputeMethod::Chaikin => worker.compute_chaikin(steps),
            ComputeMethod::CornerCutting { a, b } => worker.compute_corner_cutting(steps, a, b),
            ComputeMethod::FourPoint { w } => {
                let mut out = worker.compute_four_point(steps, w);
                // Closed polygon hack
                out.push(out.first().cloned().unwrap());
                out
            }
        }
    }

    fn compute_chaikin(self, steps: u16) -> Vec<Point> {
        self.compute_corner_cutting(steps, 0.25, 0.75)
    }

    fn compute_corner_cutting(self, steps: u16, a: f32, b: f32) -> Vec<Point> {
        assert!(0.0 < a && a < b && b < 1.0);

        if steps == 0 {
            return self.control;
        }

        let n = self.control.len();
        let mut new = Vec::<Point>::with_capacity(2 * n);

        for i in 0..n {
            let xi = self.control[i];
            let xi_p1 = self.control[(i + 1) % n];

            let x2i = ((1. - a) * xi + a * xi_p1).into_point();
            let x2i_1 = ((1. - b) * xi + b * xi_p1).into_point();

            new.push(x2i);
            new.push(x2i_1);
        }

        assert_eq!(new.len(), 2 * n);

        SubdivisionCurve { control: new }.compute_corner_cutting(steps - 1, a, b)
    }

    fn compute_four_point(self, steps: u16, w: f32) -> Vec<Point> {
        if steps == 0 {
            return self.control;
        }

        let n = self.control.len();
        let mut new = Vec::<Point>::with_capacity(2 * n);

        for i in 0..n {
            let xi_m1 = if i != 0 {
                self.control[i - 1]
            } else {
                self.control[n - 1]
            };
            let xi = self.control[i];
            let xi_p1 = self.control[(i + 1) % n];
            let xi_p2 = self.control[(i + 2) % n];

            let x2i = xi;
            let x2i_1 =
                (-w * xi_m1 + (1. / 2. + w) * xi + (1. / 2. + w) * xi_p1 - w * xi_p2).into_point();

            new.push(x2i);
            new.push(x2i_1);
        }

        assert_eq!(new.len(), 2 * n);

        SubdivisionCurve { control: new }.compute_four_point(steps - 1, w)
    }
}

impl FromCSV for SubdivisionCurve {
    fn read(mut reader: geonum_common::CSVReader) -> Self {
        let control: Vec<_> = reader
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

impl BoundingBox for SubdivisionCurve {
    fn bounding_box(&self) -> (Point, Point) {
        self.control.bounding_box()
    }
}
