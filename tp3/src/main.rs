use bspline::BSpline;
use geonum_common::FromCSV;

mod bspline;

fn main() {
    let spline = BSpline::from_csv("tp3/data/simple.bspline");

    println!("{:?}", spline);
}
