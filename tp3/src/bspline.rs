use geonum_common::{FromCSV, Point};

#[derive(Debug)]
pub struct BSpline {
    pub degree: usize,
    pub control: Vec<Point>,
    pub knots: Vec<f32>,
}

impl BSpline {
    pub fn evaluate(&self, t: f32) -> Point {
        // Normalize to max knot
        let t = t * self.knots.last().unwrap() * 0.99;

        let k = self.degree;
        let i = (0..(self.knots.len() - 1))
            .find(|i| self.knots[*i] <= t && t < self.knots[i + 1])
            .unwrap();

        let mut d = (0..(k + 1))
            .map(|j| self.control[j + i - k])
            .collect::<Vec<_>>();

        for r in 1..(k + 1) {
            for j in (r..k + 1).rev() {
                let denominator = self.knots[j + 1 + k - r] - self.knots[j + i - k];
                let alpha = if denominator != 0.0 {
                    (t - self.knots[j + i - k]) / denominator
                } else {
                    0.0
                };

                d[j] = ((1.0 - alpha) * d[j - 1] + alpha * d[j]).into_point();
            }
        }

        d[k]
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
