use geonum_common::{FromCSV, Point};

pub struct SubdivisionSurface {
    pub control: Vec<Vec<Point<3>>>,
}

impl FromCSV for SubdivisionSurface {
    fn read(mut reader: geonum_common::CSVReader) -> Self {
        assert!(reader.has_headers());
        let header = reader.headers().unwrap();
        let degree_x: usize = header.get(0).unwrap().parse().unwrap();
        let degree_y: usize = header.get(1).unwrap().parse().unwrap();

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
        }
    }
}
