use geonum_common::FromCSV as _;
use triangle_mesh::TriangleMesh;

mod triangle_mesh;

fn main() {
    let mesh = TriangleMesh::from_csv("tp9/data/testsurf.off");

    println!("{:?}", mesh);
}
