use geonum_common::Point;

pub struct TriangleMesh {
    vertex: Vec<Point<3>>,
    indices: Vec<(i32, i32, i32)>,
}
