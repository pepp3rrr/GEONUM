use geonum_common::{FromCSV, Point};

#[derive(Clone)]
pub struct UniformBezierSpline {
    pub control: Vec<Point>,
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

        // Average multiple times (smoothing)
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

    pub fn compute_four_point(self, steps: u16, degree: u16) -> Vec<Point> {
        if steps == 0 {
            return self.control;
        }

        let n = self.control.len();
        let s = 2 * n;
        let mut new = Vec::<Point>::with_capacity(s);

        // Refine
        for i in 0..n {
            let p_im1 = self.control[(i + n - 1) % n];
            let p_i = self.control[i];
            let p_ip1 = self.control[(i + 1) % n];
            let p_ip2 = self.control[(i + 2) % n];

            let v_2i_p1 =
                (1. / 16. * (-1. * p_im1 + 9. * p_i + 9. * p_ip1 - 1. * p_ip2)).into_point();
            let v_2i = p_i;

            new.push(v_2i);
            new.push(v_2i_p1);
        }

        assert_eq!(new.len(), s);

        // Smooth
        for _d in 0..degree {
            let mut new_smoothed = Vec::<Point>::with_capacity(s);

            for i in 0..(2 * n) {
                let v_im1 = new[(i + s - 1) % s];
                let v_i = new[i];
                let v_ip1 = new[(i + 1) % s];
                let v_ip2 = new[(i + 2) % s];

                let v_d_i =
                    (1. / 16. * (-1. * v_im1 + 9. * v_i + 9. * v_ip1 - 1. * v_ip2)).into_point();

                new_smoothed.push(v_d_i);
            }

            new = new_smoothed;
        }

        UniformBezierSpline { control: new }.compute_four_point(steps - 1, degree)
    }
}

impl FromCSV for UniformBezierSpline {
    fn read(reader: geonum_common::CSVReader) -> Self {
        Self {
            control: Vec::<Point>::read(reader),
        }
    }
}
