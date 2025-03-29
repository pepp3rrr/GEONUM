use geonum_common::{FromCSV, Point};

#[derive(Clone)]
pub struct SubdivisionSurface {
    pub control: Vec<Vec<Point<3>>>,
    pub closed_x: bool,
    pub closed_y: bool,
}

impl SubdivisionSurface {
    pub fn compute(&self, steps: u16) -> Vec<Vec<Point<3>>> {
        self.clone().compute_worker(steps)
    }

    fn compute_worker(self, steps: u16) -> Vec<Vec<Point<3>>> {
        if steps == 0 {
            return self.control;
        }

        let h = self.control.len();

        // Duplication
        let mut new = Vec::with_capacity(h * 2);
        for line in self.control.iter() {
            let mut new_row = Vec::with_capacity(2 * line.len());
            for point in line {
                // Cloning point data is kinda useless
                new_row.push(point.clone());
                new_row.push(point.clone());
            }
            new.push(new_row.clone());
            new.push(new_row);
        }

        for (i, line) in self.control.iter().enumerate() {
            let w = line.len();

            for j in 0..w {
                if self.closed_y && i + 1 == h || self.closed_x && j + 1 == w {
                    continue;
                }

                let v_i_j = self.control[i][j];
                let v_ip1_j = self.control[(i + 1) % h][j];
                let v_i_jp1 = self.control[i][(j + 1) % w];
                let v_ip1_jp1 = self.control[(i + 1) % h][(j + 1) % w];

                let v_2i_2j =
                    1.0 / 16.0 * (9.0 * v_i_j + 3.0 * v_ip1_j + 3.0 * v_i_jp1 + 1.0 * v_ip1_jp1);
                let v_2ip1_2j =
                    1.0 / 16.0 * (3.0 * v_i_j + 9.0 * v_ip1_j + 1.0 * v_i_jp1 + 3.0 * v_ip1_jp1);
                let v_2i_2jp1 =
                    1.0 / 16.0 * (3.0 * v_i_j + 1.0 * v_ip1_j + 9.0 * v_i_jp1 + 3.0 * v_ip1_jp1);
                let v_2ip1_2jp1 =
                    1.0 / 16.0 * (1.0 * v_i_j + 3.0 * v_ip1_j + 3.0 * v_i_jp1 + 9.0 * v_ip1_jp1);

                new[2 * i][2 * j] = v_2i_2j.into_point();
                new[2 * i + 1][2 * j] = v_2ip1_2j.into_point();
                new[2 * i][2 * j + 1] = v_2i_2jp1.into_point();
                new[2 * i + 1][2 * j + 1] = v_2ip1_2jp1.into_point();
            }
        }

        SubdivisionSurface {
            control: new,
            closed_x: self.closed_x,
            closed_y: self.closed_y,
        }
        .compute_worker(steps - 1)
    }
}

impl FromCSV for SubdivisionSurface {
    fn read(mut reader: geonum_common::CSVReader) -> Self {
        assert!(reader.has_headers());
        let header = reader.headers().unwrap();
        let degree_x: usize = header.get(0).unwrap().parse().unwrap();
        let degree_y: usize = header.get(1).unwrap().parse().unwrap();
        let closed_x: bool = header.get(2).unwrap().parse::<u8>().unwrap() == 1;
        let closed_y: bool = header.get(3).unwrap().parse::<u8>().unwrap() == 1;

        let mut control_net = Vec::with_capacity(degree_y);

        let mut iter = reader.records();
        for _y in 0..(degree_y) {
            let mut line: Vec<Point<3>> = Vec::with_capacity(degree_x);
            for _x in 0..(degree_x) {
                let point_line = iter.next().unwrap().unwrap();
                let x: f32 = point_line.get(0).unwrap().parse().unwrap();
                let y: f32 = point_line.get(1).unwrap().parse().unwrap();
                let z: f32 = point_line.get(2).unwrap().parse().unwrap();

                line.push(Point::<3>::new(x, y, z));
            }

            control_net.push(line);
        }

        Self {
            control: control_net,
            closed_x,
            closed_y,
        }
    }
}
