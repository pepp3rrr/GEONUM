use geonum_common::FromCSV;
use uniform_spline::UniformBezierSpline;

mod uniform_spline;

fn main() {
    let spline = UniformBezierSpline::from_csv("tp5/data/bone.data");
    let points = spline.compute_two_point(8, 4);

    println!("{:?}", points);
}
