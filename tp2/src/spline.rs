use geonum_common::*;
use tp1_bezier::Bezier;

pub struct BezierSpline {
    pub curves: Vec<Bezier>,
}

impl From<Vec<Point>> for BezierSpline {
    fn from(value: Vec<Point>) -> Self {
        let mut prev: Option<Point> = None;
        let curves = value
            .windows(2)
            .map(|k| {
                let [start, end]: [Point; 2] = k.try_into().expect("Should be size 2");
                let b_1 = match prev {
                    // b^{i+1}_1 = b^{i+1}_0 - b^i_1 + b^{i+i}_0
                    Some(prev) => (start - prev) + start,
                    // b^0_1 = 0.5(p0+p1)
                    None => start + 0.5 * (end - start),
                };
                prev = Some(b_1);

                Bezier {
                    control: vec![start, b_1, end],
                }
            })
            .collect();
        Self { curves }
    }
}

impl FromCSV for BezierSpline {
    fn read(mut reader: CSVReader) -> Self {
        let control: Vec<Point> = reader
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

        Self::from(control)
    }
}

impl Plot for BezierSpline {
    fn sample(&self, t: f32) -> Point {
        let scaled_t = t * self.curves.len() as f32;
        let curve_index = scaled_t.floor().min(self.curves.len() as f32 - 1.0);

        let curve = self
            .curves
            .get(curve_index as usize)
            .expect("Should be a correct index");

        curve.sample(scaled_t - curve_index)
    }

    fn bounding_box(&self) -> (Point, Point) {
        let sub_bboxes = self
            .curves
            .iter()
            .map(|b| b.bounding_box())
            .collect::<Vec<_>>();

        let right = sub_bboxes
            .iter()
            .map(|b| b.1.x)
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap();
        let top = sub_bboxes
            .iter()
            .map(|b| b.1.y)
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap();
        let left = sub_bboxes
            .iter()
            .map(|b| b.0.x)
            .max_by(|a, b| b.total_cmp(&a))
            .unwrap();
        let bottom = sub_bboxes
            .iter()
            .map(|b| b.0.y)
            .max_by(|a, b| b.total_cmp(&a))
            .unwrap();

        (Point::new(left, bottom), Point::new(right, top))
    }
}
