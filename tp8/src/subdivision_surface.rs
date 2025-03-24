use geonum_common::{FromCSV, Point};

pub struct SubdivisionSurface {
    pub control: Vec<Vec<Point<3>>>,
    pub closed_x: bool,
    pub closed_y: bool,
}

impl SubdivisionSurface {
    pub fn compute(&self, steps: u16) -> Vec<Vec<Point<3>>> {
        // Subdivide the X axis (lines)
        let curves_x = self
            .control
            .iter()
            .map(|points| {
                tp4_subdivision::SubdivisionCurve::<3> {
                    control: points.clone(),
                }
                .compute(tp4_subdivision::ComputeMethod::Chaikin, steps)
            })
            .collect::<Vec<_>>();

        let mut curves_y = Vec::new();
        let x_dim = curves_x.first().unwrap().len();
        for x in 0..x_dim {
            let control = curves_x.iter().map(|line| line[x]).collect();
            curves_y.push(tp4_subdivision::SubdivisionCurve::<3> { control });
        }

        curves_y
            .into_iter()
            .map(|column| column.compute(tp4_subdivision::ComputeMethod::Chaikin, steps))
            .collect()
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
