use geonum_common::{FromCSV, Point};

pub struct UniformBezierSpline {
    control: Vec<Point>,
}

impl UniformBezierSpline {
    pub fn compute_two_point(self, steps: u16, degree: u16) -> Vec<Point> {
        if steps == 0 {
            return self.control;
        }

        let n = self.control.len();
        let mut new = Vec::<Point>::with_capacity(2 * n);

        // Double the points (refining)
        for point in self.control.into_iter() {
            new.push(point.clone());
            new.push(point);
        }

        for _d in 0..degree {
            let first = new.first().cloned().expect("Should not be empty");
            for i in 0..(2 * n) {
                let v_dm1_i = new[i];
                let v_dm1_ip1 = new.get(i + 1).unwrap_or(&first).clone();

                let v_d_i = (1. / 2. * (v_dm1_i + v_dm1_ip1)).into_point();
                new[i] = v_d_i;
            }
        }

        assert_eq!(new.len(), 2 * n);

        UniformBezierSpline { control: new }.compute_two_point(steps - 1, degree)
    }
}

impl FromCSV for UniformBezierSpline {
    fn read(reader: geonum_common::CSVReader) -> Self {
        Self {
            control: Vec::<Point>::read(reader),
        }
    }
}
