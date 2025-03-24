use geonum_common::FromCSV;
use subdivision_surface::SubdivisionSurface;

mod subdivision_surface;

fn main() {
    let surface = SubdivisionSurface::from_csv("tp8/data/grid.net");

    println!("{:?}", surface.control);
}
