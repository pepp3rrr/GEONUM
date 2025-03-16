use geonum_common::{FromCSV, Point};

pub struct BezierSurface {
    pub control: Vec<Vec<Point<3>>>,
}

pub struct PiecewiseBezierSurface {
    pub patches: Vec<BezierSurface>,
}

impl BezierSurface {
    pub fn evalutate(&self, u: f32, v: f32) -> Point<3> {
        // Curves along the X (i on the figure) axis
        let curves_x = self.control.iter().map(|points| {
            // Y coord is constant
            tp1_bezier::Bezier::new(points.clone())
        });

        // Computed new control points along the Y axis (j)
        let control_y = curves_x
            .map(|bezier_curve| bezier_curve.compute(u))
            .collect::<Vec<_>>();

        let curve_y = tp1_bezier::Bezier::new(control_y);

        curve_y.compute(v)
    }
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
