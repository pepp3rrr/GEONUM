use geonum_common::{FromCSV, Point};

#[derive(Debug)]
pub struct BSpline {
    pub degree: usize,
    pub control: Vec<Point>,
    pub knots: Vec<f32>,
}

impl BSpline {
    pub fn sample(&self, density: usize) -> Vec<Point> {
        let mut out = Vec::new();

        for j in self.degree..(self.knots.len() - self.degree) {
            // Skip identical
            if self.knots[j] == self.knots[j + 1] {
                continue;
            }

            let mut new = (0..density)
                .map(|n| {
                    let k = self.knots[j] + (n as f32 / density as f32) * self.knots[j + 1];
                    self.compute_worker(self.degree, j, k)
                })
                .collect::<Vec<_>>();

            out.append(&mut new);
        }

        out
    }

    fn compute_worker(&self, r: usize, j: usize, t: f32) -> Point {
        if r == 0 {
            return self.control[j];
        }

        let d = self.knots[j + self.degree - (r - 1)] - self.knots[j];
        let w = if d == 0.0 {
            0.0
        } else {
            (t - self.knots[j]) / d
        };

        let d_jm1 = self.compute_worker(r - 1, j - 1, t);
        let d_j = self.compute_worker(r - 1, j, t);

        ((1.0 - w) * d_jm1 + w * d_j).into_point()
    }
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

        let n = control.len() - 1;
        let m = knots.len() - 1;
        let degree = m - n - 1;

        Self {
            control,
            knots,
            degree,
        }
    }
}
