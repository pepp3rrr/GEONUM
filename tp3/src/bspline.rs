use geonum_common::{FromCSV, Point};

#[derive(Debug)]
pub struct BSpline {
    pub degree: usize,
    pub control: Vec<Point>,
    pub knots: Vec<f32>,
}

impl BSpline {
    pub fn evaluate(&self, density: usize) -> Vec<Vec<Point>> {
        let mut out = Vec::new();

        for j in self.degree..(self.knots.len() - self.degree) {
            if self.knots[j] == self.knots[j + 1] {
                continue;
            }

            let t_values = linspace(self.knots[j], self.knots[j + 1], density);

            let mut segment = Vec::with_capacity(t_values.len());
            for t in t_values {
                segment.push(self.de_boor(self.degree, j, t));
            }

            out.push(segment);
        }

        out
    }

    fn de_boor(&self, r: usize, j: usize, t: f32) -> Point {
        if r == 0 {
            return self.control[j];
        }

        let denominator = self.knots[j + self.degree - (r - 1)] - self.knots[j];
        let omega = if denominator != 0.0 {
            (t - self.knots[j]) / denominator
        } else {
            0.0
        };

        let dj = self.de_boor(r - 1, j - 1, t);
        let dj_m1 = self.de_boor(r - 1, j, t);

        ((1.0 - omega) * dj + omega * dj_m1).into_point()
    }
}

// https://stackoverflow.com/a/70840233
pub fn linspace(x0: f32, xend: f32, n: usize) -> Vec<f32> {
    let to_float = |i: usize| i as f32;
    let dx = (xend - x0) / to_float(n - 1);
    (0..n).map(|i| x0 + to_float(i) * dx).collect()
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
