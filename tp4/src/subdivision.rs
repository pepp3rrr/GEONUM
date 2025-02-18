use geonum_common::{FromCSV, Point};

#[derive(Clone)]
pub struct SubdivisionCurve {
    pub control: Vec<Point>,
}

impl SubdivisionCurve {
    pub fn compute_chaikin(self, steps: u16) -> Vec<Point> {
        if steps == 0 {
            return self.control;
        }

        let n = self.control.len();
        let mut new = Vec::<Point>::with_capacity(2 * n);

        new.push(
            self.control
                .first()
                .cloned()
                .expect("Control vec should not be empty"),
        );

        self.control.windows(2).for_each(|k| {
            let [xi, xi_1]: [Point; 2] = k.try_into().expect("Should be size 2");

            let x2i = (3. / 4. * xi + 1. / 4. * xi_1).into_point();
            let x2i_1 = (1. / 4. * xi + 3. / 4. * xi_1).into_point();

            new.push(x2i);
            new.push(x2i_1);
        });

        new.push(
            self.control
                .last()
                .cloned()
                .expect("Control vec should not be empty"),
        );

        assert_eq!(new.len(), 2 * n);

        SubdivisionCurve { control: new }.compute_chaikin(steps - 1)
    }
}

impl FromCSV for SubdivisionCurve {
    fn read(mut reader: geonum_common::CSVReader) -> Self {
        let mut control: Vec<_> = reader
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

        // Closed polygon hack
        control.push(control.first().cloned().expect("Should not be empty"));

        Self { control }
    }
}
