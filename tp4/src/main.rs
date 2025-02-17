use geonum_common::FromCSV;
use subdivision::SubdivisionCurve;

mod subdivision;

fn main() {
    let subdivision = SubdivisionCurve::from_csv("tp4/data/simple.data");

    let points = subdivision.compute_chaikin(4);

    println!("{:?}", points);
}
