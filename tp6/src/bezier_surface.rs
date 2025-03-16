use geonum_common::{FromCSV, Point};

pub struct BezierSurface {
    pub control: Vec<Vec<Point<3>>>,
}

pub struct PiecewiseBezierSurface {
    pub patches: Vec<BezierSurface>,
}

impl FromCSV for PiecewiseBezierSurface {
    fn read(mut reader: geonum_common::CSVReader) -> Self {
        let mut patches = Vec::new();

        let mut iter = reader.records();
        while let Some(degree_line) = iter.next() {
            let degree_line = degree_line.unwrap();
            assert_eq!(degree_line.len(), 2);
            let degree_x: i32 = degree_line.get(0).unwrap().parse().unwrap();
            let degree_y: i32 = degree_line.get(1).unwrap().parse().unwrap();

            let mut control_net: Vec<Vec<Point<3>>> = Vec::new();

            for _x in 0..(degree_x + 1) {
                let mut line: Vec<Point<3>> = Vec::new();
                for _y in 0..(degree_y + 1) {
                    let point_line = iter.next().unwrap().unwrap();
                    let x: f32 = point_line.get(0).unwrap().parse().unwrap();
                    let y: f32 = point_line.get(1).unwrap().parse().unwrap();
                    let z: f32 = point_line.get(2).unwrap().parse().unwrap();

                    line.push(Point::<3>::new(x, y, z));
                }

                control_net.push(line);
            }

            patches.push(BezierSurface {
                control: control_net,
            });
        }
        Self { patches }
    }
}
